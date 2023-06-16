#[doc(hidden)]
#[macro_export]
macro_rules! _core_impl_bin {
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty => $out:ty;
    refs $lref:ident $rref:ident $($rev:ident)?;
    $(#[$attr:meta])*
    for [$trait:ident $method:ident] call $func:expr;
    $((where $($where1:tt)+))?
    $(where $($where:tt)+)?) => {
        impl $($gen)* ::core::ops::$trait<$crate::_refmut!($rref $rhs)> for $crate::_refmut!($lref $lhs)
        where $($($where)+ ,)? $($($where1)+)? {
            type Output = $out;
            $(#[$attr])*
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
    (($($gen:tt)*);
    types $lhs:ty, $rhs:ty;
    refs $lref:ident $rref:ident;
    $(#[$attr:meta])*
    for [$trait:ident $method:ident] call $func:expr;
    $((where $($where1:tt)+))?
    $(where $($where:tt)+)?) => {
        impl $($gen)* ::core::ops::$trait<$crate::_refmut!($rref $rhs)> for $crate::_refmut!($lref $lhs)
        where $($($where)+ ,)? $($($where1)+)? {
            $(#[$attr])*
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
    (($($gen:tt)*);
    types $lhs:ty => $out:ty;
    refs $lref:ident;
    $(#[$attr:meta])*
    for [$trait:ident $method:ident] call $func:expr;
    $((where $($where1:tt)+))?
    $(where $($where:tt)+)?) => {
        impl $($gen)* ::core::ops::$trait for $crate::_refmut!($lref $lhs)
        where $($($where)+ ,)? $($($where1)+)? {
            type Output = $out;
            $(#[$attr])*
            #[inline]
            fn $method(self) -> $out {
                ($func)(&self)
            }
        }
    };
}
