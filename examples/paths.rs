use frunk::LabelledGeneric;
use frunk_core::path::PathTraverser;
use frunk_proc_macros::{path, Path};

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

    // Prints height as long as `A` has the right "shape" (e.g.
    // has `dimensions.height: usize` and `dimension.unit: SizeUnit)
    fn print_height<'a, A, HeightIdx, UnitIdx>(obj: &'a A)
    where
        &'a A: PathTraverser<Path!(dimensions.height), HeightIdx, TargetValue = &'a usize>
            + PathTraverser<Path!(dimensions.unit), UnitIdx, TargetValue = &'a SizeUnit>,
    {
        println!(
            "Height [{} {:?}]",
            path!(dimensions.height).get(obj),
            path!(dimensions.unit).get(obj)
        );
    }

    print_height(&dog);
    print_height(&cat);
}
