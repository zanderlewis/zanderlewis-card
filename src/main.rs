use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Gauge},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::Span,
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{io, thread, time::Duration};

fn display_loading_bar(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    for i in 0..=100 {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(size);

            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("Loading..."))
                .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
                .percent(i);
            f.render_widget(gauge, chunks[0]);
        })?;
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

fn display_typing_animation(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, text: &[Span]) -> Result<(), io::Error> {
    let mut s = "".to_string();
    for span in text.iter() {
        s = s + &span.to_string() + "\n";
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(size);

            let paragraph = Paragraph::new(s.clone())
                .block(Block::default().borders(Borders::ALL).title("Zander Lewis"));
            f.render_widget(paragraph, chunks[1]);
        })?;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    print!("\x1B[2J\x1B[1;1H");
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    display_loading_bar(&mut terminal)?;

    let profile_text = vec![
        Span::raw("\x1B[33m\x1B[1mHello, I'm Zander Lewis\x1B[0m"),
        Span::raw("Age: 16"),
        Span::raw("🔭 I'm a small open source developer."),
        Span::raw("📚 I'm currently learning Laravel and Rust."),
        Span::raw("⚡ In my free time I prefer to code than play video games."),
        Span::raw("\x1B[33m\x1B[1mFavorite Languages\x1B[0m"),
        Span::raw("Rust, PHP, Python"),
    ];

    display_typing_animation(&mut terminal, &profile_text)?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                print!("\x1B[2J\x1B[1;1H");
                break;
            }
        }
    }

    Ok(())
}