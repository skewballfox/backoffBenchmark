use plotters::prelude::*;

fn plot_results(max_val: usize) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5_i32)
        .x_label_area_size(30_i32)
        .y_label_area_size(30_i32)
        .build_cartesian_2d(0_usize..6000_usize, 0_usize..max_val)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new());

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
