use crate::{
    basis::{Basis, ZeroVect},
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
    traits::FatDot,
};
use core::ops::Add;
use generic_array::ArrayLength;
use typenum::{Bit, Len, Sum, Unsigned, B0, B1};

// -------------------------------------------------------------------------------------
// ZeroVect + ZeroVect
impl<M: Metric> Add<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn add(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
// ZeroVect + Mvect
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> Add<Mvect<A, M, F>>
    for ZeroVect<M>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn add(self, rhs: Mvect<A, M, F>) -> Self::Output {
        rhs
    }
}
// ZeroVect + Basis
impl<U: Unsigned, M: Metric, S: Bit> Add<Basis<U, M, S>> for ZeroVect<M> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn add(self, rhs: Basis<U, M, S>) -> Self::Output {
        rhs
    }
}
// Mvect + ZeroVect
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> Add<ZeroVect<M>>
    for Mvect<A, M, F>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn add(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
// Mvect + Basis
impl<A: BasisSet<M> + Len<Output: ArrayLength>, U: Unsigned, M: Metric, F: Field + FatDot<M>>
    Add<Basis<U, M, B0>> for Mvect<A, M, F>
where
    Self: Add<Mvect<ta![U], M, F>>,
{
    type Output = Sum<Mvect<A, M, F>, Mvect<ta![U], M, F>>;
    #[inline(always)]
    fn add(self, rhs: Basis<U, M, B0>) -> Self::Output {
        self + Mvect::<ta![U], M, F>::from(rhs)
    }
}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, U: Unsigned, M: Metric, F: Field + FatDot<M>>
    Add<Basis<U, M, B1>> for Mvect<A, M, F>
where
    Self: Add<Mvect<ta![U], M, F>>,
{
    type Output = Sum<Mvect<A, M, F>, Mvect<ta![U], M, F>>;
    #[inline(always)]
    fn add(self, rhs: Basis<U, M, B1>) -> Self::Output {
        self + Mvect::<ta![U], M, F>::from(rhs)
    }
}
// Basis + Mvect
impl<A: BasisSet<M> + Len<Output: ArrayLength>, U: Unsigned, M: Metric, F: Field + FatDot<M>>
    Add<Mvect<A, M, F>> for Basis<U, M, B0>
where
    Self: Into<Mvect<ta![U], M, F>>,
    Mvect<ta![U], M, F>: Add<Mvect<A, M, F>>,
{
    type Output = Sum<Mvect<ta![U], M, F>, Mvect<A, M, F>>;
    #[inline(always)]
    fn add(self, rhs: Mvect<A, M, F>) -> Self::Output {
        Mvect::<ta![U], M, F>::from(self) + rhs
    }
}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, U: Unsigned, M: Metric, F: Field + FatDot<M>>
    Add<Mvect<A, M, F>> for Basis<U, M, B1>
where
    Self: Into<Mvect<ta![U], M, F>>,
    Mvect<ta![U], M, F>: Add<Mvect<A, M, F>>,
{
    type Output = Sum<Mvect<ta![U], M, F>, Mvect<A, M, F>>;
    #[inline(always)]
    fn add(self, rhs: Mvect<A, M, F>) -> Self::Output {
        Mvect::<ta![U], M, F>::from(self) + rhs
    }
}
// Basis + ZeroVect
impl<U: Unsigned, M: Metric, S: Bit> Add<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn add(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
// Basis + Basis
impl<U: Unsigned, M: Metric> Add<Basis<U, M, B0>> for Basis<U, M, B1> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn add(self, _: Basis<U, M, B0>) -> Self::Output {
        Self::Output::default()
    }
}
impl<U: Unsigned, M: Metric> Add<Basis<U, M, B1>> for Basis<U, M, B0> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn add(self, _: Basis<U, M, B1>) -> Self::Output {
        Self::Output::default()
    }
}
