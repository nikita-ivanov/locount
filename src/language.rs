use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Language {
    pub comments: &'static [&'static str],
    pub block_comments: Option<(&'static str, &'static str)>,
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

pub fn languages() -> HashMap<&'static str, Language> {
    HashMap::from_iter([
        ("rs", Language::new(&["//"], Some(("/*", "*/")))),
        ("py", Language::new(&["#"], None)),
        ("js", Language::new(&["//"], Some(("/*", "*/")))),
        ("ts", Language::new(&["//"], Some(("/*", "*/")))),
        ("svelte", Language::new(&["//"], Some(("/*", "*/")))),
        ("csharp", Language::new(&["//"], Some(("/*", "*/")))),
        ("c", Language::new(&["//"], Some(("/*", "*/")))),
        ("h", Language::new(&["//"], Some(("/*", "*/")))),
        ("cpp", Language::new(&["//"], Some(("/*", "*/")))),
        ("haskell", Language::new(&["--"], Some(("{-", "-}")))),
        ("sql", Language::new(&["//"], Some(("/*", "*/")))),
        ("json", Language::new(&["//"], None)),
        ("jsonl", Language::new(&["//"], None)),
        ("jsonc", Language::new(&["//"], None)),
        ("java", Language::new(&["//"], Some(("/*", "*/")))),
        ("proto", Language::new(&[], None)),
        ("css", Language::new(&[], Some(("/*", "*/")))),
        ("nix", Language::new(&["#"], None)),
        ("metal", Language::new(&[], None)),
        ("ps1", Language::new(&["#"], Some(("<#", "#>")))),
        ("sh", Language::new(&["#"], None)),
    ])
}
