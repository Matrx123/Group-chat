# Rust TCP Chat Server 💬

A high-performance, multi-client TCP chat server built with Rust and Tokio for asynchronous networking.

## ✨ Features

- **🚀 Multi-client support** - Handle multiple concurrent users seamlessly
- **⚡ Real-time messaging** - Instant message broadcasting to all connected clients
- **👤 User management** - Username registration and active user tracking
- **🔧 Command system** - Built-in commands for enhanced chat functionality
- **🛡️ Graceful shutdown** - Proper cleanup when clients disconnect or quit
- **⚙️ Async/await architecture** - Built on Tokio for maximum performance

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable version)
- Cargo

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd rust-chat-server
   ```

2. **Build and run the server**
   ```bash
   cargo run
   ```

3. **Connect clients** (in separate terminals)
   ```bash
   # Using telnet
   telnet localhost 8080
   
   # Using netcat
   nc localhost 8080
   
   # Using ncat (Windows)
   ncat localhost 8080
   ```

4. **Start chatting!**
   - Enter your username when prompted
   - Type messages to chat with other users
   - Use commands (see below) for additional functionality

## 🎮 Available Commands

| Command | Description |
|---------|-------------|
| `/quit` | Leave the chat gracefully |
| `/users` | Show the number of online users |
| `/help` | Display all available commands |


### Key Technical Components

- **🔄 Async Task Management**: Each client runs in its own async task
- **📡 Message Broadcasting**: Tokio broadcast channels for real-time communication
- **🔒 Thread-Safe State**: Arc<Mutex> for safe concurrent access to shared resources
- **🔀 Split I/O Streams**: Separate read/write operations for efficient networking
- **🛑 Graceful Shutdown**: Proper task cleanup and resource management

## 🛠️ Technical Stack

- **Language**: Rust 🦀
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **Networking**: TCP sockets with async I/O
- **Concurrency**: Arc, Mutex, and async task spawning
- **Error Handling**: Result types with comprehensive error propagation


## 🎯 Learning Objectives

This project demonstrates:

- **Async/await patterns** in Rust
- **TCP socket programming** with Tokio
- **Concurrent programming** with shared state
- **Error handling** in async contexts
- **Resource management** and cleanup
- **Real-time communication** patterns

## 🤝 Contributing

Contributions are welcome! Here are some ideas for improvements:

- [ ] Add private messaging between users
- [ ] Implement chat rooms/channels
- [ ] Add message history/logging
- [ ] Create a simple web interface
- [ ] Add user authentication
- [ ] Implement rate limiting

## 📝 Example Session

```
$ cargo run
Chat server listening on 127.0.0.1:8080

# Terminal 1 (Client 1)
$ telnet localhost 8080
Enter your username: Alice
Welcome to the chat!
Hello everyone!

# Terminal 2 (Client 2)  
$ telnet localhost 8080
Enter your username: Bob
Welcome to the chat!
Alice: Hello everyone!
Hi Alice!

# Back to Terminal 1
Bob: Hi Alice!
/users
Users online: 2
/quit
Goodbye!
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tokio](https://tokio.rs/) - An asynchronous runtime for Rust
- Inspired by classic IRC and chat server implementations
- Thanks to the Rust community for excellent async ecosystem

---

**Happy Chatting!** 🎉
