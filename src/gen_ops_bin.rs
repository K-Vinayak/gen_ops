#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_bin {
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for $op:tt call $func:expr;
    $(for $op2:tt call $func2:expr;)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for $op call $func;
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(for $op2 call $func2;)+
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for + call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Add add] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for - call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Sub sub] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for * call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Mul mul] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for / call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Div div] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for % call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Rem rem] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for & call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [BitAnd bitand] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for | call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [BitOr bitor] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for ^ call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [BitXor bitxor] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for << call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Shl shl] call $func;
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for >> call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            for [Shr shr] call $func;
            $(where $($where)+)?
        );
    };
}

