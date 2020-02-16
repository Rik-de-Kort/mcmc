use std::error::Error;
use csv::Writer;

pub fn write_vec_to_csv(result: Vec<f64>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Box::new(Writer::from_path("data.csv")?);
    wtr.write_record(&["index", "val"])?;

    for i in 0..result.len()-1 {
        wtr.write_record(&[i.to_string(), result[i].to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}
