#![cfg(test)]

#[test]
fn run() {
    let a = vec![1, 2, 3];
    let r = &a[..];
    assert_eq!(&a, r);
}
