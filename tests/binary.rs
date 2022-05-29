mod all_ops_simple {
    
    use gen_ops::gen_ops;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops!(
        types Pair, i32 => Pair;
        for + call |a: &Pair, b:&i32| {
            Pair(a.0+b, a.1)
        };
        for - call |a: &Pair, b:&i32| {
            Pair(a.0-b, a.1)
        };
        for * call |a: &Pair, b:&i32| {
            Pair(a.0*b, a.1*b)
        };
        for / call |a: &Pair, b:&i32| {
            Pair(a.0/b, a.1/b)
        };
        for % call |a: &Pair, b:&i32| {
            Pair(a.0%b, a.1%b)
        };
        for << call |a: &Pair, b:&i32| {
            Pair(a.0<<b, a.1<<b)
        };
        for >> call |a: &Pair, b:&i32| {
            Pair(a.0>>b, a.1>>b)
        };
        for & call |a: &Pair, b:&i32| {
            Pair(a.0&b, a.1&b)
        };
        for | call |a: &Pair, b:&i32| {
            Pair(a.0|b, a.1|b)
        };
        for ^ call |a: &Pair, b:&i32| {
            Pair(a.0^b, a.1^b)
        };
    );

    #[test]
    fn all_ops_simple_bin_test() {
        let a = Pair(5, 3);
        let b = 2;

        assert_eq!(a + b, Pair(7, 3));
        assert_eq!(a - b, Pair(3, 3));
        assert_eq!(a * b, Pair(10, 6));
        assert_eq!(a / b, Pair(2, 1));
        assert_eq!(a % b, Pair(1, 1));
        assert_eq!(a & b, Pair(0, 2));
        assert_eq!(a | b, Pair(7, 3));
        assert_eq!(a ^ b, Pair(7, 1));
        assert_eq!(a << b, Pair(20, 12));
        assert_eq!(a >> b, Pair(1, 0));
    }

}

mod all_ops_comm {
    use gen_ops::gen_ops_comm;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops_comm!(
        types Pair, i32 => Pair;
        for + call |a: &Pair, b:&i32| {
            Pair(a.0+b, a.1)
        };
        for * call |a: &Pair, b:&i32| {
            Pair(a.0*b, a.1*b)
        };
        for & call |a: &Pair, b:&i32| {
            Pair(a.0&b, a.1&b)
        };
        for | call |a: &Pair, b:&i32| {
            Pair(a.0|b, a.1|b)
        };
        for ^ call |a: &Pair, b:&i32| {
            Pair(a.0^b, a.1^b)
        };
    );

    #[test]
    fn all_ops_comm_bin_test() {
        let a = Pair(5, 3);
        let b = 2;

        let mut res = a+b;
        assert_eq!(res, Pair(7, 3));
        assert_eq!(b + a, res);

        res = a * b;
        assert_eq!(res, Pair(10, 6));
        assert_eq!(b * a, res);

        res = a & b;
        assert_eq!(res, Pair(0, 2));
        assert_eq!(b & a, res);

        res = a | b;
        assert_eq!(res, Pair(7, 3));
        assert_eq!(b | a, res);

        res = a ^ b;
        assert_eq!(res, Pair(7, 1));
        assert_eq!(b ^ a, res);
    }
}

mod all_ops_ex {
    use gen_ops::gen_ops_ex;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops_ex!(
        types mut Pair, mut i32 => Pair;
        for + call |a: &Pair, b:&i32| {
            Pair(a.0+b, a.1+b)
        };
        for - call |a: &Pair, b:&i32| {
            Pair(a.0-b, a.1-b)
        };
        for * call |a: &Pair, b:&i32| {
            Pair(a.0*b, a.1*b)
        };
        for / call |a: &Pair, b:&i32| {
            Pair(a.0/b, a.1/b)
        };
        for % call |a: &Pair, b:&i32| {
            Pair(a.0%b, a.1%b)
        };
        for << call |a: &Pair, b:&i32| {
            Pair(a.0<<b, a.1<<b)
        };
        for >> call |a: &Pair, b:&i32| {
            Pair(a.0>>b, a.1>>b)
        };
        for & call |a: &Pair, b:&i32| {
            Pair(a.0&b, a.1&b)
        };
        for | call |a: &Pair, b:&i32| {
            Pair(a.0|b, a.1|b)
        };
        for ^ call |a: &Pair, b:&i32| {
            Pair(a.0^b, a.1^b)
        };
    );

