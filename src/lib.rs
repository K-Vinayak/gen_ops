#![no_std]
//! Macros for operator overloading of generic types. 
//! 
//! # Usage
//! 
//! The macros need four statements 
//! 
//! 1. (Optional) Generic parameter names
//! 2. Type signature or extended type signature
//! 3. Callable expressions for each operator
//! 4. (Optional) Where clause for generic parameters
//! 
//! > **Note:** All statements end with a semicolon except the where clause.
//! 
//! 
//! ## Example
//! 
//! ```rust
//! # #[macro_use] extern crate gen_ops;
//! # use std::ops::{Add, Sub};
//! 
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct Pair<T>(pub T, pub T);
//! 
//! #[inline]
//! fn sub_pair<T>(a: &Pair<T>, b: &Pair<T>) -> Pair<T>
//! where T: Sub<Output=T> + Copy {
//!     Pair(a.0 - b.0, a.1 - b.1)
//! }
//! 
//! gen_ops!(
//!     <T>;                               // Generic parameter names
//!     types Pair<T>, Pair<T> => Pair<T>; // Type signature
//!     for + call |a: &Pair<T>, b: &Pair<T>| {
//!         Pair(a.0 + b.0, a.1 + b.1)
//!     };
//!     for - call sub_pair;               // Callable expressions for operators
//!     where T: Add<Output=T> + Sub<Output=T> + Copy
//! );
//! 
//! # fn main() {
//! let a = Pair(2, 3);
//! let b = Pair(1, 8);
//! 
//! println!("a + b = {:?}", a + b); //a + b = Pair(3, 11)
//! # assert_eq!(a + b, Pair(3, 11));
//! println!("a - b = {:?}", a - b); //a - b = Pair(1, -5)
//! # assert_eq!(a-b, Pair(1, -5));
//! # }
//! ```
//! 
//! 
//! ## 1. Generic parameter names
//! 
//! First statement is Genetic parameters for `impl`. 
//! 
//! ```rust, ignore
//! gen_ops!(
//!     <T, U, V>;
//!     .....
//! );
//! 
//! // results in
//! 
//! impl<T, U, V> .... {
//!     ....
//! }
//! ```
//! 
//! 
//! ## 2. Type signature or extended type signature 
//! 
//! Type signatures are used by [`gen_ops!`] and [`gen_ops_comm!`]. 
//! And extended type signatures are used by [`gen_ops_ex!`] 
//! and [`gen_ops_comm_ex!`]. 
//! 
//! ### Syntax
//! - For binary operators the signature is written as `types Lhs, Rhs => Out;`. 
//! - For assignment operators the signature is written as `types Lhs, Rhs;`. 
//! - For unary operators the signature is written as `types Lhs => Out;`. 
//! 
//! Type signatures can have borrowed types for `Lhs` and `Rhs`, e.g, `types &Lhs, &mut Rhs => Out;`. 
//! 
//! ### What is extended type signature?
//! 
//! Extended type signatures, however, cannot use borrowed types. 
//! The extended types signatures can have `ref` or `mut` modifiers for `Lhs` and `Rhs`. 
//! 
//! `ref Lhs` implies that the trait is implemented for both `& Lhs` and `Lhs`. 
//! `mut Lhs` implies that the trait is implemented for `&mut Lhs`, `&Lhs` and `Lhs`. 
//! 
//! For example, `types ref T, ref T;` will implement `types &T, &T;`, `types &T, T;`, 
//! `types T, &T;` and `types T, T`. 
//! 
//! 
//! ## 3. Callable expressions for each operator
//! 
//! Callable expressions are written as `for <operator> call <expr>;`
//! 
//! The expression can be a closure or a function. 
//! 
//! The closure or function must take immutable borrowed parameters except for 
//! assignment operators where the first parameter must be mutable. 
//! 
//! ### Example
//! ```
//! # #[macro_use] extern crate gen_ops;
//! struct A(i32);
//! struct B(f32);
//! struct C(f64);
//! 
//! fn sub_abc(a:&A, b:&B) -> C {
//!     C(b.0 as f64 - a.0 as f64)
//! }
//! 
//! gen_ops!(
//!     types A, B => C;
//!     for + call |a: &A, b: &B| C(b.0 as f64 + a.0 as f64);
//!     for - call sub_abc;
//! );
//! # fn main() {
//! # let a = A(5);
//! # let b = B(4.);
//! # assert_eq!((a + b).0, 9.);
//! # }
//! ```
//! 
//! ## 4. Where clause for generic parameters
//! 
//! Optionally you can add a where clause as the last statement.
//! Note that the where clause does not end with a semicolon
//! ```rust, ignore
//! gen_ops!(
//!     <T>;
//!     ...
//!     where T: Copy
//! );
//! 
//! // results in
//! 
//! impl<T> ... where T: Copy {
//!     ...
//! }
//! ```
//! 
//! # Why use/not use gen_ops crate?
//! 
//! ## Features
//! - Multiple operators can be implemented with one macro call. 
//! - Use closure or function for the implementation
//! - Supports generics
//! 
//! ## Limitations
//! - Nested Parameters (e.g <T<U>>) not supported yet
//! - Not tested for generics involving lifetime parameters
//! - The macros are recursive
//! 
//! 
//! # Quick Reference Tables
//! 
//! ## Binary 
//! 
//! | Macro                | Type sign. | Lhs type sign.        | Rhs type sign.        | `Lhs` = `Rhs` |
//! |:---------------------|:----------:|:---------------------:|:---------------------:|:-------------:|
//! | [`gen_ops!`]         | borrowed   | `L`, `&L`, `&mut L`   | `R`, `&R`, `&mut R`   | allowed       |
//! | [`gen_ops_ex!`]      | extended   | `L`, `ref L`, `mut L` | `R`, `ref R`, `mut R` | allowed       |
//! | [`gen_ops_comm!`]    | borrowed   | `L`, `&L`, `&mut L`   | `R`, `&R`, `&mut R`   | not allowed   |
//! | [`gen_ops_comm_ex!`] | extended   | `L`, `ref L`, `mut L` | `R`, `ref R`, `mut R` | not allowed   |
//! 
//! ## Assignment
//! 
//! | Macro                | Type sign. | Lhs type sign. | Rhs type sign. | `Lhs` = `Rhs` |
//! |:---------------------|:----------:|:--------------:|:--------------:|:-------------:|
//! | [`gen_ops!`]         | borrowed   | `L`, `&mut L`  | `R`, `&mut R`  | allowed       |
//! | [`gen_ops_ex!`]      | extended   | `L`, `mut L`   | `R`, `mut R`   | allowed       |
//! 
//! ## Unary
//! 
//! | Macro                | Type sign. | Lhs type sign.        |
//! |:---------------------|:----------:|:---------------------:|
//! | [`gen_ops!`]         | borrowed   | `L`, `&L`, `&mut L`   |
//! | [`gen_ops_ex!`]      | extended   | `L`, `ref L`, `mut L` |
//! 
//! ## Closure/function
//! 
//! | Operator | Lhs        | Rhs    | Return |
//! |:--------:|:----------:|:------:|:------:|
//! | binary   | `&Lhs`     | `&Rhs` | `Out`  |
//! | assign   | `&mut Lhs` | `&Rhs` | `()`   |
//! | unary    | `&Lhs`     | -      | `Out`  |
//! 

