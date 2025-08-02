use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

struct Client {
    socket: UdpSocket,
    remote_addr: SocketAddr,
    client_addr: SocketAddr,
    main_socket: Arc<UdpSocket>,
}

async fn handle_client(
    client: Arc<Client>,
    client_map: Arc<Mutex<HashMap<SocketAddr, Arc<Client>>>>,
) {
    let mut buf = [0u8; 10240];
    loop {
        match timeout(Duration::from_secs(60), client.socket.recv_from(&mut buf)).await {
            Ok(Ok((size, _src))) => {
                let data = &buf[..size];
                if let Err(e) = client.main_socket.send_to(data, client.client_addr).await {
                    eprintln!("Ошибка отправки данных клиенту {} от сервера {}: {}", client.client_addr, client.remote_addr, e);
                    break;
                }
            }
            Ok(Err(e)) => {
                eprintln!("Ошибка получения данных от удалённого сервера: {}", e);
                break;
            }
            Err(_) => {
                println!("Клиент {} неактивен (таймаут)", client.client_addr);
                break;
            }
        }
    }

    let mut map = client_map.lock().await;
    map.remove(&client.client_addr);
}

/// Получение аргументов командной строки в виде "--key=value"
fn get_argument(search: &str, default: &str) -> String {
    let search_arg = format!("--{}=", search);
    for arg in env::args() {
        if arg.starts_with(&search_arg) {
            return arg[search_arg.len()..].to_string();
        }
    }
    default.to_string()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Получаем адрес для прослушивания и адрес удалённого сервера
    let local_addr = get_argument("local", "127.0.0.1:12345");
    let remote_addr_str = get_argument("remote", "185.173.93.134:7777");
    let remote_addr: SocketAddr = remote_addr_str.parse().expect("Некорректный адрес удалённого сервера");

    // Создаем главный асинхронный сокет для приёма сообщений от клиентов
    let main_socket = UdpSocket::bind(&local_addr).await?;
    let main_socket = Arc::new(main_socket);
    println!("Прослушиваем: {}", local_addr);
    println!("Удалённый сервер: {}", remote_addr);

    // Карта для хранения активных клиентов
    let client_map: Arc<Mutex<HashMap<SocketAddr, Arc<Client>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut buf = [0u8; 10240];
    loop {
        // Принимаем сообщение от клиента
        let (size, client_addr) = match main_socket.recv_from(&mut buf).await {
            Ok((size, src)) => (size, src),
            Err(_) => continue
        };
        let data = buf[..size].to_vec();

        // Если клиента ещё нет – создаём handler
        let client = {
            let mut map = client_map.lock().await;
            if let Some(client) = map.get(&client_addr) {
                client.clone()
            } else {
                // Создаем новый сокет для общения с удалённым сервером
                let remote_socket = UdpSocket::bind("0.0.0.0:0").await?;
                // Создаем экземпляр клиента с клоном главного сокета для отправки ответа
                let client_instance = Arc::new(Client {
                    socket: remote_socket,
                    remote_addr,
                    client_addr,
                    main_socket: main_socket.clone(),
                });
                // Запускаем асинхронную задачу для прослушивания ответов от удалённого сервера
                let client_instance_clone = client_instance.clone();
                let client_map_clone = client_map.clone();
                tokio::spawn(async move {
                    handle_client(client_instance_clone, client_map_clone).await;
                });
                map.insert(client_addr, client_instance.clone());
                client_instance
            }
        };

        // Пересылаем полученные данные на удалённый сервер через клиентский сокет.
        if let Err(e) = client.socket.send_to(&data, remote_addr).await {
            eprintln!("Ошибка отправки данных на {}: {}", remote_addr, e);
        }
    }
}
