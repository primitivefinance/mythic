use iced::window::icon;
use crate::rand;
/// Placeholder for the Excalibur logo.
#[allow(dead_code)]
const EXCALIBUR_LOGO: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo.png");
const EXCALIBUR_LOGO_1: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_1.png");
const EXCALIBUR_LOGO_2: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_2.png");
const EXCALIBUR_LOGO_3: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_3.png");
const EXCALIBUR_LOGO_4: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_4.png");
const EXCALIBUR_LOGO_5: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_5.png");
const EXCALIBUR_LOGO_6: &[u8] = include_bytes!("../../../../assets/logos/excalibur_logo_6.png");

pub fn get_random_logo() -> icon::Icon {
    let logos = vec![
        excalibur_logo_1(),
        excalibur_logo_2(),
        excalibur_logo_3(),
        excalibur_logo_4(),
        excalibur_logo_5(),
        excalibur_logo_6(),
    ];
    logos[rand::random::<usize>() % logos.len()].clone()
}
#[allow(dead_code)]
pub fn excalibur_logo() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO, None).unwrap()
}
pub fn excalibur_logo_1() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_1, None).unwrap()
}

pub fn excalibur_logo_2() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_2, None).unwrap()
}
pub fn excalibur_logo_3() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_3, None).unwrap()
}
pub fn excalibur_logo_4() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_4, None).unwrap()
}
pub fn excalibur_logo_5() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_5, None).unwrap()
}
pub fn excalibur_logo_6() -> icon::Icon {
    icon::from_file_data(EXCALIBUR_LOGO_6, None).unwrap()
}