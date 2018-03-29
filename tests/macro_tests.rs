// Makes sure that the hlist macros in frunk_core are reexported by frunk

#[macro_use]
extern crate frunk;

// NOTE: Deliberately no "extern crate frunk_core;"

#[test]
fn use_frunk_macros() {
    let h1 = hlist![1i32, 2u32];
    let h2 = hlist!["cool", ...h1];
    let hlist_pat![a, ...bs]: Hlist![&'static str, i32, ...Hlist![u32]] = h2;
    assert_eq!(a, "cool");
    assert_eq!(bs, h1);
}
