#![feature(test)]

extern crate jagged_array;
extern crate test;
use cached::proc_macro::cached;

use test::Bencher;

use jagged_array::{Jagged2, Jagged2Builder};
use std::fs;
use std::str::from_utf8_unchecked;
use std::collections::HashSet;
use rust_stemmers::Stemmer;
use std::path::Path;
use glob::glob;

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

// -- Read performance comparision between `Vec<&str>`, `Vec<String>` and `Jagged2<u8>`

// sum_size
fn sum_len_str_read_inner(strings: &Vec<&str>) -> usize {
    let mut sum = 0;
    for e in strings.iter() {
        sum += e.len();
    }
    sum
}

fn sum_len_string_read_inner(strings: &Vec<String>) -> usize {
    let mut sum = 0;
    for e in strings.iter() {
        sum += e.len();
    }
    sum
}

fn sum_len_jagged_read_inner(strings: &Jagged2<u8>) -> usize {
    let mut sum = 0;
    for i in 0..strings.len() {
        let e: &str = unsafe { from_utf8_unchecked(strings.get_row(i).unwrap()) };
        sum += e.len();
    }
    sum
}

#[bench]
fn sum_len_str_read(b: &mut Bencher) {
    let text = load_text();
    let tokens_str: Vec<&str> = text.split(" ").collect();

    b.iter(|| {
        // Inner closure, the actual test
        sum_len_str_read_inner(&tokens_str)
    });

}

#[bench]
fn sum_len_string_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();
    let tokens_string: Vec<String> =  tokens_str.iter().map(|x| x.to_string()).collect();

    b.iter(|| {
        // Inner closure, the actual test
        sum_len_string_read_inner(&tokens_string)
    });
}

#[bench]
fn sum_len_jagged_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();

    // Build jagged
    //let mut builder = Jagged2Builder::with_capacity(self.len(), self.flat_len());
    let mut jagged_builder = Jagged2Builder::new();
    for word in tokens_str {
        let word_bytes = word.as_bytes();
        jagged_builder.extend(word_bytes);
    }
    let tokens_jagged: Jagged2<u8> = jagged_builder.into();

    b.iter(|| {
        // Inner closure, the actual test
        sum_len_jagged_read_inner(&tokens_jagged)
    });
}

// Unique vocab
fn unique_vocab_str_read_inner(strings: &Vec<&str>) -> HashSet<String> {
    let mut vocab = HashSet::new();
    for e in strings.iter() {
        vocab.insert(e.to_string());
    }
    vocab
}

fn unique_vocab_string_read_inner(strings: &Vec<String>) -> HashSet<String> {
    let mut vocab = HashSet::new();
    for e in strings.iter() {
        vocab.insert(e.to_string());
    }
    vocab
}

fn unique_vocab_jagged_read_inner(strings: &Jagged2<u8>) -> HashSet<String> {
    let mut vocab = HashSet::new();
    for i in 0..strings.len() {
        let e: &str = unsafe { from_utf8_unchecked(strings.get_row(i).unwrap()) };
        vocab.insert(e.to_string());
    }
    vocab
}

#[bench]
fn unique_vocab_str_read(b: &mut Bencher) {
    let text = load_text();
    let tokens_str: Vec<&str> = text.split(" ").collect();

    b.iter(|| {
        // Inner closure, the actual test
        unique_vocab_str_read_inner(&tokens_str)
    });

}

#[bench]
fn unique_vocab_string_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();
    let tokens_string: Vec<String> =  tokens_str.iter().map(|x| x.to_string()).collect();

    b.iter(|| {
        // Inner closure, the actual test
        unique_vocab_string_read_inner(&tokens_string)
    });
}

#[bench]
fn unique_vocab_jagged_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();

    // Build jagged
    //let mut builder = Jagged2Builder::with_capacity(self.len(), self.flat_len());
    let mut jagged_builder = Jagged2Builder::new();
    for word in tokens_str {
        let word_bytes = word.as_bytes();
        jagged_builder.extend(word_bytes);
    }
    let tokens_jagged: Jagged2<u8> = jagged_builder.into();

    b.iter(|| {
        // Inner closure, the actual test
        unique_vocab_jagged_read_inner(&tokens_jagged)
    });
}

