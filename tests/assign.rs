//#![trace_macros]
//trace_macros!(true);

use gen_ops::{gen_ops, gen_ops_ex};
use paste;

const EXPECTED_RES:[(i32, i32);10] = [
    (7, 5),// +
    (3, 1),// -
    (10, 6),// *
    (2, 1),// /
    (1, 1),// %
    (0, 2),// &
    (7, 3),// |
    (7, 1),// ^
    (20, 12),// <<
    (1, 0) // >>
];

macro_rules! generate_asgn_test {
    ((refs $lr:tt $rr:tt) $($ex:tt)?) => {
        crate::paste::paste! {
            mod [< $lr $rr $($ex _)? asgn >] {
                use super::*;
                #[derive(Clone, Copy, PartialEq, Debug)]
                struct IntPair(pub i32, pub i32);

                impl From<(i32, i32)> for IntPair {
                    #[inline]
                    fn from((a,b):(i32, i32))->IntPair {
                        IntPair(a,b)
                    }
                }
                
                generate_asgn_macro!($lr $rr $($ex)?);

                #[test]
                fn [< $lr $rr $($ex _)? asgn_test >]() {
                    #[allow(unused_mut)]
                    let mut b = 2;

                    gen_asserts!(b (refs $lr $rr));
                }
            }
        }
    };
}

macro_rules! generate_asgn_macro {
    (@ex ($ltype:ty = $($l:tt)?) ($rtype:ty = $($r:tt)?) ex) => {
        gen_ops_ex!(
            types $($l)? $ltype , $($r)? $rtype;
            for += call |a:&mut IntPair, b:&i32| {
                a.0 += b;
                a.1 += b;
            };
            for -= call |a:&mut IntPair, b:&i32| {
                a.0 -= b;
                a.1 -= b;
            };
            for /= call |a:&mut IntPair, b:&i32| {
                a.0 /= b;
                a.1 /= b;
            };
            for *= call |a:&mut IntPair, b:&i32| {
                a.0 *= b;
                a.1 *= b;
            };
            for %= call |a:&mut IntPair, b:&i32| {
                a.0 %= b;
                a.1 %= b;
            };
            for &= call |a:&mut IntPair, b:&i32| {
                a.0 &= b;
                a.1 &= b;
            };
            for |= call |a:&mut IntPair, b:&i32| {
                a.0 |= b;
                a.1 |= b;
            };
            for ^= call |a:&mut IntPair, b:&i32| {
                a.0 ^= b;
                a.1 ^= b;
            };
            for <<= call |a:&mut IntPair, b:&i32| {
                a.0 <<= b;
                a.1 <<= b;
            };
            for >>= call |a:&mut IntPair, b:&i32| {
                a.0 >>= b;
                a.1 >>= b;
            };
        );
    };
    (@ex ($ltype:ty) ($rtype:ty)) => {
        gen_ops!(
            types $ltype , $rtype;
            for += call |a:&mut IntPair, b:&i32| {
                a.0 += b;
                a.1 += b;
            };
            for -= call |a:&mut IntPair, b:&i32| {
                a.0 -= b;
                a.1 -= b;
            };
            for /= call |a:&mut IntPair, b:&i32| {
                a.0 /= b;
                a.1 /= b;
            };
            for *= call |a:&mut IntPair, b:&i32| {
                a.0 *= b;
                a.1 *= b;
            };
            for %= call |a:&mut IntPair, b:&i32| {
                a.0 %= b;
                a.1 %= b;
            };
            for &= call |a:&mut IntPair, b:&i32| {
                a.0 &= b;
                a.1 &= b;
            };
            for |= call |a:&mut IntPair, b:&i32| {
                a.0 |= b;
                a.1 |= b;
            };
            for ^= call |a:&mut IntPair, b:&i32| {
                a.0 ^= b;
                a.1 ^= b;
            };
            for <<= call |a:&mut IntPair, b:&i32| {
                a.0 <<= b;
                a.1 <<= b;
            };
            for >>= call |a:&mut IntPair, b:&i32| {
                a.0 >>= b;
                a.1 >>= b;
            };
        );
    };

    //rhs
    (@ex_rhs ($l:ty) own_) => {
        generate_asgn_macro!(@ex ($l) (i32));
    };
    (@ex_rhs ($l:ty) ref_) => {
        generate_asgn_macro!(@ex ($l) (&i32));
    };
    (@ex_rhs ($l:ty) mut_) => {
        generate_asgn_macro!(@ex ($l) (&mut i32));
    };
    (@ex_rhs ($lhs:ty =$($l:tt)?) own_ ex) => {
        generate_asgn_macro!(@ex ($lhs =$($l)?) (i32 =) ex);
    };
    (@ex_rhs ($lhs:ty =$($l:tt)?) ref_ ex) => {
        generate_asgn_macro!(@ex ($lhs =$($l)?) (&i32 =) ex);
    };
    (@ex_rhs ($lhs:ty =$($l:tt)?) mut_ ex) => {
        generate_asgn_macro!(@ex ($lhs =$($l)?) (&mut i32 =) ex);
    };
    (@ex_rhs ($lhs:ty =$($l:tt)?) ref_own_ ex) => {
        generate_asgn_macro!(@ex ($lhs =$($l)?) (i32 =ref) ex);
    };
    (@ex_rhs ($lhs:ty =$($l:tt)?) all_ ex) => {
        generate_asgn_macro!(@ex ($lhs =$($l)?) (i32 = mut) ex);
    };

    //lhs
    (own_ $r:tt) => {
        generate_asgn_macro!(@ex_rhs (IntPair) $r);
    };
    (mut_ $r:tt ) => {
        generate_asgn_macro!(@ex_rhs (&mut IntPair) $r);
    };
    (own_ $r:tt ex) => {
        generate_asgn_macro!(@ex_rhs (IntPair =) $r ex);
    };
    (mut_ $r:tt ex) => {
        generate_asgn_macro!(@ex_rhs (&mut IntPair =) $r ex);
    };
    (all_ $r:tt ex) => {
        generate_asgn_macro!(@ex_rhs (IntPair =mut) $r ex);
    };
}


