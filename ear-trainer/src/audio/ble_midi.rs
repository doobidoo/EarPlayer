use super::backend::AudioBackend;
use anyhow::{anyhow, Result};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;
use uuid::Uuid;

// BLE MIDI Service and Characteristic UUIDs
#[allow(dead_code)]
const BLE_MIDI_SERVICE_UUID: Uuid = Uuid::from_u128(0x03b80e5a_ede8_4b33_a751_6ce34ec4c700);
const BLE_MIDI_CHAR_UUID: Uuid = Uuid::from_u128(0x7772e5db_3868_4112_a1a9_f2669d106bf3);

#[derive(Debug, Clone)]
pub struct BleDeviceInfo {
    pub id: String,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BleConnectionState {
    Disconnected,
    Scanning,
    Connecting,
    Connected,
    Reconnecting(u32),
}

#[derive(Debug, Clone)]
pub enum BleEvent {
    StateChanged(BleConnectionState),
    DeviceDiscovered(BleDeviceInfo),
    Connected(String),
    Disconnected,
    Error(String),
    /// Prerequisite check results - (check_name, passed, guidance_message)
    PrerequisiteCheck(String, bool, String),
}

/// Result of checking BLE prerequisites
#[derive(Debug, Clone)]
pub struct BlePrerequisites {
    pub bluetooth_installed: bool,
    pub bluetooth_service_running: bool,
    pub adapter_available: bool,
    pub adapter_powered: bool,
    pub issues: Vec<String>,
    pub guidance: Vec<String>,
}

impl BlePrerequisites {
    pub fn all_passed(&self) -> bool {
        self.bluetooth_installed
            && self.bluetooth_service_running
            && self.adapter_available
            && self.adapter_powered
    }
}

/// Check system prerequisites for BLE MIDI
pub fn check_ble_prerequisites() -> BlePrerequisites {
    let mut prereqs = BlePrerequisites {
        bluetooth_installed: false,
        bluetooth_service_running: false,
        adapter_available: false,
        adapter_powered: false,
        issues: Vec::new(),
        guidance: Vec::new(),
    };

    // Check if bluetoothctl is installed
    if Command::new("which")
        .arg("bluetoothctl")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        prereqs.bluetooth_installed = true;
    } else {
        prereqs.issues.push("BlueZ not installed".to_string());
        prereqs.guidance.push(
            "Install bluez: sudo pacman -S bluez bluez-utils (Arch) or sudo apt install bluez (Debian)"
                .to_string(),
        );
    }

    // Check if bluetooth service is running
    if let Ok(output) = Command::new("systemctl")
        .args(["is-active", "bluetooth"])
        .output()
    {
        if String::from_utf8_lossy(&output.stdout).trim() == "active" {
            prereqs.bluetooth_service_running = true;
        } else {
            prereqs.issues.push("Bluetooth service not running".to_string());
            prereqs.guidance.push(
                "Start bluetooth: sudo systemctl enable --now bluetooth".to_string(),
            );
        }
    }

    // Check for Bluetooth adapters and their power state
    if let Ok(output) = Command::new("bluetoothctl").arg("list").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("Controller") {
            prereqs.adapter_available = true;

            // Check if any adapter is powered on
            if let Ok(show_output) = Command::new("bluetoothctl").arg("show").output() {
                let show_str = String::from_utf8_lossy(&show_output.stdout);
                if show_str.contains("Powered: yes") {
                    prereqs.adapter_powered = true;
                } else {
                    prereqs.issues.push("Bluetooth adapter not powered on".to_string());
                    prereqs.guidance.push(
                        "Power on adapter: bluetoothctl power on".to_string(),
                    );
                }
            }
        } else {
            prereqs.issues.push("No Bluetooth adapter found".to_string());
            prereqs.guidance.push(
                "Connect a USB Bluetooth adapter or enable built-in Bluetooth".to_string(),
            );
        }
    }

    // Check for multiple adapters and suggest which one to use
    if prereqs.adapter_available {
        if let Ok(output) = Command::new("bluetoothctl").arg("list").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let adapter_count = output_str.matches("Controller").count();
            if adapter_count > 1 {
                prereqs.guidance.push(format!(
                    "Multiple adapters detected ({}). USB BLE dongle preferred for better BLE support.",
                    adapter_count
                ));
            }
        }
    }

    prereqs
}

