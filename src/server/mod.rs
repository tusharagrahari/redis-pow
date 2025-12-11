use tokio::net::TcpListener;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address).await.unwrap();

    // println!("Server listening on {}", address);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            // Handle the connection
            println!("New connection established");
            // Here you would add code to read from/write to the socket
        });
    }
}