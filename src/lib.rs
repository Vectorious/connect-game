#[macro_use]
extern crate lazy_static;
extern crate geom;

pub use geom::point::*;
pub use geom::rect::Rect;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Connect {
    board: Rect<i32>,
    players: Vec<HashSet<Point<i32>>>,
    win_condition: usize
}

impl Connect {
    pub fn new(board_width: u16, board_height: u16, players: usize, win_condition: usize) -> Connect {
        Connect {
            board: Rect::new(0, 0, board_width as i32, board_height as i32),
            players: vec![HashSet::new(); players],
            win_condition: win_condition
        }
    }

    pub fn four() -> Connect {
        Connect {
            board: Rect::new(0, 0, 7, 6),
            players: vec![HashSet::new(); 2],
            win_condition: 4
        }
    }

    pub fn board(&self) -> &Rect<i32> {
        &self.board
    }

    pub fn players(&self) -> usize {
        self.players.len()
    }

    pub fn get_player(&self, player: usize) -> Option<&HashSet<Point<i32>>> {
        self.players.get(player)
    }

    pub fn who_owns(&self, point: &Point<i32>) -> Option<usize> {
        for (player, player_points) in self.players.iter().enumerate() {
            if player_points.contains(point) {
                return Some(player)
            }
        }
        None
    }

    pub fn push_move(&mut self, x: i32, player: usize) -> Result<Event, ConnectError> {
        let point = try!(self.push(x, player));
        Ok(
            match self.check_for_winner(point, player) {
                Some(line) => { Event::PlayerWon(line) }
                None => {
                    if self.is_board_full() {
                        Event::BoardFull
                    } else {
                        Event::PointPlaced(point)
                    }
                }
            }
        )
    }

    pub fn clear(&mut self) {
        for player_points in self.players.iter_mut() {
            player_points.clear()
        }
    }

    fn push(&mut self, x: i32, player: usize) -> Result<Point<i32>, ConnectError> {
        let column: Rect<i32> = match self.board.columns().get(x as usize) {
            Some(c) => { *c }
            None => { return Err(ConnectError::ColumnOutOfBounds(x)) }
        };

        let mut result: Result<Point<i32>, ConnectError> = Err(ConnectError::ColumnFull(x));
        for point in column.iter().rev() {
            let mut valid_point = true;
            for player_points in self.players.iter() {
                if player_points.contains(&point) {
                    valid_point = false;
                    break
                }
            }
            if valid_point {
                match self.players.get_mut(player) {
                    Some(player_points) => { player_points.insert(point); }
                    None => { return Err(ConnectError::InvalidPlayer(player)) }
                }
                result = Ok(point);
                break
            }
        }

        result
    }

    fn check_for_winner(&self, point: Point<i32>, player: usize) -> Option<HashSet<Point<i32>>> {
        lazy_static! {
            static ref UP: Point<i32> = Point::new(0, -1);
            static ref DOWN: Point<i32> = Point::new(0, 1);
            static ref LEFT: Point<i32> = Point::new(-1, 0);
            static ref RIGHT: Point<i32> = Point::new(1, 0);
            static ref UP_LEFT: Point<i32> = *UP + *LEFT;
            static ref UP_RIGHT: Point<i32> = *UP + *RIGHT;
            static ref DOWN_LEFT: Point<i32> = *DOWN + *LEFT;
            static ref DOWN_RIGHT: Point<i32> = *DOWN + *RIGHT;
        }

        let player_points = self.players.get(player).unwrap();

        let lines = [[*UP, *DOWN],              // vertical
                     [*LEFT, *RIGHT],           // horizontal
                     [*UP_LEFT, *DOWN_RIGHT],   // diagonal
                     [*UP_RIGHT, *DOWN_LEFT]];  // counter-diagonal

        for directions in lines.iter() {
            let mut line = HashSet::new();
            line.insert(point);
            for direction in directions {
                let mut cur_point = point + *direction;
                while player_points.contains(&cur_point) {
                    line.insert(cur_point);
                    cur_point = cur_point + *direction;
                }
            }
            if line.len() >= self.win_condition {
                return Some(line)
            }
        }

        None
    }

    pub fn is_board_full(&self) -> bool {
        self.players.iter().fold(0, |acc, player_points| acc + player_points.len()) >= self.board.area() as usize
    }
}

#[derive(Debug, PartialEq)]
pub enum Event {
    PlayerWon(HashSet<Point<i32>>),
    PointPlaced(Point<i32>),
    BoardFull
}

#[derive(Debug, PartialEq)]
pub enum ConnectError {
    ColumnFull(i32),
    ColumnOutOfBounds(i32),
    InvalidPlayer(usize),
}
