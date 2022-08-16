#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_asgn {
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for $op:tt call $func:expr;$((where $($where1:tt)+) $(;)? )?
    $(for $op2:tt call $func2:expr;$((where $($where2:tt)+) $(;)?)?)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for $op call $func;$((where $($where1)+))?
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(for $op2 call $func2;$((where $($where2)+))?)+
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for += call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [AddAssign add_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for -= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [SubAssign sub_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for *= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [MulAssign mul_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for /= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [DivAssign div_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for %= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [RemAssign rem_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for &= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [BitAndAssign bitand_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for |= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [BitOrAssign bitor_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for ^= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [BitXorAssign bitxor_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for <<= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [ShlAssign shl_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for >>= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            for [ShrAssign shr_assign] call $func;
            $($rest)*
        );
    };
}