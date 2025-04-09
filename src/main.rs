use chrono::Datelike;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, thread, time::Duration};

fn display_anim(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    text: &[Span],
) -> Result<(), io::Error> {
    let mut content = String::new();

    for span in text {
        content.push_str(&span.to_string());
        content.push('\n');

        terminal.draw(|f| {
            let size = f.area();
            // Create vertical layout: header and main content with a margin.
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1), // Header
                        Constraint::Min(1),    // Content
                    ]
                    .as_ref(),
                )
                .split(size);

            // Header: with a bold title and custom colors.
            let header = Paragraph::new("Zander Lewis")
                .style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                )
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(header, chunks[0]);

            // Content area with a nice border and custom border color.
            let paragraph = Paragraph::new(content.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("About Me")
                        .border_style(Style::default().fg(Color::Yellow)),
                )
                .alignment(ratatui::layout::Alignment::Left)
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, chunks[1]);
        })?;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Clear the screen.
    print!("\x1B[2J\x1B[1;1H");
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Determine age from a fixed birthdate (September 14, 2008).
    let age = {
        let birth_date = chrono::NaiveDate::from_ymd_opt(2008, 9, 14).unwrap();
        let today = chrono::Utc::now().date_naive();
        let mut age = today.year() - birth_date.year();
        if today.ordinal() < birth_date.ordinal() {
            age -= 1;
        }
        age
    };

    // Profile text lines.
    let profile_text = vec![
        Span::raw("Hello, I'm Zander Lewis"),
        Span::raw(format!("Age: {} years", age)),
        Span::raw("Location: Candler, NC"),
        Span::raw("-------------------------------------------------"),
        Span::raw("🔭 I'm a small open source developer."),
        Span::raw("📚 I'm currently learning Laravel and Rust."),
        Span::raw("⚡ In my free time I write code."),
        Span::raw("-------------------------------------------------"),
        Span::raw("Favorite Languages:"),
        Span::raw("Rust, PHP, Python, Fortran"),
    ];

    display_anim(&mut terminal, &profile_text)?;

    // Wait for the user to press 'q', 'Esc', or 'Ctrl + c' to quit.
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q')
                || key.code == KeyCode::Esc
                || (key.code == KeyCode::Char('c')
                    && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL))
            {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                print!("\x1B[2J\x1B[1;1H");
                break;
            }
        } else if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL))
                {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    print!("\x1B[2J\x1B[1;1H");
                    break;
                }
            }
        } else {
            // If no key is pressed, continue the loop.
            thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
