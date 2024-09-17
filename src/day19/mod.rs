use askama_axum::IntoResponse;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    routing::{get, post},
    Router,
};
use futures::{stream::StreamExt, SinkExt};
use serde_json::Value;
use std::{
    collections::HashMap,
    ops::ControlFlow,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
};
use tokio::sync::{watch, RwLock};

async fn ping(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match handle_message(msg, &state, &mut socket).await {
                ControlFlow::Continue(_) => continue,
                ControlFlow::Break(_) => break,
            }
        } else {
            break;
        }
    }
}

async fn handle_message(
    msg: Message,
    state: &SharedState,
    socket: &mut WebSocket,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> received {}", t);
            match t.as_str() {
                "serve" => {
                    state
                        .game_started
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                    ControlFlow::Continue(())
                }
                "ping" => {
                    if state
                        .game_started
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        println!("<<< sending pong");
                        let _ = socket.send(Message::Text("pong".to_string())).await;
                        ControlFlow::Continue(())
                    } else {
                        println!("XXX game not started");
                        ControlFlow::Continue(())
                    }
                }
                _ => ControlFlow::Continue(()),
            }
        }
        Message::Ping(_) => {
            println!(">>> received Ping message");
            ControlFlow::Continue(())
        }
        Message::Binary(_) => {
            println!(">>> received Binary message");
            ControlFlow::Continue(())
        }
        Message::Pong(_) => {
            println!(">>> received Pong message");
            ControlFlow::Continue(())
        }
        Message::Close(_) => {
            println!(">>> received Close message");
            ControlFlow::Break(())
        }
    }
}

#[derive(Clone)]
struct SharedState {
    game_started: Arc<AtomicBool>,
    room_channel: Arc<RwLock<HashMap<usize, watch::Sender<Message>>>>,
    count: Arc<AtomicUsize>,
}

async fn reset(State(state): State<SharedState>) {
    state.count.store(0, std::sync::atomic::Ordering::Relaxed);
}

async fn views(State(state): State<SharedState>) -> String {
    state
        .count
        .load(std::sync::atomic::Ordering::Relaxed)
        .to_string()
}

async fn room(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path((room_number, user_name)): Path<(usize, String)>,
) -> impl IntoResponse {
    if !state.room_channel.read().await.contains_key(&room_number) {
        let (tx, _rx) = watch::channel(Message::Text("{}".to_string()));
        state.room_channel.write().await.insert(room_number, tx);
    }
    ws.on_upgrade(move |socket| handle_chat_socket(socket, room_number, user_name, state))
}

async fn handle_chat_socket(
    socket: WebSocket,
    room_number: usize,
    user_name: String,
    state: SharedState,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state
        .room_channel
        .read()
        .await
        .get(&room_number)
        .unwrap()
        .subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            let msg = rx.borrow().clone();

            if sender.send(msg).await.is_ok() {
            } else {
                break;
            }
        }
    });

    let recv_user = user_name.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if process_chat_message(msg, room_number, recv_user.clone(), state.clone())
                    .await
                    .is_continue()
                {
                    continue;
                }
            }
            break;
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn process_chat_message(
    msg: Message,
    room_number: usize,
    user: String,
    state: SharedState,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(text) => {
            let msg = serde_json::from_str::<Value>(&text).unwrap();
            let message = msg.get("message").unwrap().as_str().unwrap();
            if msg.get("user").is_none() {
                if message.len() > 128 {
                    println!("message too long");
                    return ControlFlow::Continue(());
                }

                let broadcast_msg = serde_json::json!({
                    "user": user,
                    "message": message,
                });

                if state
                    .room_channel
                    .write()
                    .await
                    .get(&room_number)
                    .unwrap()
                    .send(Message::Text(broadcast_msg.to_string()))
                    .is_ok()
                {
                    let count = state
                        .room_channel
                        .write()
                        .await
                        .get(&room_number)
                        .unwrap()
                        .receiver_count();
                    state
                        .count
                        .fetch_add(count, std::sync::atomic::Ordering::Relaxed);
                } else {
                    println!("client disconnected");
                    return ControlFlow::Break(());
                }
            }
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> client sent close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", user);
            }
            return ControlFlow::Break(());
        }
        _ => {
            println!(">>> client sent something else: {:?} ", msg);
        }
    }
    ControlFlow::Continue(())
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/19/ws/ping", get(ping))
        .route("/19/reset", post(reset))
        .route("/19/views", get(views))
        .route("/19/ws/room/:room_number/user/:user_name", get(room))
        .with_state(SharedState {
            game_started: Arc::new(AtomicBool::new(false)),
            room_channel: Arc::new(RwLock::new(HashMap::new())),
            count: Arc::new(AtomicUsize::new(0)),
        })
}
