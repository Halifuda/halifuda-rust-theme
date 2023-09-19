//! Mod comments.
//!

extern crate std;

use core::ops::Deref;
use std::io::Result;

const N: usize = 10;

/// Doc comments.
/// Local type [`ST`].
/// Local trait [`TR`].
/// Local macro [`macro_name`].
/// Local func [`func_name`].
/// Local const [`N`].
/// Std type [`Result`], [`usize`].
/// Std trait [`Deref`].
/// Std macro [`Debug`], [`vec`].
/// Std variable [`None`].
#[derive(Debug)]
struct ST<const N: usize>([usize; N]);

enum E {
    EA,
}

trait TR {}
trait GTR<T: TR> {}

impl<const N: usize> TR for ST<N> {}
impl<const N: usize> GTR<ST<N>> for usize {}

impl<const N: usize> Deref for ST<N> {
    type Target = [usize];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn func_name(arg: usize) -> usize {
    arg + 1
}

macro_rules! macro_name {
    ($ty:tt) => {
        $ty.to_string()
    };
}

fn main() -> Result<()> {
    let mut a = func_name(N);
    a += 1;
    let mut sub = sub::Sub::new(&mut a);
    sub.set(0x22);
    println!("{}", macro_name! {(sub.get())});
    Ok(())
}

pub mod sub {
    pub(super) struct Sub<'a>(&'a mut usize);
    impl<'a> Sub<'a> {
        pub(super) fn new(a: &'a mut usize) -> Self {
            Self(a)
        }
        pub(super) fn get(&self) -> &usize {
            self.0
        }
        pub(super) fn set(&mut self, a: usize) {
            *self.0 = a;
        }
    }
}
