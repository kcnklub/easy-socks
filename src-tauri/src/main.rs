#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use easy_socks::{AsyncOutputMessage, AsyncOutputMessageType, ClientMessage, Storage};
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Window;
use tokio_tungstenite::tungstenite::Message;

use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

fn main() {
    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .manage(Storage {
            inner: Mutex::new(async_proc_input_tx),
            write: Arc::new(Mutex::new(None)),
            reader: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect_websocket,
            send_message
        ])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            // can I get the application object into the async context? I want to put the socket
            // into the meme
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        handle(output, &app_handle).await;
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect(url: String, state: tauri::State<'_, Storage>) -> Result<(), String> {
    let message = ClientMessage::new_connect(url);
    let tx = state.inner.lock().await;
    tx.send(message).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn disconnect_websocket(window: Window, state: tauri::State<'_, Storage>) -> Result<(), ()> {
    println!("going to disconnect from WS");

    let mut write = state.write.lock().await;
    *write = None;

    let mut reader = state.reader.lock().await;
    *reader = None;

    window
        .emit_all("disconnected", format!("disconnected"))
        .unwrap();

    Ok(())
}

#[tauri::command]
async fn send_message(message: String, state: tauri::State<'_, Storage>) -> Result<(), String> {
    let mut current_write = state.write.lock().await;

    if let Some(writer) = current_write.as_mut() {
        writer.send(Message::Text(message)).await.unwrap();
    } else {
        return Err(String::from("unable to send message"));
    }

    Ok(())
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<ClientMessage>,
    output_tx: mpsc::Sender<AsyncOutputMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = easy_socks::handle_message(&input).await;
        output_tx.send(output).await?;
    }
    Ok(())
}

async fn handle(output: AsyncOutputMessage, manager: &AppHandle) {
    match output.message_type {
        AsyncOutputMessageType::Connected => {
            if let Some(write_sink) = output.write_sink {
                let state = manager.state::<Storage>();
                let mut current_write = state.write.lock().await;
                *current_write = Some(write_sink);
                let stream = output.read_stream;
                let emitter = manager.clone();
                emitter.emit_all("connected", format!("connected")).unwrap();
                let reader_thread = tokio::spawn(async move {
                    if let Some(reader) = stream {
                        let read_future = reader.for_each_concurrent(None, |message| {
                            let sink = &emitter;
                            async move {
                                if let Ok(text) = message {
                                    let message_to_send = MyMessage {
                                        inner: text.to_text().unwrap().to_string(),
                                    };
                                    sink.emit_all("message_read", message_to_send).unwrap();
                                }
                            }
                        });
                        read_future.await;
                    }
                });

                let mut reader = state.reader.lock().await;
                *reader = Some(reader_thread);
            }
        }
        _ => println!("We got a different message"),
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct MyMessage {
    inner: String,
}
