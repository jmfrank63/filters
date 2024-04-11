#![cfg_attr(target_arch = "avr", no_std)]
pub struct LowPassFilter {
    alpha: u16,
    alpha_shift: u8,
    last_output: u16,
}

impl LowPassFilter {
    pub fn new(alpha: u16) -> Self {
        let alpha_bits = 16 - alpha.leading_zeros() as u8;
        let alpha_shift = if alpha_bits > 6 { alpha_bits - 6 } else { 0 };
        let alpha = alpha >> alpha_shift;
        Self {
            alpha,
            alpha_shift,
            last_output: 0,
        }
    }

    pub fn low_pass(&mut self, input: &mut u16) {
        let input_bits = 16 - input.leading_zeros() as u8;
        let last_output_bits = 16 - self.last_output.leading_zeros() as u8;

        let mut shift_amount_input = 0;
        if 6 + input_bits > 16 {
            shift_amount_input = 6 + input_bits - 16;
            *input >>= shift_amount_input;
        }

        let mut shift_amount_last_output = 0;
        if 6 + last_output_bits > 16 {
            shift_amount_last_output = 6 + last_output_bits - 16;
            self.last_output >>= shift_amount_last_output;
        }

        let alpha_input = (self.alpha * (*input >> shift_amount_input)) << self.alpha_shift;
        let one_minus_alpha_last_output = ((64 - self.alpha) * (self.last_output >> shift_amount_last_output)) << self.alpha_shift;

        self.last_output = (alpha_input + one_minus_alpha_last_output) >> (6 - shift_amount_input);
        *input = self.last_output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_pass_filter() {
        let mut filter = LowPassFilter::new(10);

        // Test with a step input
        let mut input = [0, 0, 0, 0, 0, 1023, 1023, 1023, 1023, 1023];
        let mut output = [0; 10];
        for (i, value) in input.iter_mut().enumerate() {
            filter.low_pass(value);
            output[i] = *value;
        }

        // The output should start at 0 and gradually increase to 1023
        assert!(output[0] < output[1]);
        assert!(output[1] < output[2]);
        assert!(output[2] < output[3]);
        assert!(output[3] < output[4]);
        assert!(output[4] < output[5]);
        assert!(output[5] < output[6]);
        assert!(output[6] < output[7]);
        assert!(output[7] < output[8]);
        assert!(output[8] < output[9]);

        // Test with a sinusoidal input
        let mut input = [512, 708, 866, 963, 1023, 963, 866, 708, 512, 316, 158, 61, 0, 61, 158, 316];
        let mut output = [0; 16];
        for (i, value) in input.iter_mut().enumerate() {
            filter.low_pass(value);
            output[i] = *value;
        }

        // The output should be a smoothed version of the input
        assert!(output[0] < output[1]);
        assert!(output[1] < output[2]);
        assert!(output[2] < output[3]);
        assert!(output[3] < output[4]);
        assert!(output[4] > output[5]);
        assert!(output[5] > output[6]);
        assert!(output[6] > output[7]);
        assert!(output[7] > output[8]);
    }
}
