use ::rand_distr::Normal;
use std::error::Error;

use mcmc::output;
use mcmc::metropolis::metropolis;
use mcmc::quality_of_life::*;

use plotters::prelude::*;


fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let proposal = Normal::new(0.0, 1.0).unwrap();

    // We are looking for a standard normal distribution
    // exp( -x ^ 2 ) is the distribution propertion
    let pi = |x: f32| -> f32 { exp(-x.powi(2)) };

    let result = metropolis(pi, &proposal, &mut rng);

    let n_buckets = 5000;
    let min = partial_min(&result);
    let max = partial_max(&result);
    let bucket_size = (max - min) / n_buckets as f32;
    
    let mut histogram = [0; 5001];

    for x in result.iter() {
        // min + i*bucket size <= x < min + (i+1) * bucket_size
        // therefore i <= (x - min) / bucket_size
        let i = ((x - min) / bucket_size).floor() as usize;

        // In case x = max, put in highest bucket.
        let i = if i == 5000 { 5000 - 1 } else { i };
        histogram[i] = histogram[i] + 1;
    }



    // let root = BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();
    // root.fill(&WHITE)?;

    // let mut chart = ChartBuilder::on(&root)
    //     .x_label_area_size(35)
    //     .y_label_area_size(40)
    //     .margin(5)
    //     .caption("Histogram Test", ("sans-serif", 50.0).into_font())
    //     .build_ranged(0u32..5000u32, 0u32..1000u32)?;

    // chart.configure_mesh().draw()?;

    // chart.draw_series(
    //     Histogram::vertical(&chart)
    //     .data(histogram.iter().map(|x: &u32| (*x, 1)))
    //     )?;

    // println!("min: {}, max: {}", min, max);
    // println!("maxitems: {}", partial_max(&histogram.to_vec()));

    let root = BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0).into_font())
        .build_ranged(0u32..5000u32, 0u32..1000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(histogram.iter().map(|x: &u32| (*x, 1))),
    )?;

    
    // output::write_vec_to_csv(result)
    let mut res = Vec::new();
    for x in histogram.iter(){
        res.push(*x as f32);
    }
    output::write_vec_to_csv(res)

}


