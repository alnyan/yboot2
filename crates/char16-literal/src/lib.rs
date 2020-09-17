use proc_macro::{
    TokenStream,
    TokenTree,
    Literal,
    Group,
    Delimiter,
    Punct,
    Spacing
};

fn convert(input: &[u8]) -> Vec<u16> {
    // TODO: support escaping
    let mut iter = input.iter();
    let first = *iter.next().unwrap();
    let ch = if first == b'r' {
        *iter.next().unwrap()
    } else {
        first
    };

    if ch != b'"' {
        panic!("Expected a string literal");
    }

    // TODO: check for non-ascii?
    let mut vec = Vec::<u16>::new();
    for b in iter {
        if *b == b'"' {
            break;
        }
        vec.push(*b as u16);
    }
    vec.push(0);

    vec
}

#[proc_macro]
pub fn cstr16(input: TokenStream) -> TokenStream {
    use std::iter::FromIterator;

    let s = input.into_iter().next().unwrap();
    match s {
        TokenTree::Literal(data) => {
            let text = data.to_string();
            let conv = convert(text.as_bytes());
            TokenStream::from_iter(vec![
                TokenTree::from(Punct::new('&', Spacing::Joint)),
                TokenTree::Group(Group::new(
                    Delimiter::Bracket,
                    TokenStream::from_iter(
                        conv.iter().map(|word| {
                            vec![
                                TokenTree::from(Literal::u16_suffixed(*word)),
                                TokenTree::from(Punct::new(',', Spacing::Joint)),
                            ]
                        }).flatten()
                    )
                ))
            ])
        },
        other         => panic!("Unexpected {:?}", other)
    }
}
