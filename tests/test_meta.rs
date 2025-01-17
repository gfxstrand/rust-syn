#![allow(clippy::shadow_unrelated, clippy::too_many_lines)]

#[macro_use]
mod macros;

use syn::{Meta, MetaList, MetaNameValue};

#[test]
fn test_parse_meta_item_word() {
    let input = "hello";

    snapshot!(input as Meta, @r###"
    Path(Path {
        segments: [
            PathSegment {
                ident: "hello",
                arguments: None,
            },
        ],
    })
    "###);
}

#[test]
fn test_parse_meta_name_value() {
    let input = "foo = 5";
    let (inner, meta) = (input, input);

    snapshot!(inner as MetaNameValue, @r###"
    MetaNameValue {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        value: Expr::Lit {
            lit: 5,
        },
    }
    "###);

    snapshot!(meta as Meta, @r###"
    Meta::NameValue {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        value: Expr::Lit {
            lit: 5,
        },
    }
    "###);

    assert_eq!(meta, inner.into());
}

#[test]
fn test_parse_meta_item_list_lit() {
    let input = "foo(5)";
    let (inner, meta) = (input, input);

    snapshot!(inner as MetaList, @r###"
    MetaList {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`5`),
    }
    "###);

    snapshot!(meta as Meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`5`),
    }
    "###);

    assert_eq!(meta, inner.into());
}

#[test]
fn test_parse_meta_item_multiple() {
    let input = "foo(word, name = 5, list(name2 = 6), word2)";
    let (inner, meta) = (input, input);

    snapshot!(inner as MetaList, @r###"
    MetaList {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`word , name = 5 , list (name2 = 6) , word2`),
    }
    "###);

    snapshot!(meta as Meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`word , name = 5 , list (name2 = 6) , word2`),
    }
    "###);

    assert_eq!(meta, inner.into());
}

#[test]
fn test_parse_path() {
    let input = "::serde::Serialize";
    snapshot!(input as Meta, @r###"
    Path(Path {
        leading_colon: Some,
        segments: [
            PathSegment {
                ident: "serde",
                arguments: None,
            },
            PathSegment {
                ident: "Serialize",
                arguments: None,
            },
        ],
    })
    "###);
}
