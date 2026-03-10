use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

pub type Store = Arc<Mutex<HashMap<String, Entry>>>;


pub struct Entry {
    pub value: String,
    pub expires_at: Option<Instant>,
}
impl Entry {
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(t) => Instant::now() >= t,
            None => false,
        }
    }
}

pub async fn handle_client(socket: tokio::net::TcpStream, store: Store) {
    let (read_half, mut write_half) = socket.into_split();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let response = handle_command(&line, &store).await;
        let mut out = response;
        out.push('\n');
        if write_half.write_all(out.as_bytes()).await.is_err() {
            break;
        }
    }
}

async fn handle_command(line: &str, store: &Store) -> String {
    let parsed: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return r#"{"status":"error","message":"invalid json"}"#.to_string(),
    };

    let cmd = match parsed.get("cmd").and_then(|v| v.as_str()) {
        Some(c) => c.to_uppercase(),
        None => return r#"{"status":"error","message":"missing cmd"}"#.to_string(),
    };

    match cmd.as_str() {
        "PING" => r#"{"status": "ok"}"#.to_string(),
        "SET" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            let value = parsed.get("value").and_then(|v| v.as_str());
            match (key, value) {
                (Some(k), Some(v)) => {
                    store.lock().await.insert(k.to_string(), Entry { value: v.to_string(), expires_at: None });
                    r#"{"status": "ok"}"#.to_string()
                },
                _ => r#"{"status": "error", "message": "missing a value"}"#.to_string()
            }
        }
        "GET" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            match key {
                Some(k) => {
                    let store = store.lock().await;
                    match store.get(k) {
                        Some(entry) if !entry.is_expired() => format!(r#"{{"status":"ok","value":"{}"}}"#, entry.value),
                        Some(_) | None => r#"{"status":"ok","value":null}"#.to_string(),
                    }
                },
                None => r#"{"status":"error","message":"missing key"}"#.to_string(),
            }
        }
        "DEL" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            match key {
                Some(k) => {
                    let removed = store.lock().await.remove(k).is_some();
                    let count = if removed {1} else {0};
                    format!(r#"{{"status":"ok","count":{}}}"#, count)
                },
                None => r#"{"status":"error","message":"missing key"}"#.to_string(),
            }
        }
        "KEYS" => {
            let map = store.lock().await;
            let keys: Vec<&str> = map
                .iter()
                .filter(|(_, entry)| !entry.is_expired())
                .map(|(k, _)| k.as_str())
                .collect();
            let js_keys: Vec<String> = keys.iter().map(|k| format!(r#""{}""#, k)).collect();
            format!(r#"{{"status":"ok","keys":[{}]}}"#, js_keys.join(","))
        }
        "EXPIRE" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            let seconds = parsed.get("seconds").and_then(|v| v.as_u64());
            match(key, seconds) {
                (Some(k), Some(s)) => {
                    let mut map = store.lock().await;
                    match map.get_mut(k) {
                        Some(entry) if !entry.is_expired() => {
                            entry.expires_at = Some(Instant::now() + Duration::from_secs(s));
                            r#"{"status":"ok","result":1}"#.to_string()
                        },
                        _ => r#"{"status":"ok","result":0}"#.to_string(),
                    }
                },
                _ => r#"{"status":"error","message":"missing key or seconds"}"#.to_string(),
            }
        },
        "TTL" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            match key {
                Some(k) => {
                    let map = store.lock().await;
                    match map.get(k) {
                        Some(entry) if entry.is_expired() => {
                            format!(r#"{{"status":"ok","ttl":-2}}"#)
                        },
                        Some(Entry { expires_at: Some(t), .. }) => {
                            let remaining = t.saturating_duration_since(Instant::now());
                            format!(r#"{{"status":"ok","ttl":{}}}"#, remaining.as_secs())
                        },
                        Some(_) => format!(r#"{{"status":"ok","ttl":-1}}"#),
                        None => format!(r#"{{"status":"ok","ttl":-2}}"#)
                    }
                },
                None => r#"{"status":"error","message":"missing key"}"#.to_string(),
            }
        },
        "INCR" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            match key {
                Some(k) => {
                    let mut map = store.lock().await;
                    let current = map.get(k).filter(|v| !v.is_expired()).map(|v| v.value.clone());
                    let new_val: i64 = match current {
                        None => 1,
                        Some(s) => match s.parse::<i64>() {
                            Ok(n) => n+1,
                            Err(_) => return r#"{"status":"error","message":"not an integer"}"#.to_string(),
                        }
                    };
                    map.insert(k.to_string(), Entry {value: new_val.to_string(), expires_at: None});
                    format!(r#"{{"status":"ok","value":{}}}"#, new_val)
                },
                None => r#"{"status":"error","message":"missing key"}"#.to_string(),
            }
        },
        "DECR" => {
            let key = parsed.get("key").and_then(|v| v.as_str());
            match key {
                Some(k) => {
                    let mut map = store.lock().await;
                    let current = map.get(k).filter(|v| !v.is_expired()).map(|v| v.value.clone());
                    let new_val: i64 = match current {
                        None => -1,
                        Some(s) => match s.parse::<i64>() {
                            Ok(n) => n-1,
                            Err(_) => return r#"{"status":"error","message":"not an integer"}"#.to_string(),
                        }
                    };
                    map.insert(k.to_string(), Entry {value: new_val.to_string(), expires_at: None});
                    format!(r#"{{"status":"ok","value":{}}}"#, new_val)
                },
                None => r#"{"status":"error","message":"missing key"}"#.to_string(),
            }
        },
        "SAVE" => {
            let map = store.lock().await;
            let mut obj = serde_json::Map::new();
            for(k, entry) in map.iter() {
                if !entry.is_expired() {
                    obj.insert(k.clone(), serde_json::Value::String(entry.value.clone()));
                }
            }
            let js = serde_json::Value::Object(obj).to_string();
            drop(map);
            match tokio::fs::write("dump.json", js).await {
                Ok(_) => r#"{"status":"ok"}"#.to_string(),
                Err(e) => format!(r#"{{"status":"error","message":"{}"}}"#, e),
            }
        },
        _ => r#"{"status": "error", "message": "unknown command"}"#.to_string(),
    }
}
