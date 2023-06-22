use crate::process_io::*;
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    process::{ChildStdin, ChildStdout},
};
use websocket::{self, sync::Client, Message, OwnedMessage};

pub fn handle_login(
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<&mut ChildStdout>,
    writer: &mut BufWriter<&mut ChildStdin>,
) -> Option<i64> {
    let username = read_line(reader);
    let password = read_line(reader);
    send_message(
        client,
        Message::text(format!("LOGIN\n{}\n{}", username, password)),
    );
    let recived = receive_message(client);
    if let OwnedMessage::Text(s) = recived {
        let mut message = s.split('\n');
        match message.next().expect("message is empty") {
            "OK" => {
                write_line(writer, "OK");
                Some(str::parse(message.next().unwrap()).unwrap())
            }
            "WRONG" => {
                write_line(writer, "WRONG");
                None
            }
            _ => {
                eprintln!("ERROR: Invalid login response: {}", s);
                None
            }
        }
    } else {
        eprintln!("ERROR: login got not text response.");
        None
    }
}

pub fn handle_signup(
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<&mut ChildStdout>,
    writer: &mut BufWriter<&mut ChildStdin>,
) {
    let username = read_line(reader);
    let password = read_line(reader);
    send_message(
        client,
        Message::text(format!("SIGN UP\n{}\n{}", username, password)),
    );
    let recv_message = receive_message(client);
    if let OwnedMessage::Text(s) = recv_message {
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
    } else {
        eprintln!("ERROR: Invalid Message: {:?}", recv_message);
    }
}
