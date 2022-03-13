use core::ops::{AddAssign, MulAssign, SubAssign, Sub};
use ufmt::{uWrite, uDisplay, Formatter};

pub trait Mapable : AddAssign + MulAssign + SubAssign + Sub<Output = Self> + Copy + Into<f32> + From<f32> {
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
    
    let mut output:T = From::from((output_range.into() as f32) * fraction);
    output += output_low;
    output.as_primitive()
}


macro_rules! implement_mappable {
    ($mappable:ident, $prim:ty) => {
        #[derive(Copy, Clone)]
        pub struct $mappable(pub $prim);

        impl Into<$mappable> for $prim {
            fn into(self) -> $mappable {
                $mappable(self)
            }
        }

        impl AddAssign<Self> for $mappable {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        
        impl MulAssign<Self> for $mappable {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }
        
        impl SubAssign<Self> for $mappable {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
        
        impl Sub<Self> for $mappable {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                $mappable(self.0 - rhs.0)
            }
        }
        
        impl Into<f32> for $mappable {
            fn into(self) -> f32 {
                self.0 as f32
            }
        }

        impl From<f32> for $mappable {
            fn from(v:f32) -> Self {
                $mappable(v as $prim)
            }
        }
        
        impl uDisplay for $mappable {
            fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error> 
            where
                W: uWrite + ?Sized
            {
                ufmt::uwrite!(f, "{}", self.0)
            }
        }
        
        impl Mapable for $mappable {
            type PrimitiveType = $prim;
            fn as_primitive(&self) -> $prim {
                self.0
            }
        }

    };
}

implement_mappable!(U32, u32);
implement_mappable!(U16, u16);


#[cfg(test)]
mod tests {
    #[test]
    fn test_map() {

    }
}