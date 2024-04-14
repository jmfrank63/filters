use filters::{HighPassFilter, LowPassFilter};

fn main() {
    let mut lowpass_filter = LowPassFilter::new(1);
    let mut highpass_filter = HighPassFilter::new(1);
    let samples: Vec<u16> = (0..1024)
        .map(|x| {
            let sin_value = (x as f64 / 1024.0 * 2.0 * std::f64::consts::PI).sin();
            // Scale the sine values from [-1, 1] to [0, u16::MAX]
            ((sin_value * 0.5 + 0.5) * 1024.0) as u16
        })
        .collect();
    let mut lowpass_input;
    let mut highpass_input;
    for i in 0..1024 {
        lowpass_input = samples[i];
        highpass_input = samples[i];
        print!("IL: {}, IH: {} ", lowpass_input, lowpass_input);
        lowpass_filter.low_pass(&mut lowpass_input);
        highpass_filter.high_pass(&mut highpass_input);
        print!("OL: {}, OH: {} __", lowpass_input, highpass_input);
        if i % 8 == 7 {
            println!();
        } else {
            print!(", ");
        }
    }
}