// Stemming
fn stemming_str_read_inner(strings: &Vec<&str>, stemmer: &Stemmer) {
    for e in strings.iter() {
        stemmer.stem(e).to_string();
    }
}

fn stemming_string_read_inner(strings: &Vec<String>, stemmer: &Stemmer) {
    for e in strings.iter() {
        stemmer.stem(e).to_string();
    }
}

fn stemming_jagged_read_inner(strings: &Jagged2<u8>, stemmer: &Stemmer) {
    for i in 0..strings.len() {
        let e: &str = unsafe { from_utf8_unchecked(strings.get_row(i).unwrap()) };
        stemmer.stem(e).to_string();
    }
}

#[bench]
fn stemming_str_read(b: &mut Bencher) {
    let text = load_text();
    let tokens_str: Vec<&str> = text.split(" ").collect();

    let stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);

    b.iter(|| {
        // Inner closure, the actual test
        stemming_str_read_inner(&tokens_str, &stemmer)
    });

}

#[bench]
fn stemming_string_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();
    let tokens_string: Vec<String> =  tokens_str.iter().map(|x| x.to_string()).collect();

    let stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);

    b.iter(|| {
        // Inner closure, the actual test
        stemming_string_read_inner(&tokens_string, &stemmer)
    });
}

#[bench]
fn stemming_jagged_read(b: &mut Bencher) {
    let text = load_text();

    // Tokenize
    let tokens_str: Vec<&str> = text.split(" ").collect();

    let stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);

    // Build jagged
    //let mut builder = Jagged2Builder::with_capacity(self.len(), self.flat_len());
    let mut jagged_builder = Jagged2Builder::new();
    for word in tokens_str {
        let word_bytes = word.as_bytes();
        jagged_builder.extend(word_bytes);
    }
    let tokens_jagged: Jagged2<u8> = jagged_builder.into();

    b.iter(|| {
        // Inner closure, the actual test
        stemming_jagged_read_inner(&tokens_jagged, &stemmer)
    });
}

// -- Write performance comparision between `Vec<&str>` and `Vec<String>`

// Write tokens
fn tokenize_str_write_inner(text: &str) -> Vec<&str> {
    text.split(" ").collect::<Vec<&str>>()
}

fn tokenize_string_write_inner(text: &str) -> Vec<String> {
    text.split(" ").map(|x| x.to_string()).collect::<Vec<String>>()
}

#[bench]
fn tokenize_str_write(b: &mut Bencher) {
    let text = load_text();

    b.iter(|| {
        // Inner closure, the actual test
        tokenize_str_write_inner(&text)
    });

}

#[bench]
fn tokenize_string_write(b: &mut Bencher) {
    let text = load_text();

    b.iter(|| {
        // Inner closure, the actual test
        tokenize_string_write_inner(&text)
    });
}


// Write unique tokens
fn unique_str_write_inner<'a>(tokens: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut vocab_set = HashSet::new();
    let mut vocab_vec: Vec<&str> = Vec::new();
    for token in tokens {
        if !vocab_set.contains(&token) {
            vocab_set.insert(token);
            vocab_vec.push(token);
        }
    }
    vocab_vec
}

fn unique_string_write_inner(tokens: & Vec<String>) -> Vec<String> {
    let mut vocab_set = HashSet::new();
    let mut vocab_vec: Vec<String> = Vec::new();
    for token in tokens {
        if !vocab_set.contains(token.as_str()) {
            vocab_set.insert(token.to_string());
            vocab_vec.push(token.to_string());
        }
    }
    vocab_vec
}

#[bench]
fn unique_str_write(b: &mut Bencher) {
    let text = load_text();
    let tokens = text.split(" ").collect::<Vec<&str>>();

    b.iter(|| {
        // Inner closure, the actual test
        unique_str_write_inner(&tokens)
    });

}

#[bench]
fn unique_string_write(b: &mut Bencher) {
    let text = load_text();
    let toekns = text.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();

    b.iter(|| {
        // Inner closure, the actual test
        unique_string_write_inner(&toekns)
    });
}