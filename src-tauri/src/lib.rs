#[derive(Debug)]
pub struct ClientMessage {
    message_type: ClientMessageType,
    message: String,
}

impl ClientMessage {
    pub fn new_connect(url: String) -> Self {
        ClientMessage {
            message_type: ClientMessageType::Connect,
            message: url,
        }
    }
}

#[derive(Debug)]
pub enum ClientMessageType {
    Connect,
    Disconnect,
    SendMessage,
}

pub async fn handle_message(message: &ClientMessage) {
    match message.message_type {
        ClientMessageType::Connect => connect_to_ws(message).await,
        _ => (),
    }
}

async fn connect_to_ws(message: &ClientMessage) {
    println!("connecting didn't panic!");
}
