use plotters::drawing;

use super::{line::LinePlot, *};

pub struct StatisticalPlot {
    pub plot_settings: Option<PlotSettings>,
    pub x_data: Vec<f64>,
    pub y_data: Vec<Vec<f64>>,
}

impl StatisticalPlot {
    pub fn new(x_data: Vec<f64>, y_data: Vec<Vec<f64>>) -> Self {
        Self {
            plot_settings: None,
            x_data,
            y_data,
        }
    }

    pub fn settings(mut self, plot_settings: PlotSettings) -> Self {
        self.plot_settings = Some(plot_settings);
        self
    }
}

impl From<&LinePlot> for StatisticalPlot {
    fn from(line_plot: &LinePlot) -> Self {
        let y_data = vec![line_plot.y_data.clone()];
        Self {
            plot_settings: line_plot.plot_settings.clone(),
            x_data: line_plot.x_data.clone(),
            y_data,
        }
    }
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

        let caption = &self
            .plot_settings
            .clone()
            .unwrap_or_default()
            .title
            .unwrap_or("Statistical Plot".to_owned());
        let x_desc = &self
            .plot_settings
            .clone()
            .unwrap_or_default()
            .x_desc
            .unwrap_or("X".to_owned());
        let y_desc = &self
            .plot_settings
            .clone()
            .unwrap_or_default()
            .x_desc
            .unwrap_or("Y".to_owned());
        let mut chart = ChartBuilder::on(drawing_area)
            .caption(caption, ("Arial", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart
            .configure_mesh()
            .x_desc(x_desc)
            .y_desc(y_desc)
            .draw()?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let x_data = vec![0.0, 1.0, 2.0];
        let y_data = vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]];
        let plot = StatisticalPlot::new(x_data, y_data);
    }

    #[test]
    fn settings() {
        todo!()
    }

    #[test]
    fn add_statistical_plot() {
        let mut figure = Figure::new("test_add_statistical_plot", None);
        let x_data = vec![0.0, 1.0, 2.0];
        let y_data = vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]];
        let plot = StatisticalPlot::new(x_data, y_data);
        figure.add_plot(plot);
        assert_eq!(figure.plots.len(), 1);
    }

    #[test]
    fn create_statistical_plot() {
        let mut figure = Figure::new("test_create_statistical_plot", None);
        let x_data = vec![0.0, 1.0, 2.0];
        let y_data = vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]];
        let plot = StatisticalPlot::new(x_data, y_data);
        figure.add_plot(plot);
        figure.create().unwrap();
        assert!(std::path::Path::new("test_create_statistical_plot.png").exists());
        std::fs::remove_file("test_create_statistical_plot.png").unwrap();
    }
}
