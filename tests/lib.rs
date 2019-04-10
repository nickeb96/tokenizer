
extern crate tokenizer;


#[test]
fn test_iter_tokens() {
    let source = r###"
    int main(int argc, char** argv)
    {
        while (argc>=1) printf("%s\n", argv[+--argc]);
        return 0;
    }
    "###;
    let expected = vec![
        "int",
        "main",
        "(",
        "int",
        "argc",
        ",",
        "char",
        "*",
        "*",
        "argv",
        ")",
        "{",
        "while",
        "(",
        "argc",
        ">=",
        "1",
        ")",
        "printf",
        "(",
        r#""%s\n""#,
        ",",
        "argv",
        "[",
        "+",
        "--",
        "argc",
        "]",
        ")",
        ";",
        "return",
        "0",
        ";",
        "}",
    ];

    let observed: Vec<(usize, usize)> = tokenizer::iter_tokens(source).collect();

    for (obs, exp) in observed.iter().zip(expected.iter()) {
        assert_eq!(&source[obs.0..obs.1], *exp);
    }
}


#[test]
fn test_drop() {
    let mut source = String::from(r###"
    int main(int argc, char** argv)
    {
        while (argc>=1) printf("%s\n", argv[+--argc]);
        return 0;
    }
    "###);

    let more_source = r###"
    int f(int n)
    {
        return n + 1;
    }
    "###;

    let mut iter: tokenizer::TokenIterator;

    iter = tokenizer::iter_tokens(&source);

    let dummy = String::new();
    iter.replace_text(&dummy);

    source.push_str(more_source);
}

