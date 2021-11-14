
pub fn map(input:u16, input_low:u16, input_high:u16, output_low:u16, output_high:u16) -> (u16, u16)
 {
    let input_range = input_high-input_low;
    let output_range = output_high-output_low;

    // shfit input by the lower value
    // create a fraction between 1 and 0
    let input = input - input_low;
    let fraction = (input as u32 * 1000) / input_range as u32;

    // scale fraction by the output_range and shift output by lower value
    let output = ((fraction * output_range as u32) / 1000) as u16;
    let output = output + output_low;
    return (0, output);
}