#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_asgn {
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr1:meta])*
    for $op:tt call $func:expr;$((where $($where1:tt)+) $(;)? )?
    $( $(#[$attr2:meta])* for $op2:tt call $func2:expr;$((where $($where2:tt)+) $(;)?)?)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr1])*
            for $op call $func;$((where $($where1)+))?
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $($(#[$attr2])* for $op2 call $func2;$((where $($where2)+))?)+
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for += call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [AddAssign add_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for -= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [SubAssign sub_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for *= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [MulAssign mul_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for /= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [DivAssign div_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for %= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [RemAssign rem_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for &= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [BitAndAssign bitand_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for |= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [BitOrAssign bitor_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for ^= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [BitXorAssign bitxor_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for <<= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [ShlAssign shl_assign] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for >>= call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_asgn!(
            ($($gen)*);
            types $lhs, $rhs;
            refs $lref $rref;
            $(#[$attr])*
            for [ShrAssign shr_assign] call $func;
            $($rest)*
        );
    };
}