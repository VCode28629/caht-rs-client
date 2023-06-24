use crate::process_io::*;
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    process::{ChildStdin, ChildStdout},
    sync::{mpsc::Sender, Arc, Mutex},
};
use websocket::{self, Message, OwnedMessage};

pub fn handle_login(
    sender: &Arc<Mutex<websocket::sync::Writer<TcpStream>>>,
    receiver: &mut websocket::sync::Reader<TcpStream>,
    reader: &mut BufReader<ChildStdout>,
    writer: &mut BufWriter<ChildStdin>,
    tx: &mut Sender<Vec<u8>>,
) -> Option<i64> {
    let username = read_line(reader);
    let password = read_line(reader);
    let mut sender = sender.lock().unwrap();
    sender_send_message(
        &mut sender,
        Message::text(format!("LOGIN\n{}\n{}", username, password)),
    );
    loop {
        let recived = reader_receive_message(receiver);
        match recived {
            OwnedMessage::Text(s) => {
                let mut message = s.split('\n');
                match message.next().expect("message is empty") {
                    "OK" => {
                        write_line(writer, "OK");
                        return Some(str::parse(message.next().unwrap()).unwrap());
                    }
                    "WRONG" => {
                        write_line(writer, "WRONG");
                        return None;
                    }
                    _ => {
                        eprintln!("ERROR: Invalid login response: {}", s);
                        return None;
                    }
                }
            }
            OwnedMessage::Ping(s) => {
                tx.send(s).unwrap();
                continue;
            }
            _ => {
                eprintln!("ERROR: login got not text response.");
                return None;
            }
        }
    }
}

pub fn handle_signup(
    sender: &Arc<Mutex<websocket::sync::Writer<TcpStream>>>,
    receiver: &mut websocket::sync::Reader<TcpStream>,
    reader: &mut BufReader<ChildStdout>,
    writer: &mut BufWriter<ChildStdin>,
    tx: &mut Sender<Vec<u8>>,
) {
    let username = read_line(reader);
    let password = read_line(reader);
    let mut sender = sender.lock().unwrap();
    sender_send_message(
        &mut sender,
        Message::text(format!("SIGN UP\n{}\n{}", username, password)),
    );
    let mut repeat = true;
    while repeat {
        repeat = false;
        let recv_message = reader_receive_message(receiver);
        match recv_message {
            OwnedMessage::Text(s) => {
                let mut message = s.split('\n');
                match message.next().expect("message is empty") {
                    "EXIST" => {
                        write_line(writer, "EXIST");
                    }
                    "OK" => {}
                    _ => {
                        eprintln!("ERROR: Invalid sign up response: {}", s);
                    }
                }
            }
            OwnedMessage::Ping(s) => {
                tx.send(s).unwrap();
                repeat = true;
            }
            _ => {
                eprintln!("ERROR: Invalid Message: {:?}", recv_message);
            }
        }
    }
}
