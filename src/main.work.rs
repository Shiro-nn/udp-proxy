use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    // Локальный адрес для прослушивания входящих сообщений от клиентов
    let local_addr = get_argument("local", "127.0.0.1:12345");
    // Адрес удаленного сервера, на который будут пересылаться пакеты
    let remote_addr = get_argument("remote", "185.173.93.134:7777");

    // Создаем сокет для прослушивания входящих пакетов
    let socket = UdpSocket::bind(&local_addr)?;

    println!("Listening on {}", local_addr);
    println!("Remote addr: {}", remote_addr);

    // Хранение информации о клиентах и их последних портах
    let client_map: Arc<Mutex<HashMap<SocketAddr, UdpSocket>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut buffer = [0u8; 10240];

    loop {
        // Ожидаем получения данных
        let receive = socket.recv_from(&mut buffer);

        if let Ok((size, client_addr)) = receive {
            //println!("Received {} bytes from {}", size, client_addr);

            {
                let mut clients = client_map.lock().unwrap();
                if !clients.contains_key(&client_addr) {
                    let s = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
                    s.set_nonblocking(true)?; // Устанавливаем неблокирующий режим.

                    let main_socket = socket.try_clone()?;
                    let target_socket = s.try_clone()?;
                    let client_addr_clone = client_addr.clone();
                    
                    clients.insert(client_addr, s);
                    
                    thread::spawn(move || {
                        let inactivity_limit = Duration::from_secs(60);  // Предел бездействия в 60 секунд.
                        let mut last_activity = Instant::now();  // Время последней активности.
                        
                        loop {
                            // Если время ожидания превышает 60 секунд, завершить поток.
                            if last_activity.elapsed() > inactivity_limit {
                                println!("Поток завершен из-за неактивности.");
                                break;
                            }
                            
                            // Ожидаем ответ от удаленного сервера
                            let mut response = [0u8; 10240];

                            // Запускаем таймер для таймаута в 60 секунд
                            let timeout = Duration::from_secs(60);
                            let now = Instant::now();

                            // Если получаем данные, сбрасываем таймер
                            match target_socket.recv_from(&mut response) {
                                Ok((size, _src)) => {
                                    last_activity = Instant::now();  // Сбрасываем таймер при получении данных
                                    //println!("Получены {} байт данных", size);

                                    // Отправляем ответ обратно клиенту
                                    main_socket
                                        .send_to(&response[..size], client_addr_clone)
                                        .expect("Failed to send response to client");
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // Если сокет не готов для чтения, ждем и проверяем таймаут.
                                    if now.elapsed() > timeout {
                                        println!("Таймаут: Поток не получил данные в течение 60 секунд.");
                                        break;
                                    }

                                    // Если данных нет, ждем немного
                                    thread::sleep(Duration::from_millis(100));
                                }
                                Err(e) => {
                                    eprintln!("Ошибка при чтении из сокета: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                }
            }

            let remote_addr = remote_addr.clone();
            let client_addr = client_addr.clone();
            let buffer = buffer.clone();
            let size = size.clone();
            let clients_cloned = Arc::clone(&client_map);

            thread::spawn(move || {
                let target_socket = {
                    let mut clients = clients_cloned.lock().unwrap();
                    if let Some(stored_socket) = clients.remove(&client_addr) {
                        clients.insert(client_addr, stored_socket.try_clone().unwrap());
                        stored_socket
                    } else {
                        UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket")
                    }
                };

                target_socket
                    .send_to(&buffer[..size], remote_addr)
                    .expect("Failed to send data to remote server");
            });
        }
        
    }
}


fn get_argument(search: &str, default: &str) -> String {
    let search_arg = format!("--{}=", search);
    
    for arg in std::env::args() {
        if arg.starts_with(&search_arg) {
            return arg.chars().skip(search_arg.len()).collect::<String>();
        }
    }
    
    String::from(default)
}