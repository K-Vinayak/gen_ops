//#![trace_macros]
//trace_macros!(true);
use gen_ops::{gen_ops, gen_ops_ex, gen_ops_comm, gen_ops_comm_ex};
use paste;

const EXPECTED_RES:[(i32, i32);10] = [
    (7, 3),// +
    (3, 3),// -
    (10, 6),// *
    (2, 1),// /
    (1, 1),// %
    (0, 2),// &
    (7, 3),// |
    (7, 1),// ^
    (20, 12),// <<
    (1, 0) // >>
];

const EXPECTED_RES_COMM:[(i32, i32);5] = [
    (7, 3),// +
    (10, 6),// *
    (0, 2),// &
    (7, 3),// |
    (7, 1),// ^
];

macro_rules! generate_bin_test {
    ($comm:tt (refs $lr:tt $rr:tt) $($ex:tt)?) => {
        crate::paste::paste! {
            mod [< $lr $rr $comm $(_$ex)? _bin >] {
                use super::*;
                #[derive(Clone, Copy, PartialEq, Debug)]
                struct IntPair(pub i32, pub i32);

                impl From<(i32, i32)> for IntPair {
                    #[inline]
                    fn from((a,b):(i32, i32))->IntPair {
                        IntPair(a,b)
                    }
                }
                
                generate_bin_macro!($comm $lr $rr $($ex)?);

                #[test]
                fn [< $lr $rr $comm $(_$ex)? _bin_test >]() {
                    #[allow(unused_mut)]
                    let mut a:IntPair = IntPair(5, 3);
                    #[allow(unused_mut)]
                    let mut b = 2;

                    gen_asserts!($comm (id a b) (refs $lr $rr));
                }
            }
        }
    };
}


