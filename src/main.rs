use crate::states::Counter;
use crate::views::ColumnView;

pub mod messages;
pub mod states;
pub mod views;

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Counter")
        .run()
}
