use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[allow(dead_code)]
pub fn solution() {
    let mut game = BingoGame::load("input/day4.txt");
    let winning_score = game.play();

    println!("Part 1: {}", winning_score);
}

/// BingoGame represents a game of bingo, and contains the numbers that are drawn and the boards.
struct BingoGame {
    nums: Vec<usize>,
    boards: Vec<Board>,
    num_to_board_positions: HashMap<usize, Vec<BoardPosition>>,
}

impl BingoGame {
    /// Loads a game of bingo from the given file.  The first line in the game contains a
    /// comma-separated list of numbers that are drawn, and the remaining lines are 5x5 boards
    /// separated by blank lines.
    fn load(filename: &str) -> BingoGame {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);
        let mut lines = f.lines();

        // First line is a comma-separated list of numbers.
        let nums = lines.next().unwrap().unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        // Followed by an optional newline.
        let _ = lines.next();

        // Followed by the boards.
        let boards = BingoReader::new(lines).collect::<Vec<Board>>();


        let mut num_to_board_positions = HashMap::new();
        for (i, board) in boards.iter().enumerate() {
            for row in 0..5 {
                for col in 0..5 {
                    num_to_board_positions
                        .entry(board.nums[row][col])
                        .or_insert(Vec::new())
                        .push(BoardPosition::new(i, row, col));
                }
            }
        }

        BingoGame { nums, boards, num_to_board_positions }
    }

    /// Plays a game of bingo, returning the score of the winning board.
    fn play(&mut self) -> usize {
        for num in &self.nums {
            for board_position in self.num_to_board_positions.entry(*num).or_default() {
                let board = &mut self.boards[board_position.board];
                board.mark(board_position.row, board_position.col);

                if board.is_winner() {
                    return board.score(*num);
                }
            }
        }

        panic!("No winner after all numbers drawn.")
    }
}


/// `BingoReader` is an iterator that parses bingo `Board`s from a file.
struct BingoReader {
    lines: Lines<BufReader<File>>,
}

impl BingoReader {
    /// Constructs a new BingoReader.
    /// Lines should be on the first bingo tile (past the drawn numbers).
    fn new(lines: Lines<BufReader<File>>) -> BingoReader {
        BingoReader { lines }
    }
}

impl Iterator for BingoReader {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        // A board is five lines of numbers, separated by spaces.
        let nums = (0..5)
            .flat_map(|_| self.lines.next())
            .map(|line| line.unwrap().trim().split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect())
            .collect::<Vec<Vec<usize>>>();

        self.lines.next();

        if nums.len() == 5 {
            Some(Board::new(nums))
        } else {
            None
        }
    }
}

/// BoardPosition captures the board and position where a number lives.
struct BoardPosition {
    board: usize,
    row: usize,
    col: usize,
}

impl BoardPosition {
    fn new(board: usize, row: usize, col: usize) -> Self {
        BoardPosition { board, row, col }
    }
}

/// Board is a single bingo board.
struct Board {
    nums: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    /// Builds a new Board from the given numbers
    fn new(nums: Vec<Vec<usize>>) -> Self {
        Self { nums, marked: vec![vec![false; 5]; 5]}
    }

    /// Marks the given number as called.
    fn mark(&mut self, row: usize, col: usize) {
        self.marked[row][col] = true;
    }

    /// Returns whether this board is a winner, which happens when all numbers on a row or column
    /// are marked.
    fn is_winner(&self) -> bool {
        for i in 0..self.marked.len() {
            let (row_winner, col_winner) = (0..5)
                .fold((true, true), |(row_win_acc, col_win_acc), j| (
                    row_win_acc && self.marked[i][j],
                    col_win_acc && self.marked[j][i],
                ));

            if row_winner || col_winner {
                return true;
            };
        }

        false
    }

    /// Returns the score for this board, which is the sum of all unmarked numbers multiplied
    /// by the number that was just drawn.
    fn score(&self, drawn: usize) -> usize {
        let unmarked: usize = (0..self.marked.len())
            .flat_map(move |row| (0..self.marked[row].len())
                .filter(move |&col| !self.marked[row][col])
                .map(move |col| self.nums[row][col]))
            .sum();

        unmarked * drawn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_sample() {
        let mut game = BingoGame::load("input/day4_sample.txt");
        assert_eq!(4512, game.play());
    }
}