extern crate bittorrent_tracker;

use Client;

fn main() {
    let client = Client::new("udp", "0123456789012345678901234567890123456789", "zer0day.ch:1337");

    let x = client.connection_request();
    println!("responce: {:?}", x);
}