#[macro_use]
mod util;

#[macro_use]
mod core_impl;

#[macro_use]
mod gen_ops_bin;

#[macro_use]
mod gen_ops_un;

#[macro_use]
mod gen_ops_asgn;

/// The primary macro for all operators. 
/// 
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate gen_ops;
/// # use std::ops::{Add, Sub};
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct Pair<T>(pub T, pub T);
/// 
/// gen_ops!(
///     <T>;
///     types Pair<T>, Pair<T> => Pair<T>;
///     for + call |a: &Pair<T>, b: &Pair<T>| Pair(a.0 + b.0, a.1 + b.1);
///     for - call |a: &Pair<T>, b: &Pair<T>| Pair(a.0 - b.0, a.1 - b.1);
///     where T: Add<Output=T> + Sub<Output=T> + Copy
/// );
/// 
/// # fn main() {
/// let a = Pair(10, 5);
/// let b = Pair(8, 9);
/// 
/// println!("a + b = {:?}", a + b); // a + b = Pair(18, 14)
/// # assert_eq!(a+b, Pair(18, 14));
/// println!("a - b = {:?}", a - b); // a - b = Pair(2, -4)
/// # assert_eq!(a-b, Pair(2, -4));
/// # }
/// ```
/// 
/// # Note
/// 
/// Type signatures supported for [`gen_ops!`] are 
/// - `types Lhs, Rhs => Out;` for binary
/// - `types Lhs, Rhs;` for assignment
/// - `types Lhs => Out;` for unary
/// 
/// `Lhs` and `Rhs` can be borrowed types except for assignment operators where `Lhs` cannot be 
/// immutably borrowed. 
/// 
/// For unary and binary operators, the callable expressions must take immutable borrowed types as argument(s) 
/// and return the result of the `Out` type. 
/// And for assignment operators the callable expressions must take mutable borrowed type as first 
/// argument, take immutable borrowed type as second argument and return nothing;
/// 
#[macro_export]
macro_rules! gen_ops {
    //binary
    ($(<$($T:tt),+>;)? types $lhs:ty, $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs own own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &$lhs:ty, $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs ref_ own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types $lhs:ty, &$rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs own ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)?types &$lhs:ty, &$rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs ref_ ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &mut $lhs:ty, $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs mut_ own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types $lhs:ty, &mut $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs own mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &mut $lhs:ty, &mut $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs mut_ mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &mut $lhs:ty, &$rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs mut_ ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &$lhs:ty, &mut $rhs:ty => $out:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs ref_ mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };

    //assign
    ($(<$($T:tt),+>;)? types $lhs:ty, $rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types $lhs:ty, &$rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types $lhs:ty, &mut $rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty, $rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty, &$rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty, &mut $rhs:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };

    //unary
    ($(<$($T:tt),+>;)? types $lhs:ty => $output:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs own;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &$lhs:ty => $output:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs ref_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
    ($(<$($T:tt),+>;)? types &mut$lhs:ty => $output:ty;
    $(for $op:tt call $func:expr;)+ $(where $($where:tt)+)?) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $output;
            refs mut_;
            $(for $op call $func;)+
            $(where $($where)+)?
        );
    };
}


