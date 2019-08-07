use std::iter::Peekable;

#[test]

fn peekable_test() {
    let mut chars = "carlos23Lande5".chars().peekable();

    assert_eq!(parse_string(&mut chars), "carlos");
    assert_eq!(chars.next(), Some('2'));
    assert_eq!(chars.next(), Some('3'));
    assert_eq!(parse_string(&mut chars), "Lande");
    assert_eq!(chars.next(), Some('5'));
}

#[allow(dead_code)]
fn parse_string<'a, I>(content: &'a mut Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut res = String::new();

    loop {
        match content.peek() {
            Some(v) if !(*v).is_numeric() => {
                &res.push(*v);
            }
            _ => return res,
        }
        content.next();
    }
}
