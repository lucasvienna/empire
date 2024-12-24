use empire::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tracing::info;

async fn init_host(host: &str) -> Result<UdpSocket> {
    info!("initializing host: {:?}", host);
    let socket = UdpSocket::bind(host).await?;
    Ok(socket)
}

pub async fn start() -> Result<()> {
    let socket = init_host("127.0.0.1:12345").await?;
    let r = Arc::new(socket);
    let s = r.clone();

    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
