use iced::window::icon;

const EXCALIBUR_LOGO: &[u8] = include_bytes!("./static/logos/excalibur_logo.png");

pub fn excalibur_logo() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO, None).unwrap()
}
