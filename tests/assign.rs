mod all_ops_own {
    use gen_ops::gen_ops;
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops!(
        types Pair, i32;
        for += call |a:&mut Pair, b:&i32| {
            a.0 += b;
            a.1 += b;
        };
        for -= call |a:&mut Pair, b:&i32| {
            a.0 -= b;
            a.1 -= b;
        };
        for /= call |a:&mut Pair, b:&i32| {
            a.0 /= b;
            a.1 /= b;
        };
        for *= call |a:&mut Pair, b:&i32| {
            a.0 *= b;
            a.1 *= b;
        };
        for %= call |a:&mut Pair, b:&i32| {
            a.0 %= b;
            a.1 %= b;
        };
        for &= call |a:&mut Pair, b:&i32| {
            a.0 &= b;
            a.1 &= b;
        };
        for |= call |a:&mut Pair, b:&i32| {
            a.0 |= b;
            a.1 |= b;
        };
        for ^= call |a:&mut Pair, b:&i32| {
            a.0 ^= b;
            a.1 ^= b;
        };
        for <<= call |a:&mut Pair, b:&i32| {
            a.0 <<= b;
            a.1 <<= b;
        };
        for >>= call |a:&mut Pair, b:&i32| {
            a.0 >>= b;
            a.1 >>= b;
        };
    );
    
    #[test]
    fn all_ops_own_asgn_test() {
        let mut c = Pair(10, 5);
        let b = 2;

        c += b;
        assert_eq!(c, Pair(12, 7));

        c -= b;
        assert_eq!(c, Pair(10, 5));

        c *= b;
        assert_eq!(c, Pair(20, 10));

        c /= b;
        assert_eq!(c, Pair(10, 5));

        c <<= b;
        assert_eq!(c, Pair(40, 20));

        c >>= b;
        assert_eq!(c, Pair(10, 5));

        c = Pair(10, 5); 
        c %= b;
        assert_eq!(c, Pair(0, 1));

        c = Pair(10, 5);
        c &= b;
        assert_eq!(c, Pair(2, 0));
        
        c =Pair(10, 5);
        c |= b;
        assert_eq!(c, Pair(10, 7));
        
        c =Pair(10, 5);
        c ^= b;
        assert_eq!(c, Pair(8, 7));
    }
}

mod all_ops_mut {

    use gen_ops::gen_ops;
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops!(
        types &mut Pair, i32;
        for += call |a:&mut Pair, b:&i32| {
            a.0 += b;
            a.1 += b;
        };
        for -= call |a:&mut Pair, b:&i32| {
            a.0 -= b;
            a.1 -= b;
        };
        for /= call |a:&mut Pair, b:&i32| {
            a.0 /= b;
            a.1 /= b;
        };
        for *= call |a:&mut Pair, b:&i32| {
            a.0 *= b;
            a.1 *= b;
        };
        for %= call |a:&mut Pair, b:&i32| {
            a.0 %= b;
            a.1 %= b;
        };
        for &= call |a:&mut Pair, b:&i32| {
            a.0 &= b;
            a.1 &= b;
        };
        for |= call |a:&mut Pair, b:&i32| {
            a.0 |= b;
            a.1 |= b;
        };
        for ^= call |a:&mut Pair, b:&i32| {
            a.0 ^= b;
            a.1 ^= b;
        };
        for <<= call |a:&mut Pair, b:&i32| {
            a.0 <<= b;
            a.1 <<= b;
        };
        for >>= call |a:&mut Pair, b:&i32| {
            a.0 >>= b;
            a.1 >>= b;
        };
    );

