#[macro_use]
extern crate frunk; // for derives
extern crate frunk_core; // for labelledgeneric
extern crate frunk_proc_macros;

use frunk_proc_macros::path;

#[derive(LabelledGeneric)]
struct Dog<'a> {
    name: &'a str,
    dimensions: Dimensions,
}

#[derive(LabelledGeneric)]
struct Cat<'a> {
    name: &'a str,
    dimensions: Dimensions,
}

#[derive(LabelledGeneric)]
struct Dimensions {
    height: usize,
    width: usize,
    unit: SizeUnit,
}

#[derive(Debug)]
enum SizeUnit {
    Cm,
    Inch,
}

fn main() {
    let dog = Dog {
        name: "Joe",
        dimensions: Dimensions {
            height: 10,
            width: 5,
            unit: SizeUnit::Inch,
        },
    };

    let cat = Cat {
        name: "Schmoe",
        dimensions: Dimensions {
            height: 7,
            width: 3,
            unit: SizeUnit::Cm,
        },
    };

    // generic, re-usable paths
    let height_lens = path!(dimensions.height);
    let unit_lens = path!(dimensions.unit);

    println!(
        "Dog height: [{} {:?}]",
        height_lens.get(&dog),
        unit_lens.get(&dog)
    );
    println!(
        "Cat height: [{} {:?}]",
        height_lens.get(&cat),
        unit_lens.get(&cat)
    );
}
