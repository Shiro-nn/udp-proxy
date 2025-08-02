# udp-proxy 🚀

[![GitHub stars](https://img.shields.io/github/stars/Shiro-nn/udp-proxy?style=social)](https://github.com/Shiro-nn/udp-proxy/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Shiro-nn/udp-proxy?style=social)](https://github.com/Shiro-nn/udp-proxy/network/members)
[![GitHub issues](https://img.shields.io/github/issues/Shiro-nn/udp-proxy)](https://github.com/Shiro-nn/udp-proxy/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/Shiro-nn/udp-proxy)](https://github.com/Shiro-nn/udp-proxy/commits)
[![License: MIT](https://img.shields.io/github/license/Shiro-nn/udp-proxy)](LICENSE)
[![Status: Archived](https://img.shields.io/badge/status-archived-lightgrey.svg)](https://github.com/Shiro-nn/udp-proxy)

![Repo Stats](https://github-readme-stats.vercel.app/api/pin/?username=Shiro-nn&repo=udp-proxy)

> **udp-proxy** — UDP-прокси на Rust, оптимизированный для обработки большого числа потоков и работы в сильнонагруженных системах. Разработан в 2025 году и переведён в архивный режим. Код доступен для изучения и использования, но без поддержки и обновлений.

---

## 📜 Описание

**udp-proxy** — это высокопроизводительный прокси-сервер для перенаправления UDP-трафика между клиентами и удалённым сервером. Проект разработан на языке Rust с использованием асинхронного фреймворка Tokio, что обеспечивает низкие накладные расходы и высокую надёжность при обработке множества одновременных соединений. Подходит для игровых серверов, стриминговых приложений и других систем, где важна быстрая передача данных.

---

## 🚀 Быстрый старт

### Установка и запуск

1. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/Shiro-nn/udp-proxy.git
   cd udp-proxy
   ```

2. Соберите и запустите проект:
   ```bash
   cargo build --release
   cargo run --release
   ```

3. По умолчанию прокси слушает на `127.0.0.1:12345` и перенаправляет трафик на `185.173.93.134:7777`. Для настройки используйте аргументы:
   ```bash
   cargo run -- --local=0.0.0.0:8080 --remote=example.com:1234
   ```

---

## 🛠️ Конфигурация

Прокси поддерживает настройку через аргументы командной строки:

- **`--local`**: Локальный адрес для прослушивания (по умолчанию `127.0.0.1:12345`).
- **`--remote`**: Адрес удалённого сервера (по умолчанию `185.173.93.134:7777`).

Пример запуска с кастомными параметрами:
```bash
cargo run --release -- --local=0.0.0.0:8080 --remote=example.com:1234
```

---

## ⚙️ Как это работает

1. Прокси принимает UDP-пакеты от клиентов на локальном адресе.
2. Для каждого нового клиента создаётся отдельный UDP-сокет, который перенаправляет данные на удалённый сервер.
3. Ответы от сервера возвращаются клиенту через главный сокет.
4. Неактивные клиенты (без данных более 60 секунд) автоматически удаляются из пула.

Код использует:
- **Tokio** для асинхронной обработки соединений.
- **HashMap** для отслеживания активных клиентов.
- **Mutex** для безопасной работы с общими данными в многопоточной среде.

Пример кода из `src/main.rs`:
```rust
let main_socket = UdpSocket::bind(&local_addr).await?;
let client_map: Arc<Mutex<HashMap<SocketAddr, Arc<Client>>>> = Arc::new(Mutex::new(HashMap::new()));
```

---

## 🛠️ Системные требования

- **Rust 1.75+** и `cargo`.
- Операционная система: Linux, Windows или macOS (поддержка через Tokio).

---

## 📝 Лицензия

Проект распространяется под лицензией **MIT**. Подробности — в файле [LICENSE](LICENSE).

---

## ⚠️ Примечание

Репозиторий находится в **архивном режиме** с 2025 года. Используйте код на свой риск — активная разработка и поддержка прекращены.

> Спасибо за интерес к проекту! Надеюсь, он будет полезен как пример реализации UDP-прокси на Rust.
