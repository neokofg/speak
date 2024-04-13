use tokio::net::UdpSocket;
use tokio::time::{Duration, Instant};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[tokio::main]
pub async fn server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let mut buf = [0u8; 2048];
    let timeout_duration = Duration::from_secs(30);

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        if len > 0 {
            let data = buf[..len].to_vec();

            println!("Получены данные в размере {:?}", data.len());

            let mut clients_guard = clients.lock().unwrap();

            clients_guard.insert(addr, Instant::now());

            clients_guard.retain(|_, &mut last_seen| last_seen.elapsed() < timeout_duration);

            for (&client_addr, _) in clients_guard.iter() {
                if client_addr != addr {
                    if let Err(e) = socket.send_to(&data, client_addr).await {
                        eprintln!("Ошибка при отправке данных клиенту {}: {}", client_addr, e);
                    }
                }
            }
        }
    }
}