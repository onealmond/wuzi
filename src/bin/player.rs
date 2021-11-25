use wuzi::player::player::Player;
use wuzi::board::board::{Slot};
use wuzi::config::defaults::DEFAULT_ADDRESS;
use std::process;
use std::io::Write;


fn round(player: &mut Player) {
    match player.register() {
        Ok(_) => {println!("successfully registered as color {}", player.color.name());
        },
        Err(e) => {
            println!("Error[register]: {}", e);
            process::exit(1);
        }
    }

    loop {
        match player.get_board() {
            Ok(_) => (),
            Err(e) => { eprintln!("Error[get_board]: {}", e); continue },
        }

        match player.get_winner() {
            Ok(true) => break,
            Ok(false) => (),
            Err(e) => { eprintln!("Error[get_winner]: {}", e); continue },
        }

        if player.block_until_can_place() {
            match player.get_board() {
                Ok(_) => (),
                Err(e) => { eprintln!("Error[get_board]: {}", e); continue },
            }

            match player.get_winner() {
                Ok(true) => break,
                Ok(false) => (),
                Err(e) => { eprintln!("Error[get_winner]: {}", e); continue },
            }
        }

        print!("Your turn >>> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(n) if n > 0 => {
                let i = match line.trim().parse() {
                    Ok(i) => i,
                    Err(e) => {
                        eprintln!("Error[parse]: {:?}", e);
                        continue
                    }
                };
    
                match player.place(Slot{index: i}) {
                    Err(e) => eprintln!("Error[place]: {}", e),
                    _ => (),
                }
            }
            Ok(_) => (),
            Err(e) => eprintln!("Error[read_line]: {}", e),
        };
    }
}

use std::env;

fn main() {
    let mut addr = DEFAULT_ADDRESS;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        addr = args[1].as_str();
    }
    let mut player = Player::new(addr.to_string());

    loop {
        round(&mut player);
        print!("Another round? (y/n) >>> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.trim().to_lowercase() == "n" {
                    break
                }
            },
            Err(e) => {
                eprintln!("Error[parse]: {:?}", e);
                process::exit(0);
            }
        }

        match player.reset_board() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
