use std::sync::Mutex;

struct IntHolder(i32);

fn plus_one(wrapped: &Mutex<IntHolder>) -> Option<i32> {
    match wrapped.lock() {
        Ok(ref data) => Some(data.0),
        _ => None,
    }.map(|data| data + 1)
}

fn main() {
    let holder_mutx = Mutex::new(IntHolder(420));
    let insides_incremented = plus_one(&holder_mutx);
    println!("{:?}", insides_incremented)
}
