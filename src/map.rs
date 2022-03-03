use core::ops::{AddAssign, MulAssign, SubAssign, DivAssign, Sub, Mul};
use ufmt::{uWrite, uDisplay, Formatter};

pub trait Mapable : AddAssign + MulAssign + SubAssign + Sub<Output = Self> + Mul<f32, Output = Self> + Into<f32> + Copy {}

pub fn map<T>(input:T, input_low: T, input_high: T, output_low:T, output_high:T) -> T 
where T: Mapable {
    let mut input = input;
    let input_range = input_high - input_low;
    let output_range = output_high - output_low;

    input -= input_low;
    let fraction = (input.into() as f32) / (input_range.into() as f32);
    
    let mut output:T = output_range * fraction;
    output += output_low;
    output
}

//U16
#[derive(Copy, Clone)]
pub struct U16(pub u16);

impl U16 {
    pub fn as_value(&self) -> u16 {
        self.0
    }
}

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

impl DivAssign<Self> for U16 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
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
        ufmt::uwriteln!(f, "{}", self.0)
    }
}

impl Mapable for U16 {}