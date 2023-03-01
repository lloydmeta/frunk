use core::marker::PhantomData;

use crate::{
    hlist::{HFoldRightable, HList},
    traits::{Func, Poly},
};

pub struct ApplyN<F>(PhantomData<F>);

type ApplyNAcc<R> = (Option<R>, usize, usize); // R, targetN, currN
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

type ApplyNArgsAcc<R,FA> = (Option<R>, FA, usize, usize); // R, FArgs, targetN, currN
impl<F, FA, S> Func<(ApplyNArgsAcc<<F as Func<(FA,S)>>::Output,FA>, S)> for ApplyN<F>
where
    F: Func<(FA,S)>,
    FA: Copy
{
    type Output = ApplyNArgsAcc<<F as Func<(FA,S)>>::Output, FA>;

    fn call(((r, args, tgt_n, curr_n), s): (ApplyNArgsAcc<<F as Func<(FA,S)>>::Output,FA>, S)) -> Self::Output {
        if tgt_n == curr_n {
            (Some(F::call((args, s))), args, tgt_n, curr_n + 1)
        } else {
            (r, args, tgt_n, curr_n + 1)
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

pub fn applyr_args_at<F, R, T, FA>(hl: T, n: usize, _: F, args: FA) -> Option<R>
where
    T: HFoldRightable<Poly<ApplyN<F>>, ApplyNArgsAcc<R, FA>, Output = ApplyNArgsAcc<R, FA>> + HList,
{
    let len = hl.len();
    if n >= len {
        return None;
    }
    let (r,_ , _, _) = hl.foldr(Poly(ApplyN::<F>(PhantomData)), (None, args, n, 0));
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

pub fn applyl_args_at<F, R, T, FA>(hl: T, n: usize, f: F, args: FA) -> Option<R>
where
    T: HFoldRightable<Poly<ApplyN<F>>, ApplyNArgsAcc<R, FA>, Output = ApplyNArgsAcc<R, FA>> + HList,
{
    let len = hl.len();
    if n >= len {
        return None;
    }
    applyr_args_at(hl, len - n - 1, f, args)
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
    
    struct FomatPrefix;
    impl<T> Func<(&str,T)> for FomatPrefix
    where T: std::fmt::Display
    {
        type Output = String;

        fn call((args,i): (&str, T)) -> Self::Output {
            let prefix = args;
            format!("{}: {}", prefix, i)
        }
    }

    #[test]
    fn test_apply_args() {
        let lst = hlist![10u32, "xyz", 0usize];

        assert_eq!(&applyr_args_at(lst.clone(), 0, FomatPrefix, "usize").unwrap(), "usize: 0");
        assert_eq!(&applyr_args_at(lst.clone(), 1, FomatPrefix, "str").unwrap(), "str: xyz");
        assert_eq!(&applyr_args_at(lst.clone(), 2, FomatPrefix, "u32").unwrap(), "u32: 10");

        assert_eq!(&applyl_args_at(lst.clone(), 0, FomatPrefix, "u32").unwrap(), "u32: 10");
        assert_eq!(&applyl_args_at(lst.clone(), 1, FomatPrefix, "str").unwrap(), "str: xyz");
        assert_eq!(&applyl_args_at(lst.clone(), 2, FomatPrefix, "usize").unwrap(), "usize: 0");
    }
}
