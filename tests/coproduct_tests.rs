use frunk_core::{hlist, Coprod};

#[test]
fn test_inject_coproduct() {
    type I32StrBool = Coprod!(i32, &'static str, bool);

    let co1 = I32StrBool::inject(3);
    let get_from_1a: Option<&i32> = co1.get();
    let get_from_1b: Option<&bool> = co1.get();
    assert_eq!(get_from_1a, Some(&3));
    assert_eq!(get_from_1b, None);
}

#[test]
fn test_coproduct_fold_consuming() {
    type I32StrBool = Coprod!(i32, f32, bool);

    let co1 = I32StrBool::inject(3);
    let folded = co1.fold(hlist![
        |i| format!("int {}", i),
        |f| format!("float {}", f),
        |b| (if b { "t" } else { "f" }).to_string(),
    ]);

    assert_eq!(folded, "int 3".to_string());
}

#[test]
fn test_coproduct_fold_non_consuming() {
    type I32StrBool = Coprod!(i32, f32, bool);

    let co = I32StrBool::inject(true);

    assert_eq!(
        co.to_ref().fold(hlist![
            |&i| format!("int {}", i),
            |&f| format!("float {}", f),
            |&b| (if b { "t" } else { "f" }).to_string(),
        ]),
        "t".to_string()
    );
}
