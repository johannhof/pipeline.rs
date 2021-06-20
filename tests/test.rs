#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]

use pipeline::pipe;

#[test]
fn trivial() {
    let ret = pipe!{
        4 * 4
    };
    assert_eq!(ret, 16);
}

#[test]
fn simple() {
    let double = |x: u8| x*2;
    let ret = pipe! {
        6u8 => double
    };
    assert_eq!(ret, 12);
}

#[test]
fn partial() {
    let ret = pipe! {
        Some(4) => _.is_some()
    };
    assert_eq!(ret, true);
}

#[test]
fn lambda() {
    let ret = pipe! {
        Some(4)
        => _.map(|_| 0)
        => _.unwrap()
    };
    assert_eq!(ret, 0);
}
