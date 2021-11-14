pub struct UDisplayF32 {
    pub value: f32,
    pub decimal_places: u16,
}

impl ufmt::uDisplay for UDisplayF32 {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        let mut value = self.value;
        f.write_char('a')?;

        // check if its in range
        if value > i32::MAX as f32 || value < i32::MIN as f32 {
            panic!("values above the range of i32 are not supported for UDisplayF32")
        }
        if self.decimal_places > 9 {
            panic!("more than 9 decimal places are not supported for UDisplayF32")
        }
        f.write_char('b')?;
        
        //deal with 0 case - simply print 0 for the 1st digit and any trailing 0s after the decimal place
        if value == 0.0 {
            f.write_char('0')?;

            if self.decimal_places == 0 {
                return Ok(())
            }
    
            // insert decimal place
            f.write_char('.')?;

            for _ in 0..self.decimal_places {
                f.write_char('0')?;
            }
            return Ok(())
        }

        // write the negative sign first
        if value < 0.0 {
            f.write_char('-')?;
            value = -value; // reverse it so we are dealing with positive from here on.
        }

        // we only support float range up to 10
        let integer_num =  value as u16;
        f.write_char('c')?;
        for digit in (0..10).rev() {
            let unit = pow(10, digit);
            f.write_char('d')?;
            // integer divide to move desired digit to the Oth digit position
            let v2 = integer_num / unit;
            f.write_char('e')?;
            // use modulo to remove all digits to the right of 0th digit
            let dig = v2 % 10;
            f.write_char('f')?;
            // transform that digit into as ascii character
            
            f.write_char(((dig as u8) + 48) as char)?;
            
        }

        if self.decimal_places == 0 {
            return Ok(())
        }

        // insert decimal place
        f.write_char('.')?;

        // shift the value by some number of decimal places to the left
        let shift_left = pow(10, self.decimal_places);
        let shifted = value * (shift_left as f32);
        let shifted = shifted as u16; // convert to integer
        for digit in (0..self.decimal_places).rev() {
            if let Some(c) = print_digit(shifted, digit) {
                f.write_char(c)?;
            }
        }
        
        Ok(())
    }
}

fn pow(v: u16, exp: u16) -> u16 {
    let mut v = v;
    for _ in 0..exp {
        v *= v;
    }
    return v;
}

// print_digit returns the digit in the integer value.
// 0 is the first digit before the decimal point. 1 is the next and so on.
// returns None if value doesn't have the digit because its too small
fn print_digit(value:u16, digit:u16) -> Option<char> {
    let unit = pow(10, digit);
    if unit > value {
        return None
    }

    // integer divide to move desired digit to the Oth digit position
    let value = value / unit;
    // use modulo to remove all digits to the right of 0th digit
    let dig = value % 10;
    // transform that digit into as ascii character
    Some((dig as u8 + 48) as char) // add 48 to get to ascii
}