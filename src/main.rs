extern crate pnet;

use std::env;

use log::{error, info};
use pnet::{
    datalink::{
        self,
        Channel::{self},
    },
    packet::{
        ethernet::{EtherTypes, EthernetPacket},
        ip::IpNextHeaderProtocols,
        ipv4::Ipv4Packet,
        ipv6::Ipv6Packet,
        tcp::TcpPacket,
        udp::UdpPacket, Packet,
    },
};

mod packets;
use packets::GettableEndpoints;

const WIDTH: usize = 20;

fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") };
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];

    // インターフェースの選択
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    // データリンクのチャンネルを取得
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                // 受信データからイーサネットフレームの構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6 packet");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}", e);
            }
        }
    }
}

/**
 * IPv4パケットを構築し次のレイヤのハンドラを呼び出す
 */
fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP or UDP packet");
            }
        }
    }
}

/**
 * IPv6のパケットを構築し次のレイヤのハンドラを呼び出す
 */
fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP or UDP packet");
            }
        }
    }
}

/**
 * TCP パケットを構築する
 */
fn tcp_handler(packet: &dyn GettableEndpoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

/**
 * UDP パケットを構築する
 */
fn udp_handler(packet: &dyn GettableEndpoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}

/**
 * アプリケーション層のデータをバイナリで表示する
 */
fn print_packet_info(l3: &dyn GettableEndpoints, l4: &dyn GettableEndpoints, proto: &str) {
    println!(
        "Captured a {} packet from {}|{} to {}|{}\n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination(),
    );
    let payload = l4.get_payload();
    let len = payload.len();

    // ペイロードの表示
    // 指定した定数幅で表示を行う
    for i in 0..len{
        print!("{:<02X} ",payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1{
            for _j in 0..WIDTH - 1 - (i % (WIDTH)){
                print!("  ");
            }
            print!("| ");
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic(){
                    print!("{}",payload[j] as char);
                }else{
                    // 非 ascii 文字は.で表示
                    print!(".");
                }
            }
            println!();
        } 
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}
