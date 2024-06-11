use crate::Vector;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};


impl<const M: usize, T> Add<Self> for Vector<M, T>
    where T: Add<Output=T> + Copy + Default
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output 
    {
        self.add_vec(&rhs)
    }
}


impl<const M: usize, T> Sub<Self> for Vector<M, T>
    where T: Sub<Output=T> + Copy + Default
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output 
    {
        self.sub_vec(&rhs)
    }
}


impl<const M: usize, T> Add<T> for Vector<M, T>
    where T: Add<Output=T> + Copy+ Default
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output 
    {
        self.add_scalar(rhs)
    }
}


impl<const M: usize, T> Sub<T> for Vector<M, T>
    where T: Sub<Output=T> + Copy+ Default
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output 
    {
        self.sub_scalar(rhs)
    }
}


impl<const M: usize, T> Mul<T> for Vector<M, T>
    where T: Mul<Output=T> + Copy + Default
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output 
    {
        self.mul_scalar(rhs)
    }
}


impl<const M: usize, T> Div<T> for Vector<M, T>
    where T: Div<Output=T> + Copy+ Default
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output 
    {
        self.div_scalar(rhs)
    }
}


impl<const M: usize, T> AddAssign<Self> for Vector<M, T>
    where T: Add<Output=T> + Copy+ Default
{
    fn add_assign(&mut self, rhs: Self) 
    {
        self.add_vec_inplace(&rhs);
    }
}


impl<const M: usize, T> SubAssign<Self> for Vector<M, T>
    where T: Sub<Output=T> + Copy+ Default
{
    fn sub_assign(&mut self, rhs: Self) 
    {
        self.sub_vec_inplace(&rhs);
    }
}


impl<const M: usize, T> AddAssign<T> for Vector<M, T>
    where T: Add<Output=T> + Copy + Default
{
    fn add_assign(&mut self, rhs: T) 
    {
        self.add_scalar_inplace(rhs);
    }
}


impl<const M: usize, T> SubAssign<T> for Vector<M, T>
    where T: Sub<Output=T> + Copy + Default
{
    fn sub_assign(&mut self, rhs: T) 
    {
        self.sub_scalar_inplace(rhs);
    }
}


impl<const M: usize, T> MulAssign<T> for Vector<M, T>
    where T: Mul<Output=T> + Copy + Default
{
    fn mul_assign(&mut self, rhs: T) 
    {
        self.mul_scalar_inplace(rhs);
    }
}


impl<const M: usize, T> DivAssign<T> for Vector<M, T>
    where T: Div<Output=T> + Copy + Default
{
    fn div_assign(&mut self, rhs: T) 
    {
        self.div_scalar_inplace(rhs);
    }
}

