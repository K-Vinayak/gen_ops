#[doc(hidden)]
#[macro_export]
macro_rules! _gen_ops_internal_un {
    (($($gen:tt)*);
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    $(#[$attr1:meta])*
    for $op:tt call $func:expr; $((where $($where1:tt)+) $(;)?)?
    $( $(#[$attr2:meta])* for $op2:tt call $func2:expr;$((where $($where2:tt)+) $(;)?)?)+
    $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_un!(
            ($($gen)*);
            types $lhs => $output;
            refs $lref;
            $(#[$attr1])*
            for $op call $func;$((where $($where1)+))?
            $(where $($where)+)?
        );
        $crate::_gen_ops_internal_un!(
            ($($gen)*);
            types $lhs => $output;
            refs $lref;
            $( $(#[$attr2])* for $op2 call $func2;$((where $($where2)+))?)+
            $(where $($where)+)?
        );
    };
    (($($gen:tt)*);
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    $(#[$attr:meta])*
    for - call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_un!(
            ($($gen)*);
            types $lhs => $output;
            refs $lref;
            $(#[$attr])*
            for [Neg neg] call $func;
            $($rest)*
        );
    };
    (($($gen:tt)*);
    types $lhs:ty => $output:ty;
    refs $lref:ident;
    $(#[$attr:meta])*
    for ! call $func:expr;
    $($rest:tt)*) => {
        $crate::_core_impl_un!(
            ($($gen)*);
            types $lhs => $output;
            refs $lref;
            $(#[$attr])*
            for [Not not] call $func;
            $($rest)*
        );
    };
}
