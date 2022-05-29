#[doc(hidden)]
#[macro_export]
macro_rules! _refmut {
    (own $xhs:ty) => {
        $xhs
    };
    (ref_ $xhs:ty) => {
        &$xhs
    };
    (mut_ $xhs:ty) => {
        &mut $xhs
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _inner_func_call_bin {
    (own $lhs:ident own $rhs:ident $func:expr) => {
        ($func)(&$lhs, &$rhs)
    };
    (ref_ $lhs:ident own $rhs:ident $func:expr) => {
        ($func)($lhs, &$rhs)
    };
    (mut_ $lhs:ident own $rhs:ident $func:expr) => {
        ($func)($lhs, &$rhs)
    };
    (own $lhs:ident ref_ $rhs:ident $func:expr) => {
        ($func)(&$lhs, $rhs)
    };
    (own $lhs:ident mut_ $rhs:ident $func:expr) => {
        ($func)(&$lhs, $rhs)
    };
    (ref_ $lhs:ident ref_ $rhs:ident $func:expr) => {
        ($func)($lhs, $rhs)
    };
    (mut_ $lhs:ident ref_ $rhs:ident $func:expr) => {
        ($func)($lhs, $rhs)
    };
    (ref_ $lhs:ident mut_ $rhs:ident $func:expr) => {
        ($func)($lhs, $rhs)
    };
    (mut_ $lhs:ident mut_ $rhs:ident $func:expr) => {
        ($func)($lhs, $rhs)
    };
    (rev $lref:ident $lhs:ident $rref:ident $rhs:ident $func:expr) => {
        $crate::_inner_func_call_bin!($rref $rhs $lref $lhs $func)
    };
}

