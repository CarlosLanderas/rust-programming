use std::iter::repeat;

#[test]
fn test_zip_with_repeat() {
    let oh = vec!["pam", "pum", "plom"];

    let zipped: Vec<_> = repeat("pim").zip(oh).collect();

    assert_eq!(
        zipped,
        vec![("pim", "pam"), ("pim", "pum"), ("pim", "plom")]
    );
}

#[test]
fn test_zip_with_map() {
    let oh = vec!["pam", "pum", "plom"];

    let zipped: Vec<_> = repeat("pim")
        .zip(oh)
        .map(|x| {
            let mut s = String::from(x.0);
            s.push_str(" ");
            s.push_str(x.1);
            s
        })
        .collect();

    assert_eq!(zipped, vec!["pim pam", "pim pum", "pim plom"]);
}
