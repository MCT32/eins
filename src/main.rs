use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render).unwrap();
        if matches!(event::read().unwrap(), Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _
        })) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}