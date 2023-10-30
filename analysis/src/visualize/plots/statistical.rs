use plotters::drawing;

use super::*;

pub struct StatisticalPlot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<Vec<f64>>,
}

impl Plot for StatisticalPlot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()> {
        // Compute x range
        let x_min = *self
            .x_data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let x_max = *self
            .x_data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&1.0);

        // Compute y range considering mean and standard deviation
        let avg_with_std: Vec<(f64, f64, f64)> = (0..self.x_data.len())
            .map(|i| {
                let values_at_i: Vec<f64> = self.y_data.iter().map(|y_set| y_set[i]).collect();
                let mean = values_at_i.iter().sum::<f64>() / values_at_i.len() as f64;
                let std_dev = (values_at_i.iter().map(|&y| (y - mean).powi(2)).sum::<f64>()
                    / values_at_i.len() as f64)
                    .sqrt();
                (self.x_data[i], mean, std_dev)
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

        // let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
        // root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&drawing_area)
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
        chart.draw_series(LineSeries::new(average_signal, &RED));
        Ok(())
    }
}
