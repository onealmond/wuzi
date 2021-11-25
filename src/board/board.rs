use std::collections::HashMap;
use super::color::{COLOR_GREEN, COLOR_RED, Color};

pub const MAX_BULLETS: usize = 5usize;
pub const MAX_PLAYERS: usize = 2usize;

pub enum GameResult {
    NoWinner, // the board is full, no one wins
    Unknown,  // the game isn't over
    Winner,   // found winner 
}

impl GameResult {
    pub fn from_u8(value: u8) -> Option<GameResult> {
        match value {
            0 => return Some(GameResult::NoWinner),
            1 => return Some(GameResult::Unknown),
            2 => return Some(GameResult::Winner),
            _ => None,
        }
    }
}

pub struct Slot {
    pub index: usize,
}

struct Score {
    rows: Vec<u8>,
    columns: Vec<u8>,
    diagonals: Vec<u8>,
}

pub struct Board {
    board: Vec<Vec<Color>>,
    scores: HashMap<Color, Score>,
}

impl Score {
    pub fn new() -> Score {
        let mut score = Score{
            rows: Vec::new(),
            columns: Vec::new(),
            diagonals: Vec::new(),
        };
        score.rows.resize(MAX_BULLETS, 0);
        score.columns.resize(MAX_BULLETS, 0);
        score.diagonals.resize(2, 0);
        return score;
    }
}

pub fn get_board_from_string(s: String) -> Option<Vec<Vec<Color>>> {
    let mut board: Vec<Vec<Color>> = Vec::with_capacity(MAX_BULLETS);
    board.resize(MAX_BULLETS, Vec::new());
    let mut col = 0usize;

    for c in s.chars() {
        if c == ';' { 
            col += 1; 
            continue
        }
        match c.to_digit(10) {
            Some(c) => board[col].push(Color{value: c as u8}),
            None => return None,
        }
    }
    return Some(board);
}

pub fn print_board(board: &Vec<Vec<Color>>) {
    println!("_______________");
    // Print the slots as in real life
    for i in 0..MAX_BULLETS {
        for j in 0..MAX_BULLETS {
            if board[j].len() < MAX_BULLETS - i {
                print!("[ ]");
            } else {
                print!("[{}]", board[j][MAX_BULLETS-i-1].to_string());
            }
        }
        println!();
    }
    println!("+-+-+-+-+-+-+-+");
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            board: Vec::with_capacity(MAX_BULLETS),
            scores: HashMap::new(),
        };
        board.board.resize(MAX_BULLETS, Vec::new());
        board.scores.insert(COLOR_GREEN, Score::new());
        board.scores.insert(COLOR_RED, Score::new());
        return board;
    }

    fn get_mut_scores(&mut self, color: &Color) -> &mut Score {
        return self.scores.get_mut(&color).unwrap()
    }
    
    pub fn get_board(&self) -> &Vec<Vec<Color>> {
        return &self.board;
    }

    pub fn get_board_as_string(&self) -> String {
        let mut s = String::from("");
        for c in 0..self.board.len() {
            for r in 0..self.board[c].len() {
                s.push_str(self.board[c][r].value.to_string().as_str());
            }
            s.push_str(";");
        }
        return s;
    }

    pub fn is_full(&self) -> bool {
        for c in &self.board {
            if c.len() < MAX_BULLETS {
                return false;
            }
        }
        return true;
    }

    pub fn place(&mut self, slot: Slot, color: &Color) -> Result<bool, String> {
        if self.scores.get_mut(&color).is_none() {
            return Err("invalid color".to_string());
        }

        if slot.index >= MAX_BULLETS ||
            self.board[slot.index].len() >= MAX_BULLETS {
                return Err("invalid slot index or slot is full".to_string());
        }

        if self.is_full() {
            return Err("board is full".to_string()); 
        }

        self.board[slot.index].push(*color);
        let row = self.board[slot.index].len()-1;
        self.get_mut_scores(color).rows[row] |= 1 << slot.index;
        self.get_mut_scores(color).columns[slot.index] |= 1 << row;

        if row == slot.index {
            self.get_mut_scores(&color).diagonals[0] |= 1 << slot.index;
        }

        if row + slot.index == MAX_BULLETS-1 {
            self.get_mut_scores(&color).diagonals[1] |= 1 << slot.index;
        }

        return Ok(true);
    }

    pub fn is_winner(&mut self, color: &Color) -> bool {
        for i in &self.get_mut_scores(color).rows {
            if i == &0x1fu8 {
                return true;
            }
        }

        for i in &self.get_mut_scores(color).columns {
            if i == &0x1fu8 {
                return true;
            }
        }

        for i in &self.get_mut_scores(color).diagonals {
            if i == &0x1fu8 {
                return true;
            }
        }
        return false;
    }
}
