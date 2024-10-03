use std::{env, io::Write};

use anyhow::Ok;
use log::{debug, info, trace};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

fn init_logger() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
}

#[tokio::main]
async fn main() {
    init_logger();
    info!("redis!");
    let server = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    info!("redis start up!");
    loop {
        let (stream, addr) = server.accept().await.unwrap();
        info!("connection:{:#?}", addr);
        tokio::spawn(async move {
            handle_connection(Connection::new(stream)).await;
        });
    }
}

async fn handle_connection(mut connection: Connection) {
    let mut chunk = [0; 32];
    loop {
        if let Some(frame) = connection.parse_frame() {
            debug!("{:#?}", frame);
        }

        let n = connection.stream.read(&mut chunk).await.unwrap();
        if n == 0 {
            break;
        }
        connection.buffer.write_all(&chunk[..n]).unwrap();
    }
}

struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::default(),
        }
    }

    fn parse_frame(&mut self) -> Option<Frame> {
        let mut i = 0;
        while i < self.buffer.len() && self.buffer[i] != b'\n' {
            i += 1;
        }

        if i < self.buffer.len() {
            let result = Frame::new(&self.buffer[..i]);
            let re_len = self.buffer.len() - i - 1;
            for j in 0..re_len {
                self.buffer[j] = self.buffer[j + i + 1];
            }
            self.buffer.resize(re_len, 0);
            return Some(result);
        }
        None
    }
}

#[derive(Debug)]
struct Frame {
    command: Command,
}

impl Frame {
    fn new(bytes: &[u8]) -> Self {
        use Command::*;
        let line = String::from_utf8(bytes.to_vec()).unwrap();
        let tokens: Vec<_> = line.trim().split(' ').collect();
        trace!("{:#?}", &line);
        let command = match *tokens.first().unwrap() {
            "set" => Set {
                key: tokens[1].into(),
                val: tokens[2].into(),
            },
            "get" => Get {
                key: tokens[1].into(),
            },
            other => panic!(),
        };
        Frame { command }
    }
}

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: String },
}
