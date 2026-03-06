use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize, Color, Modifier, Style},
    symbols::border,
    text::{Line, Text, Span},
    widgets::{Block, Borders, Paragraph, Widget, Clear, Wrap},
    DefaultTerminal,
    Frame,
};
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

use crate::{Tui, controller::commands::{SYSTEM_COMMANDS, TURN_COMMANDS}};
use crate::model::board::Board;
use crate::controller::commands::{self, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    Help,
    Generate {
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
        bomb_count: usize,
    },
    Redo,
    Undo,
    Save,
    Load,
    Flag,
    Open,
    InvalidCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Help,
    Generate,
    Save,
    Load,
    Flag,
    Open,
}

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub exit: bool,
    pub overlay: Option<Overlay>,
    pub input: String,
    character_index: usize,
    pub input_mode: InputMode,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
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
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);

        let [help_area, input_area, board_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
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
                    " Flag ".into(),
                    "<F> ".blue().bold(),
                    " Open Field ".into(),
                    "<O> ".blue().bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Help => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
            InputMode::Generate => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
            InputMode::Save => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
            InputMode::Load => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
            InputMode::Flag => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
            InputMode::Open => (
                vec![
                    "Esc".blue().bold(),
                    ": go back".into(),
                ],
                Style::default(),
            ),
        };

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);

        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Help => Style::default().fg(Color::Red),
                InputMode::Generate => Style::default().fg(Color::Yellow),
                InputMode::Save => Style::default(),
                InputMode::Load => Style::default(),
                InputMode::Flag => Style::default(),
                InputMode::Open => Style::default(),
            })
            .block(Block::bordered().title("Input"));

        frame.render_widget(input, input_area);
        match self.input_mode {
            // Hide cursor
            InputMode::Normal => {}

            #[allow(clippy::cast_possible_truncation)]
            InputMode::Help
                | InputMode::Generate 
                | InputMode::Save 
                | InputMode::Load 
                | InputMode::Flag 
                | InputMode::Open => todo!()
        }
        
        frame.render_widget(self, board_area);
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
        
        match self.input_mode {
                InputMode::Normal => match key_event.code {
                    KeyCode::Char('g') => {
                        self.input.clear();
                        self.input_mode = InputMode::Generate;
                        self.overlay = Some(Overlay::Generate {
                            width: self.board.width,
                            height: self.board.height,
                            start_x: self.board.start_x,
                            start_y: self.board.start_y,
                            bomb_count: self.board.bomb_count,
                        });
                    } 
                    KeyCode::Char('h') => {
                        self.input.clear();
                        self.input_mode = InputMode::Help;
                        self.overlay = Some(Overlay::Help);
                    }
                    KeyCode::Char('s') => {
                        self.input.clear();
                        self.input_mode = InputMode::Save;
                        self.overlay = Some(Overlay::Save);
                    }
                    KeyCode::Char('l') => {
                        self.input.clear();
                        self.input_mode = InputMode::Load;
                        self.overlay = Some(Overlay::Load);
                    }
                    KeyCode::Char('f') => {
                        self.input.clear();
                        self.input_mode = InputMode::Flag;
                        self.overlay = Some(Overlay::Flag);
                    }
                    KeyCode::Char('o') => {
                        self.input.clear();
                        self.input_mode = InputMode::Open;
                        self.overlay = Some(Overlay::Open);
                    }
                    KeyCode::Enter => {

                    }
                    KeyCode::Char('q') => self.exit = true,
                    _ => self.overlay = Some(Overlay::InvalidCommand)
                },
                InputMode::Help 
                    | InputMode::Generate 
                    | InputMode::Save
                    | InputMode::Load
                    | InputMode::Flag
                    | InputMode::Open => match key_event.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
            /*KeyCode::Char('h') => self.show_help_message(),
            KeyCode::Char('g)' => self.generate_prompt(),
            KeyCode::Char('r') => self.redo(),
            KeyCode::Char('u') => self.undo(),
            KeyCode::Char('s') => self.save_game(),
            KeyCode::Char('l') => self.load_game(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
            */
        }
        Ok(())
    }
    
    fn show_help_message(&mut self) {
        self.overlay = Some(Overlay::Help);
    }

    /*fn generate_prompt(&mut self, frame: &mut Frame, app: &App) {
        let size = frame.area();

        let generate_input = Paragraph::new(app.input_as_str())
            .block(Block::default().borders(Borders::ALL).title("Generate Board"));
        frame.render_widget(generate_input, size);


    }*/

    fn generate_board(&mut self, width: usize, height: usize, 
        start_x: usize, start_y: usize, bomb_count: usize) {
        self.overlay = Some(Overlay::Generate {
            width,
            height,
            start_x,
            start_y,
            bomb_count,
        });

    }
    fn redo(&mut self) {
        self.overlay = Some(Overlay::Redo);

    }
    fn undo(&mut self) {
        self.overlay = Some(Overlay::Undo);

    }
    fn save_game(&mut self) {

    }
    fn load_game(&mut self) {

    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn help_text() -> Text<'static> {
        let mut lines = Vec::new();

        lines.extend(Command::render_commands("SYSTEM COMMANDS", SYSTEM_COMMANDS));

        lines.push(Line::raw(""));
        lines.push(Line::raw(""));

        lines.extend(Command::render_commands("TURN COMMANDS", TURN_COMMANDS));

        Text::from(lines)
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ])
                .split(popup_layout[1])[1]
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
            overlay: None,
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Winesmeeper - A Minesweeper Saga ".bold());

        let block = Block::bordered()
            .title(title.centered())
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
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();

        assert!(app.exit);
    }
}
