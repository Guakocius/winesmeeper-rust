use std::fmt;
use xmltree::{XMLNode, Element};
use anyhow::{Result, anyhow};
use rand::Rng;

use crate::model::field::Field;

#[derive(Debug, Default)]
pub struct Board {
    pub board: Vec<Vec<Field>>,
    pub width: usize,
    pub height: usize,
    pub start_x: usize,
    pub start_y: usize,
    pub bomb_count: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, start_x: usize, 
        start_y: usize, bomb_count: usize) -> Result<Self, String> {
            if width < 10 || height < 10 {
                return Err("x and y size must be >= 10".into());
            }

            if start_x >= width || start_y >= height {
                return Err("starting position must be on the field".into());
            }

            let max_bombs = Self::max_bombs(width, height, start_x, start_y);
            if bomb_count == 0 || bomb_count > max_bombs {
                return Err(format!("bomb count must be between 1 and {}", max_bombs));
            }

            let mut board = vec![
                vec![
                    Field {
                        is_bomb: false,
                        is_opened: false,
                        is_flag: false,
                    };
                    height
                ];
                width
            ];

            Self::place_bombs(&mut board, start_x, start_y, bomb_count);

            Ok(Self {
                board,
                width,
                height,
                start_x,
                start_y,
                bomb_count
            })
        }
    fn max_bombs(width: usize, height: usize, x: usize, y: usize) -> usize {
        let edges = match (
            Self::is_edge( width, x),
            Self::is_edge( height, y),
        ) {
            (true, true) => 5,
            (true, false) | (false, true) => 3,
            _ => 0,
        };

        (width * height) - 9 + edges
    }

    fn is_edge(size: usize, x: usize) -> bool {
        x == 0 || x == size - 1
    }

    fn is_neighbor(x0: usize, y0: usize, x1: usize, y1: usize) -> bool {
        x0.abs_diff(x1) <= 1 && y0.abs_diff(y1) <= 1
    }
    
    pub fn get_bomb_neighbor(&self, x: isize, y: isize) -> usize {
        (-1..=1)
            .flat_map(|vx| (-1..=1).map(move |vy| (vx, vy)))
            .filter(|&(vx, vy)| {
                let nx = x + vx;
                let ny = y + vy;
                self.board[nx as usize][ny as usize].is_bomb
            })
            .count()
    }
    pub fn get_field(&self, x: usize, y: usize) -> i32 {
        let field = &self.board[x as usize][y as usize];
        match field {
            Field { is_flag: true, .. } => -3,
            Field { is_opened: false, .. } => -1,
            Field { is_bomb: true, .. } => -2,
            _ => self.get_bomb_neighbor(x as isize, y as isize) as i32
        }
    }
    fn is_victory(&self) -> bool {
        self.board
            .iter()
            .flatten()
            .any(|f| !f.is_bomb && !f.is_opened)
    }
    fn to_xml(&self) -> Element {
        let mut board = Element::new("board");
        for row in &self.board {
            let mut row_elem = Element::new("row");

            for field in row {
                row_elem.children.push(XMLNode::Element(field.to_xml()));
            }
            board.children.push(XMLNode::Element(row_elem));
        }
        board
    }
    fn from_xml(elem: &Element) -> Result<Self> {
        let board_elem = elem
            .get_child("board")
            .ok_or_else(|| anyhow!("missing <board> element"))?;
        let board = board_elem
            .children
            .iter()
            .filter_map(|n| n.as_element())
            .filter(|e| e.name == "row")
            .map(|row_elem| {
                row_elem
                    .children
                    .iter()
                    .filter_map(|n| n.as_element())
                    .filter(|e| e.name == "field")
                    .map(Field::from_xml)
                    .collect::<Result<Vec<Field>>>()
            })
            .collect::<Result<Vec<Vec<Field>>>>()?;

        let height = board.len();
        let width = board.first().map(|r| r.len()).unwrap_or(0);
        let start_x = board_elem
            .attributes
            .get("start_x")
            .ok_or_else(|| anyhow!("missing start_x"))?
            .parse()?;

        let start_y = board_elem
            .attributes
            .get("start_y")
            .ok_or_else(|| anyhow!("missing start_y"))?
            .parse()?;

        let bomb_count = board_elem
            .attributes
            .get("bomb_count")
            .ok_or_else(|| anyhow!("missing bomb_count"))?
            .parse()?; 

        Ok(Board { board, height, width, start_x, start_y, bomb_count })
    }
    fn place_bombs(board: &mut [Vec<Field>], start_x: usize,
        start_y: usize, bomb_count: usize) {
            let mut rng = rand::rng();
            let mut remaining = bomb_count;

            let width = board.len();
            let height = board[0].len();

            while remaining > 0 {
                let x = rng.random_range(0..width);
                let y = rng.random_range(0..height);

                if Self::is_neighbor(start_x, start_y, x, y) {
                    continue;
                }

                if !board[x][y].is_bomb {
                    board[x][y].is_bomb = true;
                    remaining -= 1;
                }
            }

        }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for field in row {
                let ch = match (field.is_bomb, field.is_opened, field.is_flag) {
                    (_, _, true) => '🚩',
                    (true, true, _) =>  '💣',
                    (_, false, _) => '■',
                    (_, true, _) => '·',
                };
                write!(f, "{} ", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
