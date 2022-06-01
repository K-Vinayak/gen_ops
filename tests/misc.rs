//these modules are only to check whether it compiles

use gen_ops::*;

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