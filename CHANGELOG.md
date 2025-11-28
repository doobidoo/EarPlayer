# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2025-11-28

### Added
- feat: add BLE MIDI support, improve synth audio, update docs
- feat: implement terminal-based music ear training application

### Changed
- docs: add trigger-release script and update contributing guide
- chore: remove Claude API dependency from release workflow
- chore: add semantic versioning and release automation

### Fixed
- fix: correct rust-toolchain action name in release workflow
- fix: synth backend audio and add Bluetooth MIDI docs

## [0.2.0] - 2024-11-28

### Added
- BLE MIDI support with automatic device scanning and connection
- Support for Yamaha MD-BT01, CME WIDI, and other BLE MIDI devices
- Auto-reconnect on BLE connection loss with exponential backoff
- Prerequisite checking for BLE MIDI with user guidance in status bar
- `b` key to force BLE MIDI rescan
- BLE connection status display in header (color-coded)

### Changed
- Audio backend now managed by unified AudioManager
- Synthesizer upgraded from simple sine wave to piano-like tone with harmonics
- ADSR envelope added to eliminate audio clicks and pops
- README updated with comprehensive BLE MIDI documentation

### Fixed
- Audio crackling in synthesis backend by using phase accumulation
- Audio clipping when playing chords by reducing amplitude
- Sink accumulation by cleaning up finished sinks

## [0.1.0] - 2024-11-27

### Added
- Initial release of Ear Trainer
- Terminal-based UI using ratatui
- 50+ chord progressions across 5 genres (Jazz, Soul, Funk, Smooth Jazz, Pop)
- Piano roll visualization with color-coded chord tones and guide tones
- Chord analysis panel with intervals, scale degrees, and voice leading
- MIDI output backend for external synthesizers
- Built-in synthesis backend using sine waves
- Music theory engine with chord-scale matching
- Interactive keyboard controls for playback and navigation

[Unreleased]: https://github.com/doobidoo/EarPlayer/compare/v0.2.1...HEAD
[0.2.0]: https://github.com/doobidoo/EarPlayer/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/doobidoo/EarPlayer/releases/tag/v0.1.0
[0.2.1]: https://github.com/doobidoo/EarPlayer/compare/v0.2.0...v0.2.1
