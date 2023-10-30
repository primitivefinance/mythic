use super::*;

use plotters::prelude::*;

const OUT_FILE_NAME: &str = "sample.png";
pub fn example_multi_line_plot() -> Result<()> {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1920, 1080)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

    let (upper, lower) = root_area.split_vertically(512);

    let x_axis = (-3.4f32..3.4).step(0.1);

    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Sine and Cosine", ("sans-serif", 40))
        .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| (x, x.cos())),
        &BLUE,
    ))?
    .label("Cosine")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    cc.configure_series_labels().border_style(BLACK).draw()?;

    /*
    // It's possible to use a existing pointing element
     cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
    ))?;*/

    // Otherwise you can use a function to construct your pointing element yourself
    cc.draw_series(PointSeries::of_element(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))?;

    let drawing_areas = lower.split_evenly((1, 2));

    for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
        let mut cc = ChartBuilder::on(drawing_area)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;
        cc.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()?;

        cc.draw_series(LineSeries::new(
            (-1f32..1f32)
                .step(0.01)
                .values()
                .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
            &BLUE,
        ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

pub fn plot_statistical(x_data: &[f64], y_data: &[Vec<f64>]) -> Result<()> {
    // assert_eq!(x_data.len(), y_data.len());

    // Compute x range
    let x_min = *x_data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);
    let x_max = *x_data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&1.0);

    // Compute y range considering mean and standard deviation
    let avg_with_std: Vec<(f64, f64, f64)> = (0..x_data.len())
        .map(|i| {
            let values_at_i: Vec<f64> = y_data.iter().map(|y_set| y_set[i]).collect();
            let mean = values_at_i.iter().sum::<f64>() / values_at_i.len() as f64;
            let std_dev = (values_at_i.iter().map(|&y| (y - mean).powi(2)).sum::<f64>()
                / values_at_i.len() as f64)
                .sqrt();
            (x_data[i], mean, std_dev)
        })
        .collect();

    let y_min = avg_with_std
        .iter()
        .map(|&(_, mean, std_dev)| mean - std_dev)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    let y_max = avg_with_std
        .iter()
        .map(|&(_, mean, std_dev)| mean + std_dev)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);

    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Average Signal with Standard Deviation",
            ("Arial", 24).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    let average_signal: Vec<(f64, f64)> =
        avg_with_std.iter().map(|&(x, mean, _)| (x, mean)).collect();
    let standard_deviation: Vec<f64> = avg_with_std
        .iter()
        .map(|&(_, _, std_dev)| std_dev)
        .collect();

    // Shade the region for standard deviation
    let upper_curve: Vec<(f64, f64)> = average_signal
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (x, y + standard_deviation[i]))
        .collect();

    let lower_curve: Vec<(f64, f64)> = average_signal
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (x, y - standard_deviation[i]))
        .collect();

    chart.draw_series(AreaSeries::new(
        upper_curve
            .iter()
            .cloned()
            .chain(lower_curve.iter().rev().cloned()),
        0.0, // baseline
        BLUE.mix(0.3),
    ))?;

    // Plot average signal over shaded region
    chart.draw_series(LineSeries::new(average_signal, &RED))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_plot() {
        example_multi_line_plot();
    }

    #[test]
    fn test_plot_statistical() {
        todo!()
    }
}
