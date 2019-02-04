extern crate frunk;
#[macro_use] // for the hlist macro
extern crate frunk_core;
extern crate time; //Time library

use frunk::hlist::Sculptor;
use frunk::labelled::chars::*;
use frunk::labelled::Field;
use frunk::labelled::Transmogrifier;
use frunk::{from_labelled_generic, into_labelled_generic, transform_from};
use frunk::{HCons, LabelledGeneric};

mod common;

use common::*;

use self::time::*;

#[test]
fn test_struct_from_labelled_generic() {
    let h = hlist![
        field!((f, i, r, s, t, __, n, a, m, e), "Humpty"),
        field!((l, a, s, t, __, n, a, m, e), "Drumpty"),
        field!((a, g, e), 3),
    ];
    let u: NewUser = from_labelled_generic(h);
    assert_eq!(
        u,
        NewUser {
            first_name: "Humpty",
            last_name: "Drumpty",
            age: 3,
        }
    );
}

#[test]
fn test_labelled_generic_names() {
    let u = NewUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_labelled_generic(u);
    let l_name_field: &Field<(l, a, s, t, __, n, a, m, e), _> = h.get();
    assert_eq!(l_name_field.name, "last_name");
    let f_name_field: &Field<(f, i, r, s, t, __, n, a, m, e), _> = h.get();
    assert_eq!(f_name_field.name, "first_name")
}

#[test]
fn test_struct_into_labelled_generic() {
    let u = NewUser {
        first_name: "Humpty",
        last_name: "Drumpty",
        age: 3,
    };
    let h = into_labelled_generic(u);
    assert_eq!(
        h,
        hlist![
            field!((f, i, r, s, t, __, n, a, m, e), "Humpty", "first_name"),
            field!((l, a, s, t, __, n, a, m, e), "Drumpty", "last_name"),
            field!((a, g, e), 3),
        ]
    );
}

#[test]
fn test_reshaped_labelled_generic_conversion() {
    let n_u = NormalUser {
        first_name: "Moe",
        last_name: "Ali",
        age: 30,
    };
    // Convert to labelled-generic representation
    let n_gen = into_labelled_generic(n_u);
    // Reshape the labelled generic to fit the JumbledUser's generic Repr
    let (jumbled_gen, _): (<JumbledUser as LabelledGeneric>::Repr, _) = n_gen.sculpt();
    let j_u: JumbledUser = from_labelled_generic(jumbled_gen); // Done

    assert_eq!(j_u.first_name, "Moe");
    assert_eq!(j_u.last_name, "Ali");
    assert_eq!(j_u.age, 30)
}

#[test]
fn test_aligned_labelled_convert_from() {
    let n_u = NormalUser {
        first_name: "Moe",
        last_name: "Ali",
        age: 30,
    };
    // even less boilerplate than before
    let j_u: JumbledUser = transform_from(n_u); // Done

    assert_eq!(j_u.first_name, "Moe");
    assert_eq!(j_u.last_name, "Ali");
    assert_eq!(j_u.age, 30)
}

#[test]
fn test_transmogrify() {
    let internal_user = InternalUser {
        name: "John",
        age: 10,
        address: InternalAddress {
            is_whitelisted: true,
            name: "somewhere out there",
            phone: InternalPhoneNumber {
                main: 1234,
                secondary: None,
                emergency: Some(5678),
            },
        },
        is_banned: true,
    };
    let expected_external_user = ExternalUser {
        name: "John",
        age: 10,
        address: ExternalAddress {
            name: "somewhere out there",
            phone: ExternalPhoneNumber { main: 1234 },
        },
    };
    let external_user: ExternalUser = internal_user.transmogrify();
    assert_eq!(external_user, expected_external_user);
}

type CreatedAt = (c, r, e, a, t, e, d, __, a, t);

/// Converts from the Input type to the Output type,
/// provided that the Output type has a compatible labelled representation
/// with Input *AND* has a created_at Time field
///
/// If we wanted to, we could even make the time field type a parameter too for
/// even more generalisation.
///
/// Type parameters:
///
/// I stands for Input
/// O stands for Output
/// Indices is for the indices used for sculpting I with created_at Field into O's generic representation
fn to_audited<I, O, Indices>(o: I) -> O
where
    I: LabelledGeneric,
    O: LabelledGeneric,
    HCons<Field<CreatedAt, Tm>, <I as LabelledGeneric>::Repr>:
        Sculptor<<O as LabelledGeneric>::Repr, Indices>,
{
    // Add created_at field to LabelledGeneric repr of I
    let i_with_time = HCons {
        head: field!(CreatedAt, time::now()),
        tail: into_labelled_generic(o),
    };
    // sculpt it to fit Output LabelledGeneric representation
    let (compatible_with_o, _): (<O as LabelledGeneric>::Repr, _) = i_with_time.sculpt();
    // convert from LabelledGeneric to Output
    from_labelled_generic(compatible_with_o)
}

#[test]
fn test_generalised_auditing() {
    let now = time::now().tm_nsec;
    // Need to help the compiler out by annotating, but no biggie
    let n_u_audited: NormalUserWithAudit = to_audited(NormalUser::build());

    // We can even go from NormalUser to JumbledUser since they have compatible LabelledGeneric::Reprs
    let j_u_audited: JumbledUserWithAudit = to_audited(NormalUser::build());
    assert!(n_u_audited.created_at.tm_nsec >= now);
    assert!(j_u_audited.created_at.tm_nsec >= now);
}

#[test]
fn test_conversion_between_newtypes() {
    let s = "Foo".to_string();
    let nt = TypeWrapper(s.clone());
    let nt2: TypeWrapper2 = nt.transmogrify();
    assert_eq!(nt2.0, s);
}

#[test]
fn test_transmogrify_tuples() {
    let vec4 = Vec4f(1.0, 2.0, 0.0, 3.0);
    let vec3 = vec4.transmogrify();
    assert_eq!(Vec3f(1.0, 2.0, 0.0), vec3);
}
