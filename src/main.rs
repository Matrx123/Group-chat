use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio::sync::{broadcast, Mutex};

#[derive(Clone)]
pub enum Message {
    UserJoined(String),
    UserLeft(String),
    SystemMsg(String),
    Chat { username: String, msg: String },
}

impl Message {
    pub fn to_string(&self) -> String {
        match self {
            Message::UserJoined(username) => format!("A new user @{} Joined", username),
            Message::UserLeft(username) => format!("*** User @{} Left the chat", username),
            Message::SystemMsg(msg) => format!("[:: System ::] {}", msg),
            Message::Chat { username, msg } => format!("[{}]=> {}", username, msg),
        }
    }
}

#[derive(Clone)]
pub struct ChatServer {
    sender: broadcast::Sender<Message>,
    users: Arc<Mutex<HashMap<SocketAddr, String>>>,
}
impl ChatServer {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        let users = Arc::new(Mutex::new(HashMap::new()));
        ChatServer { sender, users }
    }

    pub async fn broacast(&self, message: Message) {
        if let Err(e) = self.sender.send(message) {
            eprintln!("\n Failed to send the Message :: {}", e);
        }
    }

    pub async fn remove_user(&self, addr: SocketAddr) {
        if let Some(username) = self.users.lock().await.remove(&addr) {
            self.broacast(Message::UserLeft(username)).await;
        };
    }

    pub async fn add_user(&self, username: String, addr: SocketAddr) {
        self.users.lock().await.insert(addr, username.clone());
        self.broacast(Message::UserJoined(username)).await;
    }

    pub async fn user_count(&self) -> usize {
        self.users.lock().await.len()
    }

    pub async fn get_username(&self, addr: SocketAddr) -> Option<String> {
        self.users.lock().await.get(&addr).cloned()
    }
}

pub async fn handle_user(
    socket: TcpStream,
    addr: SocketAddr,
    server: ChatServer,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (reader, writer) = split(socket);
    let writer = Arc::new(Mutex::new(writer));
    let mut lines = BufReader::new(reader).lines();
    {
        let mut w = writer.lock().await;
        w.write_all(b"\n Enter Your username \n").await?;
        w.flush().await?;
    }

    let username = match lines.next_line().await? {
        Some(line) => {
            let username = line.trim().to_string();
            if username.is_empty() {
                let mut w = writer.lock().await;
                w.write_all(b"\n username can't be empty! \n").await?;
                return Ok(());
            }
            username
        }
        None => {
            return Ok(());
        }
    };

    server.add_user(username.clone(), addr).await;
    {
        let mut w = writer.lock().await;
        w.write_all(b"\n:: Welcome to the chat ::\n").await?;
        w.flush().await?;
    }

    let mut receiver = server.sender.subscribe();
    let server_clone = server.clone();
    let writer_clone = writer.clone();

    spawn(async move {
        while let Ok(message) = receiver.recv().await {
            let msg = format!("{}\n", message.to_string());
            let mut w = writer_clone.lock().await;
            if w.write_all(msg.as_bytes()).await.is_err() {
                break;
            }
            if w.flush().await.is_err() {
                break;
            }
        }
    });

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("/") {
            let mut w = writer.lock().await;
            match line {
                "/quit" => {
                    w.write_all(b"GoodBye!\n").await?;
                    break;
                }
                "/users" => {
                    let count = server_clone.user_count().await;
                    w.write_all(format!("\n{} users online\n", count).as_bytes())
                        .await?;
                }
                "/help" => {
                    w.write_all(b"Commands::\n").await?;
                    w.write_all(b"/quit - Leave the server\n").await?;
                    w.write_all(b"/users - Get the count of users online \n")
                        .await?;
                    w.write_all(b"/help - For Guide\n").await?;
                }
                _ => {
                    w.write_all(b"Unknown command.\nType /help for a guide.\n")
                        .await?;
                }
            }
        } else {
            server_clone
                .broacast(Message::Chat {
                    username: username.clone(),
                    msg: line.to_string(),
                })
                .await;
        }
    }

    server.remove_user(addr).await;
    println!("Client {} @{} diconnected!", addr, username);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listner = TcpListener::bind(addr).await?;
    println!("\nWelcome to the Group Chat.\n");
    println!("You can join via telnet 127.0.0.1 8080\n");

    let server = ChatServer::new();

    while let Ok((socket, addr)) = listner.accept().await {
        let server_clone = server.clone();
        spawn(async move {
            if let Err(e) = handle_user(socket, addr, server_clone).await {
                eprintln!("Error in handling the user !! {}", e);
            }
        });
    }

    Ok(())
}