    #[test]
    fn all_ops_mut_asgn_test() {
        let mut a = Pair(10, 5);
        let b = 2;
        {
            let mut c;
            c = &mut a;
            c += b;
            assert_eq!(*c, Pair(12, 7));

            c -= b;
            assert_eq!(*c, Pair(10, 5));

            c *= b;
            assert_eq!(*c, Pair(20, 10));

            c /= b;
            assert_eq!(*c, Pair(10, 5));

            c <<= b;
            assert_eq!(*c, Pair(40, 20));

            c >>= b;
            assert_eq!(*c, Pair(10, 5));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c %= b;
            assert_eq!(*c, Pair(0, 1));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c &= b;
            assert_eq!(*c, Pair(2, 0));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c |= b;
            assert_eq!(*c, Pair(10, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c ^= b;
            assert_eq!(*c, Pair(8, 7));
        }
    }
}

mod all_ops_ex {
    use gen_ops::gen_ops_ex;
    
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Pair(pub i32, pub i32);

    gen_ops_ex!(
        types mut Pair, mut i32;
        for += call |a:&mut Pair, b:&i32| {
            a.0 += b;
            a.1 += b;
        };
        for -= call |a:&mut Pair, b:&i32| {
            a.0 -= b;
            a.1 -= b;
        };
        for /= call |a:&mut Pair, b:&i32| {
            a.0 /= b;
            a.1 /= b;
        };
        for *= call |a:&mut Pair, b:&i32| {
            a.0 *= b;
            a.1 *= b;
        };
        for %= call |a:&mut Pair, b:&i32| {
            a.0 %= b;
            a.1 %= b;
        };
        for &= call |a:&mut Pair, b:&i32| {
            a.0 &= b;
            a.1 &= b;
        };
        for |= call |a:&mut Pair, b:&i32| {
            a.0 |= b;
            a.1 |= b;
        };
        for ^= call |a:&mut Pair, b:&i32| {
            a.0 ^= b;
            a.1 ^= b;
        };
        for <<= call |a:&mut Pair, b:&i32| {
            a.0 <<= b;
            a.1 <<= b;
        };
        for >>= call |a:&mut Pair, b:&i32| {
            a.0 >>= b;
            a.1 >>= b;
        };
    );

    #[test]
    fn all_ops_mut_ex_asgn_test() {
        let mut a = Pair(10, 5);
        let mut b = 2;
        {
            let mut c;
            c = &mut a;
            c += &mut b;
            assert_eq!(*c, Pair(12, 7));
            c += &b;
            assert_eq!(*c, Pair(14, 9));
            c += b;
            assert_eq!(*c, Pair(16, 11));

            c -= &mut b;
            assert_eq!(*c, Pair(14, 9));
            c -= &b;
            assert_eq!(*c, Pair(12, 7));
            c -= b;
            assert_eq!(*c, Pair(10, 5));

            c *= &mut b;
            assert_eq!(*c, Pair(20, 10));
            c *= &b;
            assert_eq!(*c, Pair(40, 20));
            c *= b;
            assert_eq!(*c, Pair(80, 40));

            c /= &mut b;
            assert_eq!(*c, Pair(40, 20));
            c /= &b;
            assert_eq!(*c, Pair(20, 10));
            c /= b;
            assert_eq!(*c, Pair(10, 5));

            c <<= &mut b;
            assert_eq!(*c, Pair(40, 20));
            c <<= &b;
            assert_eq!(*c, Pair(160, 80));
            c <<= b;
            assert_eq!(*c, Pair(640, 320));

            c >>= &mut b;
            assert_eq!(*c, Pair(160, 80));
            c >>= &b;
            assert_eq!(*c, Pair(40, 20));
            c >>= b;
            assert_eq!(*c, Pair(10, 5));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c %= &mut b;
            assert_eq!(*c, Pair(0, 1));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c %= &b;
            assert_eq!(*c, Pair(0, 1));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c %= b;
            assert_eq!(*c, Pair(0, 1));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c &= &mut b;
            assert_eq!(*c, Pair(2, 0));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c &= &b;
            assert_eq!(*c, Pair(2, 0));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c &= b;
            assert_eq!(*c, Pair(2, 0));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c |= &mut b;
            assert_eq!(*c, Pair(10, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c |= &b;
            assert_eq!(*c, Pair(10, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c |= b;
            assert_eq!(*c, Pair(10, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c ^= &mut b;
            assert_eq!(*c, Pair(8, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c ^= &b;
            assert_eq!(*c, Pair(8, 7));
        }
        a = Pair(10, 5);
        {
            let mut c;
            c = &mut a;
            c ^= b;
            assert_eq!(*c, Pair(8, 7));
        }
    }


    #[test]
    fn all_ops_own_ex_asgn_test() {
        let mut c = Pair(10, 5);
        let mut b = 2;

        c += &mut b;
        assert_eq!(c, Pair(12, 7));
        c += &b;
        assert_eq!(c, Pair(14, 9));
        c += b;
        assert_eq!(c, Pair(16, 11));

        c -= &mut b;
        assert_eq!(c, Pair(14, 9));
        c -= &b;
        assert_eq!(c, Pair(12, 7));
        c -= b;
        assert_eq!(c, Pair(10, 5));

        c *= &mut b;
        assert_eq!(c, Pair(20, 10));
        c *= &b;
        assert_eq!(c, Pair(40, 20));
        c *= b;
        assert_eq!(c, Pair(80, 40));

        c /= &mut b;
        assert_eq!(c, Pair(40, 20));
        c /= &b;
        assert_eq!(c, Pair(20, 10));
        c /= b;
        assert_eq!(c, Pair(10, 5));

        c <<= &mut b;
        assert_eq!(c, Pair(40, 20));
        c <<= &b;
        assert_eq!(c, Pair(160, 80));
        c <<= b;
        assert_eq!(c, Pair(640, 320));

        c >>= &mut b;
        assert_eq!(c, Pair(160, 80));
        c >>= &b;
        assert_eq!(c, Pair(40, 20));
        c >>= b;
        assert_eq!(c, Pair(10, 5));

        c = Pair(10, 5);
        c %= &mut b;
        assert_eq!(c, Pair(0, 1));
        c = Pair(10, 5);
        c %= &b;
        assert_eq!(c, Pair(0, 1));
        c = Pair(10, 5);
        c %= b;
        assert_eq!(c, Pair(0, 1));

        c = Pair(10, 5);
        c &= &mut b;
        assert_eq!(c, Pair(2, 0));
        c = Pair(10, 5);
        c &= &b;
        assert_eq!(c, Pair(2, 0));
        c = Pair(10, 5);
        c &= b;
        assert_eq!(c, Pair(2, 0));
        
        c =Pair(10, 5);
        c |= &mut b;
        assert_eq!(c, Pair(10, 7));
        c =Pair(10, 5);
        c |= &b;
        assert_eq!(c, Pair(10, 7));
        c =Pair(10, 5);
        c |= b;
        assert_eq!(c, Pair(10, 7));
        
        c =Pair(10, 5);
        c ^= &mut b;
        assert_eq!(c, Pair(8, 7));
        c =Pair(10, 5);
        c ^= &b;
        assert_eq!(c, Pair(8, 7));
        c =Pair(10, 5);
        c ^= b;
        assert_eq!(c, Pair(8, 7));
    }
}
