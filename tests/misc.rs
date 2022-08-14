//purpose of these modules is only to check whether they compile

use gen_ops::*;
use std::ops::*;

mod const_gen {
    use super::*;

    #[derive(Copy, Clone)]
    struct IntSlice<const SZ: usize>( pub [i32; SZ] );

    impl<const SZ: usize> IntSlice<SZ> {
        pub fn add_each(&mut self, other: &Self) {
            self.0.iter_mut().zip(other.0).for_each(|(x, y)| *x += y);
        }
    }

    gen_ops!(
        <| const SZ: usize>;
        types IntSlice<SZ>, IntSlice<SZ>;
        for += call IntSlice::<SZ>::add_each;
    );
}

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
        for - call Complex::<T>::negative; //just to know it's possible
        where T: Neg<Output=T> + Copy
    );
    
    gen_ops_ex!(
        <T>;
        types ref Complex<T>, ref Complex<T> => Complex<T>;
        for + call |a: &Complex<T>, b: &Complex<T>| Complex(a.0+b.0, a.1+b.1);
        for - call |a: &Complex<T>, b: &Complex<T>| Complex(a.0-b.0, a.1-b.1);
        where T: Copy + Add<Output=T> + Sub<Output=T>
    );
}
