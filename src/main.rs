use tungstenite::{connect, Message};
use url::Url;
use json::{object, stringify};

fn main () {

    let (mut socket, response) = connect(Url::parse("wss://127.0.0.1:8090").unwrap()).expect("Can't connect");

    println!("Connected: {:?}", response);

    let req = object!{
        method: "call",
        params: [1, "login", ["",""]],
        id: 1
    };

    socket.write_message(Message::Text(stringify(req))).unwrap();

    let msg = socket.read_message().expect("Error reading message");
    println!("Received: {}", msg);

    let req = object!{
        method: "call",
        params: [1, "database", []],
        id: 2
    };

    socket.write_message(Message::Text(stringify(req))).unwrap();

    let msg = socket.read_message().expect("Error reading message");
    println!("Received: {}", msg);

    let req = object!{
        method: "call",
        params: [2, "get_chain_id", []],
        id: 3
    };

    socket.write_message(Message::Text(stringify(req))).unwrap();

    let msg = socket.read_message().expect("Error reading message");
    println!("Received: {}", msg);

}
