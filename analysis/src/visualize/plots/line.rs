use super::*;

pub struct LinePlot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
}

impl Plot for LinePlot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()> {
        let statistical_plot_with_zero_std = StatisticalPlot::from(self);
        statistical_plot_with_zero_std.plot(drawing_area)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_line_plot() {
        let line_plot = LinePlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![0.0, 1.0, 2.0],
        };
        let mut figure = Figure::new("test_add_line_plot", None);
        figure.add_plot(line_plot);
        assert_eq!(figure.plots.len(), 1);
    }

    #[test]
    fn create_line_plot() {
        let line_plot = LinePlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![0.0, 1.0, 2.0],
        };
        let mut figure = Figure::new("test_create_line_plot", None);
        figure.add_plot(line_plot);
        figure.create().unwrap();
        assert!(std::path::Path::new("test_create_line_plot.png").exists());
        std::fs::remove_file("test_create_line_plot.png").unwrap();
    }
}
