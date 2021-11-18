use fixed::types::{*};

pub fn map(input:u16, input_low: u16, input_high: u16, output_low:u16, output_high:u16) -> u16 {
    let input_range = input_high - input_low;
    let output_range = output_high - output_low;
    let mut input = U10F6::from_num(input);
    input -= U10F6::from_num(input_low);
    input /= input_range;

    let mut output = input;
    output *= output_range;
    output += U10F6::from_num(output_low);
    output.to_num()
}