/// Implements commutative operations. 
/// 
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate gen_ops;
/// # use std::ops::{Mul, BitAnd};
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct Pair<T>(pub T, pub T);
/// 
/// gen_ops_comm!(
///     <T>;
///     types Pair<T>, i32 => Pair<T>;
///     for * call |a: &Pair<T>, b:&i32| Pair(a.0 * *b, a.1 * *b);
///     for & call |a: &Pair<T>, b:&i32| Pair(a.0 & *b, a.1 & *b);
///     where T: Mul<i32, Output=T> + BitAnd<i32, Output=T> + Copy
/// );
/// 
/// # fn main() {
/// let a = Pair(12, 3);
/// 
/// println!("a * 5 = {:?}", a * 5); //a * 5 = Pair(60, 15)
/// # assert_eq!(a*5, Pair(60, 15));
/// println!("5 * a = {:?}", 5 * a); //5 * a = Pair(60, 15)
/// # assert_eq!(5* a, Pair(60, 15));
/// println!("a & 2 = {:?}", a & 2); //a & 2 = Pair(0, 2)
/// # assert_eq!(a & 2, Pair(0, 2));
/// println!("2 & a = {:?}", 2 & a); //2 & a = Pair(0, 2)
/// # assert_eq!(2 & a, Pair(0, 2));
/// # }
/// ```
/// 
/// # Note
/// 
/// The only type signature supported for [`gen_ops_comm!`] is `types Lhs, Rhs => Out;`  
/// It implements both `types Lhs, Rhs => Out;` and `types Rhs, Lhs => Out;`. 
/// 
/// `Lhs` and `Rhs` can be borrowed types except for assignment operators where `Lhs` cannot be 
/// immutably borrowed. 
/// Also make sure that `Lhs` and `Rhs` are of different types. 
/// 
/// Callable expressions must take immutable borrowed types as arguments 
/// and return the result of the `Out` type
/// 
#[macro_export]
macro_rules! gen_ops_comm {
    ($(<$($T:tt),+>;)? types $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_comm!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types &$lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_comm!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types &mut $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_comm!(_step2; $(<$($T),+>;)? types mut_: $lhs, $($rest)+);
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, $rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $lhs, $rhs => $out; 
            refs $lref own;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $rhs, $lhs => $out; 
            refs own $lref rev;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, &$rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $lhs, $rhs => $out; 
            refs $lref ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $rhs, $lhs => $out; 
            refs ref_ $lref rev;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, &mut $rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $lhs, $rhs => $out; 
            refs $lref mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? 
            types $rhs, $lhs => $out; 
            refs mut_ $lref rev;
            $($rest)+
        );
    };
}


