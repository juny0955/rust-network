use crate::{
    arp::Arp,
    ethernet::Ethernet,
    icmp::Icmp,
    ipv4::{IPv4, Protocol},
};

mod arp;
mod ethernet;
mod icmp;
mod ipv4;

fn main() {
    #[rustfmt::skip]
    let bytes = [
        // Ethernet Header: 14 Bytes
        0xaa, 0xbb, 0xcc, 0xdd, 
        0xee, 0xff, 0x11, 0x22, 
        0x33, 0x44, 0x55, 0x66,
        0x08, 0x00,

        // IPv4 Header: 20 Bytes
        0x45,           // version=4, IHL=5
        0b101010_11,    // DSCP=42, ECN=3
        0x00, 0x1C,     // total length=28
        0x12, 0x34,     // identification
        0x00, 0x00,     // flags, fragment offset
        0x40,           // TTL=64
        0x01,           // protocol=ICMP
        0x97, 0x49,     // IPv4 header checksum
        192, 168, 0, 1, // source IP
        8, 8, 8, 8,     // destination IP

        // IPv4 Payload
        8, 
        0, 
        0xE5, 0xCA, 
        0x12, 0x34, 
        0x00, 0x01
    ];

    let ethernet = match Ethernet::parse(&bytes) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Ethernet 패킷 파싱 실패: {e:?}");
            return;
        }
    };
    println!("Ethernet Packet: {ethernet:?}");

    match ethernet.ether_type {
        ethernet::EtherType::ARP => {
            arp(&ethernet.payload);
        }
        ethernet::EtherType::IPv4 => {
            ipv4(&ethernet.payload);
        }
        ethernet::EtherType::IPv6 => unreachable!(),
    }
}

fn ipv4(payload: &[u8]) {
    let ipv4 = match IPv4::parse(payload) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("IPv4 패킷 파싱 실패: {e:?}");
            return;
        }
    };
    println!("IPv4 Packet: {ipv4:?}");

    match ipv4.protocol {
        Protocol::ICMP => {
            let icmp = match Icmp::parse(&ipv4.payload) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("ICMP 파싱 실패: {e:?}");
                    return;
                }
            };
            println!("ICMP: {icmp:?}");
        }
        _ => unreachable!(),
    }
}

fn arp(payload: &[u8]) {
    let arp = match Arp::parse(payload) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("ARP 파싱 실패: {e:?}");
            return;
        }
    };
    println!("ARP: {arp:?}");
}
