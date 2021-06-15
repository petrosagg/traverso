use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;

use crate::message::Message;

mod message;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("keftes.di.uoa.gr:18018").await.unwrap();

    let (reader, _writer) = stream.split();
    let reader = BufReader::new(reader);

    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let message: Message = serde_json::from_str(&line).unwrap();

        println!("received message: {:?}", &message);
    }
}
