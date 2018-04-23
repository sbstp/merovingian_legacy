pub fn join<I, G, P>(pieces: I, glue: G) -> String
where
    I: IntoIterator<Item = P>,
    P: AsRef<str>,
    G: AsRef<str>,
{
    let mut buff = String::new();
    let mut pieces = pieces.into_iter();
    let glue = glue.as_ref();
    if let Some(first) = pieces.next() {
        buff.push_str(first.as_ref());
        for piece in pieces {
            buff.push_str(glue);
            buff.push_str(piece.as_ref());
        }
    }
    buff
}

#[test]
fn test_join() {
    assert_eq!(
        join(&["abc", "def", "jih"], "--"),
        "abc--def--jih".to_string()
    );
}

#[test]
fn test_join_empty() {
    assert_eq!(join(&[""], "-"), "".to_string());
}
