use num::Float;
use std::string::ToString;

use std::error::Error;
use csv::Writer;

use crate::quality_of_life::*;


pub fn write_vec_to_csv<F: Float+ToString>(result: Vec<F>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Box::new(Writer::from_path("data.csv")?);
    wtr.write_record(&["index", "val"])?;

    for i in 0..result.len()-1 {
        wtr.write_record(&[i.to_string(), result[i].to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn get_hist(data: Vec<f32>, n_buckets: usize) -> Vec<f32> {
    let min = partial_min(&data);
    let max = partial_max(&data);
    let bucket_size = (max - min) / n_buckets as f32;
    
    // let mut histogram = Vec::with_capacity(n_buckets);
    let mut histogram = vec![0.0; n_buckets];

    for x in data.iter() {
        // min + i*bucket size <= x < min + (i+1) * bucket_size
        // therefore i <= (x - min) / bucket_size
        let i = ((x - min) / bucket_size).floor() as usize;

        // In case x = max, put in highest bucket.
        let i = if i == 500 { 500 - 1 } else { i };
        histogram[i] = histogram[i] + 1.0;
    }
    histogram
}
