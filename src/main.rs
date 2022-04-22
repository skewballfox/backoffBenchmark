use backoff_benchmark::backoff_protocol::BackoffProtocol;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //first lets declare a type to make the declarations below a bit more legible
    type BackoffFn = fn(usize) -> usize;

    //these are the functions that will be passed to the structs used to simulate each of the
    //backoff protocols, they determine the growth rate of the window size
    let linear_expansion: BackoffFn = |len: usize| len + 1;

    let binary_exponential_expansion: BackoffFn = |len: usize| len * 2;

    let loglog_expansion: BackoffFn =
        |len: usize| ((1. + (1. / (len as f64).log2().log2())) * len as f64).floor() as usize;

    //these are the structs that actually run the individivual simulations
    //initial device count starts at 100
    let mut linear_backoff = BackoffProtocol::new(2, linear_expansion);
    let mut binary_exponential_backoff = BackoffProtocol::new(2, binary_exponential_expansion);
    let mut loglog_backoff = BackoffProtocol::new(4, loglog_expansion);

    let mut linear_results = Vec::with_capacity(60);
    let mut binary_exponential_results = Vec::with_capacity(60);
    let mut loglog_results = Vec::with_capacity(60);

    for _i in 0..60 {
        println!("experiment: {}", _i);
        let x = linear_backoff.run_experiment();
        println!("yeet");
        let y = binary_exponential_backoff.run_experiment();
        println!("yeet");
        let z = loglog_backoff.run_experiment();
        println!("yeet");

        linear_results.push(x);
        binary_exponential_results.push(y);
        loglog_results.push(z);
        println!(
            "{:?}\n{:?}\n{:?}",
            linear_results, binary_exponential_results, loglog_results
        );
    }
    let mut big_max = linear_results.iter().max().unwrap();
    big_max = big_max.max(binary_exponential_results.iter().max().unwrap());
    big_max = big_max.max(loglog_results.iter().max().unwrap());
    println!(
        "results: {:?}\n{:?}\n{:?}\n\nmax val: {}",
        linear_results, binary_exponential_results, loglog_results, big_max
    );
    //now we'll create the plot we'll be using to display the results
    let root = BitMapBackend::new("assets/big_brain_time.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "backoff protocols comparision",
            ("sans-serif", 50).into_font(),
        )
        .margin(5_i32)
        .x_label_area_size(30_i32)
        .y_label_area_size(30_i32)
        .build_cartesian_2d(0_usize..6000_usize, 0_usize..*big_max)?;

    chart
        .configure_mesh()
        .x_desc("Number of Nodes(Devices) N")
        .y_desc("Average Latency")
        .draw()?;

    //chart the data
    let color = Palette99::pick(0).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            linear_results
                .iter()
                .enumerate()
                .map(|(i, val)| -> (usize, usize) { ((i + 1) * 100, val.clone()) }),
            color.stroke_width(3),
        ))?
        .label("linear backoff")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    let color = Palette99::pick(1).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            binary_exponential_results
                .iter()
                .enumerate()
                .map(|(i, val)| -> (usize, usize) { ((i + 1) * 100, val.clone()) }),
            color.stroke_width(3),
        ))?
        .label("binary exponential backoff")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    let color = Palette99::pick(2).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            loglog_results
                .iter()
                .enumerate()
                .map(|(i, val)| -> (usize, usize) { ((i + 1) * 100, val.clone()) }),
            color.stroke_width(3),
        ))?
        .label("loglog backoff")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    //give it a pretty grid
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
