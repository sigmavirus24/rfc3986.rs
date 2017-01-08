#![feature(test)]
extern crate test;
extern crate rfc3986;

use test::Bencher;
use rfc3986::uri::Uri;

#[bench]
fn benchmark_url_parsing(b: &mut Bencher) {
    b.iter(|| {
        Uri::from_str(
            "https://username:password@github.com/path/to?query=foo#fragment"
        )
    });
}
