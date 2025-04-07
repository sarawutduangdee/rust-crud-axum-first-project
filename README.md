# 🦀 Rust Axum CRUD API

Simple REST API built with [Rust](https://www.rust-lang.org/) using [Axum](https://docs.rs/axum/) framework.

## 🚀 Features

- Create, Read, Update, Delete (CRUD) for Todo items
- In-memory storage (thread-safe with `Arc<Mutex<_>>`)
- Auto-generated UUIDs
- JSON API with Serde
  
## 📦 Technologies

- Axum
- Tokio
- Serde
- UUID
 
## ▶️ Run

```bash
cargo run
```

Server will start at:
👉 http://localhost:3000


## 📡 API Endpoints

| Method | Endpoint | Description |
| :---- | :----: | ----: |
| GET | /todos | Get all todos | 
| POST | /todos | Create new todo | 
| PUT | /todos/{id} | Update todo by ID | 
| DELETE | /todos/{id} | Delete todo by ID | 



📌 Sample JSON
```json
{
  "title": "เรียนรู้ Rust"
}
```

🧪 Test with Postman or curl
```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "ลองเล่น Axum"}'
```
