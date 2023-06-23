mod login;
mod process_io;
mod ui;

use configparser;
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    process::{Command, Stdio},
    sync::{mpsc, Arc, Mutex},
    thread,
};
use ui::{get_friend_list, get_group_list, get_username};
use websocket::{self, sync::Client, Message, OwnedMessage};

use login::*;
use process_io::*;

fn login(client: &mut Client<TcpStream>) -> Option<i64> {
    // websocket connection
    let mut login = Command::new("login")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut writer = BufWriter::new(login.stdin.unwrap());
    let mut reader = BufReader::new(login.stdout.unwrap());

    let mut uid: Option<i64> = None;
    loop {
        let input = read_line(&mut reader);
        match input.as_str() {
            "Login" => {
                uid = handle_login(client, &mut reader, &mut writer);
                if let Some(_) = uid {
                    break;
                }
            }
            "SignUp" => {
                handle_signup(client, &mut reader, &mut writer);
            }
            "Exit" => {
                eprintln!("Exiting...");
                break;
            }
            _ => {
                eprintln!("unreachable input: {input}");
                unreachable!()
            }
        }
    }
    uid
}

fn start(mut client: Client<TcpStream>, uid: i64) {
    let mut ui = Command::new("ui")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut writer = BufWriter::new(ui.stdin.unwrap());
    let mut reader = BufReader::new(ui.stdout.unwrap());

    write_line(&mut writer, uid.to_string().as_str());
    let username = get_username(uid, &mut client);
    write_line(&mut writer, username.as_str());
    loop {
        let input = read_line(&mut reader);
        match input.as_str() {
            "getFrinedList" => {
                get_friend_list(uid, &mut client, &mut reader, &mut writer);
            }
            "getGroupList" => {
                get_group_list(uid, &mut client, &mut reader, &mut writer);
            }
            "Exit" => {
                eprintln!("Exiting...");
                return;
            }
            "START" => {
                break;
            }
            _ => {
                eprintln!("unreachable input: {input}");
                unreachable!()
            }
        }
    }

    let (mut receiver, mut sender) = client.split().unwrap();
    let (mut tx, mut rx) = mpsc::channel();

    let from_server = thread::spawn(move || -> ! {
        loop {
            let msg = reader_receive_message(&mut receiver);
            match msg {
                OwnedMessage::Text(s) => match s.lines().next().unwrap() {
                    "add friend" => {
                        write_line(&mut writer, s.as_str());
                    }
                    "DM" => {
                        write_line(&mut writer, "DM");
                        let x = s.lines().count() - 1;
                        write_line(&mut writer, &format!("{x}"));
                        for line in s.lines().skip(1) {
                            write_line(&mut writer, line);
                        }
                    }
                    "group message" => {
                        write_line(&mut writer, "group message");
                        let x = s.lines().count() - 1;
                        write_line(&mut writer, &format!("{x}"));
                        for line in s.lines().skip(1) {
                            write_line(&mut writer, line);
                        }
                    }
                    _ => {
                        eprintln!("unreachable input: {s}");
                        unreachable!()
                    }
                },
                OwnedMessage::Ping(s) => {
                    tx.send(s);
                }
                _ => {}
            }
        }
    });

    let mut sender = Arc::new(Mutex::new(sender));
    let mut sender2 = Arc::clone(&sender);

    let pingpong = thread::spawn(move || loop {
        if let Ok(x) = rx.recv() {
            let mut sender2 = sender2.lock().unwrap();
            sender_send_message(&mut sender2, Message::pong(x));
        }
    });

    let from_client = thread::spawn(move || loop {
        let s = read_line(&mut reader);
        let mut sender = sender.lock().unwrap();
        match s.as_str() {
            "DM" => {
                let uid = read_line(&mut reader);
                let tid = read_line(&mut reader);
                let s = read_line(&mut reader);
                let msg = Message::text(format!("send DM\n{uid}\n{tid}\n{s}"));
                sender_send_message(&mut sender, msg);
            }
            "GROUP" => {
                let uid = read_line(&mut reader);
                let gid = read_line(&mut reader);
                let s = read_line(&mut reader);
                let msg = Message::text(format!("send group\n{uid}\n{gid}\n{s}"));
                sender_send_message(&mut sender, msg);
            }
            "joinGroup" => {
                let gid = read_line(&mut reader);
                let msg = Message::text(format!("add group\n{uid}\n{gid}"));
                sender_send_message(&mut sender, msg);
            }
            "newDM" => {
                let id = read_line(&mut reader);
                let msg = Message::text(format!("add friend\n{uid}\n{id}"));
                sender_send_message(&mut sender, msg);
            }
            _ => {
                eprintln!("unreachable input: {s}");
                unreachable!()
            }
        }
    });

    from_client.join().unwrap();
    from_server.join().unwrap();
}

pub fn main() {
    env_logger::init();
    // read configuration
    let mut config = configparser::ini::Ini::new();
    config.load("config.ini").unwrap();
    let address = config.get("network", "host").unwrap();
    eprintln!("got address {}", address);
    let mut client = websocket::ClientBuilder::new(address.as_str())
        .unwrap()
        .connect_insecure()
        .unwrap();

    let uid = login(&mut client);
    match uid {
        Some(uid) => {
            eprintln!("login successfully, uid: {}", uid);
        }
        None => {
            eprintln!("failed to login");
            return;
        }
    }
    let uid = uid.unwrap();
    start(client, uid);
}
