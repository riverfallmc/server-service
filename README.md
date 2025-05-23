![logo](./assets/logo.png)
# WebSocket Service
Микросервис, отвечающий за список серверов и их мониторинг.

## Содержимое
* [Сборка](#сборка)
* [Деплой](#деплой)
* [Настройка](#настройка)
  * [Переменные окружения](#переменные-окружения)
* [Описание эндпоинтов](#эндпоинты)

# Сборка
Микросервис написан на Rust, так что чтобы\
собрать его вам необходимо лишь установить ``cargo`` на ваш ПК,
и прописать следующую команду

```bash
cargo build --release
```

После успешной сборки вы сможете найти артефакт по этому пути ``./target/release/server_service``.

# Деплой
Команды для деплоя уже есть в нашем [Puff-файле](./puff.yml).

[Узнать подробнее что такое Puff-файл](https://github.com/smokingplaya/puff)

```bash
# Собирает сервис и пушит его в регистр под тегом latest
puff deploy
```

<!-- # Настройка -->

<!-- ## Переменные окружения -->
<!-- ``DATABASE_URL: string`` - URL для подключения к PostgreSQL. -->

# Эндпоинты

## GET ``/(server|client)s``

### Описание
Возвращает список (сервер|клиент)ов

## GET ``/(server|client)/{id}``

### Описание
Возвращает (сервер|клиент) по ``Id``

## POST ``/(server|client)/{id}``

### Описание
Добавляет (сервер|клиент).

### Тело
#### Клиент
```json
{
  "name": "",
  "description": "",
  "modloader": "Forge | Fabric",
  "version": "x.x.x",
  "mods": [""]
}
```

#### Сервер
```json
{
  "name": "",
  "client": "name клиента",
  "ip": "x.x.x.x:xxxxx",
  "icon": "image url",
  "background": "image url"
}
```

## PATCH ``/(server|client)/{id}``

### Описание
Обновляет запись о (сервер|клиент)е

### Тело
```json
{}
```

## DELETE ``/(server|client)/{id}``

### Описание
Удаляет (сервер|клиент)