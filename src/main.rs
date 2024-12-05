use tungstenite::{
    connect,
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use minicbor::Decoder;


fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect("wss://bsky.network/xrpc/com.atproto.sync.subscribeRepos").expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    for _ in 0..10 {
        let msg: Message = socket.read().expect("Error reading message");
        let msg_txt = msg.into_data();
        let mut decoder  = Decoder::new(&msg_txt);
        println!("Received: {:?}", decoder.str());
    }
    let _ = socket.close(Some(CloseFrame {
        code: (CloseCode::Normal),
        reason: ("finished".into()),
    }));
}
