extern crate time;

use self::time::*;
#[allow(unused_imports)]
use frunk::*;

#[derive(Generic, Debug, PartialEq)]
pub struct Person<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(Generic, Debug, PartialEq, Clone)]
pub struct Strategist<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(Generic, Debug, PartialEq)]
pub struct President<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(LabelledGeneric, Debug, PartialEq, Clone)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[derive(LabelledGeneric, Generic, Debug, PartialEq, Clone)]
pub struct SavedUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

#[allow(non_snake_case)]
#[derive(LabelledGeneric, Generic, Debug, PartialEq)]
pub struct ApiUser<'a> {
    pub FirstName: &'a str,
    pub LastName: &'a str,
    pub Age: usize,
}

#[allow(non_snake_case)]
#[derive(LabelledGeneric, Debug, PartialEq)]
pub struct SuperLongField {
    pub abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789: i32,
}

#[derive(Generic, Debug, PartialEq)]
pub struct TupleStruct<'a>(pub &'a str, pub i32);

#[derive(LabelledGeneric)]
pub struct NormalUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
}

impl<'a> NormalUser<'a> {
    /// Helper function for building a NormalUser
    #[allow(dead_code)]
    pub fn build() -> NormalUser<'a> {
        NormalUser {
            first_name: "Moe",
            last_name: "Ali",
            age: 30,
        }
    }
}

// Fields are jumbled :(
#[derive(LabelledGeneric)]
pub struct JumbledUser<'a> {
    pub last_name: &'a str,
    pub age: usize,
    pub first_name: &'a str,
}

#[derive(LabelledGeneric)]
pub struct NormalUserWithAudit<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
    pub created_at: Tm,
}

#[derive(LabelledGeneric)]
pub struct JumbledUserWithAudit<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: usize,
    pub created_at: Tm,
}

// Nested + Jumbled

#[derive(LabelledGeneric)]
pub struct InternalPhoneNumber {
    pub emergency: Option<usize>,
    pub main: usize,
    pub secondary: Option<usize>,
}

#[derive(LabelledGeneric)]
pub struct InternalAddress<'a> {
    pub is_whitelisted: bool,
    pub name: &'a str,
    pub phone: InternalPhoneNumber,
}

#[derive(LabelledGeneric)]
pub struct InternalUser<'a> {
    pub name: &'a str,
    pub age: usize,
    pub address: InternalAddress<'a>,
    pub is_banned: bool,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct ExternalPhoneNumber {
    pub main: usize,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct ExternalAddress<'a> {
    pub name: &'a str,
    pub phone: ExternalPhoneNumber,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct ExternalUser<'a> {
    pub age: usize,
    pub address: ExternalAddress<'a>,
    pub name: &'a str,
}

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct TypeWrapper(pub String);

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct TypeWrapper2(pub String);

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct Vec4f(pub f32, pub f32, pub f32, pub f32);

#[derive(LabelledGeneric, PartialEq, Debug)]
pub struct Vec3f(pub f32, pub f32, pub f32);
