use {
    etherparse::{Ipv4Slice, Ipv6Slice, UdpHeader},
    std::{
        sync::Arc,
        net::IpAddr,
    },
    tokio::net::UdpSocket,
    tun::{
        create_as_async as create_tun,
        AsyncDevice as Tun,
        Configuration as TunConfig
    }
};

#[tokio::main]
async fn main() {
    let spider  = Arc::new(Spider::new().await);
    tokio::spawn({
        let spider = spider.clone();
        async move {
            spider.tun_listen().await;
        }
    });

    loop {
        let spider = spider.clone();
        spider.listen().await;
    }
}

struct Spider {
    send_socket: UdpSocket,
    recv_socket: UdpSocket,
    tun: Tun,
}

impl Spider {
    async fn new() -> Self {
        let send_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        let recv_socket = UdpSocket::bind("0.0.0.0:7890").await.unwrap();
        let mut config = TunConfig::default();
        config
            .address((10, 200, 0, 0))
            .netmask((255, 255, 0, 0))
            // .destination((10,0, 200, 1))
            .up();
        #[cfg(target_os = "linux")]
        config.platform_config(|cfg| {
            cfg.ensure_root_privileges(true);
        });
        let tun = create_tun(&config).unwrap();
        Self { send_socket, recv_socket, tun }
    }

    async fn tun_listen(self: Arc<Self>) {
        let mut buf = [0; 4096];

        loop {
            let amount = self.tun.recv(buf.as_mut()).await.unwrap();

            let packet = buf[..amount].to_vec();
            // try to parse the ipv4 header,
            // if it fails, just ignore the packet
            if let Ok(slice) = Ipv4Slice::from_slice(&packet) {
                let header = slice.header();
                let dest = header.destination();
                let dest = IpAddr::from(dest);
                // TODO: find the destination address by header dest address
                tokio::spawn({
                    let spider =  self.clone();
                    async move {
                        spider.send_socket.send_to(&packet, "10.200.0.1:1234").await.unwrap()
                    }
                });
            }


        }
    }

    async fn listen(self: Arc<Self>) {
        let mut buf = [0; 65535];
        let amount = self.recv_socket.recv(&mut buf).await.unwrap();
        let data = buf[..amount].to_vec();

        let spider = self.clone();
        tokio::spawn(async move {
            spider.handle_owned(data).await;
        });
    }

    async fn handle_owned(&self, buf: Vec<u8>) {
        // dispatch packets to kernel
        match self.tun.send(&buf).await {
            Ok(size) => {
                // println!("Sent {} bytes", size);
            },
            Err(e) => {
                println!("Error sending data: {}", e);
            }
        }
    }
}
