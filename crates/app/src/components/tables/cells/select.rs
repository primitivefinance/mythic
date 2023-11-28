use std::borrow::Cow;

use components::select::custom_pick_list;
use iced::widget::PickList;

use super::*;

pub fn cell_select<'a, Message>(
    options: impl Into<Cow<'a, [String]>>,
    selected: Option<String>,
    on_selected: impl Fn(String) -> Message + 'a,
    placeholder: Option<String>,
) -> PickList<'a, String, Message>
where
    Message: 'static + Default,
{
    let mut select = custom_pick_list(options, selected, on_selected, placeholder);
    select
}
