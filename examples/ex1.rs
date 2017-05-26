extern crate bittorrent_tracker;

use bittorrent_tracker::Client;

fn main() {
    let client = Client::new("udp", "0123456789012345678901234567890123456789", "0.0.0.0:1337");

    let x = client.scrape();

    println!("scrape: {:?}", x);
}
