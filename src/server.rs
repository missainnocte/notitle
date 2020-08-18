use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::net::SocketAddr;

struct StreamInfo {
    peer_addr: SocketAddr,
    local_addr: SocketAddr,
    b_data: Vec<u8>,
}

#[tokio::main]
pub async fn run_server() {
    let mut listener = TcpListener::bind("127.0.0.1:3154").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        let info = StreamInfo {
                            peer_addr: socket.peer_addr().unwrap(),
                            local_addr: socket.local_addr().unwrap(),
                            b_data: buf.clone()
                        };
                        println!(
                            "peer_addr:{}\tlocal_addr:{}\tcontent:{}",
                            info.peer_addr,
                            info.local_addr,
                            String::from_utf8_lossy(&info.b_data)
                        );
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}
