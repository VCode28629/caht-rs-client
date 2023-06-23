use crate::process_io::*;
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    process::{ChildStdin, ChildStdout},
};
use websocket::{self, sync::Client, Message, OwnedMessage};

pub fn get_friend_list(
    uid: i64,
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<ChildStdout>,
    writer: &mut BufWriter<ChildStdin>,
) {
    send_message(client, Message::text(format!("get friend list\n{uid}")));
    let msg = receive_message(client);
    let id = -1;
    if let OwnedMessage::Text(text) = msg {
        for line in text.lines() {
            write_line(writer, line);
        }
        write_line(writer, "-1");
    } else {
        eprintln!("ERROR: get_friend_list got not text response.");
    }
}

pub fn get_group_list(
    uid: i64,
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<ChildStdout>,
    writer: &mut BufWriter<ChildStdin>,
) {
    send_message(client, Message::text(format!("get group list\n{uid}")));
    let msg = receive_message(client);
    let id = -1;
    if let OwnedMessage::Text(text) = msg {
        for line in text.lines() {
            write_line(writer, line);
        }
        write_line(writer, "-1");
    } else {
        eprintln!("ERROR: get_group_list got not text response.");
    }
}

pub fn get_username(uid: i64, client: &mut Client<TcpStream>) -> String {
    send_message(client, Message::text(format!("get username\n{uid}")));
    let msg = receive_message(client);
    if let OwnedMessage::Text(text) = msg {
        text
    } else {
        panic!("ERROR: get_group_list got not text response.");
    }
}
