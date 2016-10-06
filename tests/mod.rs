extern crate connectgame;
use connectgame::*;
use std::collections::HashSet;

#[test]
fn test_connect_four_win() {
        let mut game = Connect::four();
        let win_set_a = {
            let mut win_set = HashSet::new();
            win_set.insert(Point::new(0, 1));
            win_set.insert(Point::new(0, 0));
            win_set.insert(Point::new(0, 2));
            win_set.insert(Point::new(0, 3));
            win_set
        };

        let win_set_b = {
            let mut win_set = HashSet::new();
            win_set.insert(Point::new(0, 5));
            win_set.insert(Point::new(1, 5));
            win_set.insert(Point::new(2, 5));
            win_set.insert(Point::new(3, 5));
            win_set
        };

        let win_set_c = {
            let mut win_set = HashSet::new();
            win_set.insert(Point::new(3, 4));
            win_set.insert(Point::new(2, 4));
            win_set.insert(Point::new(1, 4));
            win_set.insert(Point::new(0, 4));
            win_set
        };

        let win_set_d = {
            let mut win_set = HashSet::new();
            win_set.insert(Point::new(4, 5));
            win_set.insert(Point::new(3, 4));
            win_set.insert(Point::new(2, 3));
            win_set.insert(Point::new(1, 2));
            win_set
        };

        assert_eq!(game.push_move(0, 0), Ok(Event::PointPlaced(Point::new(0, 5))));
        assert_eq!(game.push_move(0, 1), Ok(Event::PointPlaced(Point::new(0, 4))));
        assert_eq!(game.push_move(0, 0), Ok(Event::PointPlaced(Point::new(0, 3))));
        assert_eq!(game.push_move(0, 0), Ok(Event::PointPlaced(Point::new(0, 2))));
        assert_eq!(game.push_move(0, 0), Ok(Event::PointPlaced(Point::new(0, 1))));
        assert_eq!(game.push_move(0, 0), Ok(Event::PlayerWon(win_set_a)));  // vertical, player 0

        assert_eq!(game.push_move(1, 0), Ok(Event::PointPlaced(Point::new(1, 5))));
        assert_eq!(game.push_move(2, 0), Ok(Event::PointPlaced(Point::new(2, 5))));
        assert_eq!(game.push_move(3, 0), Ok(Event::PlayerWon(win_set_b)));  // horizontal, player 0

        assert_eq!(game.push_move(2, 1), Ok(Event::PointPlaced(Point::new(2, 4))));
        assert_eq!(game.push_move(3, 1), Ok(Event::PointPlaced(Point::new(3, 4))));
        assert_eq!(game.push_move(1, 1), Ok(Event::PlayerWon(win_set_c)));  // horizontal, player 1

        assert_eq!(game.push_move(4, 1), Ok(Event::PointPlaced(Point::new(4, 5))));
        assert_eq!(game.push_move(2, 1), Ok(Event::PointPlaced(Point::new(2, 3))));
        assert_eq!(game.push_move(1, 1), Ok(Event::PointPlaced(Point::new(1, 3))));
        assert_eq!(game.push_move(1, 1), Ok(Event::PlayerWon(win_set_d)));  // diagonal, player 1
}

game::xD :-P
