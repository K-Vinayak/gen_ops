use gen_ops::*;
use std::ops::*;

mod where_clause_ncomm {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);

    gen_ops!(
        <T>;
        types Complex<T>, Complex<T> => Complex<T>;
        for + call |a: &Complex<T>, b: &Complex<T>| Complex(a.0+b.0, a.1+b.1); 
        (where T: Add<Output=T>)

        for - call |a: &Complex<T>, b: &Complex<T>| Complex(a.0-b.0, a.1-b.1);
        (where T: Sub<Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_bin_test() {
        assert_eq!(Complex(1, -5) + Complex(5, 6), Complex(6, 1));
        assert_eq!(Complex(1, -5) - Complex(5, 6), Complex(-4, -11));
    }

    gen_ops!(
        <T>;
        types Complex<T>, Complex<T>;
        for += call |a: &mut Complex<T>, b: &Complex<T>| {
            a.0 += b.0;
            a.1 += b.1;
        }; (where T: AddAssign<T>)

        for -= call |a: &mut Complex<T>, b: &Complex<T>| {
            a.0 -= b.0;
            a.1 -= b.1;
        }; (where T: SubAssign<T>)

        where T: Copy
    );

    #[test]
    fn where_clause_asgn_test() {
        let x = Complex(5, -2);
        let b = Complex(1, -4);
        {
            let mut a = x;
            a += b;
            assert_eq!(a, Complex(6, -6));
        }
        {
            let mut a = x;
            a -= b;
            assert_eq!(a, Complex(4, 2));
        }
    }

    gen_ops!(
        <T>;
        types Complex<T> => Complex<T>;
        for - call |a: &Complex<T>| Complex(-a.0, -a.1);
        (where T: Neg<Output=T>)

        for ! call |a: &Complex<T>| Complex(a.0, -a.1);
        (where T: Neg<Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_un_test() {
        let a = Complex(5, -4);
        assert_eq!(-a, Complex(-5, 4));
        assert_eq!(!a, Complex(5, 4));
    }
}

mod where_clause_ncomm_ex {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);

    gen_ops_ex!(
        <T>;
        types ref Complex<T>, ref Complex<T> => Complex<T>;
        for + call |a: &Complex<T>, b: &Complex<T>| Complex(a.0+b.0, a.1+b.1); 
        (where T: Add<Output=T>)

        for - call |a: &Complex<T>, b: &Complex<T>| Complex(a.0-b.0, a.1-b.1);
        (where T: Sub<Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_bin_ex_test() {
        let a = Complex(1, -5);
        let b = Complex(5, 6);
        let res_add = Complex(6, 1);
        let res_sub = Complex(-4, -11);

        assert_eq!(a + b, res_add);
        assert_eq!(&a + b, res_add);
        assert_eq!(a + b, res_add);
        assert_eq!(&a + &b, res_add);

        assert_eq!(a - b, res_sub);
        assert_eq!(&a - b, res_sub);
        assert_eq!(a - &b, res_sub);
        assert_eq!(&a - &b, res_sub);
    }

    gen_ops_ex!(
        <T>;
        types mut Complex<T>, ref Complex<T>;
        for += call |a: &mut Complex<T>, b: &Complex<T>| {
            a.0 += b.0;
            a.1 += b.1;
        }; (where T: AddAssign<T>)

        for -= call |a: &mut Complex<T>, b: &Complex<T>| {
            a.0 -= b.0;
            a.1 -= b.1;
        }; (where T: SubAssign<T>)

        where T: Copy
    );

    #[test]
    fn where_clause_asgn_ex_test() {
        let mut x = Complex(5, -2);
        let b = Complex(1, -4);
        let res_add = Complex(6, -6);
        let res_sub = Complex(4, 2);
        {
            let mut a = x;
            a += b;
            assert_eq!(a, res_add);
        }
        {
            let mut a = x;
            a += &b;
            assert_eq!(a, res_add);
        }
        {
            let mut a = &mut x;
            a += b;
            assert_eq!(x, res_add);
        }
        x = Complex(5, -2);
        {
            let mut a = &mut x;
            a += &b;
            assert_eq!(x, res_add);
        }

        x = Complex(5, -2);
        {
            let mut a = x;
            a -= b;
            assert_eq!(a, res_sub);
        }
        {
            let mut a = x;
            a -= &b;
            assert_eq!(a, res_sub);
        }
        {
            let mut a = &mut x;
            a -= b;
            assert_eq!(x, res_sub);
        }
        x = Complex(5, -2);
        {
            let mut a = &mut x;
            a -= &b;
            assert_eq!(x, res_sub);
        }
    }

    gen_ops_ex!(
        <T>;
        types ref Complex<T> => Complex<T>;
        for - call |a: &Complex<T>| Complex(-a.0, -a.1);
        (where T: Neg<Output=T>)

        for ! call |a: &Complex<T>| Complex(a.0, -a.1);
        (where T: Neg<Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_un_ex_test() {
        let a = Complex(5, -4);
        assert_eq!(-a, Complex(-5, 4));
        assert_eq!(-&a, Complex(-5, 4));
        assert_eq!(!a, Complex(5, 4));
        assert_eq!(!&a, Complex(5, 4));
    }
}

mod where_clause_comm {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);

    gen_ops_comm!(
        <T>;
        types Complex<T>, i32 => Complex<T>;
        for + call |a: &Complex<T>, b: &i32| Complex(a.0+*b, a.1); 
        (where T: Add<i32, Output=T>)

        for * call |a: &Complex<T>, b:&i32| Complex(a.0*(*b), a.1*(*b));
        (where T: Mul<i32, Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_bin_comm_test() {
        let a = Complex(1, -5);
        let b = -3;
        let res_add = Complex(-2, -5);
        let res_mul = Complex(-3, 15);

        assert_eq!(a + b, res_add);
        assert_eq!(b + a, res_add);
        assert_eq!(a * b, res_mul);
        assert_eq!(b * a, res_mul);
    }
}

mod where_clause_comm_ex {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);

    gen_ops_comm_ex!(
        <T>;
        types ref Complex<T>, ref i32 => Complex<T>;
        for + call |a: &Complex<T>, b: &i32| Complex(a.0+*b, a.1); 
        (where T: Add<i32, Output=T>)

        for * call |a: &Complex<T>, b:&i32| Complex(a.0*(*b), a.1*(*b));
        (where T: Mul<i32, Output=T>)

        where T: Copy
    );

    #[test]
    fn where_clause_bin_comm_ex_test() {
        let a = Complex(1, -5);
        let b = -3;
        let res_add = Complex(-2, -5);
        let res_mul = Complex(-3, 15);
        assert_eq!(a + b, res_add);
        assert_eq!(&a + b, res_add);
        assert_eq!(a + &b, res_add);
        assert_eq!(&a + &b, res_add);

        assert_eq!(b + a, res_add);
        assert_eq!(&b + a, res_add);
        assert_eq!(b + &a, res_add);
        assert_eq!(&b + &a, res_add);

        assert_eq!(a * b, res_mul);
        assert_eq!(&a * b, res_mul);
        assert_eq!(a * &b, res_mul);
        assert_eq!(&a * &b, res_mul);

        assert_eq!(b * a, res_mul);
        assert_eq!(&b * a, res_mul);
        assert_eq!(b * &a, res_mul);
        assert_eq!(&b * &a, res_mul);
    }
}
