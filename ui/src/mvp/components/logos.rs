use iced::window::icon;

/// Placeholder for the Excalibur logo.
#[allow(dead_code)]
const EXCALIBUR_LOGO: &[u8] = include_bytes!("../static/logos/excalibur_logo.png");

#[allow(dead_code)]
pub fn excalibur_logo() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO, None).unwrap()
}

const EXCALIBUR_LOGO_2: &[u8] = include_bytes!("../static/logos/excalibur_logo_2.png");

pub fn excalibur_logo_2() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_2, None).unwrap()
}
