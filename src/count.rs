use std::{fs, ops::Add, path::PathBuf};

#[derive(Debug)]
pub struct Counts {
    code: usize,
    comments: usize,
    empty: usize,
}

impl Counts {
    pub fn new(code: usize, comments: usize, empty: usize) -> Counts {
        Counts {
            code,
            comments,
            empty,
        }
    }

    pub fn empty() -> Counts {
        Counts::new(0, 0, 0)
    }

    pub fn total(&self) -> usize {
        self.code + self.comments + self.empty
    }
}

impl Add for Counts {
    type Output = Counts;

    fn add(self, rhs: Self) -> Self::Output {
        Counts {
            code: self.code + rhs.code,
            comments: self.comments + rhs.comments,
            empty: self.empty + rhs.empty,
        }
    }
}

pub fn count_lines(path: &PathBuf) -> Counts {
    let contents = fs::read_to_string(path).unwrap();

    let mut total = 0;
    let mut total_comments = 0;
    let mut total_empty = 0;

    for line in contents.split("\n") {
        if line.starts_with("//") {
            total_comments += 1;
        } else if line.is_empty() {
            total_empty += 1;
        } else {
            total += 1;
        }
    }

    Counts::new(total, total_comments, total_empty)
}
