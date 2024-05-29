use anyhow::{anyhow, Result};

use tokio_tungstenite::connect_async;

use futures_util::{stream::StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

use serde_json::Value;

/// Function to connect to the WebSocket and handle messages
pub async fn connect_to_websocket(url: Url) -> Result<()> {
    let (mut ws_stream, _) = connect_async(url)
        .await
        .map_err(|e| anyhow!("Failed to connect: {}", e))?;
    println!("WebSocket connected!");

    // Subscribe to the ticker channel for tBTCUSD
    let subscribe_message = r#"{
        "event": "subscribe",
        "channel": "ticker",
        "symbol": "tBTCUSD"
    }"#;

    ws_stream
        .send(Message::Text(subscribe_message.into()))
        .await?;

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                handle_message(text).await?;
            }
            Ok(Message::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
            }
            Ok(Message::Ping(ping)) => {
                ws_stream.send(Message::Pong(ping)).await?;
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(reason)) => {
                println!("WebSocket closed: {:?}", reason);
                break;
            }
            Err(e) => {
                println!("WebSocket error: {}", e);
                break;
            }
            _ => {
                panic!("not covered")
            }
        }
    }

    Ok(())
}

/// Function to handle incoming WebSocket messages
async fn handle_message(message: String) -> Result<()> {
    // Parse the message as JSON
    let json_msg: Value = serde_json::from_str(&message)?;

    // Check if it's a ticker update
    if json_msg.is_array() {
        let data = json_msg.as_array().unwrap();
        if data.len() > 1 {
            if let Some(channel_id) = data.get(0) {
                if channel_id.is_number() {
                    let channel_id = channel_id.as_i64().unwrap();
                    if let Some(update) = data.get(1) {
                        if update.is_array() {
                            let update = update.as_array().unwrap();
                            // Handle trading snapshot/update
                            if update.len() >= 10 {
                                println!(
                                    "Channel {}: BID: {}, ASK: {}, LAST PRICE: {}",
                                    channel_id,
                                    update[0], // BID
                                    update[2], // ASK
                                    update[6]  // LAST PRICE
                                );
                            }
                        }
                    }
                }
            }
        }
    } else if json_msg.is_object() {
        let obj = json_msg.as_object().unwrap();
        if obj.contains_key("event") && obj["event"] == "subscribed" {
            println!("Subscribed: {:?}", obj);
        }
    } else {
        println!("Received unknown message format: {}", message);
    }

    Ok(())
}
