use gen_ops::*;
use std::ops::*;

mod method_call {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);
    
    impl<T> Complex<T> {
        fn negative(&self) -> Self where T: Neg<Output=T> + Copy {
            Complex(-self.0, -self.1)
        }
    }
    
    gen_ops!(
        <T>;
        types Complex<T> => Complex<T>;
        for - call Complex::<T>::negative; 
        where T: Neg<Output=T> + Copy
    );

    #[test]
    fn method_call_neg_test() {
        let c = Complex(1, -5);
        assert_eq!(-c, Complex(-1, 5));
    }
}

mod func_call {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Complex<T>(pub T, pub T);
    
    fn neg_complex<T>(c: &Complex<T>) -> Complex<T>
    where T: Neg<Output=T> + Copy {
        Complex(-c.0, -c.1)
    }
    
    gen_ops!(
        <T>;
        types Complex<T> => Complex<T>;
        for - call neg_complex; 
        where T: Neg<Output=T> + Copy
    );

    #[test]
    fn func_call_neg_test() {
        let c = Complex(1, -5);
        assert_eq!(-c, Complex(-1, 5));
    }
}

mod lifetime_params {
    use super::*;
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Button {
        id: u32,
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct ButtonPair<'a, 'b> {
        pub btn1: &'a Button,
        pub btn2: &'b Button
    }

    gen_ops!(
        <'a, 'b>;
        types &'a Button , &'b Button => ButtonPair<'a, 'b>;
        for + call |a: &'a Button, b: &'b Button| ButtonPair{btn1:a, btn2:b};
    );

    #[test]
    fn lifetime_params_test() {
        let b1 = Button{id:1};
        {
            let b2 = Button{id:2};
            let p = &b1 + &b2;

            assert_eq!(p.btn1.id, 1);
            assert_eq!(p.btn2.id, 2);
        }
    }
}

