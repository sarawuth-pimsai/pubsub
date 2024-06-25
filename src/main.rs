use std::{sync::Arc, thread};

use axum::{extract::State, routing::get, Router};
use redis::{Commands, ControlFlow, PubSubCommands};
use sharestate::state::{self, global::Global};

#[tokio::main]
async fn main() {
    println!("Start PUB/SUB");

    println!("Start Application");
    let global = Global::new(&vec![]);
    let state = Arc::new(global);
    subscribe(State(state.clone()));
    let app = Router::new()
        .route("/pubsub", get(pubsub))
        .route("/add", get(add))
        .route("/", get(check))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}

async fn pubsub() {
    publish_message();
}

pub fn publish_message() {
    let redis_url = "redis://127.0.0.1/";
    let client = redis::Client::open(redis_url).unwrap();
    let mut conn = client.get_connection().unwrap();
    let _: () = conn
        .publish("promotions".to_string(), 1.to_string())
        .unwrap();
}
pub fn subscribe(State(state): State<Arc<Global>>) {
    let redis_url = "redis://127.0.0.1/";
    let client = redis::Client::open(redis_url).unwrap();
    let mut conn = client.get_connection().unwrap();
    thread::spawn(move || {
        conn.subscribe(&["promotions".to_string()], |msg| {
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            println!("{:?}, {:?}", ch, payload);

            let promotion: state::promotion::State = state::promotion::State { promotion_id: payload.trim().parse().unwrap() };
            let mut global = state.promotions.lock().unwrap();
            global.push(promotion);
            
            ControlFlow::<String>::Continue
        })
        .unwrap();
    });
}

async fn add(State(state): State<Arc<Global>>) {
    let x = Arc::clone(&state);
    let promotion: state::promotion::State = state::promotion::State { promotion_id: 2 };
    let mut global = x.promotions.lock().unwrap();
    global.push(promotion);
}

async fn check(State(state): State<Arc<Global>>) {
    let x = Arc::clone(&state);
    println!("{:?}", x);
}
