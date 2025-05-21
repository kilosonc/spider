use {
    etherparse::{Ipv4Slice, UdpHeader},
    tokio::{
        net::UdpSocket,
        time::{self, Duration},
    },
};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        tun_listen().await;
    });

    time::sleep(Duration::from_secs(1)).await;
    send().await;
    time::sleep(Duration::from_secs(1)).await;
}

async fn send() {
    // send data to 10.0.200.9
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket
        .send_to(b"Hello, world!", "10.0.200.9:80")
        .await
        .unwrap();
}

async fn tun_listen() {
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 200, 9))
        .netmask((255, 255, 255, 0))
        // .destination((10, 0, 200, 1))
        .up();

    let dev = tun::create_as_async(&config).unwrap();
    let mut buf = [0; 4096];

    loop {
        let amount = dev.recv(buf.as_mut()).await.unwrap();
        let slice = Ipv4Slice::from_slice(&buf[..amount]).unwrap();
        let (udp_header, payload) = UdpHeader::from_slice(slice.payload().payload).unwrap();
        let content = String::from_utf8_lossy(payload);
        println!("Received: {:?}, {:?}", udp_header, content);
    }
}
