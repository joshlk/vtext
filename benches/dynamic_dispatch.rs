#![feature(test)]
extern crate test;
use test::Bencher;

use cached::proc_macro::cached;

use rust_stemmers::Stemmer;
use std::collections::HashSet;
use std::path::Path;
use glob::glob;
use std::fs;

extern crate regex;

static N_FILES: usize = 1000;

#[cached]
fn load_text() -> String {
    let data_dir = Path::new(file!()).parent().unwrap().parent().unwrap().join("data");
    let data_glob_pat = data_dir.join("*").join("*").to_str().unwrap().to_string();

    let mut text = String::new();
    for file_path in glob(data_glob_pat.as_str()).expect("Failed to read glob pattern").take(N_FILES) {
        let file_text = fs::read_to_string(file_path.unwrap())
            .expect("Something went wrong reading the file");

        text = [text, file_text].join("\n")
    }
    text

}

#[derive(Debug)]
enum StringWrap<'a> {
    Slice(&'a str),
    String(String),
}


// ----- Using `impl Iterator` type signature

struct TokenizerImpl {
    split_on: String
}

impl TokenizerImpl {
    pub fn transform<'a>(&'a self, text: &'a str) -> impl Iterator<Item = StringWrap<'a>> + 'a {
        text.split(&self.split_on).map(|x| StringWrap::Slice(x))
    }
}

struct FilterImpl {
    words: HashSet<String>,
}

impl FilterImpl {
    pub fn transform<'a>(
        &'a self,
        tokens: impl Iterator<Item = StringWrap<'a>> + 'a,
    ) -> impl Iterator<Item = StringWrap<'a>> + 'a {
        tokens.filter(move |x| match x {
            StringWrap::Slice(s) => !self.words.contains(s.clone()),
            StringWrap::String(s) => !self.words.contains(&s.clone()),
        })
    }
}

struct StemmerImpl {
    stemmer: rust_stemmers::Stemmer
}

impl StemmerImpl {
    pub fn transform<'a>(
        &'a self,
        tokens: impl Iterator<Item = StringWrap<'a>> + 'a,
    ) -> impl Iterator<Item = StringWrap<'a>> + 'a {
        // Outputs a StringWrap::string
        tokens.map(move |x| match x {
            StringWrap::Slice(s) => StringWrap::String(self.stemmer.stem(s).to_string()),
            StringWrap::String(s) => StringWrap::String(self.stemmer.stem(&s).to_string()),
        })
    }
}

// ----- Using `dym Iterator` type signature

struct TokenizerDyn {
    split_on: String
}

impl TokenizerDyn {
    pub fn transform<'a>(&'a self, text: &'a str) -> Box<dyn Iterator<Item = StringWrap<'a>> + 'a> {
        Box::new(text.split(&self.split_on).map(|x| StringWrap::Slice(x)))
    }
}

struct FilterDyn {
    words: HashSet<String>,
}

impl FilterDyn {
    pub fn transform<'a>(
        &'a self,
        tokens: Box<dyn Iterator<Item = StringWrap<'a>> + 'a>,
    ) -> Box<dyn Iterator<Item = StringWrap<'a>> + 'a> {
        Box::new(tokens.filter(move |x| match x {
            StringWrap::Slice(s) => !self.words.contains(s.clone()),
            StringWrap::String(s) => !self.words.contains(&s.clone()),
        }))
    }
}

struct StemmerDyn {
    stemmer: rust_stemmers::Stemmer
}

impl StemmerDyn {
    pub fn transform<'a>(
        &'a self,
        tokens: Box<dyn Iterator<Item = StringWrap<'a>> + 'a>,
    ) -> Box<dyn Iterator<Item = StringWrap<'a>> + 'a> {
        // Outputs a StringWrap::string
        Box::new(tokens.map(move |x| match x {
            StringWrap::Slice(s) => StringWrap::String(self.stemmer.stem(s).to_string()),
            StringWrap::String(s) => StringWrap::String(self.stemmer.stem(&s).to_string()),
        }))
    }
}

// ----

macro_rules! hashset_str {
    ($( $elem:expr ),*) => {{
        let mut hs: HashSet<String> = HashSet::new();
        $(
            hs.insert($elem.to_string());
        )*
        hs
    }}
}

#[bench]
fn impl_pipeline(b: &mut Bencher) {
    let text = load_text();

    // Pipeline components
    let tokenizer = TokenizerImpl {
        split_on: " ".to_string(),
    };
    let filter = FilterImpl {
        words: hashset_str!("i", "me","my","myself","we","our","ours","ourselves","you","your","yours","yourself","yourselves","he","him","his","himself","she","her","hers","herself","it","its","itself","they","them","their","theirs","themselves","what","which","who","whom","this","that","these","those","am","is","are","was","were","be","been","being","have","has","had","having","do","does","did","doing","a","an","the","and","but","if","or","because","as","until","while","of","at","by","for","with","about","against","between","into","through","during","before","after","above","below","to","from","up","down","in","out","on","off","over","under","again","further","then","once","here","there","when","where","why","how","all","any","both","each","few","more","most","other","some","such","no","nor","not","only","own","same","so","than","too","very","s","t","can","will"),
    };
    let stemmer = StemmerImpl {
        stemmer: rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English)
    };

    b.iter(|| {
        let output = tokenizer.transform(&text);
        let output = filter.transform(output);
        let output = stemmer.transform(output).collect::<Vec<_>>();
    });
}


#[bench]
fn dyn_pipeline(b: &mut Bencher) {
    let text = load_text();

    // Pipeline components
    let tokenizer = TokenizerDyn {
        split_on: " ".to_string(),
    };
    let filter = FilterDyn {
        words: hashset_str!("i", "me","my","myself","we","our","ours","ourselves","you","your","yours","yourself","yourselves","he","him","his","himself","she","her","hers","herself","it","its","itself","they","them","their","theirs","themselves","what","which","who","whom","this","that","these","those","am","is","are","was","were","be","been","being","have","has","had","having","do","does","did","doing","a","an","the","and","but","if","or","because","as","until","while","of","at","by","for","with","about","against","between","into","through","during","before","after","above","below","to","from","up","down","in","out","on","off","over","under","again","further","then","once","here","there","when","where","why","how","all","any","both","each","few","more","most","other","some","such","no","nor","not","only","own","same","so","than","too","very","s","t","can","will"),
    };
    let stemmer = StemmerDyn {
        stemmer: rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English)
    };

    b.iter(|| {
        let output = tokenizer.transform(&text);
        let output = filter.transform(output);
        let output = stemmer.transform(output).collect::<Vec<_>>();
    });
}
