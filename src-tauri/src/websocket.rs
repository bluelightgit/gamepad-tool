use std::{net::SocketAddr, sync::{Arc, Mutex}};
use futures::{SinkExt, StreamExt};
use tauri::{Emitter, Manager};
use tokio::{net::{TcpListener, TcpStream}, sync::mpsc::{self, UnboundedSender}};
use tokio_tungstenite::{accept_async, tungstenite::Error};

/// 启动WebSocket服务器，用于高性能数据传输
pub async fn start_websocket_server(clients: Arc<Mutex<Vec<UnboundedSender<String>>>>) {
    // 使用本地回环地址和动态端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    
    // 获取实际绑定的地址
    let actual_addr = listener.local_addr().expect("Failed to get local address");
    println!("WebSocket server listening on: ws://{}", actual_addr);
    
    // 创建一个通道用于传递绑定端口号
    let (tx, _rx) = mpsc::unbounded_channel::<u16>();
    
    // 发送实际绑定的端口
    let _ = tx.send(actual_addr.port());
    
    // 开始接受连接
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("Connected streams should have a peer address");
        println!("WebSocket connection established with: {}", peer);
        
        let clients_clone = clients.clone();
        tokio::spawn(handle_connection(stream, clients_clone));
    }
}

/// 处理WebSocket连接
async fn handle_connection(stream: TcpStream, clients: Arc<Mutex<Vec<UnboundedSender<String>>>>) {
    // 从TCP连接创建WebSocket
    let ws_stream = match accept_async(stream).await {
        Ok(ws_stream) => ws_stream,
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {}", e);
            return;
        }
    };
    
    println!("WebSocket connection established");
    
    // 分离WebSocket的读写部分
    let (mut write, mut read) = ws_stream.split();
    
    // 创建一个通道用于向此WebSocket发送消息
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    
    // 添加此客户端到客户端列表
    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.push(tx);
    }
    
    // 处理从通道接收的数据，并发送到WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = write.send(message.into()).await {
                eprintln!("Error sending WebSocket message: {}", e);
                break;
            }
        }
    });
    
    // 处理从WebSocket接收的数据（目前只是记录，不处理）
    let mut recv_task = tokio::spawn(async move {
        while let Some(result) = read.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_close() {
                        println!("WebSocket connection closed");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error in WebSocket stream: {}", e);
                    break;
                }
            }
        }
    });
    
    // 等待任何任务完成（发送或接收失败）
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    
    println!("WebSocket connection closed");
}

/// 获取WebSocket服务器端口
#[tauri::command]
pub async fn get_websocket_port() -> u16 {
    // 使用共享状态存储端口
    use tokio::sync::OnceCell;
    static PORT: OnceCell<u16> = OnceCell::const_new();
    
    *PORT.get().unwrap_or(&0)
}

#[tauri::command]
pub fn start_websocket_server_command(app_handle: tauri::AppHandle) {
    use tokio::sync::OnceCell;
    static PORT: OnceCell<u16> = OnceCell::const_new();
    
    // Get clients from the global state
    let clients = {
        let state = app_handle.state::<crate::cmds::GlobalGamepadState>();
        state.clients.clone()
    };
    
    tauri::async_runtime::spawn(async move {
        // 创建通道用于获取端口
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<u16>();
        
        // 启动WebSocket服务器
        tokio::spawn(async move {
            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 0));
            let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind");
            let actual_addr = listener.local_addr().expect("Failed to get local address");
            
            // 存储和发送端口
            let port = actual_addr.port();
            let _ = tx.send(port);
            let _ = PORT.set(port);
            
            println!("WebSocket server listening on: ws://{}", actual_addr);
            
            while let Ok((stream, _)) = listener.accept().await {
                let peer = stream.peer_addr().expect("Connected streams should have a peer address");
                println!("WebSocket connection established with: {}", peer);
                
                let clients_clone = clients.clone();
                tokio::spawn(handle_connection(stream, clients_clone));
            }
        });
        
        // 获取端口并通知前端
        if let Some(port) = rx.recv().await {
            app_handle.emit("websocket_port", port).expect("Failed to emit websocket_port event");
        }
    });
}
