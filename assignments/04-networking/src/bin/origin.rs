use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::handle_client::handle_client;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

/// Change this path to match where you want to store the database file
const DB_PATH: &str = "aspirin_eats.db";

fn main() {
    let db = AspirinEatsDb::from_path(DB_PATH).expect("Failed to open database");

    let arc = Arc::new(Mutex::new(db));

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    println!("Server listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let arc = Arc::clone(&arc);
                thread::spawn(move || {
                    handle_client(stream, arc);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
