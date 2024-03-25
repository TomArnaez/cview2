pub type Histogram = Vec<u32>;

pub fn calculate_histogram<I>(vals: I, max_value: u32, num_bins: u32) -> Histogram
where
    I: IntoIterator<Item = u16>,
{
    let bin_size = (max_value + 1) / num_bins;

    vals.into_iter()
        .fold(vec![0; num_bins as usize], |mut histogram, value| {
            let bin_index = (value as u32 / bin_size) as usize;
            histogram[bin_index] += 1;
            histogram
        })
}

pub fn calculate_std<I>(vals: I) -> f64
where
    I: IntoIterator<Item = u16>,
{
    let vals: Vec<u16> = vals.into_iter().collect();
    if vals.is_empty() {
        return 0.0;
    }
    
    let mean = vals.iter().map(|&val| val as f64).sum::<f64>() / vals.len() as f64;
    let variance = vals.iter().map(|&val| {
        let diff = val as f64 - mean;
        diff * diff
    }).sum::<f64>() / vals.len() as f64;

    variance.sqrt()
}