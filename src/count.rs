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

#[derive(Debug, Clone)]
enum State {
    Code,
    Comment,
    CommentBlock,
}

fn is_comment(line: &str, comment_strings: &[&str]) -> bool {
    comment_strings
        .iter()
        .any(|comment| line.starts_with(comment))
}

fn scan_line(
    state: &State,
    line: &str,
    comment_strings: &[&str],
    block_comment: Option<(&str, &str)>,
) -> (State, Counts) {
    let line_trimmed = line.trim();

    if line_trimmed.is_empty() {
        return (state.clone(), Counts::new(0, 0, 1));
    }

    match state {
        State::Code | State::Comment => {
            if is_comment(line_trimmed, comment_strings) {
                (State::Comment, Counts::new(0, 1, 0))
            } else if block_comment.is_some() && line_trimmed.starts_with(block_comment.unwrap().0)
            {
                // we entered and exited block comment same line
                if line_trimmed.contains(block_comment.unwrap().1) {
                    return (State::Code, Counts::new(0, 1, 0));
                }

                (State::CommentBlock, Counts::new(0, 1, 0))
            } else if block_comment.is_some() && line_trimmed.starts_with(block_comment.unwrap().1)
            {
                // this should be unreachable
                (State::CommentBlock, Counts::new(0, 1, 0))
            } else {
                (State::Code, Counts::new(1, 0, 0))
            }
        }
        State::CommentBlock => {
            // we check contains because block comment end mark does not have to appear at start of line
            if block_comment.is_some() && line_trimmed.contains(block_comment.unwrap().1) {
                (State::Code, Counts::new(0, 1, 0))
            } else {
                (State::CommentBlock, Counts::new(0, 1, 0))
            }
        }
    }
}

pub fn count_lines(
    path: &Path,
    comment_strings: &[&str],
    block_comment: Option<(&str, &str)>,
) -> Counts {
    let contents = fs::read_to_string(path).unwrap();

    let mut state = State::Code;
    let mut counts = Counts::empty();

    for line in str::lines(&contents) {
        let (s, c) = scan_line(&state, line, comment_strings, block_comment);
        state = s;
        counts = counts + c;
    }

    counts
}
