use {
    etherparse::{Ipv4Slice, Ipv6Slice, UdpHeader}, 
};

#[tokio::main]
async fn main() {
    tun_listen().await;
}

async fn tun_listen() {
    println!("Listen on 10.200.0.0");
    let mut config = tun::Configuration::default();
    config
        .address((10, 200, 0, 0))
        .netmask((255, 255, 0, 0))
        // .destination((10,0, 200, 1))
        .up();

    #[cfg(target_os = "linux")]
    config.platform_config(|cfg| {
        cfg.ensure_root_privileges(true);
    });

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
