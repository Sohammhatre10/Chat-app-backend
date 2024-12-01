# Rust Client-Server Chat Application

A simple non-blocking chat application built with Rust. The server broadcasts messages from one client to all connected clients in real-time.

---

## Features
- **Non-blocking I/O** for handling multiple clients.
- **Message Broadcasting** from one client to all connected clients.
- **Graceful Shutdown** when clients disconnect.

---

## Requirements
- **Rust** (1.60+)
- **Cargo** (Rust's package manager)

---

## Setup Instructions

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/rust-chat-app.git
cd rust-chat-app
```

### 2. Build the Project
```bash
cargo build --release
```

---

## Usage

### Step-by-step Instructions:

1. **Open Two Terminals:**
   - One for the server.
   - One for the client.

2. **Run the Server:**
   ```bash
   cargo run --bin server
   ```

3. **Run the Client:**
   ```bash
   cargo run --bin client
   ```

4. **Write a Message in the Client:**
   - Enter your message in the client terminal and press `Enter`.

5. **Observe on the Server:**
   - The server will broadcast the message to all connected clients.

6. **Exit the Chat:**
   - Type `:quit` in the client to disconnect.

---

## Contribution
Feel free to open issues and submit pull requests for improvements!

---

## License
This project is licensed under the MIT License.
