use std::{f32::consts::PI, time::Instant};

use iced::{
    widget::canvas::{self, path::arc, stroke, Cache, Canvas, Geometry, Path, Stroke},
    window::icon,
    Application, Command, Element, Length, Point, Rectangle, Renderer, Settings, Subscription,
    Theme,
};

use crate::components::styles::MINT_500;

/// Placeholder for the Excalibur logo.
#[allow(dead_code)]
const EXCALIBUR_LOGO: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo.png");

#[allow(dead_code)]
pub fn excalibur_logo() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO, None).unwrap()
}

const EXCALIBUR_LOGO_2: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_2.png");

pub fn excalibur_logo_2() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_2, None).unwrap()
}

pub struct PhiLogo {
    pub start: Instant,
    pub rotation: f32,
    pub cache: Cache,
}

pub const GOLDEN_RATIO: f32 = 1.61803398875;

impl<'a, Message> iced::widget::canvas::Program<Message> for PhiLogo {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let palette = theme.palette();

            let center = frame.center();

            let circle_radius = frame.width().min(frame.height()) / (GOLDEN_RATIO * 5.0);

            let start = Point::new(center.x, center.y);

            let raw_angle = (self.start.elapsed().as_millis() % ((1_000.0 * GOLDEN_RATIO) as u128))
                as f32
                / (1_000.0 * GOLDEN_RATIO);
            let s_curve_angle = 3.0 * raw_angle.powi(2) - 2.0 * raw_angle.powi(3);
            let angle_offset = -PI / 3.0;
            let angle = s_curve_angle * 2.0 * PI + angle_offset;

            // Draw the oval part of the phi symbol
            let oval = Path::new(|b| {
                b.ellipse(arc::Elliptical {
                    radii: [circle_radius, circle_radius * 0.8].into(),
                    start_angle: 0.0,
                    end_angle: 2.0 * PI,
                    rotation: angle,
                    center: start,
                });
            });

            // 5.0 / 365, because 5.0 width looks good on a 365px wide canvas
            const STROKE_PROPORTION: f32 = 0.0139 * GOLDEN_RATIO; // Adjust this value to change the stroke size

            let stroke_size = frame.width().min(frame.height()) * STROKE_PROPORTION;

            // Stroke the oval instead of filling it
            frame.stroke(
                &oval,
                Stroke {
                    style: stroke::Style::Solid(MINT_500),
                    width: stroke_size,
                    ..Stroke::default()
                },
            );

            let line_radius = frame.width().min(frame.height()) / (GOLDEN_RATIO * 2.0); // Golden ratio

            let start = Point::new(
                center.x - line_radius * angle.cos(),
                center.y - line_radius * angle.sin(),
            );

            let end = Point::new(
                center.x + line_radius * angle.cos(),
                center.y + line_radius * angle.sin(),
            );

            // Draw the vertical line part of the phi symbol
            let path = Path::new(|b| {
                b.move_to(start);
                b.line_to(end);
            });

            frame.stroke(
                &path,
                Stroke {
                    style: stroke::Style::Solid(MINT_500),
                    width: stroke_size,
                    ..Stroke::default()
                },
            );
        });

        vec![geometry]
    }
}