/// Implements trait for borrowed types. 
/// 
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate gen_ops;
/// # use std::ops::Mul;
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct Pair<T>(pub T, pub T);
/// 
/// gen_ops_ex!(
///     <T>;
///     types mut Pair<T>, T => Pair<T>;
///     for * call |a: &Pair<T>, b:&T| Pair(a.0 * *b, a.1 * *b);
///     where T: Mul<Output=T> + Copy
/// );
/// 
/// # fn main() {
/// let mut a = Pair(12, 3);
/// {
///     let mut b = &mut a;
///     println!("&mut a * 2 = {:?}", b * 2);// &mut a * 2 = Pair(24, 6)
/// }
/// println!("&a * 2 = {:?}", &a * 2);// &a * 2 = Pair(24, 6)
/// # assert_eq!(&a * 2, Pair(24, 6));
/// println!("a * 2 = {:?}", a * 2);// a * 2 = Pair(24, 6)
/// # assert_eq!(a * 2, Pair(24, 6));
/// # }
/// ```
/// 
/// # Note
/// 
/// [`gen_ops_ex!`] uses [extended] type signature. 
/// 
/// Type signatures supported for [`gen_ops_ex!`] are 
/// - `types Lhs, Rhs => Out;` for binary 
/// - `types Lhs, Rhs;` for assignment 
/// - `types Lhs => Out` for unary 
/// 
/// `Lhs` and `Rhs` must be owned types. 
/// 
/// For unary and binary operators, the callable expressions must take immutable borrowed types 
/// as argument(s) and return the result of the `Out` type. 
/// And for assignment operators the callable expressions must take mutable borrowed type as first 
/// argument, take immutable borrowed type as second argument and return nothing;
/// 
/// [extended]: crate#2-type-signature-or-extended-type-signature
#[macro_export]
macro_rules! gen_ops_ex {
    ($(<$($T:tt),+>;)? types $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types ref $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types mut_: $lhs, $($rest)+);
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty, $($rest:tt)+) => {
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types mut_: $lhs, $($rest)+);
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
        $crate::gen_ops_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };

    //binary
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, $rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, ref $rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, mut $rhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)?
            types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
    };

    //assign
    (_step2; $(<$($T:tt),+>;)? types own: $lhs:ty, $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types own: $lhs:ty, ref $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types own: $lhs:ty, mut $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs own own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types mut_: $lhs:ty, $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types mut_: $lhs:ty, ref $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types mut_: $lhs:ty, mut $rhs:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_asgn!(
            $(<$($T),+>;)?
            types $lhs, $rhs;
            refs mut_ own;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types ref_: $lhs:ty, mut $rhs:ty; $($rest:tt)+) => {};
    (_step2; $(<$($T:tt),+>;)? types ref_: $lhs:ty, ref $rhs:ty; $($rest:tt)+) => {};
    (_step2; $(<$($T:tt),+>;)? types ref_: $lhs:ty, $rhs:ty; $($rest:tt)+) => {};

    //unary
    ($(<$($T:tt),+>;)? types $lhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs own;
            $($rest)+
        );
    };
    ($(<$($T:tt),+>;)? types ref $lhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs own;
            $($rest)+
        );
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty => $out:ty; $($rest:tt)+) => {
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_un!(
            $(<$($T),+>;)?
            types $lhs => $out;
            refs own;
            $($rest)+
        );
    };
}

