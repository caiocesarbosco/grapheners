use tungstenite::{connect, Message};
use url::Url;
use json::{object, stringify};

/// A WebSocket echo server
fn main () {


    let (mut socket, response) = connect(Url::parse("ws://127.0.0.1:8090").unwrap()).expect("Can't connect");

    let req = object!{
        method: "call",
        params: [1, "database", []]
    };

    socket.write_message(Message::Text(stringify(req))).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }   
}
