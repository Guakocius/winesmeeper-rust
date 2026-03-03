use ratatui::{text::{Line, Span}, style::{Stylize, Style}};

#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub name: &'static str,
    pub key: char,
    pub description: &'static str,
}

pub const SYSTEM_COMMANDS: &[Command] = &[
    Command { name: " Help", key: 'H', description: " Help: Prints an overview of all System and Turn Commands" },
    Command { name: " Generate", key: 'G', description: " Starts the generation of a new Board:\n\n\t<x-size> <y-size> <x-start> <y-start> <bomb-count> "},
    Command { name: " Redo", key: 'R', description: " Redo <count>:\n\tmakes your last <count> undo's done "},
    Command { name: " Save", key: 'S', description: " Save <filename>:\n\tsaves your game at a given file "},
    Command { name: " Load", key: 'L', description: " Load <filename>:\n\tloads the save game at the specified file "},
    Command { name: " Quit", key: 'Q', description: " Quit:\n\tcloses the game"},
];

pub const TURN_COMMANDS: &[Command] = &[
    Command { name: " Flag", key: 'F', description: " Flag <x> <y>:\n\tmark this position as flag or remove the flag "},
    Command { name: " Open Field", key: 'O', description: " Open <x> <y>:\n\tOpens a field, and if you hit a bomb, you lose\n\tBut fortunately you can undo your destiny. "},
];

impl Command {

    fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        lines.push(Line::from(vec![
            Span::styled(
                format!("({})", self.key),
                Style::default().blue().bold(),
        ),
        Span::raw(self.name),
        ]));

        for line in self.description.lines() {
            lines.push(Line::from(Span::raw(format!("\t{}", line))));
        }
        lines.push(Line::raw(""));
        lines

    }

    pub fn render_commands<'a>(title: &'a str, commands: &'a [Command]) -> Vec<Line<'a>> {
        let mut lines = Vec::new();

        lines.push(Line::styled(
            title,
            Style::default().bold().underlined(),
        ));
        lines.push(Line::raw(""));

        for cmd in commands {
            lines.extend(cmd.render());
        }
        lines
    }
}