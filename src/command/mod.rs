use crate::database::Database;

pub async fn command_parser(db: &Database, command: &str) -> Result<String, String> {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0].to_uppercase().as_str() {
        "SET" => {
            if parts.len() < 3 {
                return Err("-ERR\r\nUsage: SET <key> <value> [TTL]".to_string());
            }
            let key = parts[1].to_string();
            let value = parts[2].to_string();
            let ttl = if parts.len() == 4 {
                Some(
                    parts[3]
                        .parse::<u64>()
                        .map_err(|_| "Invalid TTL".to_string())?,
                )
            } else {
                None
            };
            db.set(key, value, ttl).await;
            Ok("OK".to_string())
        }
        "GET" => {
            if parts.len() != 2 {
                return Err("Usage: GET <key>".to_string());
            }
            let key = parts[1];
            match db.get(key).await {
                Some(value) => Ok(format!("${}\r\n{}\r\n", value.len(), value)),
                None => Ok("$-1\r\n".to_string()),
            }
        }
        "DEL" => {
            if parts.len() != 2 {
                return Err("Usage: DEL <key>".to_string());
            }
            let key = parts[1];
            if db.del(key).await {
                Ok("$-1\r\n".to_string())
            } else {
                Ok("$-0\r\n".to_string())
            }
        }
        _ => Err("Unknown command".to_string()),
    }
}
