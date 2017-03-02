use std::marker::PhantomData;

// Deriving for all BitAnds sucks because we are then bound on ::Output, which may not be the same type
macro_rules! simple_impls_for {
    ($($i: ident)*) => {
        $(
            #[allow(non_snake_case, dead_code, non_camel_case_types)]
            enum $i {}
        )*
    }
}

// Add more as needed
simple_impls_for! { a b c d e f g h i j k m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z __ _1 _2 _3 _4 _5 _6 _7 _8 _9 _0 }

pub struct Field<A, N> {
    pub value: A,
    type_name: PhantomData<N>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_construction() {
        let f: Field<i32, (a,g,e)> = Field { value: 3, type_name: PhantomData };
        assert_eq!(f.value, 3)
    }
}