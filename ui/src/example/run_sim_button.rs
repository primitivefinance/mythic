//! Widget for running a simulation!
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use iced::{
    alignment::{self},
    widget::{button, column, component, text, Component},
    Element, Length, Renderer,
};
use simulation::simulations;

#[derive(Debug)]
pub struct RunSimButton {
    config_path: PathBuf,
}

impl Default for RunSimButton {
    fn default() -> Self {
        Self {
            config_path: Path::new(std::env::current_dir().unwrap().to_str().unwrap())
                .join("simulation")
                .join("configs")
                .join("test")
                .join("static.toml"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Run,
}

impl RunSimButton {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    /// Runs the simulation on a separate thread.
    pub fn run(&self) -> anyhow::Result<()> {
        let config_path = self.config_path.clone();

        std::thread::spawn(move || {
            let start = Instant::now();
            match simulations::batch(config_path.to_str().unwrap()) {
                Ok(_) => {
                    let duration = start.elapsed();
                    tracing::info!("Total duration of simulations: {:?}", duration);
                }
                Err(e) => {
                    tracing::error!("Simulation failed: {:?}", e);
                }
            }
        });

        Ok(())
    }
}

impl<Message> Component<Message, Renderer> for RunSimButton {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::Run => {
                tracing::info!("Run button pressed");
                match self.run() {
                    Ok(_) => {
                        tracing::info!("Simulation complete");
                    }
                    Err(e) => {
                        tracing::error!("Simulation failed: {:?}", e);
                    }
                }
                None
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        let mut content = column![];

        let button = |label, on_press| {
            button(
                text(label)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(on_press)
            .padding(10)
            .width(Length::Fill)
        };

        content = content.push(button("Run", Event::Run));

        content.into()
    }
}

impl<'a, Message> From<RunSimButton> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(counter: RunSimButton) -> Self {
        component(counter).into()
    }
}
