extern crate unidiff;

use std::io::prelude::*;
use std::fs::File;

use unidiff::PatchSet;

#[test]
fn test_parse_sample0_diff() {
    let mut buf = String::new();
    File::open("tests/fixtures/sample0.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

    let mut patch = PatchSet::new();
    patch.parse(&buf).unwrap();

    assert_eq!(3, patch.len());
}

#[test]
fn test_parse_git_diff() {
    let mut buf = String::new();
    File::open("tests/fixtures/git.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

    let mut patch = PatchSet::new();
    patch.parse(&buf).unwrap();

    assert_eq!(3, patch.len());

    let added_files = patch.added_files();
    assert_eq!(1, added_files.len());
    assert_eq!("added_file", added_files[0].path());
    assert_eq!(4, added_files[0].added());
    assert_eq!(0, added_files[0].removed());

    let removed_files = patch.removed_files();
    assert_eq!(1, removed_files.len());
    assert_eq!("removed_file", removed_files[0].path());
    assert_eq!(0, removed_files[0].added());
    assert_eq!(3, removed_files[0].removed());

    let modified_files = patch.modified_files();
    assert_eq!(1, modified_files.len());
    assert_eq!("modified_file", modified_files[0].path());
    assert_eq!(3, modified_files[0].added());
    assert_eq!(1, modified_files[0].removed());
}

#[test]
fn test_parse_bzr_diff() {
    let mut buf = String::new();
    File::open("tests/fixtures/bzr.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

    let mut patch = PatchSet::new();
    patch.parse(&buf).unwrap();

    assert_eq!(3, patch.len());

    let added_files = patch.added_files();
    assert_eq!(1, added_files.len());
    assert_eq!("added_file", added_files[0].path());
    assert_eq!(4, added_files[0].added());
    assert_eq!(0, added_files[0].removed());

    let removed_files = patch.removed_files();
    assert_eq!(1, removed_files.len());
    assert_eq!("removed_file", removed_files[0].path());
    assert_eq!(0, removed_files[0].added());
    assert_eq!(3, removed_files[0].removed());

    let modified_files = patch.modified_files();
    assert_eq!(1, modified_files.len());
    assert_eq!("modified_file", modified_files[0].path());
    assert_eq!(3, modified_files[0].added());
    assert_eq!(1, modified_files[0].removed());
}

#[test]
fn test_parse_hg_diff() {
    let mut buf = String::new();
    File::open("tests/fixtures/hg.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

    let mut patch = PatchSet::new();
    patch.parse(&buf).unwrap();

    assert_eq!(3, patch.len());

    let added_files = patch.added_files();
    assert_eq!(1, added_files.len());
    assert_eq!("added_file", added_files[0].path());
    assert_eq!(4, added_files[0].added());
    assert_eq!(0, added_files[0].removed());

    let removed_files = patch.removed_files();
    assert_eq!(1, removed_files.len());
    assert_eq!("removed_file", removed_files[0].path());
    assert_eq!(0, removed_files[0].added());
    assert_eq!(3, removed_files[0].removed());

    let modified_files = patch.modified_files();
    assert_eq!(1, modified_files.len());
    assert_eq!("modified_file", modified_files[0].path());
    assert_eq!(3, modified_files[0].added());
    assert_eq!(1, modified_files[0].removed());
}

#[test]
fn test_parse_svn_diff() {
    let mut buf = String::new();
    File::open("tests/fixtures/svn.diff").and_then(|mut r| r.read_to_string(&mut buf)).unwrap();

    let mut patch = PatchSet::new();
    patch.parse(&buf).unwrap();

    assert_eq!(3, patch.len());

    let added_files = patch.added_files();
    assert_eq!(1, added_files.len());
    assert_eq!("added_file", added_files[0].path());
    assert_eq!(4, added_files[0].added());
    assert_eq!(0, added_files[0].removed());

    let removed_files = patch.removed_files();
    assert_eq!(1, removed_files.len());
    assert_eq!("removed_file", removed_files[0].path());
    assert_eq!(0, removed_files[0].added());
    assert_eq!(3, removed_files[0].removed());

    let modified_files = patch.modified_files();
    assert_eq!(1, modified_files.len());
    assert_eq!("modified_file", modified_files[0].path());
    assert_eq!(3, modified_files[0].added());
    assert_eq!(1, modified_files[0].removed());
}
