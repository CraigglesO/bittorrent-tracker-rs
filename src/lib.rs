mod client;
mod server;

pub use client::{Client};
// pub use server::{Server};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() -> () {
        // Wire::from_file("screen.magnet").unwrap();
        ()
    }

    #[bench]
    fn bench_test1(b: &mut Bencher) {
        b.iter(|| {
            // Wire::from_file("screen.magnet").unwrap()
        });
    }
}
