#![feature(test)]
extern crate test;
extern crate unidiff;

use std::io::prelude::*;
use std::fs::File;

use test::Bencher;
use unidiff::PatchSet;


#[bench]
fn bench_parse_diff(b: &mut Bencher) {
    b.iter(|| {
        let mut buf = String::new();
        File::open("tests/fixtures/sample0.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

        let mut patch = PatchSet::new();
        patch.parse(&buf).unwrap();
    });
}
