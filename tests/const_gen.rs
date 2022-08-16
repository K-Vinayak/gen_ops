use gen_ops::*;

fn add_arr<const SZ:usize>(a: &[i32;SZ], b: &[i32;SZ]) -> [i32; SZ] {
    let mut ret = *a;
    for i in 0..SZ {
        ret[i] += b[i];
    }
    return ret;
}

fn add_assign_arr<const SZ:usize>(a: &mut [i32; SZ], b: &[i32;SZ]) {
    for i in 0..SZ {
        a[i] += b[i];
    }
}

fn neg_arr<const SZ:usize>(a: & [i32; SZ]) -> [i32; SZ] {
    let mut ret = [0; SZ];
    for i in 0..SZ {
        ret[i] = -a[i];
    }
    return ret;
}

mod const_gen_ncomm {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops!(
        <| const SZ: usize>;
        types IntArray<SZ>, IntArray<SZ> => IntArray<SZ>;
        for + call |a: &IntArray<SZ>, b: &IntArray<SZ>| { IntArray(add_arr(&a.0, &b.0)) };
    );

    gen_ops!(
        <| const SZ: usize>;
        types IntArray<SZ>, IntArray<SZ> ;
        for += call |a: &mut IntArray<SZ>, b: &IntArray<SZ>| { add_assign_arr(&mut a.0, &b.0) };
    );

    gen_ops!(
        <| const SZ: usize>;
        types IntArray<SZ> => IntArray<SZ>;
        for - call |a: &IntArray<SZ>| { IntArray(neg_arr(&a.0)) };
    );

    #[test]
    fn ncomm_add_const_gen_test() {
        let a = IntArray([10, 20, 30]);
        let b = IntArray([5, 3, 2]);
        assert_eq!(a+b, IntArray([15, 23, 32]));
    }

    #[test]
    fn add_assign_const_gen_test() {
        let mut a = IntArray([10, 20, 30]);
        let b = IntArray([5, 3, 2]);
        a += b;
        assert_eq!(a, IntArray([15, 23, 32]));
    }

    #[test]
    fn neg_const_gen_test() {
        let a = IntArray([10, 20, 30]);
        let b = -a;
        assert_eq!(b, IntArray([-10, -20, -30]));
    }
}

mod const_gen_bin_comm {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops_comm!(
        <| const SZ: usize>;
        types IntArray<SZ>, i32 => IntArray<SZ>;
        for + call |a: &IntArray<SZ>, b: &i32| { IntArray(add_arr(&a.0, &[*b; SZ])) };
    );

    #[test]
    fn comm_add_const_gen_test() {
        let a = IntArray([10, 20, 30]);
        assert_eq!(a+5, IntArray([15, 25, 35]));
        assert_eq!(5+a, IntArray([15, 25, 35]));
    }
}

mod const_gen_bin_ex {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops_ex!(
        <| const SZ: usize>;
        types ref IntArray<SZ>, i32 => IntArray<SZ>;
        for + call |a: &IntArray<SZ>, b: &i32| { IntArray(add_arr(&a.0, &[*b; SZ])) };
    );

    #[test]
    fn ncomm_ex_add_const_gen_test() {
        let a = IntArray([10, 20, 30]);
        assert_eq!(a + 5, IntArray([15, 25, 35]));
        assert_eq!(&a + 5, IntArray([15, 25, 35]));
    }
}

mod const_gen_bin_comm_ex {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops_comm_ex!(
        <| const SZ: usize>;
        types ref IntArray<SZ>, i32 => IntArray<SZ>;
        for + call |a: &IntArray<SZ>, b: &i32| { IntArray(add_arr(&a.0, &[*b; SZ])) };
    );

    #[test]
    fn comm_add_ex_const_gen_test() {
        let a = IntArray([10, 20, 30]);
        assert_eq!(a+5, IntArray([15, 25, 35]));
        assert_eq!(5+a, IntArray([15, 25, 35]));
        assert_eq!(&a+5, IntArray([15, 25, 35]));
        assert_eq!(5+&a, IntArray([15, 25, 35]));
    }
}

mod const_gen_asgn_ex {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops_ex!(
        <| const SZ: usize>;
        types mut IntArray<SZ>, IntArray<SZ> ;
        for += call |a: &mut IntArray<SZ>, b: &IntArray<SZ>| { add_assign_arr(&mut a.0, &b.0) };
    );

    #[test]
    fn add_assign_ex_const_gen_test() {
        let mut x = IntArray([10, 20, 30]);
        {
            let mut a = x.clone();
            let b = IntArray([5, 3, 2]);
            a += b;
            assert_eq!(a, IntArray([15, 23, 32]));
        }
        {
            let mut a = &mut x;
            let b = IntArray([5, 3, 2]);
            a += b;
            assert_eq!(x, IntArray([15, 23, 32]));
        }
    }
}

mod const_gen_un_ex {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct IntArray<const SZ: usize>( pub [i32; SZ] );

    gen_ops_ex!(
        <| const SZ: usize>;
        types mut IntArray<SZ> => IntArray<SZ>;
        for - call |a: &IntArray<SZ>| { IntArray(neg_arr(&a.0)) };
    );

    #[test]
    fn neg_const_gen_test() {
        let mut a = IntArray([10, 20, 30]);
        assert_eq!(-a, IntArray([-10, -20, -30]));
        {
            let b = &a;
            assert_eq!(-b, IntArray([-10, -20, -30]));
        }
        {
            let b = &mut a;
            assert_eq!(-b, IntArray([-10, -20, -30]));
        }
    }
}