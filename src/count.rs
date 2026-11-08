use std::{fs, ops::Add, path::Path};

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

pub fn count_lines(path: &Path, comment_strings: &[&str]) -> Counts {
    let contents = fs::read_to_string(path).unwrap();

    let mut total_code = 0;
    let mut total_comments = 0;
    let mut total_empty = 0;

    for line in str::lines(&contents) {
        let line_trimmed = line.trim();

        if line_trimmed.is_empty() {
            total_empty += 1;
            continue;
        }

        if comment_strings
            .iter()
            .any(|comment| line_trimmed.starts_with(comment))
        {
            total_comments += 1;
            continue;
        }

        total_code += 1;
    }

    Counts::new(total_code, total_comments, total_empty)
}
