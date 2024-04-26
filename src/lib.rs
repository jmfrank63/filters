#![cfg_attr(target_arch = "avr", no_std)]

const SCALE: u16 = 6;

pub struct LowPassFilter {
    alpha: u16,
    one_minus_alpha: u16,
    last_output: u16,
}

impl LowPassFilter {
    pub fn new(alpha: u16) -> Self {
        let alpha = 1 + alpha;
        let one_minus_alpha = (2 << SCALE) - alpha - 1;
        Self {
            alpha,
            one_minus_alpha,
            last_output: 0,
        }
    }
    pub fn low_pass(&mut self, input: u16) -> u16 {
        self.last_output = ((self.alpha as u32 * input as u32)
            + (self.one_minus_alpha as u32 * self.last_output as u32)
            >> SCALE + 1) as u16;
        self.last_output as u16
    }

    pub fn set_alpha(&mut self, alpha: u16) {
        self.alpha = 1 + alpha;
        self.one_minus_alpha = (2 << SCALE) - alpha - 1;
    }

    pub fn reset(&mut self) {
        self.last_output = 0;
    }
}

pub struct HighPassFilter {
    low_pass: LowPassFilter,
}

impl HighPassFilter {
    pub fn new(alpha: u16) -> Self {
        let low_pass = LowPassFilter::new(alpha);
        Self {
            low_pass,
        }
    }

    pub fn high_pass(&mut self, input: u16) -> u16 {
        // High pass filter is the difference between the input and the output of the low pass filter
        let output = (self.low_pass.low_pass(input) as u32) << 16;
        let input = (input as u32) << 16;
        let diff = (input - output) >> 16;
        diff as u16
    }

    pub fn set_alpha(&mut self, alpha: u16) {  
        self.low_pass.set_alpha(64 - alpha);
    }

    pub fn reset(&mut self) {
        self.low_pass.reset()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_pass_filter() {
        let mut lowpass_filter = LowPassFilter::new(0);
        for alpha in 1..(2 << (SCALE - 1)) {
            #[cfg(not(target_arch = "avr"))]
            println!("alpha: {alpha}");
            lowpass_filter.set_alpha(alpha);
            for i in 0..1024_u16 {
                let output = lowpass_filter.low_pass(i);
                assert!(output <= i);
            }
            lowpass_filter.reset();
        }
    }

    #[test]
    fn test_high_pass_filter() {
        let mut lowpass_filter = LowPassFilter::new(1);
        let mut highpass_filter = HighPassFilter::new(1);

        for alpha in 1..(2 << (SCALE - 1)) {
            lowpass_filter.set_alpha(alpha);
            highpass_filter.set_alpha(alpha);
            let mut h_output = 0;
            let mut l_output = 0;
            for i in 0..1024_u16 {
                l_output = lowpass_filter.low_pass(i);
                h_output = highpass_filter.high_pass(i);
            }
            #[cfg(not(target_arch = "avr"))]
            println!("alpha {alpha} - l_output {l_output} h_output: {h_output}");
            lowpass_filter.reset();
            highpass_filter.reset();
        }
        
    }
}
