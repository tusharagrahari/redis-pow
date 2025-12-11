mod command;
mod server;
mod database;

#[tokio::main]
async fn main() {
    let add = "127.0.0.1:6379";
    println!("Starting Redis server at {}", add);
}
