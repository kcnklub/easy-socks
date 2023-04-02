#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use easy_socks::ClientMessage;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<ClientMessage>>,
}

fn main() {
    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
        })
        .invoke_handler(tauri::generate_handler![connect, disconnect])
        .setup(|_application| {
            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        println!("We have a message {:?} to send back to the client", output);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect(url: String, state: tauri::State<'_, AsyncProcInputTx>) -> Result<(), String> {
    println!("going to create a connection to {url}");
    let message = ClientMessage::new_connect(url);
    let tx = state.inner.lock().await;
    tx.send(message).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn disconnect() {
    println!("going to disconnect from WS");
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<ClientMessage>,
    output_tx: mpsc::Sender<ClientMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        easy_socks::handle_message(&output).await;
        output_tx.send(output).await?;
    }
    Ok(())
}
