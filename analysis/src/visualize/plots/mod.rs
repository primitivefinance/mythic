use super::*;

pub mod heatmap;
pub mod statistical;

pub trait Plot {
    fn plot(&self, drawing_area: &DrawingArea<BitMapBackend, Shift>) -> Result<()>;
}

pub struct PlotSettings {
    pub file_name: Option<String>,
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
}

impl PlotSettings {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            file_name: None,
            title: None,
            x_label: None,
            y_label: None,
        }
    }

    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = Some(file_name.to_owned());
        self
    }

    pub fn labels(mut self, x_label: &str, y_label: &str) -> Self {
        self.x_label = Some(x_label.to_owned());
        self.y_label = Some(y_label.to_owned());
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
    fn file_name() {
        let settings = PlotSettings::new().file_name("test.png");
        assert_eq!(settings.file_name, Some("test.png".to_owned()));
    }

    #[test]
    fn labels() {
        let settings = PlotSettings::new().labels("x", "y");
        assert_eq!(settings.x_label, Some("x".to_owned()));
        assert_eq!(settings.y_label, Some("y".to_owned()));
    }

    #[test]
    fn title() {
        let settings = PlotSettings::new().title("title");
        assert_eq!(settings.title, Some("title".to_owned()));
    }
}
