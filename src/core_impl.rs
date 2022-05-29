#[doc(hidden)]
#[macro_export]
macro_rules! _core_impl_bin {
    ($(<$($T:tt),+>;)?
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    for [$trait:ident $method:ident] call $func:expr;
    $(where $($where:tt)+)?) => {
        impl $(<$($T),+>)? ::std::ops::$trait<$crate::_refmut!($rref $rhs)> for $crate::_refmut!($lref $lhs)
        $(where $($where)+)? {
            type Output = $out;
            #[inline]
            fn $method(self, rhs: $crate::_refmut!($rref $rhs)) -> $out {
                $crate::_inner_func_call_bin!($($rev)? $lref self $rref rhs $func)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _core_impl_asgn {
    ($(<$($T:tt),+>;)?
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    for [$trait:ident $method:ident] call $func:expr;
    $(where $($where:tt)+)?) => {
        impl $(<$($T),+>)? ::std::ops::$trait<$crate::_refmut!($rref $rhs)> for $crate::_refmut!($lref $lhs)
        $(where $($where)+)? {
            #[inline]
            fn $method(&mut self, rhs: $crate::_refmut!($rref $rhs)) {
                ($func)(self, &rhs);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _core_impl_un {
    ($(<$($T:tt),+>;)?
    types $lhs:ty => $out:ty;
    refs $lref:ident;
    for [$trait:ident $method:ident] call $func:expr;
    $(where $($where:tt)+)?) => {
        impl $(<$($T),+>)? std::ops::$trait for $crate::_refmut!($lref $lhs)
        $(where $($where)+)? {
            type Output = $out;
            #[inline]
            fn $method(self) -> $out {
                ($func)(&self)
            }
        }
    };
}
