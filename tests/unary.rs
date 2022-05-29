mod un_simple {
    use gen_ops::gen_ops;
    use std::ops::{Neg};

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Pair<T>(pub T, pub T);

    fn swap_pair<T>(p: &Pair<T>) -> Pair<T> where T:Copy {
        Pair(p.1, p.0)
    }

    gen_ops!(
        <T>; 
        types Pair<T> => Pair<T>;
        for - call |a:&Pair<T>| {
            Pair(-a.0, -a.1)
        };
        for ! call swap_pair;
        where T: Neg<Output=T> + Copy
    );

    #[test]
    fn not_test() {
        let a = Pair(1.5, -3.);
        assert_eq!(!a, Pair(-3., 1.5));
    }

    #[test]
    fn neg_test() {
        let a = Pair(1.5, -3.);
        assert_eq!(-a, Pair(-1.5, 3.));
    }
}

mod un_ex {
    use gen_ops::gen_ops_ex;
    use std::ops::{Neg};

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Pair<T>(pub T, pub T);

    fn swap_pair<T>(p: &Pair<T>) -> Pair<T> where T:Copy {
        Pair(p.1, p.0)
    }

    gen_ops_ex!(
        <T>; types mut Pair<T> => Pair<T>;
        for - call |a:&Pair<T>| {
            Pair(-a.0, -a.1)
        };
        for ! call swap_pair;
        where T: Neg<Output=T> + Copy
    );

    #[test]
    fn not_ex_test() {
        let mut a = Pair(1., 3.);
        
        let res = !a;
        assert_eq!(res, Pair(3., 1.));
        assert_eq!(!&a, res);
        assert_eq!(!&mut a, res);
    }

    #[test]
    fn neg_ex_test() {
        let mut a = Pair(1., 3.);
        
        let res = -a;
        assert_eq!(res, Pair(-1., -3.));
        assert_eq!(-&a, res);
        assert_eq!(-&mut a, res);
    }
}
