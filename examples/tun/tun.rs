use {
    etherparse::{Ipv4Slice, Ipv6Slice, UdpHeader},
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

    for _ in 0..10 {
        time::sleep(Duration::from_secs(1)).await;
        send().await;
    }
    // time::sleep(Duration::from_secs(100)).await;
}

async fn send() {
    // send data to 10.0.200.9
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket
        .send_to(b"Hello, world!", "10.200.9.1:80")
        .await
        .unwrap();
    println!("Sent data to 10.200.9.1:80")
}

async fn tun_listen() {
    println!("Listen on 10.200.9.1");
    let mut config = tun::Configuration::default();
    config
        .address((10, 200, 9,1))
        .netmask((255, 255, 0, 0))
        // .destination((10, 0, 200, 1))
        .up();

    let dev = tun::create_as_async(&config).unwrap();
    let mut buf = [0; 4096];

    loop {
        let amount = dev.recv(buf.as_mut()).await.unwrap();
        println!("Received {} bytes", amount);

        if let Ok(slice) = Ipv6Slice::from_slice(&buf[..amount]) {
            if let Ok((udp_header, payload)) = UdpHeader::from_slice(slice.payload().payload) {
                let content = String::from_utf8_lossy(payload);
                println!("Received: {:?}, {:?}", udp_header, content);
            }
        }
        if let Ok(slice) = Ipv4Slice::from_slice(&buf[..amount]) {
            if let Ok((udp_header, payload)) = UdpHeader::from_slice(slice.payload().payload) {
                let content = String::from_utf8_lossy(payload);
                println!("Received: {:?}, {:?}", udp_header, content);
            }
        }
    }
}
