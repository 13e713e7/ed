use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use anyhow::Result;
use crossterm::{
    event::{read as event_read, Event, KeyCode, KeyEvent, KeyModifiers},
    // event::{Event, EventStream},
    execute,
    terminal,
};

// struct ApplicationConfig {}

struct Application {}
impl Application {
    pub fn new() -> Self {
        Application {}
    }
    pub fn run(&mut self) -> Result<()> {
        start_raw()?;
        panic_hook()?;
        self.event_loop()?;
        restore_raw()
    }
    pub fn event_loop(&mut self) -> Result<()> {
        loop {
            let event = event_read()?;
            println!("Event: {:?}\r", event);
            match event {
                Event::Key(ke) => match ke {
                    KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: KeyModifiers::NONE,
                    } => break,
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}

fn panic_hook() -> Result<()> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_raw();
        hook(info);
    }));
    Ok(())
}

fn start_raw() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    match write!(stdout, "\x1B[2 q") {
        Ok(_) => {},
        Err(e) => {
            execute!(stdout, terminal::LeaveAlternateScreen)?;
            println!("can not enter raw mode: {}", e);
        }
    };
    Ok(())
}
fn restore_raw() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn logging() -> Result<()> {
    let path = PathBuf::from("/var/log/ed.log");
    let mut base_config = fern::Dispatch::new();
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(path)?);
    base_config = base_config.level(log::LevelFilter::Debug);
    base_config.chain(file_config).apply()?;
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    logging()?;
    let mut app = Application::new();
    app.run()?;
    Ok(())
}
