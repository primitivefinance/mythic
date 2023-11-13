use super::*;

pub mod heatmap;
pub mod line;
pub mod statistical;

pub trait Plot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()>;
}

#[derive(Clone, Debug, Default)]
pub struct PlotSettings {
    pub title: Option<String>,
    pub x_desc: Option<String>,
    pub y_desc: Option<String>,
}

impl PlotSettings {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            title: None,
            x_desc: None,
            y_desc: None,
        }
    }

    pub fn labels(mut self, x_label: &str, y_label: &str) -> Self {
        self.x_desc = Some(x_label.to_owned());
        self.y_desc = Some(y_label.to_owned());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels() {
        let settings = PlotSettings::new().labels("x", "y");
        assert_eq!(settings.x_desc, Some("x".to_owned()));
        assert_eq!(settings.y_desc, Some("y".to_owned()));
    }

    #[test]
    fn title() {
        let settings = PlotSettings::new().title("title");
        assert_eq!(settings.title, Some("title".to_owned()));
    }
}
