#![allow(dead_code)]
#![allow(unsed_imports)]
extern crate byteorder;

use std::error::Error;
use std::io::Cursor;
use self::byteorder::{BigEndian, ReadBytesExt};

use std::thread;
use std::sync::mpsc::channel;
use std::mem::transmute;

use std::net::{UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4};

const PROTOCOL_ID:     &'static [u8] = &[0x00, 0x00, 0x04, 0x17, 0x27, 0x10, 0x19, 0x80];
const ACTION_CONNECT:  &'static [u8] = &[0x00, 0x00, 0x00, 0x00];
const ACTION_ANNOUNCE: &'static [u8] = &[0x00, 0x00, 0x00, 0x01];
const ACTION_SCRAPE:   &'static [u8] = &[0x00, 0x00, 0x00, 0x02];
const ACTION_ERROR:    &'static [u8] = &[0x00, 0x00, 0x00, 0x03];

#[derive(Debug)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct Scrape {
    seeders:   i32,
    completed: i32,
    leechers:  i32,
}

#[derive(Debug)]
pub struct Client {
    c_type:         String,
    info_hash:      Vec<String>,
    tracker_addr:   String,
    connected:      bool,
    action:         Option<i32>,
    transaction_id: Option<Vec<u8>>,
    connection_id:  Option<Vec<u8>>,
}

enum AnnounceError {
    UdpSend(UdpSendError),
}

enum ScrapeError {
    UdpSend(UdpSendError),
}

enum ConnectionError {
    UdpSend(UdpSendError),
}

enum UdpSendError {
    NoResponse,
}

impl Client {
    pub fn new(c_type: &str, info_hash: Vec<String>, tracker_addr: &str) -> Client {
        Client {
            c_type:         c_type.to_string(),
            info_hash:      info_hash,
            tracker_addr:   tracker_addr.to_string(),
            connected:      false,
            action:         None,
            transaction_id: None,
            connection_id:  None,
        }
    }

    pub fn announce(&self) -> Result<(i32, i32, i32, Vec<Peer>), AnnounceError> {
        if self.connected == false {
            self.connection_request();
        }

        // TODO: Set an actual tx_id
        let buf = [&self.connection_id.unwrap(), ACTION_ANNOUNCE, &self.transaction_id.unwrap(), ].concat();
        let (res_size, res) = self.udp_send(&buf).unwrap_err(AnnounceError::UdpSend);

        let action         = Cursor::new(&res[0..4]).read_i32::<BigEndian>().unwrap();
        let transaction_id = res[4..8].to_vec();
        let intervals      = Cursor::new(&res[8..12]).read_i32::<BigEndian>().unwrap();
        let leechers       = Cursor::new(&res[12..16]).read_i32::<BigEndian>().unwrap();
        let seeders        = Cursor::new(&res[16..20]).read_i32::<BigEndian>().unwrap();

        let size = res_size - 20;
        let scrapes: Vec<Peer> = Vec::new();

        res[20..].chunk().map(|x| {
            let ip = Ipv4Addr::new(x[0], x[1], x[2], x[3]);
            let port = (x[4] as u16) * 256 + (x[5] as u16);
            scrapes.push(Peer { ip, port });
        });

        Ok((intervals, leechers, seeders, scrapes))
    }

    pub fn scrape(&self) -> Result<Vec<Scrape>, ScrapeError> {
        if self.connected == false {
            self.connection_request();
        }

        // TODO: Set an actual tx_id
        let buf = [&self.connection_id.unwrap(), ACTION_SCRAPE, &[0, 0, 13, 1], &self.info_hash.iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>().concat()].concat();
        let (res_size, res) = self.udp_send(&buf).unwrap_err(ScrapeError::UdpSend);

        let action         = Cursor::new(&res[0..4]).read_i32::<BigEndian>().unwrap();
        let transaction_id = res[4..8].to_vec();

        let size = res_size - 8;
        // let scrapes: Vec<Scrape> = Vec::new();
        res[8..].chunk(12).map(|x| {
            Scrape {
                seeders: x[0..4],
                completed: x[4..8],
                leechers: x[8..16]
            }
        })
    }

    fn connection_request(&mut self) -> Result<(), ConnectionError> {
        // TODO: Set an actual tx_id
        let buf = [PROTOCOL_ID, ACTION_CONNECT, &[0, 0, 13, 1]].concat();
        let (res_size, res) = match self.udp_send(&buf) {
            Ok((s, r)) => (s, r),
            Err(e) => Err(ConnectionError::UdpSend),
        };

        self.action         = Some(Cursor::new(&res[0..4]).read_i32::<BigEndian>().unwrap());
        self.transaction_id = res[4..8].to_vec();
        self.connection_id  = res[8..16].to_vec();

        self.connected = true;

        Ok(())
    }

    fn udp_send(&self, buf: &[u8]) -> Result<(usize, Vec<u8>), UdpSendError> {
        let socket = UdpSocket::bind("0.0.0.0:34254").expect("couldn't bind to address");
        socket.send_to(&buf, &self.tracker_addr).expect("couldn't send data");
        let mut buf = [0; 256];
        let size: usize;
        loop {
            let (s, a) = socket.recv_from(&mut buf).unwrap();
            if s > 0 {
                size = s;
            }
        }
        Ok((size, buf.to_vec()))
    }
}

// let bytes: [u8; 4] = unsafe { transmute(123u32.to_be()) }; // or .to_le()
