use plotters::{
    backend::BitMapBackend,
    coord::{types::RangedCoordf64, Shift},
    prelude::*,
};

use self::plots::{statistical::StatisticalPlot, Plot};
use super::*;

pub mod plots;

pub struct Figure {
    pub file_name: String,
    pub dimensions: Option<(u32, u32)>,
    pub plots: Vec<Box<dyn Plot>>,
}

impl Figure {
    pub fn new(name: &str, dimensions: Option<(u32, u32)>) -> Self {
        Self {
            file_name: format!("{}.png", name),
            dimensions,
            plots: Vec::new(),
        }
    }

    pub fn add_statistical_plot(&mut self, plot: StatisticalPlot) -> &mut Self {
        self.plots.push(Box::new(plot));
        self
    }

    pub fn create(&self) -> Result<()> {
        let drawing_area = BitMapBackend::new(&self.file_name, (800, 600)).into_drawing_area();
        drawing_area.fill(&WHITE)?;

        // Partition the main drawing area into subplots
        let num_plots = self.plots.len();
        if num_plots == 0 {
            return Ok(()); // If there are no plots, just return.
        }

        // Split the main drawing area into sub-areas
        let sub_areas = drawing_area.split_evenly((1, num_plots));

        // Plot each plot on its respective drawing area
        for (i, plot) in self.plots.iter().enumerate() {
            plot.plot(&sub_areas[i])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_statistical_plot() {
        let mut figure = Figure::new("test", None);
        let plot = StatisticalPlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]],
        };
        figure.add_statistical_plot(plot);
        assert_eq!(figure.plots.len(), 1);
    }

    #[test]
    fn create_statistical_plot() {
        let mut figure = Figure::new("test_create_statistical_plot", None);
        let plot = StatisticalPlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]],
        };
        figure.add_statistical_plot(plot);
        figure.create().unwrap();
        assert!(std::path::Path::new("test_create_statistical_plot.png").exists());
        std::fs::remove_file("test_create_statistical_plot.png").unwrap();
    }
}
