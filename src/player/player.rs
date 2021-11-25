use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::{self};
use super::super::board::board::{get_board_from_string, Slot, GameResult, print_board};
use super::super::board::color::{Color, COLOR_RED};
use super::super::board::board_server::{DONE, EWAITING_FOR_PLAYER};
use super::helper::{connect, recv};
use std::thread;
use std::time::Duration;


pub struct Player {
    pub color: Color,
    sock_addr: SocketAddr,
}

impl Player {
    pub fn new(addr: String) -> Player {
        let sock_addr = match addr.parse() {
            Ok(a) => a,
            Err(e) => panic!("Error[parse]: {}", e),
        };

        return Player{
            color: COLOR_RED,
            sock_addr: sock_addr,
        }
    }

    fn send(&mut self, stream: &mut TcpStream, 
            data: &[u8]) -> io::Result<bool> {
        match stream.write(data) {
            Ok(n) if n < data.len() => { return Err(io::ErrorKind::WriteZero.into()); },
            Ok(_) => (),
            Err(e) => { return Err(e); },
        }

        Ok(true)
    }

    pub fn register(&mut self) -> Result<bool, String> {
        let msg = format!("register");
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            },
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let buf = match recv(&mut stream) {
            Some(buf) => buf,
            None => {
                return Err("Empty response".to_string());
            }
        };

        match String::from_utf8(buf) {
            Ok(r) => {
                let reply: Vec<&str> = r.trim().split(' ').collect();
                self.color.value = match reply[0].parse() {
                    Ok(val) => val,
                    Err(e) => return Err(e.to_string()),
                };
            },
            Err(e) => return Err(e.to_string())
        };

        Ok(true)
    }

    pub fn block_until_can_place(&mut self) -> bool {
        let mut need_refresh = false;
        let mut promt = true;

        while ! match self.can_place() {
            Ok(true) => return need_refresh,
            Ok(false) => { 
                if promt {
                    promt = false;
                    println!("Waiting for the other player...");
                }
                need_refresh = true; false }
            Err(e) => { eprintln!("Error[can_place]: {}", e); false},
        }{
            thread::sleep(Duration::from_secs(2));
        }

        need_refresh
    }


    pub fn can_place(&mut self) -> Result<bool, String> {
        let msg = format!("can_place {}", self.color.value);
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            }
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        match recv(&mut stream) {
            Some(buf) => {
                match String::from_utf8(buf) {
                    Ok(r) => {
                        let reply: Vec<&str> = r.trim().split(' ').collect();
                        match reply[0].parse() {
                            Ok(r) => return Ok(r),
                            Err(e) => return Err(e.to_string()),
                        }
                    },
                    Err(e) => return Err(e.to_string()),
                }
            },
            None => return Err("Empty response".to_string()),
        }

        //Ok(true)
    }

    pub fn place(&mut self, slot: Slot) -> Result<bool, String> {
        let msg = format!("place {} {}", slot.index, self.color.value);
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            }
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        match recv(&mut stream) {
            Some(buf) => {
                match String::from_utf8(buf) {
                    Ok(r) => {
                        println!("{:?}", r);
                        match r.trim() {
                            DONE => (),
                            _ => return Err(r.to_string()), 
                        }
                    },
                    Err(e) => return Err(e.to_string()),
                }
            },
            None => return Err("Empty response".to_string()),
        }

        Ok(true)
    }

    pub fn get_winner(&mut self) -> Result<bool, String> {
        let msg = String::from("get_winner");
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            }
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let buf = match recv(&mut stream) {
            Some(buf) => buf,
            None => {
                return Err("Empty response".to_string());
            }
        };

        match String::from_utf8(buf) {
            Ok(r) => {
                let reply: Vec<&str> = r.trim().split(' ').collect();
                let res: u8 = match reply[0].parse() {
                    Ok(r) => r,
                    Err(e) => return Err(e.to_string()),
                };

                match GameResult::from_u8(res) {
                    Some(GameResult::NoWinner) => {
                        println!("No winner for this round");
                        return Ok(true)
                    },
                    Some(GameResult::Unknown) => {
                        return Ok(false)
                    },
                    Some(GameResult::Winner) => {
                        if reply.len() < 2 {
                            return Err("Incomplete response from server".to_string());
                        }

                        let winner: u8 = match reply[1].parse() {
                            Ok(val) => val,
                            Err(e) => return Err(format!("{}", e)),
                        };
                        
                        if winner == self.color.value {
                            println!("You won!");
                        } else {
                            println!("You lose!");
                        }
                        return Ok(true)
                    },
                    _ => (),
                }
            },
            Err(e) => return Err(e.to_string())
        }

        Ok(false)
    }

    pub fn get_board(&mut self) -> Result<bool, String> {
        let msg = String::from("get_board");
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            }
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let buf = match recv(&mut stream) {
            Some(buf) => buf,
            None => {
                return Err("Empty response".to_string());
            }
        };

        match String::from_utf8(buf) {
            Ok(r) => {
                let reply: Vec<&str> = r.trim().split(' ').collect();
                let board = match get_board_from_string(reply[0].to_string()) {
                    Some(b) => b,
                    None => return Err("failed to get board from string".to_string()),
                };
                print_board(&board);
            },
            Err(e) => return Err(e.to_string())
        }

        Ok(true)
    }

    pub fn reset_board(&mut self) -> Result<bool, String> {
        let msg = String::from("reset_board");
        let mut stream = match connect(self.sock_addr) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error[connect]: {}", e));
            }
        };

        match self.send(&mut stream, msg.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let buf = match recv(&mut stream) {
            Some(buf) => buf,
            None => {
                return Err("Empty response".to_string());
            }
        };

        match String::from_utf8(buf) {
            Ok(r) => {
                match r.as_str() {
                     DONE => (),
                     EWAITING_FOR_PLAYER => (),
                     _ => return Err(r.to_string()), 
                 }
            },
            Err(e) => return Err(e.to_string())
        }
        
        Ok(true)
    }
}
