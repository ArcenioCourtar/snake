use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    match app.run(&mut terminal) {
        Err(e) => eprintln!("{:?}", e),
        _ => ()
    }
    ratatui::restore();
}

#[derive(Default)]
enum CurrentScreen {
    #[default]
    Main,
    Game,
    Options,
}


// tutorial stuff
pub struct App {
    exit: bool,
    state: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            state: CurrentScreen::Main,
        }
    }
        
    pub fn run(&mut self, terminal: &mut DefaultTerminal) 
        -> io::Result<()> {
        while !self.exit {
            match self.state {
                CurrentScreen::Main => {
                    terminal.draw(|frame| self.draw(frame))?;
                    self.handle_events()?;
                },
                CurrentScreen::Game => todo!(),
                CurrentScreen::Options => todo!(),
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Snake main menu ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new("Hello there")
            .centered()
            .block(block)
            .render(area, buf);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
