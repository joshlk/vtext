use crate::ngram_utils::*;

#[test]
fn test_padding() {
    let sent = "Mary had a little lamb".split(" ");

    let output: Vec<&str> =
        pad_items(Box::new(sent.clone()), 3, Some("<s>"), Some("</s>")).collect();
    let expected = vec![
        "<s>", "<s>", "Mary", "had", "a", "little", "lamb", "</s>", "</s>",
    ];
    assert_eq!(output, expected);

    let output: Vec<&str> = pad_items(Box::new(sent.clone()), 2, Some("<s>"), None).collect();
    let expected = vec!["<s>", "Mary", "had", "a", "little", "lamb"];
    assert_eq!(output, expected);

    let output: Vec<&str> = pad_items(Box::new(sent.clone()), 2, None, Some("</s>")).collect();
    let expected = vec!["Mary", "had", "a", "little", "lamb", "</s>"];
    assert_eq!(output, expected);
}

#[test]
fn test_bigram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = bigram(Box::new(sent), None, None).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary", "had"],
        vec!["had", "a"],
        vec!["a", "little"],
        vec!["little", "lamb"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_trigram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = ngrams(Box::new(sent.clone()), 3, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);

    let output_iter = ngrams(Box::new(sent.clone()), 3, None, Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary", "had", "a"],
        vec!["had", "a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_ngrams() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = ngrams(Box::new(sent), 4, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "<s>", "Mary"],
        vec!["<s>", "<s>", "Mary", "had"],
        vec!["<s>", "Mary", "had", "a"],
        vec!["Mary", "had", "a", "little"],
        vec!["had", "a", "little", "lamb"],
        vec!["a", "little", "lamb", "</s>"],
        vec!["little", "lamb", "</s>", "</s>"],
        vec!["lamb", "</s>", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_everygram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = everygrams(Box::new(sent), 1, 3, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "Mary"],
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "Mary", "had"],
        vec!["Mary"],
        vec!["Mary", "had"],
        vec!["Mary", "had", "a"],
        vec!["had"],
        vec!["had", "a"],
        vec!["had", "a", "little"],
        vec!["a"],
        vec!["a", "little"],
        vec!["a", "little", "lamb"],
        vec!["little"],
        vec!["little", "lamb"],
        vec!["lamb"],
        vec!["lamb", "</s>"],
        vec!["little", "lamb", "</s>"],
        vec!["lamb", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_skipgram() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = skipgrams(Box::new(sent.clone()), 2, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "Mary"],
        vec!["<s>", "had"],
        vec!["Mary", "had"],
        vec!["Mary", "a"],
        vec!["had", "a"],
        vec!["had", "little"],
        vec!["a", "little"],
        vec!["a", "lamb"],
        vec!["little", "lamb"],
        vec!["lamb", "</s>"],
        vec!["little", "</s>"],
    ];

    assert_eq!(output, expected);

    let output_iter = skipgrams(Box::new(sent.clone()), 3, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["<s>", "<s>", "Mary"],
        vec!["<s>", "<s>", "had"],
        vec!["<s>", "Mary", "had"],
        vec!["<s>", "Mary", "a"],
        vec!["<s>", "had", "a"],
        vec!["Mary", "had", "a"],
        vec!["Mary", "had", "little"],
        vec!["Mary", "a", "little"],
        vec!["had", "a", "little"],
        vec!["had", "a", "lamb"],
        vec!["had", "little", "lamb"],
        vec!["a", "little", "lamb"],
        vec!["little", "lamb", "</s>"],
        vec!["a", "lamb", "</s>"],
        vec!["a", "little", "</s>"],
        vec!["lamb", "</s>", "</s>"],
        vec!["little", "</s>", "</s>"],
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_ngram_edge_cases() {
    let sent = "Mary had a little lamb".split(" ");

    let output_iter = build_k_skip_n_grams_iter(
        Box::new(sent.clone()), 1, 1, 1, 1, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    let expected = vec![
        vec!["Mary"],
        vec!["had"],
        vec!["a"],
        vec!["little"],
        vec!["lamb"],
    ];

    assert_eq!(output, expected);

    let output_iter = build_k_skip_n_grams_iter(
        Box::new(sent.clone()), 1, 1, 2, 2, Some("<s>"), Some("</s>")).unwrap();
    let output: Vec<Vec<&str>> = output_iter.collect();

    assert_eq!(output, expected);
}

#[test]
fn test_skip_vec_iter() {

    let output: Vec<Vec<usize>> = SkipVecIter::new(3, 2).collect();

    let expected = vec![
        vec![0, 0, 0],
        vec![0, 0, 1],
        vec![0, 0, 2],
        vec![0, 1, 0],
        vec![0, 1, 1],
        vec![0, 2, 0],
        vec![1, 0, 0],
        vec![1, 0, 1],
        vec![1, 1, 0],
        vec![2, 0, 0],
    ];
    assert_eq!(output, expected);

}