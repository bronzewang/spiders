use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use clap::Parser;
use ratatui::crossterm::event::Event;
use ratatui::Terminal;
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode}, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};

use app::App;

mod ui;
mod app;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 100)]
    rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Spiders Visitor startup");
    let rate = Duration::from_millis(cli.rate);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("Spiders Visitor");
    let result = run(&mut terminal, app, rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(),
             LeaveAlternateScreen,
             DisableMouseCapture)?;

    if let Err(err) = result {
        println!("{err:?}");
    }

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    rate: Duration
) -> Result<(), Box<dyn Error>> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let timeout = rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            app.on_tick();
            last_tick = Instant::now();
            continue;
        }
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('h') | KeyCode::Left => app.on_left(),
                KeyCode::Char('j') | KeyCode::Down => app.on_down(),
                KeyCode::Char('k') | KeyCode::Up => app.on_up(),
                KeyCode::Char('l') | KeyCode::Right => app.on_right(),
                KeyCode::Char(c) => app.on_key(c),
                _ => (),
            }
        }
        if app.quit {
            return Ok(())
        }
    }
}
