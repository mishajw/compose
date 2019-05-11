use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

/// Performs fourier transform on the input
pub fn fourier(input: &Vec<i32>) -> Vec<i32> {
    let mut planner = FFTplanner::new(false);
    let ftt = planner.plan_fft(input.len() as usize);

    assert!(!input.is_empty());
    let input_min = input.into_iter().min().unwrap();
    let input_max = input.into_iter().max().unwrap();
    let mut input = input
        .into_iter()
        .map(|x| (x - input_min) as f64 / (input_max - input_min) as f64 * 2.0 - 1.0)
        .map(|x| Complex::new(x, 0.0))
        .collect::<Vec<_>>();
    let mut output = vec![Complex::zero(); input.len()];

    ftt.process(&mut input, &mut output);
    output
        .into_iter()
        .map(|x| (x.re.powf(2.0) + x.im.powf(2.0)).sqrt())
        .map(|x| x * (input_max - input_min) as f64)
        .map(|x| x as i32)
        .collect()
}
