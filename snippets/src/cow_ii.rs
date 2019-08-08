use std::borrow::Cow;

fn get_name() -> Cow<'static, str> {
    std::env::var("YOUR_NAME")
        //.map(|v| Cow::Owned(v))
        .map(|v| v.into())
        //.unwrap_or(Cow::Borrowed("whoever you are"))
        .unwrap_or("whoever you are".into())
}

#[test]
fn cow_owned_test() {
    std::env::set_var("YOUR_NAME", "Lande");

    let r = get_name();
    assert_eq!(r, "Lande");
}

#[test]
fn cow_borrow_test() {
    std::env::remove_var("YOUR_NAME");

    let r = get_name();
    assert_eq!(r, "whoever you are");
}
