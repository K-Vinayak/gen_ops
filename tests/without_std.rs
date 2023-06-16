#![no_std]

mod test_nostd {
    use gen_ops::gen_ops;
    use core::ops::*;

    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);
    
    gen_ops!(
        <T>;
        types Complex<T> => Complex<T>;
        for - call |c:&Complex<T>| Complex(-c.0, -c.1); 
        where T: Neg<Output=T> + Copy
    );

    #[test]
    fn no_std_neg_test() {
        let c = Complex(1, -5);
        assert_eq!(-c, Complex(-1, 5));
    }
}