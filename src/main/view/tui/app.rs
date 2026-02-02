use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

use crate::{Tui, controller::commands::{SYSTEM_COMMANDS, TURN_COMMANDS}};
use crate::model::board::Board;
use crate::controller::commands::Command;

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn board_to_text(&self) -> Text<'static> {
        let lines: Vec<Line> = self.board.board.iter().enumerate().map(|(y, row)| {
            let spans: Vec<Span> = row.iter().enumerate().map(|(x, field)| {
                match field {
                    f if f.is_flag => "⚑".red(),
                    f if !f.is_opened => "■".gray(),
                    f if f.is_bomb => "*".red(),
                    _ => {
                        let n = self.board.get_bomb_neighbor(x as isize, y as isize);
                        if n == 0 {
                            " ".into()
                        } else {
                            n.to_string().cyan()
                        }
                    }
                }
            }).collect();

            Line::from(spans)
        }).collect();
        Text::from(lines)
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(())
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('h') => self.show_help_message(),
            KeyCode::Char('g') => self.generate_board(),
            KeyCode::Char('r') => self.redo(),
            KeyCode::Char('u') => self.undo(),
            KeyCode::Char('s') => self.save_game(),
            KeyCode::Char('l') => self.load_game(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn show_help_message(&mut self) {
        let title = Line::from(" Help: Prints an overview of all System and Turn Commands");
        let system_commands = Command::render_commands(SYSTEM_COMMANDS);
        let turn_commands = Command::render_commands(TURN_COMMANDS);

        let sys_rec = Rect::new(0, 0, self.board.width as u16 / 2, self.board.height as u16 / 2);
        let turn_rec = Rect::new(self.board.width as u16 / 2, self.board.height as u16 / 2, self.board.width as u16 / 2, self.board.height as u16 / 2);

        let sys_block = Block::bordered()
            .title(title);
        

    }

    fn generate_board(&mut self) {

    }
    fn redo(&mut self) {

    }
    fn undo(&mut self) {

    }
    fn save_game(&mut self) {

    }
    fn load_game(&mut self) {

    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        let board = Board::new(
            10, 10,
            5, 5,
            10
        ).expect("failed to create board");

        Self {
            board,
            exit: false,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Winesmeeper - A Minesweeper Saga ".bold());
        let sys_commands = Line::from(vec![
            " Help ".into(),
            "<H> ".blue().bold(),
            " Generate ".into(),
            "<G> ".blue().bold(),
            " Redo ".into(),
            "<R> ".blue().bold(),
            " Undo ".into(),
            "<U> ".blue().bold(),
            " Save ".into(),
            "<S> ".blue().bold(),
            " Load ".into(),
            "<L> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let turn_commands = Line::from(vec![
            " Flag ".into(),
            "<F> ".blue().bold(),
            " Open Field ".into(),
            "<O> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_top(sys_commands.centered())
            .title_bottom(turn_commands.centered())
            .border_set(border::THICK);

        let board = self.board_to_text();

        Paragraph::new(board)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    fn buffer_to_string(buf: &Buffer) -> Vec<String> {
        let Rect { width, height, .. } = buf.area;
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| buf.get(x, y).symbol())
                        .collect::<String>()
            })
            .collect()
    }

    #[test]
    fn render() {
        let app = App::default();


        let mut expected = Buffer::with_lines(vec![
            "┏━━━━ Winesmeeper - A Minesweeper Saga ━ Help <H>  Generate <G>  Redo <R>  Undo <U>  Save <S>  Load <L>  Quit <Q> ━━━━━┓",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┃                                                      ■■■■■■■■■■                                                      ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Flag <F>  Open Field <O> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);

        let mut buf = Buffer::empty(Rect::new(0, 0, expected.area.width, expected.area.height));

        app.render(buf.area, &mut buf);

        assert_eq!(buffer_to_string(&buf), buffer_to_string(&expected));
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        /*
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        //assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        //assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();*/

        assert!(app.exit);
    }
}