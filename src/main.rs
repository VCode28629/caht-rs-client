mod login;
mod process_io;

use configparser;
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    process::{ChildStdin, ChildStdout, Command, Stdio},
};
use websocket::{self, sync::Client};

use login::*;
use process_io::*;

fn login(client: &mut Client<TcpStream>) -> Option<i64> {
    // websocket connection
    let mut login = Command::new("login")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut writer: BufWriter<&mut ChildStdin> = BufWriter::new(login.stdin.as_mut().unwrap());
    let mut reader: BufReader<&mut ChildStdout> = BufReader::new(login.stdout.as_mut().unwrap());
    // step1: login/signup
    let mut uid: Option<i64> = None;
    loop {
        let input = read_line(&mut reader);
        match input.as_str() {
            "Login" => {
                uid = handle_login(client, &mut reader, &mut writer);
                if let None = uid {
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
    drop(writer);
    login.wait().unwrap();
    uid
}

fn start(uid: i64) {
    let mut ui = Command::new("login")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut writer: BufWriter<&mut ChildStdin> = BufWriter::new(ui.stdin.as_mut().unwrap());
    let mut reader: BufReader<&mut ChildStdout> = BufReader::new(ui.stdout.as_mut().unwrap());

    loop {
        let input = read_line(&mut reader);
        match input.as_str() {
            _ => {
                eprintln!("unreachable input: {input}");
                unreachable!()
            }
        }
    }
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
    start(uid);
}
