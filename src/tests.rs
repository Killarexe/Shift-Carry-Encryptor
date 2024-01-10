use crate::{shift_carry_left, shift_carry_right, process_input};

#[test]
fn single_input() {
    let input: u8 = 0b01101110;
    let iterations: u8 = 2;
    let output: u8 = shift_carry_left(input, iterations);
    assert_eq!(output, 0b10111001);
    assert_eq!(input, shift_carry_right(output, iterations));
}

#[test]
fn single_input_with_inversion() {
    let input: u8 = 0b01101110;
    let iterations: u8 = 2;
    let output: u8 = !shift_carry_left(input, iterations);
    assert_eq!(output, !0b10111001);
    assert_eq!(input, shift_carry_right(!output, iterations));
}

#[test]
fn multiple_inputs() {
    let inputs: Vec<u8> = vec![0b10010101, 0b11101001, 0b11101011, 0b11000010];
    let iterations: u8 = 2;
    let outputs: Vec<u8> = process_input(inputs.clone(), iterations, true, false);
    assert_eq!(outputs, vec![0b01010110, 0b10100111, 0b10101111, 0b00001011]);
    assert_eq!(inputs, process_input(outputs, iterations, false, false));
}

#[test]
fn multiple_inputs_with_inversion() {
    let inputs: Vec<u8> = vec![0b10010101, 0b11101001, 0b11101011, 0b11000010];
    let iterations: u8 = 2;
    let outputs: Vec<u8> = process_input(inputs.clone(), iterations, true, true);
    assert_eq!(outputs, vec![!0b01010110, !0b10100111, !0b10101111, !0b00001011]);
    assert_eq!(inputs, process_input(outputs, iterations, false, true));
}
