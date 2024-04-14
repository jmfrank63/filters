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
    pub fn low_pass(&mut self, input: &mut u16) {
        self.last_output = ((self.alpha as u32 * *input as u32)
            + (self.one_minus_alpha as u32 * self.last_output as u32)
            >> SCALE + 1) as u16;
        *input = self.last_output as u16;
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
    alpha: u16,
    one_minus_alpha: u16,
    last_output: u16,
}

impl HighPassFilter {
    pub fn new(alpha: u16) -> Self {
        #[cfg(not(target_arch = "avr"))]
        println!("Alpha: {}", alpha);
        let alpha = 1 + alpha;
        let one_minus_alpha = (2 << SCALE) - alpha - 1;
        Self {
            alpha,
            one_minus_alpha,
            last_output: 0,
        }
    }

    pub fn high_pass(&mut self, input: &mut u16) {
        // High pass filter is the difference between the input and the output of the low pass filter
        self.last_output = ((self.alpha as u32 * *input as u32)
            + (self.one_minus_alpha as u32 * self.last_output as u32) >> SCALE + 1) as u16;
        let diff = *input as i32 - self.last_output as i32;
        *input = ((diff + 512_i32) >> 1 )as u16;
    }

    pub fn set_alpha(&mut self, alpha: u16) {  
        let alpha = 1 + alpha; 
        self.one_minus_alpha = (2 << SCALE) - alpha - 1;
    }

    pub fn reset(&mut self) {
        self.last_output = 0;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdLowPassFilter {
        alpha: u16,
        one_minus_alpha: u16,
        last_output: u16,
    }

    impl StdLowPassFilter {
        fn new(alpha: u16) -> Self {
            let alpha = 1 + alpha;
            let one_minus_alpha = (2 << SCALE) - alpha - 1;
            Self {
                alpha,
                one_minus_alpha,
                last_output: 0,
            }
        }
        fn low_pass(&mut self, input: &mut u16) {
            self.last_output = ((self.alpha as u32 * *input as u32)
                + (self.one_minus_alpha as u32 * self.last_output as u32)
                >> SCALE + 1) as u16;
            *input = self.last_output as u16;
        }

        fn set_alpha(&mut self, alpha: u16) {
            self.alpha = 1 + alpha;
            self.one_minus_alpha = (2 << SCALE) - alpha - 1;
        }
    }

    #[test]
    fn test_low_pass_filter() {
        let mut std_lowpass_filter = StdLowPassFilter::new(0);
        let mut lowpass_filter = LowPassFilter::new(0);
        for alpha in 1..(2 << (SCALE - 1)) {
            std_lowpass_filter.set_alpha(alpha);
            lowpass_filter.set_alpha(alpha);
            for i in 0..1024_u16 {
                let mut input = i;
                let mut input_std = i;
                #[cfg(not(target_arch = "avr"))]
                println!(
                    "Before: fa: {} sfa:  {} flo: {} sflo: {} i: {}",
                    lowpass_filter.alpha,
                    std_lowpass_filter.alpha,
                    lowpass_filter.last_output,
                    std_lowpass_filter.last_output,
                    i
                );
                lowpass_filter.low_pass(&mut input);
                std_lowpass_filter.low_pass(&mut input_std);
                #[cfg(not(target_arch = "avr"))]
                println!(
                    "After: fa: {} sfa:  {} flo: {} sflo: {} i: {}",
                    lowpass_filter.alpha,
                    std_lowpass_filter.alpha,
                    lowpass_filter.last_output,
                    std_lowpass_filter.last_output,
                    i
                );
                assert_eq!(std_lowpass_filter.alpha, lowpass_filter.alpha.into());
                assert_eq!(
                    std_lowpass_filter.last_output,
                    lowpass_filter.last_output as u16
                );
                assert_eq!(input_std, input);
            }
        }
    }

    #[test]
    fn test_high_pass_filter() {
        let mut lowpass_filter = LowPassFilter::new(1);
        let mut highpass_filter = HighPassFilter::new(1);
        for alpha in 1..32 {
            lowpass_filter.set_alpha(alpha);
            highpass_filter.set_alpha(alpha);
            for i in 0..1024_u16 {
                let mut highpass_input = i;
                let mut lowpass_input = i;
                #[cfg(not(target_arch = "avr"))]
                println!(
                    "Before: alpha: {} lo: {} ho: {} i: {}",
                    alpha, lowpass_input, highpass_input, i
                );
                highpass_filter.high_pass(&mut highpass_input);
                lowpass_filter.low_pass(&mut lowpass_input);
                #[cfg(not(target_arch = "avr"))]
                println!(
                    "After: alpha: {} lo: {} ho: {} i: {}",
                    alpha, lowpass_input, highpass_input, i
                );
                #[cfg(not(target_arch = "avr"))]
                println!("{} - {} = {}", 1024, highpass_input, lowpass_input);
            }
        }
    }
}
