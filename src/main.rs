use crate::states::Counter;
use crate::views::ColumnView;

pub mod messages;
pub mod states;
pub mod views;

fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
