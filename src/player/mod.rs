pub mod player;
pub mod helper;

mod tests {
    #[cfg(test)]
    use super::player::Player;
    #[cfg(test)]
    use super::super::board::board::{Slot};

    #[test]
    fn test_connection() {

        let server_addr = "127.0.0.1:5555".parse().unwrap();
        let mut player = Player::new(server_addr);
        
        match player.register() {
            Ok(_) => println!("successfully registered as color {}", player.color.name()),
            Err(e) => println!("Error: {}", e),
        }

        match player.place(Slot{index: 3}) {
            Err(e) => eprintln!("Error[place]: {}", e),
            _ => (),
        }
    }
}
