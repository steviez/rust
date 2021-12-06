// check-pass

#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]

trait Convert<T> {
    fn to(self) -> T;
}

impl<A, B> const Convert<B> for A where B: ~const From<A> {
    fn to(self) -> B {
        B::from(self)
    }
}

const FOO: fn() -> String = || "foo".to();

fn main() {}