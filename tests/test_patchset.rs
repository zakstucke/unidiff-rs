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

    // three file in the patch
    assert_eq!(3, patch.len());
    // three hunks
    assert_eq!(3, patch[0].len());

    // first file is modified
    assert!(patch[0].is_modified_file());
    assert!(!patch[0].is_added_file());
    assert!(!patch[0].is_removed_file());

    // Hunk 1: five additions, no deletions, a section header
    assert_eq!(6, patch[0][0].added());
    assert_eq!(0, patch[0][0].removed());
    assert_eq!("Section Header", &patch[0][0].section_header);

    // Hunk 2: 2 additions, 8 deletions, no section header
    assert_eq!(2, patch[0][1].added());
    assert_eq!(8, patch[0][1].removed());
    assert_eq!("", &patch[0][1].section_header);

    // Hunk 3: four additions, no deletions, no section header
    assert_eq!(4, patch[0][2].added());
    assert_eq!(0, patch[0][2].removed());
    assert_eq!("", &patch[0][2].section_header);

    // Check file totals
    assert_eq!(12, patch[0].added());
    assert_eq!(8, patch[0].removed());

    // second file is added
    assert!(!patch[1].is_modified_file());
    assert!(patch[1].is_added_file());
    assert!(!patch[1].is_removed_file());

    // third file is removed
    assert!(!patch[2].is_modified_file());
    assert!(!patch[2].is_added_file());
    assert!(patch[2].is_removed_file());
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
