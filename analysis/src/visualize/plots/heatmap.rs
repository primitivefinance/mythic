use super::*;

pub struct HeatMapPlot {
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub value: Vec<Vec<f64>>,
}

impl Plot for HeatMapPlot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()> {
        // let mut chart = ChartBuilder::on(drawing_area)
        //     .caption("Matshow Example", ("sans-serif", 80))
        //     .margin(5)
        //     .top_x_label_area_size(40)
        //     .y_label_area_size(40)
        //     .build_cartesian_2d(
        //         *self.x_data.first().unwrap()..*self.x_data.last().unwrap(),
        //         *self.y_data.first().unwrap()..*self.y_data.last().unwrap(),
        //     )?;

        // chart
        //     .configure_mesh()
        //     .x_labels(15)
        //     .y_labels(15)
        //     .max_light_lines(4)
        //     .x_label_offset(35)
        //     .y_label_offset(25)
        //     .disable_x_mesh()
        //     .disable_y_mesh()
        //     .label_style(("sans-serif", 20))
        //     .draw()?;

        // We assume that `x_data` and `y_data` are evenly spaced. If not, additional calculations would be required.
        let x_interval = if self.x_data.len() > 1 {
            self.x_data[1] - self.x_data[0]
        } else {
            1.0
        };

        let y_interval = if self.y_data.len() > 1 {
            self.y_data[1] - self.y_data[0]
        } else {
            1.0
        };

        let mut chart = ChartBuilder::on(drawing_area)
            .caption("HeatMap", ("sans-serif", 40).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                self.x_data[0]..self.x_data.last().unwrap() + x_interval,
                self.y_data[0]..self.y_data.last().unwrap() + y_interval,
            )?;

        let max_value = self
            .value
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NAN, f64::max);

        chart.draw_series(
            self.value
                .iter()
                .enumerate()
                .flat_map(|(y_idx, row)| {
                    row.iter().enumerate().map(move |(x_idx, &value)| {
                        let x = self.x_data[x_idx];
                        let y = self.y_data[y_idx];
                        (x, y, value)
                    })
                })
                .map(|(x, y, value)| {
                    Rectangle::new(
                        [(x, y), (x + x_interval, y + y_interval)],
                        HSLColor(
                            240.0 / 360.0 - 240.0 / 360.0 * (value / max_value),
                            0.7,
                            0.1 + 0.4 * value / max_value,
                        )
                        .filled(),
                    )
                }),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_heatmap_plot() {
        let mut figure = Figure::new("test_add_heatmap_plot", None);
        let heatmap = HeatMapPlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![0.0, 1.0],
            value: vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]],
        };
    }

    #[test]
    fn create_heatmap_plot() {
        let mut figure = Figure::new("test_create_heatmap_plot", None);
        let heatmap = HeatMapPlot {
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![0.0, 1.0],
            value: vec![vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 3.0]],
        };
        figure.add_plot(heatmap);
    }
}
// let mut matrix = [[0; 15]; 15];

//         for i in 0..15 {
//             matrix[i][i] = i + 4;
//         }

//         chart.draw_series(
//             matrix
//                 .iter()
//                 .zip(0..)
//                 .flat_map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x, y, v)))
//                 .map(|(x, y, v)| {
//                     Rectangle::new(
//                         [(x, y), (x + 1, y + 1)],
//                         HSLColor(
//                             240.0 / 360.0 - 240.0 / 360.0 * (*v as f64 / 20.0),
//                             0.7,
//                             0.1 + 0.4 * *v as f64 / 20.0,
//                         )
//                         .filled(),
//                     )
//                 }),
//         )?;
