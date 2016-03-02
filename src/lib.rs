//! Unified diff parsing/metadata extraction library for Rust
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]
#![cfg_attr(feature="clippy", warn(cyclomatic_complexity))]

extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::error;

use regex::Regex;


lazy_static! {
    static ref RE_SOURCE_FILENAME: Regex = Regex::new(r"^--- (?P<filename>[^\t\n]+)(?:\t(?P<timestamp>[^\n]+))?").unwrap();
    static ref RE_TARGET_FILENAME: Regex = Regex::new(r"^\+\+\+ (?P<filename>[^\t\n]+)(?:\t(?P<timestamp>[^\n]+))?").unwrap();
    static ref RE_HUNK_HEADER: Regex = Regex::new(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@[ ]?(.*)").unwrap();
    static ref RE_HUNK_BODY_LINE: Regex = Regex::new(r"^(?P<line_type>[- \n\+\\]?)(?P<value>.*)").unwrap();
}

pub const LINE_TYPE_ADDED: &'static str = "+";
pub const LINE_TYPE_REMOVED: &'static str = "-";
pub const LINE_TYPE_CONTEXT: &'static str = " ";
pub const LINE_TYPE_EMPTY: &'static str = "\n";

#[derive(Debug)]
pub enum Error {
    TargetWithoutSource(String),
    UnexpectedHunk(String),
    ExpectLine(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TargetWithoutSource(ref l) => write!(f, "Target without source: {}", l),
            Error::UnexpectedHunk(ref l) => write!(f, "Unexpected hunk found: {}", l),
            Error::ExpectLine(ref l) => write!(f, "Hunk line expected: {}", l),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TargetWithoutSource(..) => "Target without source",
            Error::UnexpectedHunk(..) => "Unexpected hunk found",
            Error::ExpectLine(..) => "Hunk line expected",
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;


/// A diff line
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line {
    pub source_line_no: Option<usize>,
    pub target_line_no: Option<usize>,
    pub diff_line_no: usize,
    pub line_type: String,
    pub value: String,
}

impl Line {
    pub fn new<T: Into<String>>(value: T, line_type: T) -> Line {
        Line {
            source_line_no: Some(0usize),
            target_line_no: Some(0usize),
            diff_line_no: 0usize,
            line_type: line_type.into(),
            value: value.into(),
        }
    }

    pub fn is_added(&self) -> bool {
        LINE_TYPE_ADDED == &self.line_type
    }

    pub fn is_removed(&self) -> bool {
        LINE_TYPE_REMOVED == &self.line_type
    }

    pub fn is_context(&self) -> bool {
        LINE_TYPE_CONTEXT == &self.line_type
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.line_type, self.value)
    }
}

/// Each of the modified blocks of a file
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hunk {
    pub added: usize,
    pub removed: usize,
    pub source_start: usize,
    pub source_length: usize,
    pub target_start: usize,
    pub target_length: usize,
    pub section_header: String,
    lines: Vec<Line>,
    source: Vec<String>,
    target: Vec<String>,
}

impl Hunk {
    pub fn new<T: Into<String>>(source_start: usize,
                                source_length: usize,
                                target_start: usize,
                                target_length: usize,
                                section_header: T)
                                -> Hunk {
        Hunk {
            added: 0usize,
            removed: 0usize,
            source_start: source_start,
            source_length: source_length,
            target_start: target_start,
            target_length: target_length,
            section_header: section_header.into(),
            lines: vec![],
            source: vec![],
            target: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.source.len() == self.source_length && self.target.len() == self.target_length
    }

    pub fn source_lines(&self) -> Vec<Line> {
        self.lines.iter().cloned().filter(|l| l.is_context() || l.is_removed()).collect()
    }

    pub fn target_lines(&self) -> Vec<Line> {
        self.lines.iter().cloned().filter(|l| l.is_context() || l.is_added()).collect()
    }

    pub fn append(&mut self, line: Line) {
        if line.is_added() {
            self.added = self.added + 1;
            self.target.push(format!("{}{}", line.line_type, line.value));
        } else if line.is_removed() {
            self.removed = self.removed + 1;
            self.source.push(format!("{}{}", line.line_type, line.value));
        } else if line.is_context() {
            self.source.push(format!("{}{}", line.line_type, line.value));
            self.target.push(format!("{}{}", line.line_type, line.value));
        }
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

impl fmt::Display for Hunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = format!("@@ -{},{} +{},{} @@ {}\n",
                             self.source_start,
                             self.source_length,
                             self.target_start,
                             self.target_length,
                             self.section_header);
        let content = self.lines.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "{}{}", header, content)
    }
}

impl IntoIterator for Hunk {
    type Item = Line;
    type IntoIter = ::std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

/// Patch updated file, contains a list of Hunks
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatchedFile {
    pub source_file: String,
    pub source_timestamp: Option<String>,
    pub target_file: String,
    pub target_timestamp: Option<String>,
    hunks: Vec<Hunk>,
}

impl PatchedFile {
    pub fn new<T: Into<String>>(source_file: T, target_file: T) -> PatchedFile {
        PatchedFile {
            source_file: source_file.into(),
            target_file: target_file.into(),
            source_timestamp: None,
            target_timestamp: None,
            hunks: vec![],
        }
    }

    pub fn with_hunks<T: Into<String>>(source_file: T, target_file: T, hunks: Vec<Hunk>) -> PatchedFile {
        PatchedFile {
            source_file: source_file.into(),
            target_file: target_file.into(),
            source_timestamp: None,
            target_timestamp: None,
            hunks: hunks,
        }
    }

    pub fn path(&self) -> String {
        if self.source_file.starts_with("a/") && self.target_file.starts_with("b/") {
            return self.source_file[2..].to_owned();
        }
        if self.source_file.starts_with("a/") && "/dev/null" == &self.target_file {
            return self.source_file[2..].to_owned();
        }
        if self.target_file.starts_with("b/") && "/dev/null" == &self.source_file {
            return self.target_file[2..].to_owned();
        }
        self.source_file.clone()
    }

    pub fn added(&self) -> usize {
        self.hunks.iter().map(|h| h.added).fold(0, |acc, x| acc + x)
    }

    pub fn removed(&self) -> usize {
        self.hunks.iter().map(|h| h.removed).fold(0, |acc, x| acc + x)
    }

    pub fn is_added_file(&self) -> bool {
        self.hunks.len() == 1 && self.hunks[0].source_start == 0 && self.hunks[0].source_length == 0
    }

    pub fn is_removed_file(&self) -> bool {
        self.hunks.len() == 1 && self.hunks[0].target_start == 0 && self.hunks[0].target_length == 0
    }

    pub fn is_modified_file(&self) -> bool {
        !self.is_added_file() && !self.is_removed_file()
    }

    fn parse_hunk(&mut self, header_line_no: usize, header: &str, diff: &[(usize, &str)]) -> Result<()> {
        let header_info = RE_HUNK_HEADER.captures(header).unwrap();
        let source_start = header_info.at(1).unwrap().parse::<usize>().unwrap();
        let source_length = header_info.at(2).unwrap().parse::<usize>().unwrap();
        let target_start = header_info.at(3).unwrap().parse::<usize>().unwrap();
        let target_length = header_info.at(4).unwrap().parse::<usize>().unwrap();
        let section_header = header_info.at(5).unwrap();
        let mut hunk = Hunk {
            added: 0usize,
            removed: 0usize,
            source: vec![],
            target: vec![],
            lines: vec![],
            source_start: source_start,
            source_length: source_length,
            target_start: target_start,
            target_length: target_length,
            section_header: section_header.to_owned(),
        };
        let mut source_line_no = source_start;
        let mut target_line_no = target_start;
        let expected_source_end = source_start + source_length;
        let expected_target_end = target_start + target_length;
        for &(diff_line_no, line) in diff {
            if let Some(valid_line) = RE_HUNK_BODY_LINE.captures(line) {
                let mut line_type = valid_line.name("line_type").unwrap();
                if line_type == LINE_TYPE_EMPTY || line_type == "" {
                    line_type = LINE_TYPE_CONTEXT;
                }
                let value = valid_line.name("value").unwrap();
                let mut original_line = Line {
                    source_line_no: None,
                    target_line_no: None,
                    diff_line_no: diff_line_no + header_line_no + 1,
                    line_type: line_type.to_owned(),
                    value: value.to_owned(),
                };
                match line_type {
                    LINE_TYPE_ADDED => {
                        original_line.target_line_no = Some(target_line_no);
                        target_line_no = target_line_no + 1;
                    }
                    LINE_TYPE_REMOVED => {
                        original_line.source_line_no = Some(source_line_no);
                        source_line_no = source_line_no + 1;
                    }
                    LINE_TYPE_CONTEXT => {
                        original_line.target_line_no = Some(target_line_no);
                        target_line_no = target_line_no + 1;
                        original_line.source_line_no = Some(source_line_no);
                        source_line_no = source_line_no + 1;
                    }
                    _ => {}
                }
                hunk.append(original_line);
                if source_line_no == expected_source_end && target_line_no == expected_target_end {
                    break;
                }
            } else {
                return Err(Error::ExpectLine(line.to_owned()));
            }
        }
        self.hunks.push(hunk);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.hunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }
}

impl fmt::Display for PatchedFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source = format!("--- {}\n", self.source_file);
        let target = format!("+++ {}\n", self.target_file);
        let hunks = self.hunks.iter().map(|h| h.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "{}{}{}", source, target, hunks)
    }
}

impl IntoIterator for PatchedFile {
    type Item = Hunk;
    type IntoIter = ::std::vec::IntoIter<Hunk>;

    fn into_iter(self) -> Self::IntoIter {
        self.hunks.into_iter()
    }
}

/// Unfied patchset
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatchSet {
    files: Vec<PatchedFile>,
}

impl PatchSet {
    pub fn added_files(&self) -> Vec<PatchedFile> {
        self.files.iter().cloned().filter(|f| f.is_added_file()).collect()
    }

    pub fn removed_files(&self) -> Vec<PatchedFile> {
        self.files.iter().cloned().filter(|f| f.is_removed_file()).collect()
    }

    pub fn modified_files(&self) -> Vec<PatchedFile> {
        self.files.iter().cloned().filter(|f| f.is_modified_file()).collect()
    }

    pub fn new() -> PatchSet {
        PatchSet { files: vec![] }
    }

    pub fn parse<T: AsRef<str>>(&mut self, input: T) -> Result<()> {
        let mut current_file: Option<PatchedFile> = None;
        let diff: Vec<(usize, &str)> = input.as_ref().split('\n').enumerate().collect();
        let mut source_file: Option<String> = None;
        let mut source_timestamp: Option<String> = None;

        for &(line_no, line) in &diff {
            // check for source file header
            if let Some(captures) = RE_SOURCE_FILENAME.captures(line) {
                source_file = Some(captures.name("filename").unwrap_or("").to_owned());
                source_timestamp = Some(captures.name("timestamp").unwrap_or("").to_owned());
                if let Some(patched_file) = current_file {
                    self.files.push(patched_file.clone());
                }
                current_file = None;
                continue;
            }
            // check for target file header
            if let Some(captures) = RE_TARGET_FILENAME.captures(line) {
                if current_file.is_some() {
                    return Err(Error::TargetWithoutSource(line.to_owned()));
                }
                let target_file = Some(captures.name("filename").unwrap_or("").to_owned());
                let target_timestamp = Some(captures.name("timestamp").unwrap_or("").to_owned());

                // add current file to PatchSet
                current_file = Some(PatchedFile {
                    source_file: source_file.clone().unwrap(),
                    target_file: target_file.clone().unwrap(),
                    source_timestamp: source_timestamp.clone(),
                    target_timestamp: target_timestamp.clone(),
                    hunks: vec![],
                });
                continue;
            }
            // check for hunk header
            if RE_HUNK_HEADER.is_match(line) {
                if let Some(ref mut patched_file) = current_file {
                    try!(patched_file.parse_hunk(line_no, line, &diff[line_no + 1..]));
                } else {
                    return Err(Error::UnexpectedHunk(line.to_owned()));
                }
            }
        }
        if let Some(patched_file) = current_file {
            self.files.push(patched_file.clone());
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

impl fmt::Display for PatchSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let diff = self.files.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "{}", diff)
    }
}

impl IntoIterator for PatchSet {
    type Item = PatchedFile;
    type IntoIter = ::std::vec::IntoIter<PatchedFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}
