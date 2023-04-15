//! this module does random case testing. Use cargo expand to inspect expanded macros

use gen_ops::*;
use std::ops::*;

pub mod doc_comments_single_line {

    use super::*;
    struct Pair<T>(T, T);

    gen_ops!(
        <T>;
        types Pair<T>, Pair<T> => Pair<T>;
        /// Add two pairs
        for + call |a:&Pair<T>, b:&Pair<T>| Pair(a.0+b.0, a.1+b.1);
        (where T: Add<T, Output=T> + Copy)
    );

    /// This is a redundant test written to see if hovering over + sign shows doc string
    pub fn doc_comments_single_line_test() {
        let a = Pair(1, 2);
        let b = Pair(3, 5);
        let _c = a + b;
    }
}


pub mod doc_comments_multi_line {

    use super::*;
    struct Pair<T>(T, T);

    gen_ops!(
        <T>;
        types Pair<T>, Pair<T> => Pair<T>;
        /// Add two pairs
        /// 
        /// Returns another `Pair`
        for + call |a:&Pair<T>, b:&Pair<T>| Pair(a.0+b.0, a.1+b.1);
        (where T: Add<T, Output=T> + Copy)
    );

    /// This is a redundant test written to see if hovering over + sign shows doc string
    pub fn doc_comments_multi_line_test() {
        let a = Pair(1, 2);
        let b = Pair(3, 5);
        let _c = a + b;
    }
}

pub mod doc_comments_multi_op {

    use super::*;
    struct Pair<T>(T, T);

    gen_ops_ex!(
        <T>;
        types ref Pair<T>, ref Pair<T> => Pair<T>;
        /// Add two pairs
        /// 
        /// Returns another `Pair`
        for + call |a:&Pair<T>, b:&Pair<T>| Pair(a.0+b.0, a.1+b.1);
        (where T: Add<T, Output=T>)

        /// Subtract one pair from another
        /// 
        /// Returns another `Pair`
        for - call |a:&Pair<T>, b:&Pair<T>| Pair(a.0-b.0, a.1-b.1);
        (where T: Sub<T, Output=T>)

        where T:Copy
    );

    /// This is a redundant test written to see if hovering over + and - signs shows doc string
    pub fn doc_comments_multi_op_test() {
        let a = Pair(1, 2);
        let b = Pair(3, 5);
        let _c = &a + &b;
        let _d = &a - &b;
    }
}
