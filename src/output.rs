use std::string::ToString;

use std::error::Error;
use csv::Writer;

use crate::quality_of_life::*;


pub fn write_vec_to_csv<S: ToString, T: ToString>(index: Vec<S>, vals: Vec<T>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Box::new(Writer::from_path("data.csv")?);
    wtr.write_record(&["index", "val"])?;

    for (idx, val) in index.iter().zip(vals.iter()) {
        wtr.write_record(&[idx.to_string(), val.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}


pub fn get_hist(data: Vec<f32>, n_buckets: usize) -> (Vec<f32>, Vec<usize>) {
    let min = partial_min(&data);
    let max = partial_max(&data);

    let bucket_size = (max - min) / n_buckets as f32;

    if bucket_size == 0.0 {
        return (vec![min], vec![data.len()])
    }
    
    let mut histogram = vec![0; n_buckets];

    for x in data.iter() {
        // min + i*bucket size <= x < min + (i+1) * bucket_size
        // therefore i <= (x - min) / bucket_size
        // In case x = max, the bucket index will be n+1
        // Put it in bucket n instead.
        let i = ((x - min) / bucket_size).floor() as usize;
        let i = if i == n_buckets { n_buckets - 1 } else { i };

        histogram[i] = histogram[i] + 1;
    }

    let bins: Vec<f32> = (0..n_buckets).into_iter().map( |i| {
        min + (i as f32 + 0.5)*bucket_size 
    }).collect();

    (bins, histogram)
}
