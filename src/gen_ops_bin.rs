#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_bin {
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr1:meta])*
    for $op:tt call $func:expr;$((where $($where1:tt)+) $(;)?)?
    $( $(#[$attr2:meta])* for $op2:tt call $func2:expr;$((where $($where2:tt)+) $(;)?)?)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr1])*
            for $op call $func;$((where $($where1)+))?
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $( $(#[$attr2])* for $op2 call $func2;$((where $($where2)+))?)+
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for + call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Add add] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for - call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Sub sub] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for * call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Mul mul] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for / call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Div div] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for % call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Rem rem] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for & call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [BitAnd bitand] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for | call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [BitOr bitor] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for ^ call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [BitXor bitxor] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for << call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Shl shl] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for >> call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_bin!(
            ($($gen)*);
            types $lhs, $rhs => $out;
            refs $lref $rref $($rev)?;
            $(#[$attr])*
            for [Shr shr] call $func;
            $($rest)*
        );
    };
}

