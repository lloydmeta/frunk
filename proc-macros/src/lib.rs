//! Module that holds procedural macros for Frunk
//!
//!
//! # Examples
//!
//! ```
//! #[macro_use] extern crate frunk;
//! #[macro_use] extern crate frunk_core;
//! # extern crate frunk_proc_macros;
//! # use frunk_proc_macros::path;
//! # fn main() {
//! #[derive(LabelledGeneric)]
//! struct Dog<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Cat<'a> {
//!     name: &'a str,
//!     dimensions: Dimensions,
//! }
//!
//! #[derive(LabelledGeneric)]
//! struct Dimensions {
//!     height: usize,
//!     width: usize,
//!     unit: SizeUnit,
//! }
//!
//! #[derive(Debug, Eq, PartialEq)]
//! enum SizeUnit {
//!     Cm,
//!     Inch,
//! }
//!
//! let mut dog = Dog {
//!     name: "Joe",
//!     dimensions: Dimensions {
//!         height: 10,
//!         width: 5,
//!         unit: SizeUnit::Inch,
//!     },
//! };
//!
//! let cat = Cat {
//!     name: "Schmoe",
//!     dimensions: Dimensions {
//!         height: 7,
//!         width: 3,
//!         unit: SizeUnit::Cm,
//!     },
//! };
//!
//! // generic, re-usable paths
//! let height_lens = path!(dimensions.height);
//! let unit_lens = path!(dimensions.unit);
//!
//! assert_eq!(*height_lens.get(&dog), 10);
//! assert_eq!(*height_lens.get(&cat), 7);
//! assert_eq!(*unit_lens.get(&dog), SizeUnit::Inch);
//! assert_eq!(*unit_lens.get(&cat), SizeUnit::Cm);
//!
//! // modify
//! *height_lens.get(&mut dog) = 13;
//! assert_eq!(*height_lens.get(&dog), 13);
//! # }
//! ```

extern crate frunk_core;
extern crate frunk_proc_macros_impl;
extern crate proc_macro_hack;

use proc_macro_hack::proc_macro_hack;

/// Add one to an expression.
#[proc_macro_hack]
pub use frunk_proc_macros_impl::path;

#[cfg(test)]
#[macro_use]
extern crate frunk;

#[cfg(test)]
mod tests {

    #[test]
    fn test_path() {
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

        #[derive(Debug, Eq, PartialEq)]
        enum SizeUnit {
            Cm,
            Inch,
        }

        let mut dog = Dog {
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

        assert_eq!(*height_lens.get(&dog), 10);
        assert_eq!(*height_lens.get(&cat), 7);
        assert_eq!(*unit_lens.get(&dog), SizeUnit::Inch);
        assert_eq!(*unit_lens.get(&cat), SizeUnit::Cm);

        // modify
        *height_lens.get(&mut dog) = 13;
        assert_eq!(*height_lens.get(&dog), 13);
    }
}
