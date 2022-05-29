#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_un {
    ($(<$($T:tt),+>;)?
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    for $op:tt call $func:expr;
    $(for $op2:tt call $func2:expr;)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs $lref;
            for $op call $func;
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs $lref;
            $(for $op2 call $func2;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)?
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    for - call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs $lref;
            for [Neg neg] call $func;
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)?
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    for ! call $func:expr;
    $(where $($where:tt)+)?) => {
        $crate::_core_impl_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs $lref;
            for [Not not] call $func;
            $(where $($where)+)?
        );
    };
}