/// Implements commutative operations for borrowed types. 
/// 
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate gen_ops;
/// # use std::ops::{Mul, BitAnd};
/// #[derive(Debug, Copy, Clone, PartialEq)]
/// struct Pair<T>(pub T, pub T);
/// 
/// gen_ops_comm_ex!(
///     <T>;
///     types ref Pair<T>, i32 => Pair<T>;
///     for * call |a: &Pair<T>, b:&i32| Pair(a.0 * *b, a.1 * *b);
///     where T: Mul<i32, Output=T> + BitAnd<i32, Output=T> + Copy
/// );
/// 
/// # fn main() {
/// let a = Pair(12, 3);
/// println!("a * 5 = {:?}", a * 5); //a * 5 = Pair(60, 15)
/// # assert_eq!(a * 5, Pair(60, 15));
/// println!("5 * a = {:?}", 5 * a); //5 * a = Pair(60, 15)
/// # assert_eq!(5 * a, Pair(60, 15));
/// println!("5 * &a = {:?}", 5 * &a); //5 * &a = Pair(60, 15)
/// # assert_eq!(5 * &a, Pair(60, 15));
/// println!("&a * 5 = {:?}", &a * 5); //&a * 5 = Pair(60, 15)
/// # assert_eq!(&a * 5, Pair(60, 15));
/// # }
/// ```
/// 
/// # Note
/// 
/// [`gen_ops_comm_ex!`] uses [extended] type signature. The only type signature 
/// supported for [`gen_ops_comm_ex!`] is `types Lhs, Rhs => Out;`. 
/// It implements both `types Lhs, Rhs => Out;` and `types Rhs, Lhs => Out;`. 
/// 
/// `Lhs` and `Rhs` must be owned types 
/// and must be of different types. 
/// 
/// Callable expressions must take immutable borrowed types as arguments 
/// and return the result of the `Out` type
/// 
/// [extended]: crate#2-type-signature-or-extended-type-signature
#[macro_export]
macro_rules! gen_ops_comm_ex {
    ($(<$($T:tt),+>;)? types $lhs:ty,  $($rest:tt)+) => {
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types ref $lhs:ty,  $($rest:tt)+) => {
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    ($(<$($T:tt),+>;)? types mut $lhs:ty,  $($rest:tt)+) => {
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types mut_: $lhs, $($rest)+);
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types ref_: $lhs, $($rest)+);
        $crate::gen_ops_comm_ex!(_step2; $(<$($T),+>;)? types own: $lhs, $($rest)+);
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, $rhs:ty => $out:ty;  $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs own $lref rev;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, ref $rhs:ty => $out:ty;  $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs ref_ $lref rev;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs own $lref rev;
            $($rest)+
        );
    };
    (_step2; $(<$($T:tt),+>;)? types $lref:ident: $lhs:ty, mut $rhs:ty => $out:ty;  $($rest:tt)+) => {
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref mut_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs mut_ $lref rev;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref ref_;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs ref_ $lref rev;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $lhs, $rhs => $out;
            refs $lref own;
            $($rest)+
        );
        $crate::_gen_ops_internal_bin!(
            $(<$($T),+>;)? types $rhs, $lhs => $out;
            refs own $lref rev;
            $($rest)+
        );
    };
}