pub struct BleMidiBackend {
    runtime: Arc<Runtime>,
    adapter: Arc<TokioMutex<Option<Adapter>>>,
    peripheral: Arc<TokioMutex<Option<Peripheral>>>,
    state: Arc<TokioMutex<BleConnectionState>>,
    event_tx: mpsc::UnboundedSender<BleEvent>,
    event_rx: Arc<TokioMutex<mpsc::UnboundedReceiver<BleEvent>>>,
    should_reconnect: Arc<AtomicBool>,
    connected_device_name: Arc<TokioMutex<Option<String>>>,
    preferred_device_address: Arc<TokioMutex<Option<String>>>,
}

impl BleMidiBackend {
    pub fn new() -> Result<Self> {
        let runtime = Runtime::new()?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let backend = Self {
            runtime: Arc::new(runtime),
            adapter: Arc::new(TokioMutex::new(None)),
            peripheral: Arc::new(TokioMutex::new(None)),
            state: Arc::new(TokioMutex::new(BleConnectionState::Disconnected)),
            event_tx,
            event_rx: Arc::new(TokioMutex::new(event_rx)),
            should_reconnect: Arc::new(AtomicBool::new(true)),
            connected_device_name: Arc::new(TokioMutex::new(None)),
            preferred_device_address: Arc::new(TokioMutex::new(None)),
        };

        // Initialize adapter
        backend.init_adapter()?;

        Ok(backend)
    }

