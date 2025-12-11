use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::database::Database;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address).await.unwrap();
    let db = Database::new();
    // println!("Server listening on {}", address);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = tokio::io::BufReader::new(reader);
            let mut buffer = String::new();

            while reader.read_line(&mut buffer).await.unwrap() > 0 {
                let response = match crate::command::command_parser(&db, &buffer).await {
                    Ok(resp) => resp,
                    Err(err) => format!("-ERR {}\r\n", err),
                };
                writer.write_all(response.as_bytes()).await.unwrap();
                buffer.clear();
            }
        });
    }
}
