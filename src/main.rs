use configparser;
use log::{debug, error, info, log_enabled, Level};
use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
    process::{Command, Stdio},
};
use websocket::{self, sync::Client, Message, OwnedMessage};

fn handle_login<R: Read, W: Write>(
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
) -> i64 {
    let mut username = String::new();
    reader
        .read_line(&mut username)
        .expect("handle_login cannot read the username");
    let mut password = String::new();
    reader
        .read_line(&mut password)
        .expect("handle_login cannot read the password");
    let username = username.trim_end();
    let password = password.trim_end();

    client
        .send_message(&Message::text(format!("LOGIN\n{}\n{}", username, password)))
        .unwrap();

    let recived = client.recv_message().unwrap();

    if let OwnedMessage::Text(s) = recived {
        let mut message = s.split('\n');
        match message.next().expect("message is empty") {
            "OK" => {
                writer.write("OK\n".as_bytes()).unwrap();
                str::parse(message.next().unwrap()).unwrap()
            }
            "WRONG" => {
                writer.write("WRONG\n".as_bytes()).unwrap();
                -1
            }
            _ => {
                eprintln!("ERROR: Invalid login response: {}", s);
                -1
            }
        }
    } else {
        eprintln!("ERROR: login got not text response.");
        -1
    }
}

fn handle_signup<R: Read, W: Write>(
    client: &mut Client<TcpStream>,
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
) {
    debug!("in fn handle_signup");
    let mut username = String::new();
    reader
        .read_line(&mut username)
        .expect("handle_signup cannot read the username");
    let mut password = String::new();
    reader
        .read_line(&mut password)
        .expect("handle_signup cannot read the password");
    let username = username.trim_end();
    let password = password.trim_end();

    debug!("username: {}", username);
    debug!("password: {}", password);

    let send_message = Message::text(format!("SIGN UP\n{}\n{}", username, password));
    client.send_message(&send_message).unwrap();
    let recv_message = client.recv_message().unwrap();
    if let OwnedMessage::Text(s) = recv_message {
        let mut message = s.split('\n');
        match message.next().expect("message is empty") {
            "EXIST" => {
                writer.write("EXIST\n".as_bytes()).unwrap();
            }
            "OK" => {
                writer.write("OK\n".as_bytes()).unwrap();
            }
            _ => {
                eprintln!("ERROR: Invalid sign up response: {}", s);
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
    println!("got address {}", address);

    // websocket connection
    let mut client = websocket::ClientBuilder::new(address.as_str())
        .unwrap()
        .connect_insecure()
        .unwrap();

    let mut login = Command::new("login")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut writer = BufWriter::new(login.stdin.as_mut().unwrap());
    let mut reader = BufReader::new(login.stdout.as_mut().unwrap());
    // step1: login/signup
    let mut uid: i64 = -1;
    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.trim_end();
        eprintln!("{input}");
        debug!("from login: {:?}", input);
        match input {
            "Login" => {
                uid = handle_login(&mut client, &mut reader, &mut writer);
                if uid != -1 {
                    break;
                }
            }
            "SignUp" => {
                handle_signup(&mut client, &mut reader, &mut writer);
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
        // let mut input = vec![];
        // reader.read_until('\0' as u8, &mut input).unwrap();
        // writer.write_all(input.as_bytes()).unwrap();
    }
    println!("login successfully, uid: {}", uid);
}
