use frunk::{convert_from, from_generic, into_generic, Coproduct, Generic};
use frunk_core::hlist;

mod common;
use crate::common::*;

#[test]
fn test_struct_from_generic() {
    let h = hlist!("Humpty", "Drumpty", 3);
    let p: Person = from_generic(h);
    assert_eq!(
        p,
        Person {
            first_name: "Humpty",
            last_name: "Drumpty",
            age: 3,
        }
    );
}

#[test]
fn test_tuple_struct_from_generic() {
    let h = hlist!("Drumpty", 3);
    let p: TupleStruct = from_generic(h);
    assert_eq!(p, TupleStruct("Drumpty", 3));
}

#[test]
fn test_enum_from_generic() {
    let variant_a = Coproduct::inject(hlist![]);
    let variant_b = Coproduct::inject(hlist![42i32]);
    let variant_c = Coproduct::inject(hlist!["test".to_string(), true]);

    assert_eq!(
        from_generic::<GenericEnum, _>(variant_a),
        GenericEnum::VariantA,
    );
    assert_eq!(
        from_generic::<GenericEnum, _>(variant_b),
        GenericEnum::VariantB(42),
    );
    assert_eq!(
        from_generic::<GenericEnum, _>(variant_c),
        GenericEnum::VariantC {
            x: "test".into(),
            y: true,
        },
    );
}

#[test]
fn test_struct_into_generic() {
    let p = Person {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_generic(p);
    assert_eq!(h, hlist!("Humpty", "Drumpty", 3));
}

#[test]
fn test_enum_into_generic() {
    let variant_a = into_generic(GenericEnum::VariantA);
    let variant_b = into_generic(GenericEnum::VariantB(42));
    let variant_c = into_generic(GenericEnum::VariantC {
        x: "test".into(),
        y: true,
    });

    assert_eq!(variant_a, Coproduct::inject(hlist![]));
    assert_eq!(variant_b, Coproduct::inject(hlist![42i32]));
    assert_eq!(
        variant_c,
        Coproduct::inject(hlist!["test".to_string(), true])
    );
}

#[test]
fn test_option_generic() {
    let none_repr = into_generic(None::<i32>);
    let some_repr = into_generic(Some(42i32));

    assert_eq!(none_repr, Coproduct::Inl(hlist![]));
    assert_eq!(some_repr, Coproduct::Inr(Coproduct::Inl(hlist![42i32])));
    assert_eq!(from_generic::<Option<i32>, _>(none_repr), None);
    assert_eq!(from_generic::<Option<i32>, _>(some_repr), Some(42));
}

#[test]
fn test_result_generic() {
    let ok_repr = into_generic(Ok::<i32, &str>(42));
    let err_repr = into_generic(Err::<i32, &str>("error"));

    assert_eq!(ok_repr, Coproduct::Inl(hlist![42i32]));
    assert_eq!(err_repr, Coproduct::Inr(Coproduct::Inl(hlist!["error"])));
    assert_eq!(from_generic::<Result<i32, &str>, _>(ok_repr), Ok(42));
    assert_eq!(from_generic::<Result<i32, &str>, _>(err_repr), Err("error"));
}

#[test]
fn test_bool_generic() {
    type BoolRepr = <bool as Generic>::Repr;

    let false_repr: BoolRepr = into_generic(false);
    let true_repr: BoolRepr = into_generic(true);
    let expected_false: BoolRepr = Coproduct::Inl(hlist![]);
    let expected_true: BoolRepr = Coproduct::Inr(Coproduct::Inl(hlist![]));

    assert_eq!(false_repr, expected_false);
    assert_eq!(true_repr, expected_true);
    assert!(!from_generic::<bool, _>(expected_false));
    assert!(from_generic::<bool, _>(expected_true));
}

#[test]
fn test_struct_conversion() {
    let a = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let pres: President = frunk::convert_from(a);
    assert_eq!(
        pres,
        President {
            first_name: "Steve",
            last_name: "Cannon",
            age: 3,
        }
    )
}

#[test]
fn test_struct_conversion_round_trip() {
    let a = Strategist {
        first_name: "Steve",
        last_name: "Cannon",
        age: 3,
    };
    let before = a.clone();
    let p: President = convert_from(a);
    let a_again: Strategist = convert_from(p);
    assert_eq!(a_again, before)
}

#[test]
fn test_mixed_conversions_round_trip() {
    // Both SavedUser and ApiUser derive both Generic and LabelledGeneric
    //
    // Because their field names are different, their LabelledGeneric representations
    // differ, so we can't use the LabelledGeneric typeclass to convert to and fro.
    // Instead, we'll use the Generic typeclass to get the job done.
    let u = SavedUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let before = u.clone();
    let au: ApiUser = convert_from(u);
    // let au2 = <ApiUser as LabelledGeneric>::convert_from(u); <-- will fail at compile time
    let u_again: SavedUser = convert_from(au);
    assert_eq!(u_again, before)
}
