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

    pub fn add_plot<P: Plot + 'static>(&mut self, plot: P) -> &mut Self {
        self.plots.push(Box::new(plot));
        self
    }

    pub fn create(&self) -> Result<()> {
        let dimensions = self.dimensions.unwrap_or((1920, 1080));
        let drawing_area = BitMapBackend::new(&self.file_name, dimensions).into_drawing_area();
        drawing_area.fill(&WHITE)?;

        // Partition the main drawing area into subplots
        let num_plots = self.plots.len();
        if num_plots == 0 {
            return Ok(()); // If there are no plots, just return.
        }

        // Split the main drawing area into sub-areas
        let sub_areas = drawing_area.split_evenly((num_plots, 1));

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

    struct MockPlot {}

    impl Plot for MockPlot {
        fn plot(&self, _drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn new() {
        let name = "test_name";
        let dimensions = Some((100, 200));
        let figure = Figure::new("test_name", Some((100, 200)));
        assert_eq!(figure.file_name, format!("{}.png", name));
        assert_eq!(figure.dimensions, dimensions);
        assert_eq!(figure.plots.len(), 0);
    }

    #[test]
    fn add_plot() {
        let mut figure = Figure::new("test_add_plot", None);
        let plot = MockPlot {};
        figure.add_plot(plot);
        assert_eq!(figure.plots.len(), 1);
    }

    #[test]
    fn create() {
        let figure = Figure::new("test_create", None);
        figure.create().unwrap();
        assert!(std::path::Path::new("test_create.png").exists());
        std::fs::remove_file("test_create.png").unwrap();
    }
}
