use serde::ser::Serialize;

use csv::Writer;
use std::error::Error;

use crate::quality_of_life::*;

pub fn write_vec_to_csv<T: Serialize>(vals: Vec<T>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    for val in vals.iter() {
        wtr.serialize(val)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn get_hist(data: Vec<f64>, n_buckets: usize) -> (Vec<f64>, Vec<usize>) {
    let min = partial_min(&data);
    let max = partial_max(&data);

    let bucket_size = (max - min) / n_buckets as f64;

    if bucket_size == 0.0 {
        return (vec![min], vec![data.len()]);
    }

    let mut histogram = vec![0; n_buckets];

    for x in data.iter() {
        // min + i*bucket size <= x < min + (i+1) * bucket_size
        // therefore i <= (x - min) / bucket_size
        // In case x = max, the bucket index will be n+1
        // Put it in bucket n instead.
        let i = ((x - min) / bucket_size).floor() as usize;
        let i = if i == n_buckets { n_buckets - 1 } else { i };

        histogram[i] += 1;
    }

    let bins: Vec<f64> = (0..n_buckets)
        .map(|i| min + (i as f64 + 0.5) * bucket_size)
        .collect();

    (bins, histogram)
}
