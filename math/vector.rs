use std::fmt::{Debug, Display};
use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};
use serde::ser::SerializeSeq;
use serde::de::Visitor;



pub type Vec3 = Vector<3, f32>;


impl Serialize for Vec3
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer 
    {
        let buf = self.buffer();
        let mut seq = serializer.serialize_seq(Some(buf.len()))?;

        for element in buf
        {
            seq.serialize_element(element)?;
        }

        seq.end()
    }
}


impl<'de> Deserialize<'de> for Vec3
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_seq(Vec3Visitor)
    }
}


struct Vec3Visitor;
impl<'de> Visitor<'de> for Vec3Visitor
{
    type Value = Vec3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        formatter.write_str("a sequence of 3 floats")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: serde::de::SeqAccess<'de>,
    {
        let mut parts = vec![];

        while let Some(part) = seq.next_element()?
        {
            parts.push(part);
        }

        if parts.len() != 3
        {
            return Err(serde::de::Error::custom("Expected exaclty 3 components"));
        }
        
        Ok(Vec3::new_3d(parts[0], parts[1] , parts[2]))
    }
}


pub struct Vector<const M: usize, T>
{
    buffer: [T; M],
}


  ////////////////////////////////////////////////////////////////////////////
 // Common Trait Implementations  ///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

impl<const M: usize, T: Copy> Copy for Vector<M, T> {}
impl<const M: usize, T: Clone> Clone for Vector<M, T>
{
    fn clone(&self) -> Self
    {
        Self {buffer: self.buffer.clone()}
    }
}


impl<const M: usize, T: Debug> Debug for Vector<M, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "Vector<{:?}>", self.buffer)
    }
}


impl<const M: usize, T: Display + Debug> Display for Vector<M, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "Vector<{:?}>", self.buffer)
    }
}


impl <const M: usize, T: Default + Copy> Default for Vector<M, T>
{
    fn default() -> Self 
    {
        Self {buffer: [T::default(); M]}
    }
}


impl<const M: usize, T: Eq> Eq for Vector<M, T> {}
impl<const M: usize, T: PartialEq> PartialEq for Vector<M, T>
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.buffer == other.buffer    
    }
}


  //////////////////////////////////////////////////////////////////////////
 // Common Math Operations  ///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

#[allow(clippy::needless_range_loop)]
impl<const M: usize, T: Add<Output=T> + Default + Copy> Vector<M, T>
{
    pub fn add_vec(&self, rhs: &Self) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] + rhs.buffer[i];
        }

        Self {buffer}
    }

    pub fn add_scalar(&self, rhs: T) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] + rhs;
        }

        Self {buffer}
    }

    pub fn add_vec_inplace(&mut self, rhs: &Self)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] + rhs.buffer[i];
        }
    }

    pub fn add_scalar_inplace(&mut self, rhs: T)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] + rhs;
        }
    }
}


#[allow(clippy::needless_range_loop)]
impl<const M: usize, T: Sub<Output=T> + Default + Copy> Vector<M, T>
{
    pub fn sub_vec(&self, rhs: &Self) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] - rhs.buffer[i];
        }

        Self {buffer}
    }

    pub fn sub_scalar(&self, rhs: T) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] - rhs;
        }

        Self {buffer}
    }

    pub fn sub_vec_inplace(&mut self, rhs: &Self)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] - rhs.buffer[i];
        }
    }

    pub fn sub_scalar_inplace(&mut self, rhs: T)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] - rhs;
        }
    }
}


#[allow(clippy::needless_range_loop)]
impl<const M: usize, T: Mul<Output=T> + Default + Copy> Vector<M, T>
{
    pub fn mul_scalar(&self, rhs: T) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] * rhs;
        }

        Self {buffer}
    }

    pub fn mul_scalar_inplace(&mut self, rhs: T)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] * rhs;
        }
    }
}


#[allow(clippy::needless_range_loop)]
impl<const M: usize, T: Div<Output=T> + Default + Copy> Vector<M, T>
{
    pub fn div_scalar(&self, rhs: T) -> Self
    {
        let mut buffer = [T::default(); M];

        for i in 0..M
        {
            buffer[i] = self.buffer[i] / rhs;
        }

        Self {buffer}
    }

    pub fn div_scalar_inplace(&mut self, rhs: T)
    {
        for i in 0..M
        {
            self.buffer[i] = self.buffer[i] / rhs;
        }
    }
}