macro_rules! generate_bin_macro {
    //commutative & extended or not
    (@comm [ncomm] ($lhs:ty) ($rhs:ty)) => {
        gen_ops!(
            types $lhs, $rhs => IntPair;
            for + call |a: &IntPair, b:&i32| {
                IntPair(a.0+b, a.1)
            };
            for - call |a: &IntPair, b:&i32| {
                IntPair(a.0-b, a.1)
            };
            for * call |a: &IntPair, b:&i32| {
                IntPair(a.0*b, a.1*b)
            };
            for / call |a: &IntPair, b:&i32| {
                IntPair(a.0/b, a.1/b)
            };
            for % call |a: &IntPair, b:&i32| {
                IntPair(a.0%b, a.1%b)
            };
            for << call |a: &IntPair, b:&i32| {
                IntPair(a.0<<b, a.1<<b)
            };
            for >> call |a: &IntPair, b:&i32| {
                IntPair(a.0>>b, a.1>>b)
            };
            for & call |a: &IntPair, b:&i32| {
                IntPair(a.0&b, a.1&b)
            };
            for | call |a: &IntPair, b:&i32| {
                IntPair(a.0|b, a.1|b)
            };
            for ^ call |a: &IntPair, b:&i32| {
                IntPair(a.0^b, a.1^b)
            };
        );
    };
    (@comm [ncomm ex] ($lhs:ty =$($l:tt)?) ($rhs:ty =$($r:tt)?)) => {
        gen_ops_ex!(
            types $($l)? $lhs, $($r)? $rhs => IntPair;
            for + call |a: &IntPair, b:&i32| {
                IntPair(a.0+b, a.1)
            };
            for - call |a: &IntPair, b:&i32| {
                IntPair(a.0-b, a.1)
            };
            for * call |a: &IntPair, b:&i32| {
                IntPair(a.0*b, a.1*b)
            };
            for / call |a: &IntPair, b:&i32| {
                IntPair(a.0/b, a.1/b)
            };
            for % call |a: &IntPair, b:&i32| {
                IntPair(a.0%b, a.1%b)
            };
            for << call |a: &IntPair, b:&i32| {
                IntPair(a.0<<b, a.1<<b)
            };
            for >> call |a: &IntPair, b:&i32| {
                IntPair(a.0>>b, a.1>>b)
            };
            for & call |a: &IntPair, b:&i32| {
                IntPair(a.0&b, a.1&b)
            };
            for | call |a: &IntPair, b:&i32| {
                IntPair(a.0|b, a.1|b)
            };
            for ^ call |a: &IntPair, b:&i32| {
                IntPair(a.0^b, a.1^b)
            };
        );
    };
    (@comm [comm] ($lhs:ty) ($rhs:ty)) => {
        gen_ops_comm!(
            types $lhs, $rhs => IntPair;
            for + call |a: &IntPair, b:&i32| {
                IntPair(a.0+b, a.1)
            };
            for * call |a: &IntPair, b:&i32| {
                IntPair(a.0*b, a.1*b)
            };
            for & call |a: &IntPair, b:&i32| {
                IntPair(a.0&b, a.1&b)
            };
            for | call |a: &IntPair, b:&i32| {
                IntPair(a.0|b, a.1|b)
            };
            for ^ call |a: &IntPair, b:&i32| {
                IntPair(a.0^b, a.1^b)
            };
        );
    };
    (@comm [comm ex] ($lhs:ty =$($l:tt)?) ($rhs:ty =$($r:tt)?)) => {
        gen_ops_comm_ex!(
            types $($l)? $lhs, $($r)? $rhs => IntPair;
            for + call |a: &IntPair, b:&i32| {
                IntPair(a.0+b, a.1)
            };
            for * call |a: &IntPair, b:&i32| {
                IntPair(a.0*b, a.1*b)
            };
            for & call |a: &IntPair, b:&i32| {
                IntPair(a.0&b, a.1&b)
            };
            for | call |a: &IntPair, b:&i32| {
                IntPair(a.0|b, a.1|b)
            };
            for ^ call |a: &IntPair, b:&i32| {
                IntPair(a.0^b, a.1^b)
            };
        );
    };

    //rhs
    (@ex_rhs $comm:tt ($l:ty) own_) => {
        generate_bin_macro!(@comm [$comm] ($l) (i32));
    };
    (@ex_rhs $comm:tt ($l:ty) ref_) => {
        generate_bin_macro!(@comm [$comm] ($l) (&i32));
    };
    (@ex_rhs $comm:tt ($l:ty) mut_) => {
        generate_bin_macro!(@comm [$comm] ($l) (&mut i32));
    };
    (@ex_rhs $comm:tt ($lhs:ty =$($l:tt)?) own_ ex) => {
        generate_bin_macro!(@comm [$comm ex] ($lhs =$($l)?) (i32 =));
    };
    (@ex_rhs $comm:tt ($lhs:ty =$($l:tt)?) ref_ ex) => {
        generate_bin_macro!(@comm [$comm ex] ($lhs =$($l)?) (&i32 =));
    };
    (@ex_rhs $comm:tt ($lhs:ty =$($l:tt)?) mut_ ex) => {
        generate_bin_macro!(@comm [$comm ex] ($lhs =$($l)?) (&mut i32 =));
    };
    (@ex_rhs $comm:tt ($lhs:ty =$($l:tt)?) ref_own_ ex) => {
        generate_bin_macro!(@comm [$comm ex] ($lhs =$($l)?) (i32 = ref));
    };
    (@ex_rhs $comm:tt ($lhs:ty =$($l:tt)?) all_ ex) => {
        generate_bin_macro!(@comm [$comm ex] ($lhs =$($l)?) (i32 = mut));
    };

    //lhs
    ($comm:tt own_ $r:tt) => {
        generate_bin_macro!(@ex_rhs $comm (IntPair) $r);
    };
    ($comm:tt ref_ $r:tt ) => {
        generate_bin_macro!(@ex_rhs $comm (&IntPair) $r);
    };
    ($comm:tt mut_ $r:tt ) => {
        generate_bin_macro!(@ex_rhs $comm (&mut IntPair) $r);
    };
    ($comm:tt own_ $r:tt ex) => {
        generate_bin_macro!(@ex_rhs $comm (IntPair =) $r ex);
    };
    ($comm:tt ref_ $r:tt ex) => {
        generate_bin_macro!(@ex_rhs $comm (&IntPair =) $r ex);
    };
    ($comm:tt mut_ $r:tt ex) => {
        generate_bin_macro!(@ex_rhs $comm (&mut IntPair =) $r ex);
    };
    ($comm:tt ref_own_ $r:tt ex) => {
        generate_bin_macro!(@ex_rhs $comm (IntPair =ref) $r ex);
    };
    ($comm:tt all_ $r:tt ex) => {
        generate_bin_macro!(@ex_rhs $comm (IntPair =mut) $r ex);
    };
}

