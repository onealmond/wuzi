pub mod color;
pub mod board;
pub mod board_server;

mod tests {
    #[cfg(test)]
    use super::board::{Board, print_board, Slot};
    #[cfg(test)]
    use super::color::{COLOR_RED, COLOR_GREEN};

    #[test]
    fn test_rows() {
        let mut board = Board::new();

        for i in 0..5 {
            match board.place(Slot{index: i}, &COLOR_GREEN) {
                Ok(won) => {
                    if won {
                        println!("GREEN: {}", won);
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
        assert!(board.is_winner(&COLOR_GREEN));
    }

    #[test]
    fn test_slots() {
        let mut board = Board::new();

        for _ in 0..5 {
            match board.place(Slot{index: 2}, &COLOR_GREEN) {
                Ok(won) => {
                    if won {
                        println!("GREEN: {}", won);
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
        assert!(board.is_winner(&COLOR_GREEN));
    }

    #[test]
    fn test_diagonals() {
        let mut board = Board::new();

        for i in 0..25 {
            if i % 2 == 0 {
                match board.place(Slot{index: i%5}, &COLOR_GREEN) {
                    Ok(won) => {
                        if won {
                            println!("GREEN: {}", won);
                        }
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                }
            }
            else {
                match board.place(Slot{index: i%5}, &COLOR_RED) {
                    Ok(won) => {
                        if won {
                            println!("RED: {}", won);
                        }
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                }
            }
        }

        print_board(board.get_board());
        assert!(board.is_winner(&COLOR_GREEN));
    }
}