macro_rules! gen_asserts {
    (@ops (refs ($ref:tt $res:ident) ($r:expr)) ($op1:tt $ind1:tt)  $(($op:tt $ind:tt))+) => {
        gen_asserts!(@ops2 (refs ($ref $res) ($r)) ($op1 $ind1));
        gen_asserts!(@ops (refs ($ref $res) ($r)) $(($op $ind))+);
    };
    (@ops (refs ($ref:tt $res:ident) ($r:expr)) ($op1:tt $ind1:tt)) => {
        gen_asserts!(@ops2 (refs ($ref $res) ($r)) ($op1 $ind1));
    };

    (@ops2 (refs (mut_ $res:ident) ($r:expr)) ($op1:tt $ind1:tt)) => {
        {
            let mut x = &mut $res [$ind1];
            x $op1 $r;
        }
    };
    (@ops2 (refs (own_ $res:ident) ($r:expr)) ($op1:tt $ind1:tt)) => {
        $res[$ind1] $op1 $r;
    };

    (@asrt $l:tt ($r:expr)) => {
        {
            let mut actual_res = [IntPair(5, 3); 10];
            gen_asserts!(@ops (refs ($l actual_res) ($r)) 
            (+= 0) (-= 1) (*= 2) (/= 3) (%= 4)
            (&= 5) (|= 6) (^= 7) (<<= 8) (>>= 9));

            actual_res.iter().zip(EXPECTED_RES).for_each(|(res,exp)|{
                assert_eq!(*res, Into::<IntPair>::into(exp), "\nresult={:?}", actual_res);
            });
        }
    };

    //lhs
    ((refs own_ ($r:expr))) => {
        gen_asserts!(@asrt own_ ($r));
    };
    ((refs mut_ ($r:expr))) => {
        gen_asserts!(@asrt mut_ ($r));
    };
    ((refs all_ ($r:expr))) => {
        gen_asserts!(@asrt own_ ($r));
        gen_asserts!(@asrt mut_ ($r));
    };

    //rhs
    ($b:ident (refs $l:tt own_)) => {
        gen_asserts!((refs $l ($b)));
    };
    ($b:ident (refs $l:tt ref_)) => {
        gen_asserts!((refs $l (&$b)));
    };
    ($b:ident (refs $l:tt mut_)) => {
        gen_asserts!((refs $l (&mut $b)));
    };
    ($b:ident (refs $l:tt ref_own_)) => {
        gen_asserts!((refs $l ($b)));
        gen_asserts!((refs $l (&$b)));
    };
    ($b:ident (refs $l:tt all_)) => {
        gen_asserts!((refs $l ($b)));
        gen_asserts!((refs $l (&$b)));
        gen_asserts!((refs $l (&mut $b)));
    };
}


generate_asgn_test!((refs own_ own_));
generate_asgn_test!((refs own_ ref_));
generate_asgn_test!((refs own_ mut_));

generate_asgn_test!((refs mut_ own_));
generate_asgn_test!((refs mut_ ref_));
generate_asgn_test!((refs mut_ mut_));

generate_asgn_test!((refs own_ ref_own_) ex);
generate_asgn_test!((refs own_ all_) ex);
generate_asgn_test!((refs mut_ ref_own_) ex);
generate_asgn_test!((refs mut_ all_) ex);

generate_asgn_test!((refs all_ own_) ex);
generate_asgn_test!((refs all_ ref_) ex);
generate_asgn_test!((refs all_ mut_) ex);
generate_asgn_test!((refs all_ ref_own_) ex);
generate_asgn_test!((refs all_ all_) ex);
