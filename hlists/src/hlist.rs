// A heterogenous list of channels
// See https://github.com/lloydmeta/frunk
// and https://github.com/Sgeo/hlist

use std::ops::Add;

/// An empty `HList`
#[derive(Clone, Copy, Debug)]
pub struct HNil;

/// Represents the most basic non-empty HList. Its value is held in `head`
/// while its tail is another HList.
#[derive(Clone, Copy, Debug)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

/// A marker trait that `Nil` and `Cons<H, T>` satisfies.
/// Provides the `prepend()` method
pub trait HList: Sized {

    /// Length
    const LEN: usize;

    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    fn static_len(&self) -> usize;

    /// Consumes the `HList`, and returns a new HList with `item` at the beginning.
    fn prepend<N>(self, item: N) -> HCons<N, Self> {
        HCons { head: item, tail: self}
    }
    
}

impl HList for HNil {
    const LEN: usize = 0;
    fn static_len(&self) -> usize {
        Self::LEN
    }
}

impl<H, T: HList> HList for HCons<H, T> {
    const LEN: usize = 1 + <T as HList>::LEN;
    fn static_len(&self) -> usize {
        Self::LEN
    }
}

impl<H, T> HCons<H, T> {
    /// Returns the head of the list and the tail of the list as a tuple2.
    /// The original list is consumed
    pub fn pop(self) -> (H, T) {
        (self.head, self.tail)
    }
}


impl<RHS> Add<RHS> for HNil {
    type Output = RHS;
    
    fn add(self, rhs: RHS) -> RHS {
        rhs
    }
}

impl<H, T, RHS> Add<RHS> for HCons<H, T>
where
    T: Add<RHS>,
    RHS: HList,
{
    type Output = HCons<H, <T as Add<RHS>>::Output>;

    fn add(self, rhs: RHS) -> Self::Output {
        HCons {
            head: self.head,
            tail: self.tail + rhs,
        }
    }
}








/// Trait for transforming an HList into a nested tuple.
pub trait IntoTuple2 {
    /// The 0 element in the output tuple
    type HeadType;

    /// The 1 element in the output tuple
    type TailOutput;

    /// Turns an HList into nested Tuple2s
    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput);
}

impl<T1, T2> IntoTuple2 for HCons<T1, HCons<T2, HNil>> {
    type HeadType = T1;
    type TailOutput = T2;

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.head)
    }
}

impl<T, Tail> IntoTuple2 for HCons<T, Tail>
where
    Tail: IntoTuple2,
{
    type HeadType = T;
    type TailOutput = (
        <Tail as IntoTuple2>::HeadType,
        <Tail as IntoTuple2>::TailOutput,
    );

    fn into_tuple2(self) -> (Self::HeadType, Self::TailOutput) {
        (self.head, self.tail.into_tuple2())
    }
}
