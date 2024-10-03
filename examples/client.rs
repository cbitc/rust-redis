use std::io::stdin;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    loop {
        let mut line = String::default();
        stdin().read_line(&mut line).unwrap();
        // line.push('\n');
        stream.write_all(line.as_ref()).await.unwrap();
    }
}