macro_rules! gen_asserts {
    (@comm ncomm (refs ($l:expr) ($r:expr))) => {
        [$l + $r, $l - $r, $l * $r, $l / $r, $l % $r,
        $l & $r, $l | $r, $l ^ $r, $l << $r, $l >> $r]
        .iter().zip(EXPECTED_RES).for_each(|(res,exp)|{
            assert_eq!(*res, Into::<IntPair>::into(exp));
        });
    };
    (@comm comm (refs ($l:expr) ($r:expr))) => {
        [$l + $r, $l * $r, $l & $r, $l | $r, $l ^ $r]
        .iter().zip(EXPECTED_RES_COMM).for_each(|(res,exp)|{
            assert_eq!(*res, Into::<IntPair>::into(exp));
        });
        [$r + $l, $r * $l, $r & $l, $r | $l, $r ^ $l]
        .iter().zip(EXPECTED_RES_COMM).for_each(|(res,exp)|{
            assert_eq!(*res, Into::<IntPair>::into(exp));
        });
    };

    (@refs2 $comm:tt (id $a:ident $b:ident) (refs ($l:expr) own_)) => {
        gen_asserts!(@comm $comm (refs ($l) ($b)));
    };
    (@refs2 $comm:tt (id $a:ident $b:ident) (refs ($l:expr) ref_)) => {
        gen_asserts!(@comm $comm (refs ($l) (&$b)));
    };
    (@refs2 $comm:tt (id $a:ident $b:ident) (refs ($l:expr) mut_)) => {
        gen_asserts!(@comm $comm (refs ($l) (&mut $b)));
    };
    (@refs2 $comm:tt (id $a:ident $b:ident) (refs ($l:expr) ref_own_)) => {
        gen_asserts!(@comm $comm (refs ($l) ($b)));
        gen_asserts!(@comm $comm (refs ($l) (&$b)));
    };
    (@refs2 $comm:tt (id $a:ident $b:ident) (refs ($l:expr) all_)) => {
        gen_asserts!(@comm $comm (refs ($l) ($b)));
        gen_asserts!(@comm $comm (refs ($l) (&$b)));
        gen_asserts!(@comm $comm (refs ($l) (&mut $b)));
    };

    ($comm:tt (id $a:ident $b:ident) (refs own_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs ($a) $r1));
    };
    ($comm:tt (id $a:ident $b:ident) (refs ref_own_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs (&$a) $r1));
        gen_asserts!(@refs2 $comm (id $a $b) (refs ($a) $r1));
    };
    ($comm:tt (id $a:ident $b:ident) (refs all_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs (&mut $a) $r1));
        gen_asserts!(@refs2 $comm (id $a $b) (refs (&$a) $r1));
        gen_asserts!(@refs2 $comm (id $a $b) (refs ($a) $r1));
    };
    ($comm:tt (id $a:ident $b:ident) (refs own_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs ($a) $r1));
    };
    ($comm:tt (id $a:ident $b:ident) (refs ref_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs (&$a) $r1));
    };
    ($comm:tt (id $a:ident $b:ident) (refs mut_ $r1:tt)) => {
        gen_asserts!(@refs2 $comm (id $a $b) (refs (&mut $a) $r1));
    };
}

generate_bin_test!(ncomm (refs own_ own_));
generate_bin_test!(ncomm (refs own_ ref_));
generate_bin_test!(ncomm (refs own_ mut_));
generate_bin_test!(ncomm (refs ref_ own_));
generate_bin_test!(ncomm (refs ref_ ref_));
generate_bin_test!(ncomm (refs ref_ mut_));

generate_bin_test!(ncomm  (refs own_ ref_own_) ex);
generate_bin_test!(ncomm (refs own_ all_) ex);

generate_bin_test!(ncomm (refs ref_own_ own_) ex);
generate_bin_test!(ncomm (refs ref_own_ ref_) ex);
generate_bin_test!(ncomm (refs ref_ ref_own_) ex);
generate_bin_test!(ncomm (refs ref_own_ ref_own_) ex);
generate_bin_test!(ncomm (refs ref_ all_) ex);
generate_bin_test!(ncomm (refs ref_own_ mut_) ex);
generate_bin_test!(ncomm (refs ref_own_ all_) ex);

generate_bin_test!(ncomm (refs all_ own_) ex);
generate_bin_test!(ncomm (refs all_ ref_) ex);
generate_bin_test!(ncomm (refs mut_ ref_own_) ex);
generate_bin_test!(ncomm (refs all_ ref_own_) ex);
generate_bin_test!(ncomm (refs mut_ all_) ex);
generate_bin_test!(ncomm (refs all_ mut_) ex);
generate_bin_test!(ncomm (refs all_ all_) ex);

generate_bin_test!(comm (refs own_ own_));
generate_bin_test!(comm (refs own_ ref_));
generate_bin_test!(comm (refs own_ mut_));
generate_bin_test!(comm (refs ref_ own_));
generate_bin_test!(comm (refs ref_ ref_));
generate_bin_test!(comm (refs ref_ mut_));

generate_bin_test!(comm (refs own_ ref_own_) ex);
generate_bin_test!(comm (refs own_ all_) ex);

generate_bin_test!(comm (refs ref_own_ own_) ex);
generate_bin_test!(comm (refs ref_own_ ref_) ex);
generate_bin_test!(comm (refs ref_ ref_own_) ex);
generate_bin_test!(comm (refs ref_own_ ref_own_) ex);
generate_bin_test!(comm (refs ref_ all_) ex);
generate_bin_test!(comm (refs ref_own_ mut_) ex);
generate_bin_test!(comm (refs ref_own_ all_) ex);

generate_bin_test!(comm (refs all_ own_) ex);
generate_bin_test!(comm (refs all_ ref_) ex);
generate_bin_test!(comm (refs mut_ ref_own_) ex);
generate_bin_test!(comm (refs all_ ref_own_) ex);
generate_bin_test!(comm (refs mut_ all_) ex);
generate_bin_test!(comm (refs all_ mut_) ex);
generate_bin_test!(comm (refs all_ all_) ex);
