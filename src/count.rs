use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Language {
    comments: &'static [&'static str],
    block_comments: Option<(&'static str, &'static str)>,
}

impl Language {
    pub fn new(
        comments: &'static [&'static str],
        block_comments: Option<(&'static str, &'static str)>,
    ) -> Language {
        Language {
            comments,
            block_comments,
        }
    }
}

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

    if line_trimmed.is_empty() || line_trimmed == "\n" || line_trimmed == "\r\n" {
        return (State::Code, Counts::new(0, 0, 1));
    }

    match state {
        State::Code => {
            if is_comment(line_trimmed, comment_strings) {
                return (State::Code, Counts::new(0, 1, 0));
            }

            if let Some(block) = block_comment {
                if line_trimmed.starts_with(block.0) {
                    // we might exit block comment same line, in this case we are back to State::Code
                    if line_trimmed.contains(block.1) {
                        return (State::Code, Counts::new(0, 1, 0));
                    }

                    return (State::CommentBlock, Counts::new(0, 1, 0));
                }

                // this is when the line is code but then has opening block comment;
                // we still count it as code but change the state to block comment
                if line_trimmed.contains(block.0) {
                    // we might exit block comment same line, in case we are back to State::Code
                    if line_trimmed.contains(block.1) {
                        return (State::Code, Counts::new(0, 1, 0));
                    }
                    return (State::CommentBlock, Counts::new(1, 0, 0));
                }

                return (State::Code, Counts::new(1, 0, 0));
            } else {
                return (State::Code, Counts::new(1, 0, 0));
            }
        }
        State::CommentBlock => {
            let block =
                block_comment.expect("We must have block_comment if we are in comment block");

            if line_trimmed.contains(block.1) {
                return (State::Code, Counts::new(0, 1, 0));
            } else {
                return (State::CommentBlock, Counts::new(0, 1, 0));
            }
        }
    }
}

pub fn count_lines(contents: &str, lang: &Language) -> Counts {
    let mut state = State::Code;
    let mut counts = Counts::empty();

    for line in str::lines(&contents) {
        let (s, c) = scan_line(&state, line, lang.comments, lang.block_comments);
        state = s;
        counts = counts + c;
    }

    counts
}

#[cfg(test)]
mod tests {
    use crate::count::{Language, count_lines};

    #[test]
    fn test_count_lines_rust() {
        let code = "use std;

            // function that does something
            // useful
            pub fn hello() {
                do something
                do something else // chicky comment
            }";

        let lang = Language::new(&["//"], None);

        let counts = count_lines(code, &lang);

        assert_eq!(counts.code, 5);
        assert_eq!(counts.comments, 2);
        assert_eq!(counts.empty, 1);
        assert_eq!(counts.total(), 8);
    }

    #[test]
    fn test_count_lines_js() {
        let code = "
            // function that does something
            function add(a, b) {
                return a + b;
            }

            /* this is
            standard block comment
            */
            let sum = add;

            /* this block comment ends same line */
            function average(a, b, fn) {
                return fn(a, b) / 2;
            }

            function average(a, b, fn) {
                return fn(a, b) / 2;
            }

            let result = average(10, 20, sum); /* chicky
            block
            comment */

            console.log(result);";

        let lang = Language::new(&["//"], Some(("/*", "*/")));

        let counts = count_lines(code, &lang);

        assert_eq!(counts.code, 12);

        // note the block comment that starts with "/* chicky" does not count towards comments
        // because we treat this line as code
        assert_eq!(counts.comments, 7);
        assert_eq!(counts.empty, 6);
        assert_eq!(counts.total(), 25);
    }
}
