mod audio;
mod config;
mod music;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use ui::{App, AppMode};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if !ui::handle_input(app, key) {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    if app.show_help {
        render_help(f, app);
        return;
    }

    // Adaptive layout based on terminal height
    let terminal_height = f.size().height;

    if terminal_height < 30 {
        // Compact layout for small terminals
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(8),     // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.size());

        render_header(f, app, chunks[0]);
        render_main_content(f, app, chunks[1]);
        render_status_bar(f, app, chunks[2]);
    } else {
        // Full layout with piano roll panel
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),   // Header
                Constraint::Percentage(35), // Piano Roll Panel (NEW)
                Constraint::Min(8),      // Main content
                Constraint::Length(3),   // Status bar
            ])
            .split(f.size());

        render_header(f, app, chunks[0]);
        render_piano_roll_panel(f, app, chunks[1]);
        render_main_content(f, app, chunks[2]);
        render_status_bar(f, app, chunks[3]);
    }
}

fn render_piano_roll_panel(f: &mut Frame, app: &App, area: Rect) {
    // Split the panel: timeline piano roll on top, keyboard reference on bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6),      // Timeline piano roll (takes most space)
            Constraint::Length(5),   // Horizontal keyboard reference
        ])
        .split(area);

    // Render enhanced piano roll with timeline
    let enhanced_roll = ui::EnhancedPianoRoll::new(&app.timeline_state)
        .with_voice_leading(app.show_voice_leading);
    f.render_widget(enhanced_roll, chunks[0]);

    // Render horizontal keyboard reference
    if let (Some(chord), Some(scale)) = (app.current_chord(), app.current_scale()) {
        let keyboard = ui::HorizontalKeyboard::new(chord, &scale)
            .with_range(48, 2); // 2 octaves starting from C3
        f.render_widget(keyboard, chunks[1]);
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    use crate::audio::BleConnectionState;

    // Determine audio status color based on BLE state
    let (audio_text, audio_color) = {
        let status = app.audio_status_line();
        let ble_status = app.ble_status();

        let color = match ble_status.state {
            BleConnectionState::Connected => Color::Green,
            BleConnectionState::Scanning | BleConnectionState::Connecting => Color::Yellow,
            BleConnectionState::Reconnecting(_) => Color::Yellow,
            BleConnectionState::Disconnected => Color::Magenta,
        };

        (status, color)
    };

    let title = vec![
        Span::styled(
            "♫ EAR TRAINER ♫",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Genre: ", Style::default().fg(Color::Gray)),
        Span::styled(
            &app.current_genre,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Audio: ", Style::default().fg(Color::Gray)),
        Span::styled(audio_text, Style::default().fg(audio_color)),
        Span::raw("  |  Press 'h' for help"),
    ];

    let header = Paragraph::new(Line::from(title))
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    f.render_widget(header, area);
}

fn render_main_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // Left side: Progression list
    render_progression_list(f, app, chunks[0]);

    // Right side: Chord analysis (moved here from separate panel)
    render_chord_analysis(f, app, chunks[1]);
}

fn render_progression_list(f: &mut Frame, app: &App, area: Rect) {
    if let Some(progression) = app.current_progression() {
        let items: Vec<ListItem> = progression
            .changes
            .iter()
            .enumerate()
            .map(|(i, change)| {
                let is_current = i == app.current_chord_idx;
                let prefix = if is_current { "► " } else { "  " };

                let style = if is_current {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = format!(
                    "{}{} ({} beats)",
                    prefix,
                    change.chord.name(),
                    change.duration
                );

                ListItem::new(content).style(style)
            })
            .collect();

        let title = format!("{} - {}", progression.name, progression.genre);
        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default());

        f.render_widget(list, area);
    }
}

fn render_chord_analysis(f: &mut Frame, app: &App, area: Rect) {
    if let (Some(chord), Some(scale)) = (app.current_chord(), app.current_scale()) {
        let next_chord = if app.show_voice_leading {
            app.current_progression().and_then(|prog| {
                let next_idx = (app.current_chord_idx + 1) % prog.changes.len();
                prog.changes.get(next_idx).map(|c| &c.chord)
            })
        } else {
            None
        };

        let mut notation = ui::render_notation(chord, &scale);
        if let Some(next) = next_chord {
            notation = notation.with_next_chord(next);
        }

        f.render_widget(notation, area);
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let ble_status = app.ble_status();

    // Check if there are BLE prerequisite issues to show
    let has_prereq_issues = !ble_status.prerequisites.is_empty()
        && ble_status.prerequisites.iter().any(|p| !p.passed);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    let playback_status = if app.is_playing {
        Span::styled("▶ PLAYING", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("■ STOPPED", Style::default().fg(Color::Red))
    };

    let mode_text = match app.mode {
        AppMode::Listen => "Listen Mode",
        AppMode::Practice => "Practice Mode",
        AppMode::Quiz => "Quiz Mode",
    };

    // Build status line - show BLE issues if any, otherwise normal status
    let status_line = if has_prereq_issues {
        let mut spans = vec![
            Span::styled("BLE Issues: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ];
        for prereq in &ble_status.prerequisites {
            if !prereq.passed {
                spans.push(Span::styled(
                    format!("[{}] ", prereq.message),
                    Style::default().fg(Color::Yellow),
                ));
            }
        }
        Line::from(spans)
    } else {
        Line::from(vec![
            playback_status,
            Span::raw("  |  "),
            Span::styled("Mode: ", Style::default().fg(Color::Gray)),
            Span::styled(mode_text, Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::styled("Beat: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}", app.current_beat),
                Style::default().fg(Color::Yellow),
            ),
        ])
    };

    let controls_line = Line::from(vec![
        Span::styled("Controls: ", Style::default().fg(Color::Gray)),
        Span::raw("SPACE=Play  "),
        Span::raw("n/p=Prog  "),
        Span::raw("g=Genre  "),
        Span::raw("m=Audio  "),
        Span::raw("b=BLE Scan  "),
        Span::raw("q=Quit"),
    ]);

    if let Some(prog) = app.current_progression() {
        let tempo_line = Line::from(vec![
            Span::styled("Tempo: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.0} BPM", prog.tempo),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  (+/- to adjust)  |  "),
            Span::styled("Key: ", Style::default().fg(Color::Gray)),
            Span::styled(
                prog.key.name(),
                Style::default().fg(Color::Magenta),
            ),
        ]);

        f.render_widget(Paragraph::new(tempo_line), chunks[0]);
    }

    f.render_widget(Paragraph::new(status_line), chunks[1]);
    f.render_widget(Paragraph::new(controls_line), chunks[2]);
}

fn render_help(f: &mut Frame, app: &App) {
    let area = f.size();
    let help_text = vec![
        Line::from(vec![
            Span::styled(
                "EAR TRAINER - HELP",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Playback Controls:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  SPACE      - Play/Pause progression"),
        Line::from("  n          - Next progression"),
        Line::from("  p          - Previous progression"),
        Line::from("  +/-        - Increase/Decrease tempo"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  g          - Next genre"),
        Line::from("  G          - Previous genre"),
        Line::from("  1          - Listen mode"),
        Line::from("  2          - Practice mode"),
        Line::from("  3          - Quiz mode"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Display Options:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  s          - Toggle scale display"),
        Line::from("  v          - Toggle voice leading arrows"),
        Line::from("  [/]        - Scroll timeline left/right"),
        Line::from("  m          - Cycle audio: MIDI -> Synth -> BLE MIDI"),
        Line::from("  b          - Force BLE MIDI rescan"),
        Line::from("  h          - Toggle this help screen"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Other:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  q          - Quit application"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Legend:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("●", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Chord Tone  "),
            Span::styled("●", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Guide Tone (3rd/7th)  "),
            Span::styled("·", Style::default().fg(Color::Blue)),
            Span::raw(" Scale Note  "),
            Span::styled("×", Style::default().fg(Color::DarkGray)),
            Span::raw(" Avoid Note"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Available Genres:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let mut all_lines = help_text;
    let genres = app.library.all_genres();
    for genre in genres {
        if let Some(progs) = app.library.get_by_genre(&genre) {
            all_lines.push(Line::from(format!("  {} ({} progressions)", genre, progs.len())));
        }
    }

    all_lines.push(Line::from(""));
    all_lines.push(Line::from("Press 'h' to close this help screen"));

    let paragraph = Paragraph::new(all_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
