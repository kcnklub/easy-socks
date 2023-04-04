use tokio::task::JoinHandle;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{Future, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

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

pub async fn handle_message(message: &ClientMessage) -> AsyncOutputMessage {
    match message.message_type {
        ClientMessageType::Connect => connect_to_ws(message).await,
        _ => AsyncOutputMessage {
            message_type: AsyncOutputMessageType::Error,
            write_sink: None,
            read_stream: None,
        },
    }
}

async fn connect_to_ws(message: &ClientMessage) -> AsyncOutputMessage {
    // this is where I am going to be creating the sockect connection. I think that I am goign to
    // need need to return the constucted WS back so that I can store it in the storage for latter.
    let url = url::Url::parse(&message.message).unwrap();

    let (ws_stream, _) = connect_async(&url).await.unwrap();
    let (write, read) = ws_stream.split();

    AsyncOutputMessage {
        message_type: AsyncOutputMessageType::Connected,
        write_sink: Some(write),
        read_stream: Some(read),
    }
}

pub type ClientWebSocketConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct AsyncOutputMessage {
    pub message_type: AsyncOutputMessageType,
    pub write_sink: Option<SplitSink<ClientWebSocketConnection, Message>>,
    pub read_stream: Option<SplitStream<ClientWebSocketConnection>>,
}

#[derive(Debug)]
pub enum AsyncOutputMessageType {
    Connected,
    Error,
}

#[derive(Debug)]
pub struct Storage {
    pub inner: Mutex<mpsc::Sender<ClientMessage>>,
    pub write: Arc<Mutex<Option<SplitSink<ClientWebSocketConnection, Message>>>>,
    pub reader: Arc<Mutex<Option<JoinHandle<()>>>>,
}
