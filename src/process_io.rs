use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
    process::{ChildStdin, ChildStdout},
};
use websocket::{self, sync::{Client, Reader, Writer}, Message, OwnedMessage};

pub fn receive_message(client: &mut Client<TcpStream>) -> OwnedMessage {
    let s = client.recv_message().unwrap();
    eprintln!("Received: {:?}", s);
    s
}
pub fn reader_receive_message(client: &mut Reader<TcpStream>) -> OwnedMessage {
    let s = client.recv_message().unwrap();
    eprintln!("Received: {:?}", s);
    s
}

pub fn send_message(client: &mut Client<TcpStream>, message: Message) {
    eprintln!("Sending: {:?}", message);
    client.send_message(&message).unwrap();
}

pub fn sender_send_message(client: &mut Writer<TcpStream>, message: Message) {
    eprintln!("Sending: {:?}", message);
    client.send_message(&message).unwrap();
}

pub fn write_line(writer: &mut BufWriter<ChildStdin>, message: &str) {
    eprintln!("Writing: {}", message);
    writer.write(message.as_bytes()).unwrap();
    writer.write("\n".as_bytes()).unwrap();
    writer.flush().unwrap();
}

pub fn read_line(reader: &mut BufReader<ChildStdout>) -> String {
    let mut res = String::new();
    reader.read_line(&mut res).unwrap();
    let res = res.trim_end().to_string();
    eprintln!("Read: {}", res);
    res
}
