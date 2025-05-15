use tokio::{
    net::UdpSocket,
    time::{self, Duration},
};

#[tokio::main]
async fn main() {
    loop {
        send().await;
        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn send() {
    // send data to 10.0.200.9
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket
        .send_to(b"Hello, world!", "10.0.200.9:80")
        .await
        .unwrap();
}