    #[test]
    fn all_ops_ex_bin_test() {
        let mut a = Pair(5, 3);
        let mut b = 2;

        let mut res = a+b;
        assert_eq!(res, Pair(7, 5));
        assert_eq!(&mut a+ &mut b, res);
        assert_eq!(&mut a + &b, res);
        assert_eq!(&mut a + b, res);
        assert_eq!(&a + &mut b, res);
        assert_eq!(&a + &b, res);
        assert_eq!(&a + b, res);
        assert_eq!(a + &mut b, res);
        assert_eq!(a + &b, res);

        res = a - b;
        assert_eq!(res, Pair(3, 1));
        assert_eq!(&mut a - &mut b, res);
        assert_eq!(&mut a - &b, res);
        assert_eq!(&mut a - b, res);
        assert_eq!(&a - &mut b, res);
        assert_eq!(&a - &b, res);
        assert_eq!(&a - b, res);
        assert_eq!(a - &mut b, res);
        assert_eq!(a - &b, res);

        res = a * b;
        assert_eq!(res, Pair(10, 6));
        assert_eq!(&mut a * &mut b, res);
        assert_eq!(&mut a * &b, res);
        assert_eq!(&mut a * b, res);
        assert_eq!(&a * &mut b, res);
        assert_eq!(&a * &b, res);
        assert_eq!(&a * b, res);
        assert_eq!(a * &mut b, res);
        assert_eq!(a * &b, res);

        res = a/b;
        assert_eq!(res, Pair(2, 1));
        assert_eq!(&mut a / &mut b, res);
        assert_eq!(&mut a / &b, res);
        assert_eq!(&mut a / b, res);
        assert_eq!(&a / &mut b, res);
        assert_eq!(&a / &b, res);
        assert_eq!(&a / b, res);
        assert_eq!(a / &mut b, res);
        assert_eq!(a / &b, res);

        res = a%b;
        assert_eq!(res, Pair(1, 1));
        assert_eq!(&mut a % &mut b, res);
        assert_eq!(&mut a % &b, res);
        assert_eq!(&mut a % b, res);
        assert_eq!(&a % &mut b, res);
        assert_eq!(&a % &b, res);
        assert_eq!(&a % b, res);
        assert_eq!(a % &mut b, res);
        assert_eq!(a % &b, res);

        res = a & b;
        assert_eq!(res, Pair(0, 2));
        assert_eq!(&mut a & &mut b, res);
        assert_eq!(&mut a & &b, res);
        assert_eq!(&mut a & b, res);
        assert_eq!(&a & &mut b, res);
        assert_eq!(&a & &b, res);
        assert_eq!(&a & b, res);
        assert_eq!(a & &mut b, res);
        assert_eq!(a & &b, res);

        res = a | b;
        assert_eq!(res, Pair(7, 3));
        assert_eq!(&mut a | &mut b, res);
        assert_eq!(&mut a | &b, res);
        assert_eq!(&mut a | b, res);
        assert_eq!(&a | &mut b, res);
        assert_eq!(&a | &b, res);
        assert_eq!(&a | b, res);
        assert_eq!(a | &mut b, res);
        assert_eq!(a | &b, res);

        res = a ^ b;
        assert_eq!(res, Pair(7, 1));
        assert_eq!(&mut a ^ &mut b, res);
        assert_eq!(&mut a ^ &b, res);
        assert_eq!(&mut a ^ b, res);
        assert_eq!(&a ^ &mut b, res);
        assert_eq!(&a ^ &b, res);
        assert_eq!(&a ^ b, res);
        assert_eq!(a ^ &mut b, res);
        assert_eq!(a ^ &b, res);

        res = a << b;
        assert_eq!(res, Pair(20, 12));
        assert_eq!(&mut a << &mut b, res);
        assert_eq!(&mut a << &b, res);
        assert_eq!(&mut a << b, res);
        assert_eq!(&a << &mut b, res);
        assert_eq!(&a << &b, res);
        assert_eq!(&a << b, res);
        assert_eq!(a << &mut b, res);
        assert_eq!(a << &b, res);

        res = a >> b;
        assert_eq!(res, Pair(1, 0));
        assert_eq!(&mut a >> &mut b, res);
        assert_eq!(&mut a >> &b, res);
        assert_eq!(&mut a >> b, res);
        assert_eq!(&a >> &mut b, res);
        assert_eq!(&a >> &b, res);
        assert_eq!(&a >> b, res);
        assert_eq!(a >> &mut b, res);
        assert_eq!(a >> &b, res);
    }
}

mod all_ops_comm_ex {
    use gen_ops::gen_ops_comm_ex;
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops_comm_ex!(
        types mut Pair, mut i32 => Pair;
        for + call |a: &Pair, b:&i32| {
            Pair(a.0+b, a.1+b)
        };
        for * call |a: &Pair, b:&i32| {
            Pair(a.0*b, a.1*b)
        };
        for & call |a: &Pair, b:&i32| {
            Pair(a.0&b, a.1&b)
        };
        for | call |a: &Pair, b:&i32| {
            Pair(a.0|b, a.1|b)
        };
        for ^ call |a: &Pair, b:&i32| {
            Pair(a.0^b, a.1^b)
        };
    );