    /// Create a dummy backend that doesn't do anything (for fallback when BLE is unavailable)
    pub fn new_dummy() -> Self {
        let runtime = Runtime::new().expect("Tokio runtime required");
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            runtime: Arc::new(runtime),
            adapter: Arc::new(TokioMutex::new(None)),
            peripheral: Arc::new(TokioMutex::new(None)),
            state: Arc::new(TokioMutex::new(BleConnectionState::Disconnected)),
            event_tx,
            event_rx: Arc::new(TokioMutex::new(event_rx)),
            should_reconnect: Arc::new(AtomicBool::new(false)),
            connected_device_name: Arc::new(TokioMutex::new(None)),
            preferred_device_address: Arc::new(TokioMutex::new(None)),
        }
    }

    fn init_adapter(&self) -> Result<()> {
        let adapter = self.adapter.clone();
        let event_tx = self.event_tx.clone();

        self.runtime.block_on(async {
            let manager = Manager::new().await?;
            let adapters = manager.adapters().await?;

            // Prefer USB BLE adapter (60:F8:1D:CA:DF:95) over internal MacBook adapter
            // The internal adapter often doesn't support BLE properly
            let mut selected_adapter = None;
            for adapter in adapters {
                if let Ok(info) = adapter.adapter_info().await {
                    let info_str = format!("{:?}", info);
                    // Check if this is the USB BLE dongle (preferred)
                    if info_str.contains("60:F8:1D") || info_str.contains("hci1") {
                        selected_adapter = Some(adapter);
                        break;
                    }
                    // Keep first adapter as fallback
                    if selected_adapter.is_none() {
                        selected_adapter = Some(adapter);
                    }
                }
            }

            if let Some(central) = selected_adapter {
                *adapter.lock().await = Some(central);
                Ok(())
            } else {
                let _ = event_tx.send(BleEvent::Error("No Bluetooth adapter found".to_string()));
                Err(anyhow!("No Bluetooth adapter found"))
            }
        })
    }

    pub fn set_preferred_device(&self, address: Option<String>) {
        let preferred = self.preferred_device_address.clone();
        self.runtime.block_on(async {
            *preferred.lock().await = address;
        });
    }

    /// Check prerequisites before scanning
    pub fn check_prerequisites(&self) -> BlePrerequisites {
        check_ble_prerequisites()
    }

    pub fn start_scan(&self) {
        // First check prerequisites and report any issues
        let prereqs = check_ble_prerequisites();

        // Send prerequisite check events
        let event_tx = self.event_tx.clone();
        let _ = event_tx.send(BleEvent::PrerequisiteCheck(
            "BlueZ installed".to_string(),
            prereqs.bluetooth_installed,
            if prereqs.bluetooth_installed {
                "OK".to_string()
            } else {
                "Install: sudo pacman -S bluez bluez-utils".to_string()
            },
        ));
        let _ = event_tx.send(BleEvent::PrerequisiteCheck(
            "Bluetooth service".to_string(),
            prereqs.bluetooth_service_running,
            if prereqs.bluetooth_service_running {
                "Running".to_string()
            } else {
                "Run: sudo systemctl enable --now bluetooth".to_string()
            },
        ));
        let _ = event_tx.send(BleEvent::PrerequisiteCheck(
            "Bluetooth adapter".to_string(),
            prereqs.adapter_available,
            if prereqs.adapter_available {
                "Found".to_string()
            } else {
                "Connect USB Bluetooth adapter".to_string()
            },
        ));
        let _ = event_tx.send(BleEvent::PrerequisiteCheck(
            "Adapter powered".to_string(),
            prereqs.adapter_powered,
            if prereqs.adapter_powered {
                "On".to_string()
            } else {
                "Run: bluetoothctl power on".to_string()
            },
        ));

        // If prerequisites aren't met, report the issues and don't scan
        if !prereqs.all_passed() {
            for issue in &prereqs.issues {
                let _ = event_tx.send(BleEvent::Error(issue.clone()));
            }
            for guidance in &prereqs.guidance {
                let _ = event_tx.send(BleEvent::Error(format!("Fix: {}", guidance)));
            }
            return;
        }

        let adapter = self.adapter.clone();
        let peripheral = self.peripheral.clone();
        let state = self.state.clone();
        let connected_device_name = self.connected_device_name.clone();
        let preferred_device_address = self.preferred_device_address.clone();

        self.runtime.spawn(async move {
            *state.lock().await = BleConnectionState::Scanning;
            let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Scanning));

            let preferred = preferred_device_address.lock().await.clone();

            // Start scanning - get adapter reference for starting scan
            {
                let adapter_guard = adapter.lock().await;
                let Some(central) = adapter_guard.as_ref() else {
                    let _ = event_tx.send(BleEvent::Error("No adapter available".to_string()));
                    return;
                };

                // Don't filter by service UUID - many BLE MIDI devices don't advertise
                // the MIDI service until after connection. We'll filter by name instead.
                let filter = ScanFilter::default();

                if let Err(e) = central.start_scan(filter).await {
                    let _ = event_tx.send(BleEvent::Error(format!("Scan failed: {}", e)));
                    return;
                }
            }

            // Scan for devices with timeout
            let scan_duration = Duration::from_secs(10);
            let scan_start = std::time::Instant::now();

            while scan_start.elapsed() < scan_duration {
                tokio::time::sleep(Duration::from_millis(500)).await;

                // Collect peripherals while holding the lock, then release it
                let peripherals = {
                    let adapter_guard = adapter.lock().await;
                    let Some(central) = adapter_guard.as_ref() else {
                        continue;
                    };
                    central.peripherals().await.ok()
                };

                let Some(peripherals) = peripherals else {
                    continue;
                };

                for p in peripherals {
                    let props = p.properties().await.ok().flatten();
                    let name = props
                        .as_ref()
                        .and_then(|p| p.local_name.clone())
                        .unwrap_or_else(|| "Unknown".to_string());
                    let address = props
                        .as_ref()
                        .map(|p| p.address.to_string())
                        .unwrap_or_default();

                    // Skip if no name (not a real device)
                    if name == "Unknown" && address.is_empty() {
                        continue;
                    }

                    // Check if this looks like a MIDI device by name
                    let name_upper = name.to_uppercase();
                    let is_midi_device = name_upper.contains("MIDI")
                        || name_upper.contains("MD-BT")
                        || name_upper.contains("YAMAHA")
                        || name_upper.contains("ROLAND")
                        || name_upper.contains("KORG")
                        || name_upper.contains("CME")
                        || name_upper.contains("WIDI");

                    // Only report MIDI-looking devices
                    if is_midi_device {
                        let device_info = BleDeviceInfo {
                            id: address.clone(),
                            name: name.clone(),
                            address: address.clone(),
                        };
                        let _ = event_tx.send(BleEvent::DeviceDiscovered(device_info));
                    }

                    // Auto-connect if this is preferred device or a MIDI device
                    let should_connect = preferred
                        .as_ref()
                        .map(|pref| address.contains(pref) || name.contains(pref))
                        .unwrap_or(is_midi_device);

                    if should_connect {
                        *state.lock().await = BleConnectionState::Connecting;
                        let _ = event_tx
                            .send(BleEvent::StateChanged(BleConnectionState::Connecting));

                        if Self::connect_to_peripheral(&p, &event_tx).await.is_ok() {
                            *peripheral.lock().await = Some(p);
                            *connected_device_name.lock().await = Some(name.clone());
                            *state.lock().await = BleConnectionState::Connected;
                            let _ = event_tx.send(BleEvent::Connected(name));
                            let _ = event_tx
                                .send(BleEvent::StateChanged(BleConnectionState::Connected));

                            // Stop scanning
                            if let Some(central) = adapter.lock().await.as_ref() {
                                let _ = central.stop_scan().await;
                            }
                            return;
                        }
                    }
                }
            }

            // Stop scanning
            if let Some(central) = adapter.lock().await.as_ref() {
                let _ = central.stop_scan().await;
            }

            // If not connected, set state back to disconnected
            if *state.lock().await != BleConnectionState::Connected {
                *state.lock().await = BleConnectionState::Disconnected;
                let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Disconnected));
            }
        });
    }

    async fn connect_to_peripheral(
        peripheral: &Peripheral,
        event_tx: &mpsc::UnboundedSender<BleEvent>,
    ) -> Result<()> {
        // Connect
        peripheral.connect().await?;

        // Discover services
        peripheral.discover_services().await?;

        // Find MIDI characteristic
        let chars = peripheral.characteristics();
        let midi_char = chars
            .iter()
            .find(|c| c.uuid == BLE_MIDI_CHAR_UUID)
            .ok_or_else(|| anyhow!("MIDI characteristic not found"))?;

        // Subscribe to notifications (for receiving MIDI)
        if let Err(e) = peripheral.subscribe(midi_char).await {
            let _ = event_tx.send(BleEvent::Error(format!("Subscribe failed: {}", e)));
        }

        Ok(())
    }

    pub fn start_reconnect(&self) {
        if !self.should_reconnect.load(Ordering::SeqCst) {
            return;
        }

        let adapter = self.adapter.clone();
        let peripheral = self.peripheral.clone();
        let state = self.state.clone();
        let event_tx = self.event_tx.clone();
        let connected_device_name = self.connected_device_name.clone();
        let should_reconnect = self.should_reconnect.clone();

        self.runtime.spawn(async move {
            let max_attempts = 5;
            let mut attempt = 0;

            while attempt < max_attempts && should_reconnect.load(Ordering::SeqCst) {
                attempt += 1;
                *state.lock().await = BleConnectionState::Reconnecting(attempt);
                let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Reconnecting(
                    attempt,
                )));

                // Exponential backoff: 1s, 2s, 4s, 8s, 16s
                let delay = Duration::from_secs(1 << (attempt - 1));
                tokio::time::sleep(delay).await;

                // Try to reconnect to the last peripheral
                if let Some(p) = peripheral.lock().await.as_ref() {
                    if Self::connect_to_peripheral(p, &event_tx).await.is_ok() {
                        *state.lock().await = BleConnectionState::Connected;
                        if let Some(name) = connected_device_name.lock().await.clone() {
                            let _ = event_tx.send(BleEvent::Connected(name));
                        }
                        let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Connected));
                        return;
                    }
                }

                // If peripheral is gone, try scanning again
                let adapter_guard = adapter.lock().await;
                if let Some(central) = adapter_guard.as_ref() {
                    let filter = ScanFilter {
                        services: vec![BLE_MIDI_SERVICE_UUID],
                    };

                    if central.start_scan(filter).await.is_ok() {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        let _ = central.stop_scan().await;

                        // Check if we found our device
                        if let Ok(peripherals) = central.peripherals().await {
                            for p in peripherals {
                                if let Ok(Some(props)) = p.properties().await {
                                    if Self::connect_to_peripheral(&p, &event_tx).await.is_ok() {
                                        let name = props
                                            .local_name
                                            .clone()
                                            .unwrap_or_else(|| "Unknown".to_string());
                                        *peripheral.lock().await = Some(p);
                                        *connected_device_name.lock().await = Some(name.clone());
                                        *state.lock().await = BleConnectionState::Connected;
                                        let _ = event_tx.send(BleEvent::Connected(name));
                                        let _ = event_tx
                                            .send(BleEvent::StateChanged(BleConnectionState::Connected));
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Failed to reconnect
            *state.lock().await = BleConnectionState::Disconnected;
            let _ = event_tx.send(BleEvent::Disconnected);
            let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Disconnected));
            let _ = event_tx.send(BleEvent::Error("Reconnection failed after 5 attempts".to_string()));
        });
    }

    fn send_midi(&self, data: &[u8]) -> Result<()> {
        let peripheral = self.peripheral.clone();
        let event_tx = self.event_tx.clone();
        let state = self.state.clone();

        // Create BLE MIDI packet
        // BLE MIDI format: [header, timestamp_high, timestamp_low, status, data...]
        // For simplicity, we'll use a minimal timestamp
        let timestamp_high = 0x80; // Header + high bit of timestamp
        let timestamp_low = 0x80; // Low 7 bits of timestamp

        let mut packet = vec![timestamp_high, timestamp_low];
        packet.extend_from_slice(data);

        self.runtime.block_on(async {
            let peripheral_guard = peripheral.lock().await;
            let Some(p) = peripheral_guard.as_ref() else {
                return Err(anyhow!("No peripheral connected"));
            };

            // Check if still connected
            if !p.is_connected().await.unwrap_or(false) {
                drop(peripheral_guard);
                *state.lock().await = BleConnectionState::Disconnected;
                let _ = event_tx.send(BleEvent::Disconnected);
                let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Disconnected));
                return Err(anyhow!("Peripheral disconnected"));
            }

            // Find MIDI characteristic
            let chars = p.characteristics();
            let midi_char = chars
                .iter()
                .find(|c| c.uuid == BLE_MIDI_CHAR_UUID)
                .ok_or_else(|| anyhow!("MIDI characteristic not found"))?;

            // Write MIDI data
            p.write(midi_char, &packet, WriteType::WithoutResponse)
                .await?;

            Ok(())
        })
    }

    pub fn get_state(&self) -> BleConnectionState {
        self.runtime
            .block_on(async { self.state.lock().await.clone() })
    }

    pub fn get_connected_device_name(&self) -> Option<String> {
        self.runtime
            .block_on(async { self.connected_device_name.lock().await.clone() })
    }

    pub fn poll_events(&self) -> Vec<BleEvent> {
        let event_rx = self.event_rx.clone();
        self.runtime.block_on(async {
            let mut events = Vec::new();
            let mut rx = event_rx.lock().await;
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
            events
        })
    }

    pub fn disconnect(&self) {
        self.should_reconnect.store(false, Ordering::SeqCst);
        let peripheral = self.peripheral.clone();
        let state = self.state.clone();
        let event_tx = self.event_tx.clone();

        self.runtime.block_on(async {
            if let Some(p) = peripheral.lock().await.as_ref() {
                let _ = p.disconnect().await;
            }
            *state.lock().await = BleConnectionState::Disconnected;
            let _ = event_tx.send(BleEvent::StateChanged(BleConnectionState::Disconnected));
        });
    }

    pub fn is_connected(&self) -> bool {
        self.get_state() == BleConnectionState::Connected
    }
}

impl AudioBackend for BleMidiBackend {
    fn play_note(&mut self, note: u8, velocity: u8) -> Result<()> {
        if !self.is_connected() {
            return Ok(());
        }
        // Note On: 0x90 = channel 1
        let data = [0x90, note, velocity];
        self.send_midi(&data)
    }

    fn stop_note(&mut self, note: u8) -> Result<()> {
        if !self.is_connected() {
            return Ok(());
        }
        // Note Off: 0x80 = channel 1
        let data = [0x80, note, 0];
        self.send_midi(&data)
    }

    fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()> {
        for &note in notes {
            self.play_note(note, velocity)?;
        }
        Ok(())
    }

    fn stop_all(&mut self) -> Result<()> {
        if !self.is_connected() {
            return Ok(());
        }
        // All Notes Off on channel 1
        let data = [0xB0, 123, 0];
        self.send_midi(&data)
    }

    fn name(&self) -> &'static str {
        "BLE MIDI"
    }
}

impl Default for BleMidiBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create BLE MIDI backend")
    }
}
