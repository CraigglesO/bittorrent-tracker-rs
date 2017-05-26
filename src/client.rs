#![allow(dead_code)]

use std::mem::transmute;

use std::net::UdpSocket;
use std::net::{SocketAddr, SocketAddrV4};

const PROTOCOL_ID:     &'static [u8] = [0x00, 0x00, 0x04, 0x17, 0x27, 0x10, 0x19, 0x80];
const ACTION_CONNECT:  &'static [u8] = [0x00, 0x00, 0x00, 0x00];
const ACTION_ANNOUNCE: &'static [u8] = [0x00, 0x00, 0x00, 0x01];
const ACTION_SCRAPE:   &'static [u8] = [0x00, 0x00, 0x00, 0x02];
const ACTION_ERROR:    &'static [u8] = [0x00, 0x00, 0x00, 0x03];

pub struct Client {
    c_type:       String,
    info_hash:    String,
    tracker_addr: SocketAddrV4,
}

impl Client {
    pub fn new(c_type: &str, info_hash: &str, tracker_addr: &str) -> Client {
        Client {
            c_type,
            info_hash,
            tracker_addr,
        }
    }

    pub fn get_connection_key() {

    }

    pub fn announce() {

    }

    pub fn scrape(&self) {

    }

    pub fn connection_request(&self) -> Result<(usize, SocketAddr)> {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
        socket.send_to([PROTOCOL_ID, ACTION_CONNECT, [0x00, 0x00, 0x13, 0x01]].concat(), self.tracker_addr).expect("couldn't send data");
        let mut buf = [0; 100];
        (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data")

    }
}

// let bytes: [u8; 4] = unsafe { transmute(123u32.to_be()) }; // or .to_le()