    #[test]
    fn all_ops_comm_ex_bin_test() {
        let mut a = Pair(5, 3);
        let mut b = 2;

        let mut res = a+b;
        assert_eq!(res, Pair(7, 5));
        assert_eq!(&mut a+ &mut b, res);
        assert_eq!(&mut a + &b, res);
        assert_eq!(&mut a + b, res);
        assert_eq!(&a + &mut b, res);
        assert_eq!(&a + &b, res);
        assert_eq!(&a + b, res);
        assert_eq!(a + &mut b, res);
        assert_eq!(a + &b, res);
        assert_eq!(&mut b + &mut a, res);
        assert_eq!(&mut b + &a, res);
        assert_eq!(&mut b + a, res);
        assert_eq!(&b + &mut a, res);
        assert_eq!(&b + &a, res);
        assert_eq!(&b + a, res);
        assert_eq!(b + &mut a, res);
        assert_eq!(b + &a, res);
        assert_eq!(b + a, res);

        res = a * b;
        assert_eq!(res, Pair(10, 6));
        assert_eq!(&mut a * &mut b, res);
        assert_eq!(&mut a * &b, res);
        assert_eq!(&mut a * b, res);
        assert_eq!(&a * &mut b, res);
        assert_eq!(&a * &b, res);
        assert_eq!(&a * b, res);
        assert_eq!(a * &mut b, res);
        assert_eq!(a * &b, res);
        assert_eq!(&mut b * &mut a, res);
        assert_eq!(&mut b * &a, res);
        assert_eq!(&mut b * a, res);
        assert_eq!(&b * &mut a, res);
        assert_eq!(&b * &a, res);
        assert_eq!(&b * a, res);
        assert_eq!(b * &mut a, res);
        assert_eq!(b * &a, res);
        assert_eq!(b * a, res);

        res = a & b;
        assert_eq!(res, Pair(0, 2));
        assert_eq!(&mut a & &mut b, res);
        assert_eq!(&mut a & &b, res);
        assert_eq!(&mut a & b, res);
        assert_eq!(&a & &mut b, res);
        assert_eq!(&a & &b, res);
        assert_eq!(&a & b, res);
        assert_eq!(a & &mut b, res);
        assert_eq!(a & &b, res);
        assert_eq!(&mut b & &mut a, res);
        assert_eq!(&mut b & &a, res);
        assert_eq!(&mut b & a, res);
        assert_eq!(&b & &mut a, res);
        assert_eq!(&b & &a, res);
        assert_eq!(&b & a, res);
        assert_eq!(b & &mut a, res);
        assert_eq!(b & &a, res);
        assert_eq!(b & a, res);

        res = a | b;
        assert_eq!(res, Pair(7, 3));
        assert_eq!(&mut a | &mut b, res);
        assert_eq!(&mut a | &b, res);
        assert_eq!(&mut a | b, res);
        assert_eq!(&a | &mut b, res);
        assert_eq!(&a | &b, res);
        assert_eq!(&a | b, res);
        assert_eq!(a | &mut b, res);
        assert_eq!(a | &b, res);
        assert_eq!(&mut b | &mut a, res);
        assert_eq!(&mut b | &a, res);
        assert_eq!(&mut b | a, res);
        assert_eq!(&b | &mut a, res);
        assert_eq!(&b | &a, res);
        assert_eq!(&b | a, res);
        assert_eq!(b | &mut a, res);
        assert_eq!(b | &a, res);
        assert_eq!(b | a, res);

        res = a ^ b;
        assert_eq!(res, Pair(7, 1));
        assert_eq!(&mut a ^ &mut b, res);
        assert_eq!(&mut a ^ &b, res);
        assert_eq!(&mut a ^ b, res);
        assert_eq!(&a ^ &mut b, res);
        assert_eq!(&a ^ &b, res);
        assert_eq!(&a ^ b, res);
        assert_eq!(a ^ &mut b, res);
        assert_eq!(a ^ &b, res);
        assert_eq!(&mut b ^ &mut a, res);
        assert_eq!(&mut b ^ &a, res);
        assert_eq!(&mut b ^ a, res);
        assert_eq!(&b ^ &mut a, res);
        assert_eq!(&b ^ &a, res);
        assert_eq!(&b ^ a, res);
        assert_eq!(b ^ &mut a, res);
        assert_eq!(b ^ &a, res);
        assert_eq!(b ^ a, res);
    }
}
