use std::io::prelude::*;
use mio::event::Event;
use mio::{Poll, Events, Interest, Token, Registry};
use mio::net::{TcpListener, TcpStream};
use super::board::{Board, MAX_PLAYERS, Slot, GameResult};
use super::color::{COLOR_GREEN, COLOR_RED, Color};

use std::io::{self};
use std::collections::HashMap;

pub struct PlayerProfile {
    color: Color,
}

const SERVER: Token = Token(0);

pub const DONE: &str = "Done";   // Operation succeeded
pub const EUNREGISTRABLE: &str = "All players have been registered";
pub const ENO_ENOUGH_ARGUMENT: &str = "No enough arguments";
pub const EWAITING_FOR_PLAYER: &str = "Waiting for player";

pub struct BoardServer {
    board: Board,
    players: Vec<PlayerProfile>,
    avail_colors: Vec<Color>,
    next_turn: usize,
}

pub fn recv(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buf = vec![0; 1024];
    let mut cur = 0;

    loop {
        match stream.read(&mut buf) {
            Ok(0) => { break; }
            Ok(n) => {
                cur += n;
                if cur == buf.len() {
                    buf.resize(buf.len() + 1024, 0);
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => { break }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => { continue }
            Err(e) => {
                eprintln!("Error[recv]: {}", e.to_string());
                return None
            }
        };
    }

    if cur > 0 {
        return Some((&buf[..cur]).to_vec());
    }
    return None;
}

impl BoardServer {

    pub fn new() -> BoardServer { 
        BoardServer {
            board: Board::new(),
            players: Vec::with_capacity(MAX_PLAYERS),
            avail_colors: vec![COLOR_GREEN, COLOR_RED],
            next_turn: MAX_PLAYERS-1,
          
        }
    }

    pub fn serve(&mut self, addr: String) {
        let mut next_token = Token(SERVER.0 + 1);
        let mut events = Events::with_capacity(64);
        let mut connections: HashMap<Token, TcpStream> = HashMap::new();

        let sock_addr = match addr.parse() {
            Ok(a) => a,
            Err(e) => panic!("Error[address parse]: {}", e),
        };

        let mut poll = match Poll::new() {
            Ok(p) => p,
            Err(e) => panic!("Error[poll::new]: {}", e),
        };

        let mut listener = match TcpListener::bind(sock_addr) {
            Ok(l) => l,
            Err(e) => panic!("{}", e),
        };

        match poll.registry().register(&mut listener, SERVER, Interest::READABLE|Interest::WRITABLE) {
            Err(e) => panic!("Error[register]: {}", e),
            _ => (),
        }

        loop {
            match poll.poll(&mut events, None) {
                Err(e) => {
                    eprintln!("Error[poll]: {}", e);
                    break
                },
                _ => (),
            }
             
            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        match listener.accept() {
                            Ok((mut stream, _addr)) => {
                                //println!("accepted connection from {}", addr);
                                let token = Token(next_token.0);
                                next_token.0 += 1;
                                match poll.registry().register(&mut stream, token, Interest::READABLE.add(Interest::WRITABLE)) {
                                    Err(e) => {
                                        eprintln!("Error[register event]: {}", e);
                                        continue
                                    },
                                    _ => (),
                                }
                                connections.insert(token, stream); 
                            },
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => eprintln!("Error[accept]: {}", e),
                        };
                    },
                    token => {
                        let done = if let Some(stream) = connections.get_mut(&token) {
                            match self.handle_connection(poll.registry(), stream, event) {
                                Ok(d) => d,
                                Err(e) => {
                                    eprintln!("Error[handle_connection]: {}", e);
                                    false
                                },
                            }
                        } else { false };

                        if done {
                            if let Some(mut stream) = connections.remove(&token) {
                                match poll.registry().deregister(&mut stream) {
                                    Err(e) => eprintln!("Error[deregister]: {}", e),
                                    _ => (),
                                }
                            }
                        }
                    },
                }
            }
        }
    }

    fn register_player(&mut self, _args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        //println!("{:?}", args);
        if !self.avail_colors.is_empty() {
            let color = self.avail_colors.pop().unwrap();
            self.players.push(PlayerProfile{color: color,});
            return self.send(registry, stream, event, format!("{}", color.value).as_bytes());
        } else {
            return self.send(registry, stream, event, EUNREGISTRABLE.as_bytes());
        }
    }

    fn place(&mut self, args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        println!("{:?}", args);
        if args.len() < 3 {
            return self.send(registry, stream, event, ENO_ENOUGH_ARGUMENT.as_bytes());
        }

        let index = match args[1].parse() {
            Ok(i) => i,
            Err(e) => {
                return self.send(registry, stream, event, format!("{}", e).as_bytes());
            },
        };

        let color = match args[2].parse() {
            Ok(i) => i,
            Err(e) => {
                return self.send(registry, stream, event, format!("{}", e).as_bytes());
            },
        };

        match self.board.place(Slot{index: index}, &Color{value: color}) {
            Ok(_) => {
                self.next_turn = (self.next_turn + 1) % self.players.len();
                return self.send(registry, stream, event, DONE.as_bytes());
            },
            Err(e) => {
                return self.send(registry, stream, event, format!("Error: {}", e).as_bytes());
            },
        }
    }

    fn send(&mut self, registry: &Registry, stream: &mut TcpStream, 
            event: &Event, data: &[u8]) -> io::Result<bool> {
        match stream.write(data) {
            Ok(n) if n < data.len() => { return Err(io::ErrorKind::WriteZero.into()) },
            Ok(_) => {
                match registry.reregister(stream, event.token(), Interest::READABLE) {
                    Ok(_) => (),
                    Err(e) => { return Err(e) },
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => { 
                return self.handle_connection(registry, stream, event)
            },
            Err(e) => { println!("{}", e); return Err(e); },
        }

        Ok(true)
    }

    fn get_board(&mut self, _args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        //println!("{:?}", args);
        let state = format!("{}", self.board.get_board_as_string());
        return self.send(registry, stream, event, state.as_bytes());
    }

    fn get_winner(&mut self, _args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        //println!("{:?}", args);
        for p in & self.players {
            if self.board.is_winner(&p.color) {
                let winner = format!("{} {}", GameResult::Winner as u8, p.color.value);
                return self.send(registry, stream, event, winner.as_bytes());
            }
        }

        if self.board.is_full() {
            return self.send(registry, stream, event, format!("{}", GameResult::NoWinner as u8).as_bytes());
        }

        return self.send(registry, stream, event, format!("{}", GameResult::Unknown as u8).as_bytes());
    }

    fn can_place(&mut self, args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        //println!("{:?}", args);
        if args.len() < 2 {
            return self.send(registry, stream, event, ENO_ENOUGH_ARGUMENT.as_bytes());
        }

        if self.players.len() < MAX_PLAYERS {
            return self.send(registry, stream, event, format!("{}", false).as_bytes());
        }

        let color: u8 = match args[1].parse() {
            Ok(i) => i,
            Err(e) => {
                return self.send(registry, stream, event, format!("{}", e).as_bytes());
            },
        };

        return self.send(registry, stream, event, 
                         format!("{}", color == self.players[self.next_turn].color.value).as_bytes());
    }

    fn reset_board(&mut self, args: Vec<&str>, registry: &Registry,
                stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        println!("{:?}", args);
        if self.players.len() < MAX_PLAYERS {
            return self.send(registry, stream, event, EWAITING_FOR_PLAYER.as_bytes());
        }
        self.board = Board::new();
        self.avail_colors = vec![COLOR_GREEN, COLOR_RED];
        self.players.clear();
        return self.send(registry, stream, event, DONE.as_bytes());
    }

    fn handle_connection(&mut self, registry: &Registry,
                         stream: &mut TcpStream, event: &Event) -> io::Result<bool> {
        if event.is_readable() {
            let buf = match recv(stream) {
                Some(buf) => buf,
                None => {
                    eprintln!("Error: No data recieved");
                    return Ok(false)
                },
            };

            match String::from_utf8(buf) {
                Ok(v) => {
                    let cmd: Vec<&str> = v.trim().split(" ").collect();
                    if cmd.len() > 0 {
                        match cmd[0] {
                            "register" => return self.register_player(cmd, registry, stream, event),
                            "place" => return self.place(cmd, registry, stream, event),
                            "get_board" => return self.get_board(cmd, registry, stream, event),
                            "get_winner" => return self.get_winner(cmd, registry, stream, event),
                            "reset_board" => return self.reset_board(cmd, registry, stream, event),
                            "can_place" => return self.can_place(cmd, registry, stream, event),
                            _ => (),
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error: {}", e.to_string());
                }
            }
        } // if event.is_readable

        if event.is_writable() {
            return Ok(true);
        }

        Ok(false)
    }
}
