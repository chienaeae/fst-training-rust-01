#[cfg_attr(miri, ignore)]
#[test]
fn it_works() {
    let a = 2;
    let b = 2;
    assert_eq!(a + b, 4);
}
