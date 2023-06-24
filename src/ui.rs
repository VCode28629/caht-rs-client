use crate::process_io::*;
use std::{
    io::BufWriter,
    net::TcpStream,
    process::ChildStdin,
    sync::{mpsc::Sender, Arc, Mutex},
};
use websocket::{self, Message, OwnedMessage};

pub fn get_friend_list(
    uid: i64,
    sender: &Arc<Mutex<websocket::sync::Writer<TcpStream>>>,
    receiver: &mut websocket::sync::Reader<TcpStream>,
    writer: &mut BufWriter<ChildStdin>,
    tx: &mut Sender<Vec<u8>>,
) {
    let mut sender = sender.lock().unwrap();
    sender_send_message(
        &mut sender,
        Message::text(format!("get friend list\n{uid}")),
    );
    let mut repeat = true;
    while repeat {
        repeat = false;
        let msg = reader_receive_message(receiver);
        match msg {
            OwnedMessage::Text(text) => {
                for line in text.lines() {
                    write_line(writer, line);
                }
                write_line(writer, "-1");
            }
            OwnedMessage::Ping(s) => {
                tx.send(s).unwrap();
                repeat = true;
            }
            _ => {
                eprintln!("ERROR: get_friend_list got not text response.");
            }
        }
    }
}

pub fn get_group_list(
    uid: i64,
    sender: &Arc<Mutex<websocket::sync::Writer<TcpStream>>>,
    receiver: &mut websocket::sync::Reader<TcpStream>,
    writer: &mut BufWriter<ChildStdin>,
    tx: &mut Sender<Vec<u8>>,
) {
    let mut sender = sender.lock().unwrap();
    sender_send_message(&mut sender, Message::text(format!("get group list\n{uid}")));
    let mut repeat = true;
    while repeat {
        repeat = false;
        let msg = reader_receive_message(receiver);
        match msg {
            OwnedMessage::Text(text) => {
                for line in text.lines() {
                    write_line(writer, line);
                }
                write_line(writer, "-1");
            }
            OwnedMessage::Ping(s) => {
                tx.send(s).unwrap();
                repeat = true;
            }
            _ => {
                eprintln!("ERROR: get_group_list got not text response.");
            }
        }
    }
}

pub fn get_username(
    uid: i64,
    sender: &Arc<Mutex<websocket::sync::Writer<TcpStream>>>,
    receiver: &mut websocket::sync::Reader<TcpStream>,
    tx: &mut Sender<Vec<u8>>,
) -> String {
    let mut sender = sender.lock().unwrap();
    sender_send_message(&mut sender, Message::text(format!("get username\n{uid}")));
    loop {
        let msg = reader_receive_message(receiver);
        match msg {
            OwnedMessage::Text(text) => {
                return text;
            }
            OwnedMessage::Ping(s) => {
                tx.send(s).unwrap();
                continue;
            }
            _ => {
                panic!("ERROR: get_group_list got not text response.");
            }
        }
    }
}
