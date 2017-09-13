


macro_rules! internal_tuple_impl_builder {
    ($toInvoke:ident) => {}; // no more

    (build $toInvoke:ident) => {
        internal_tuple_impl_builder!(
            $toInvoke
            (25 => Z => ZOut => ZRHS),
            (24 => Y => YOut => YRHS),
            (23 => X => XOut => XRHS),
            (22 => W => WOut => WRHS),
            (21 => V => VOut => VRHS),
            (20 => U => UOut => URHS),
            (19 => T => TOut => TRHS),
            (18 => S => SOut => SRHS),
            (17 => R => ROut => RRHS),
            (16 => Q => QOut => QRHS),
            (15 => P => POut => PRHS),
            (14 => O => OOut => ORHS),
            (13 => N => NOut => NRHS),
            (12 => M => MOut => MRHS),
            (11 => L => LOut => LRHS),
            (10 => K => KOut => KRHS),
            (9 => J => JOut => JRHS),
            (8 => I => IOut => IRHS),
            (7 => H => HOut => HRHS),
            (6 => G => GOut => GRHS),
            (5 => F => FOut => FRHS),
            (4 => E => EOut => ERHS),
            (3 => D => DOut => DRHS),
            (2 => C => COut => CRHS),
            (1 => B => BOut => BRHS),
            (0 => A => AOut => ARHS),
        );
    }; // no more

    ($toInvoke:ident ($idx:tt => $typ:ident => $typOut:ident => $typRHS:ident), $( ($nidx:tt => $ntyp:ident => $ntypOut:ident => $ntypRHS:ident), )*) => {
// Invoke recursive reversal of list that ends in the macro expansion implementation
// of the reversed list
//
        internal_tuple_impl_builder!($toInvoke [($idx, $typ, $typOut, $typRHS);] $( ($nidx => $ntyp => $ntypOut => $ntypRHS), )* );
        internal_tuple_impl_builder!($toInvoke $( ($nidx => $ntyp => $ntypOut => $ntypRHS), )*  ); // invoke macro on tail
    };

// ([accumulatedList], listToReverse); recursively calls tuple_impls until the list to reverse
// + is empty (see next pattern)
//
    ($toInvoke:ident [$(($accIdx: tt, $accTyp: ident, $accTypOut: ident, $accTypRHS: ident);)+]  ($idx:tt => $typ:ident => $typOut: ident => $typRHS: ident), $( ($nidx:tt => $ntyp:ident => $ntypOut:ident => $ntypRHS:ident), )*) => {
      internal_tuple_impl_builder!($toInvoke [($idx, $typ, $typOut, $typRHS); $(($accIdx, $accTyp, $accTypOut, $accTypRHS); )*] $( ($nidx => $ntyp => $ntypOut => $ntypRHS), ) * );
    };

// Finally expand into our implementation and invoke the macro
    ($toInvoke:ident [ $( ($nidx:tt, $ntyp:ident, $ntypOut:ident, $ntypRHS: ident); )*] ) => {
        $toInvoke![ [ $( ($nidx, $ntyp, $ntypOut, $ntypRHS ); )* ] ];
    }
}
