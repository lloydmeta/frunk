use core::marker::PhantomData;

use crate::{
    hlist::{HFoldRightable, HList},
    traits::{Func, Poly},
};

pub struct ApplyN<F>(PhantomData<F>);
type ApplyNAcc<R> = (Option<R>, usize, usize); // targetN, currN, R

impl<F, S> Func<(ApplyNAcc<<F as Func<S>>::Output>, S)> for ApplyN<F>
where
    F: Func<S>,
{
    type Output = ApplyNAcc<<F as Func<S>>::Output>;

    fn call(((r, tgt_n, curr_n), s): (ApplyNAcc<<F as Func<S>>::Output>, S)) -> Self::Output {
        if tgt_n == curr_n {
            (Some(F::call(s)), tgt_n, curr_n + 1)
        } else {
            (r, tgt_n, curr_n + 1)
        }
    }
}

pub fn applyr_at<F, R, T>(hl: T, n: usize, _: F) -> Option<R>
where
    T: HFoldRightable<Poly<ApplyN<F>>, ApplyNAcc<R>, Output = ApplyNAcc<R>> + HList,
{
    let len = hl.len();
    if n >= len {
        return None;
    }
    let (r, _, _) = hl.foldr(Poly(ApplyN::<F>(PhantomData)), (None, n, 0));
    r
}

pub fn applyl_at<F, R, T>(hl: T, n: usize, f: F) -> Option<R>
where
    T: HFoldRightable<Poly<ApplyN<F>>, ApplyNAcc<R>, Output = ApplyNAcc<R>> + HList,
{
    let len = hl.len();
    if n >= len {
        return None;
    }
    applyr_at(hl, len - n - 1, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct IsDefault;
    impl<T> Func<T> for IsDefault
    where
        T: Default + PartialEq,
    {
        type Output = bool;

        fn call(i: T) -> Self::Output {
            let r = T::default() == i;
            r
        }
    }

    #[test]
    fn test_apply() {
        let lst = hlist![10u32, "xyz", 0usize];

        assert_eq!(applyr_at(lst.clone(), 0, IsDefault).unwrap(), true);
        assert_eq!(applyr_at(lst.clone(), 1, IsDefault).unwrap(), false);
        assert_eq!(applyr_at(lst.clone(), 2, IsDefault).unwrap(), false);

        // Couldn't figure out how to create Func impl for references :(
        // assert_eq!(applyr_at(lst.clone().to_ref(), 1, IsDefault).unwrap(), false);
        // assert_eq!(applyr_at(lst.clone().to_mut(), 2, IsDefault).unwrap(), false);

        assert_eq!(applyl_at(lst.clone(), 0, IsDefault).unwrap(), false);
        assert_eq!(applyl_at(lst.clone(), 1, IsDefault).unwrap(), false);
        assert_eq!(applyl_at(lst.clone(), 2, IsDefault).unwrap(), true);
    }
}
