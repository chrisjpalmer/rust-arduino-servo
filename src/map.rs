use core::ops::{AddAssign, MulAssign, SubAssign, Sub, Mul};
use ufmt::{uWrite, uDisplay, Formatter};

pub trait Mapable : AddAssign + MulAssign + SubAssign + Sub<Output = Self> + Mul<f32, Output = Self> + Into<f32> + Copy {
    type PrimitiveType : Into<Self>;
    fn as_primitive(&self) -> Self::PrimitiveType;
}

pub fn map<T>(input:T::PrimitiveType, input_low: T::PrimitiveType, input_high: T::PrimitiveType, output_low:T::PrimitiveType, output_high:T::PrimitiveType) -> T::PrimitiveType 
where T: Mapable {
    let mut input:T = input.into();
    let input_low:T = input_low.into();
    let input_high:T = input_high.into();
    let output_low:T = output_low.into();
    let output_high:T = output_high.into();

    let input_range:T = input_high - input_low;
    let output_range:T = output_high - output_low;

    input -= input_low;
    let fraction = (input.into() as f32) / (input_range.into() as f32);
    
    let mut output:T = output_range * fraction;
    output += output_low;
    output.as_primitive()
}

impl Into<U16> for u16 {
    fn into(self) -> U16 {
        U16(self)
    }
}

//U16
#[derive(Copy, Clone)]
pub struct U16(pub u16);

impl AddAssign<Self> for U16 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl MulAssign<Self> for U16 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl SubAssign<Self> for U16 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sub<Self> for U16 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        U16(self.0 - rhs.0)
    }
}

impl Mul<f32> for U16 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        U16(((self.0 as f32) * rhs) as u16)
    }
}

impl Into<f32> for U16 {
    fn into(self) -> f32 {
        self.0 as f32
    }
}

impl uDisplay for U16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error> 
    where
        W: uWrite + ?Sized
    {
        ufmt::uwrite!(f, "{}", self.0)
    }
}

impl Mapable for U16 {
    type PrimitiveType = u16;
    fn as_primitive(&self) -> u16 {
        self.0
    }
}