impl<const M: usize, T: Add<Output=T> + Mul<Output=T> + Default + Copy> Vector<M, T>
{
    pub fn dot(&self, rhs: &Self) -> T
    {
        let mut sum = T::default();

        for i in 0..M
        {
            sum = sum + (self.buffer[i] * rhs.buffer[i])
        }
        
        sum
    }
}


   //////////////////////////////////////////////////////////////////////////
  // Special Math Operations                                               /
 // These Require conversion to floating points                           /   
//////////////////////////////////////////////////////////////////////////

#[allow(clippy::needless_range_loop)]
impl<const M: usize, T: Default + Into<f64> + Copy> Vector<M, T>
{
    pub fn lensq(&self) -> f64
    {
        let mut sum = 0f64;

        for n in self.buffer
        {
            let n: f64 = n.into();
            sum += n*n;
        }

        sum
    }

    pub fn len(&self) -> f64
    {
        self.lensq().sqrt()
    }

    pub fn normalize(&self) -> Vector<M, f64>
    {
        let mut buffer = [0f64; M];
        let len = self.len();

        for i in 0..M
        {
            buffer[i] = self.buffer[i].into() / len;
        }

        Vector {buffer}
    }
}


  //////////////////////////////////////////////////////////////////////////
 // Constructors            ///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

impl<const M: usize, T> Vector<M, T> where [T; M]: for <'a> TryFrom<&'a[T]>
{
    pub const fn new(buffer: [T; M]) -> Self
    {
        Self {buffer}
    }

    pub fn from_slice(buffer: &[T]) -> Option<Self>
    {
        let buffer: [T; M] = match buffer.try_into()
        {
            Ok(buffer) => buffer,
            Err(_) => return None,
        };

        Some(Self{buffer})
    }

    pub const fn buffer(&self) -> &[T]
    {
        &self.buffer
    }
}


impl<T> Vector<2, T>
{
    pub const fn new_2d(x: T, y: T) -> Self
    {
        Self {buffer: [x, y]}
    }
}


impl<T> Vector<3, T>
{
    pub const fn new_3d(x: T, y: T, z: T) -> Self
    {
        Self {buffer: [x, y, z]}
    }

    pub const fn new_rgb(r: T, g: T, b: T) -> Self
    {
        Self {buffer: [r, g, b]}
    }
}

impl<T> Vector<4, T>
{
    pub const fn new_rgba(r: T, g: T, b: T, a: T) -> Self
    {
        Self {buffer: [r, g, b, a]}
    }
}


  //////////////////////////////////////////////////////////////////////////
 // Shorthands              ///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////
impl<T> Vector<2, T>
{
    pub fn x(&self) -> &T
    {
        &self.buffer[0]
    }

    pub fn y(&self) -> &T
    {
        &self.buffer[1]
    }

    pub fn set_x(&mut self, x: T)
    {
        self.buffer[0] = x;
    }

    pub fn set_y(&mut self, y: T)
    {
        self.buffer[1] = y;
    }
}


impl<T> Vector<3, T>
{
    pub fn x(&self) -> &T
    {
        &self.buffer[0]
    }

    pub fn y(&self) -> &T
    {
        &self.buffer[1]
    }

    pub fn z(&self) -> &T
    {
        &self.buffer[2]
    }

    pub fn set_x(&mut self, x: T)
    {
        self.buffer[0] = x;
    }

    pub fn set_y(&mut self, y: T)
    {
        self.buffer[1] = y;
    }

    pub fn set_z(&mut self, z: T)
    {
        self.buffer[2] = z;
    }

    pub fn r(&self) -> &T
    {
        &self.buffer[0]
    }

    pub fn g(&self) -> &T
    {
        &self.buffer[1]
    }

    pub fn b(&self) -> &T
    {
        &self.buffer[2]
    }

    pub fn set_r(&mut self, r: T)
    {
        self.buffer[0] = r;
    }

    pub fn set_g(&mut self, g: T)
    {
        self.buffer[1] = g;
    }

    pub fn set_b(&mut self, b: T)
    {
        self.buffer[2] = b;
    }
}


impl<T> Vector<4, T>
{
    pub fn r(&self) -> &T
    {
        &self.buffer[0]
    }

    pub fn g(&self) -> &T
    {
        &self.buffer[1]
    }

    pub fn b(&self) -> &T
    {
        &self.buffer[2]
    }

    pub fn a(&self) -> &T
    {
        &self.buffer[3]
    }

    pub fn set_r(&mut self, r: T)
    {
        self.buffer[0] = r;
    }

    pub fn set_g(&mut self, g: T)
    {
        self.buffer[1] = g;
    }

    pub fn set_b(&mut self, b: T)
    {
        self.buffer[2] = b;
    }
    
    pub fn set_a(&mut self, a: T)
    {
        self.buffer[3] = a;
    }
}

