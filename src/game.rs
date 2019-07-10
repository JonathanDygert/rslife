use std::collections::{HashMap, HashSet};
use std::fs;

use scan_rules::{scan, scan_rules_impl, scanner::Line};

use rand::distributions::Bernoulli;
use rand::prelude::*;

use crate::Point;

type Board = HashSet<Point<i32>>;

#[derive(Clone, Debug)]
/// This represents the game of life at a certain generation.
pub struct Game {
    generation: u32,
    board: Board,
}

impl Game {
    /// Creates a new Game from a board, at generation 0.
    pub fn new(board: Board) -> Self {
        Game {
            generation: 0,
            board,
        }
    }

    /// Creates an Game from an empty board, at generation 0.
    pub fn empty() -> Self {
        Self::new(Board::new())
    }

    /// Gets the current generation of the game. This increases by one for each
    /// simulated tick of the cells.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Gets the currently active points.
    pub fn active<'a>(&'a self) -> impl Iterator<Item = Point<i32>> + 'a {
        self.board.iter().copied()
    }

    /// Advances the game by one generation.
    pub fn advance(&mut self) {
        self.generation += 1;

        let mut counts = HashMap::new();
        for &p in &self.board {
            for n in p.neighbors() {
                *counts.entry(n).or_insert(0) += 1;
            }
        }

        self.board = counts
            .into_iter()
            .filter(|&(p, count)| count == 3 || count == 4 && self.board.contains(&p))
            .map(|(p, _)| p)
            .collect();
    }

    /// Creates a random game within the given dimensions.
    pub fn random(width: u32, height: u32) -> Self {
        let mut rng = thread_rng();
        let dist = Bernoulli::new(0.05).unwrap();
        let h = (height / 2) as i32;
        let w = (width / 2) as i32;
        let board = (-w..w)
            .flat_map(|x| (-h..h).map(move |y| Point { x, y }))
            .filter(|_| rng.sample(dist))
            .collect();
        Self::new(board)
    }

    #[allow(clippy::cognitive_complexity)]
    /// Creates a game by parsing an rle file at the given path.
    pub fn rle(path: &str) -> crate::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut board = Board::new();
        scan!(&contents; (
            ["#", let _: Line<'_, &str>]*,
            "x", "=", let w: i32, ",", "y", "=", let h: i32, let _: Line<'_, &str>,
            [[let count: i32]?, let c: char]*
        ) => {
            let start = Point {
                x: -w / 2,
                y: -h / 2,
            };
            let mut pos = start;
            for (count, c) in count.into_iter().map(|mut c| c.pop().unwrap_or(1)).zip(c) {
                match c {
                    'o' => {
                        for x in pos.x..pos.x + count {
                            board.insert(Point { x, y: pos.y });
                        }
                        pos.x += count;
                    }
                    'b' => {
                        pos.x += count;
                    }
                    '$' => {
                        pos.y += 1;
                        pos.x = start.x;
                    }
                    '!' => {
                        break;
                    }
                    _ => {
                        println!("Ignoring character {}", c)
                    }
                }
            }
            Ok(Game::new(board))
        })?
    }
}
