use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

fn fourier(input: Vec<i32>) -> Vec<i32> {
    let mut planner = FFTplanner::new(false);
    let ftt = planner.plan_fft(input.len() as usize);
    let mut input = input
        .into_iter()
        .map(|x| Complex::new(x as f32, 0.0))
        .collect::<Vec<_>>();
    let mut output = vec![Complex::zero(); input.len()];
    ftt.process(&mut input, &mut output);
    output.into_iter().map(|x| x.re as i32).collect()
}
