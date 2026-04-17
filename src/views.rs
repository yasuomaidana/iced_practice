use crate::messages::Message;
use crate::states::Counter;
use iced::alignment::Horizontal;
use iced::widget::{Button, Column, button, column, text};
use iced::{Fill, Font};

pub trait ColumnView<T> {
    // &'_ self means self is borrowed for some lifetime chosen by the compiler.
    // In practice, this expresses that the returned UI value cannot outlive the borrow of self.
    fn view(&'_ self) -> Column<'_, T>;
}

fn controller_button(content: &str, message: Message) -> Button<'_, Message> {
    button(
        text(content)
            .font(Font::MONOSPACE)
            .size(35)
            .width(Fill)
            .height(Fill)
            .center(),
    )
    .on_press(message)
    .width(Fill)
}

impl ColumnView<Message> for Counter {
    fn view(&'_ self) -> Column<'_, Message> {
        // buttons
        let increment = controller_button("+", Message::Increment);

        let decrement = controller_button("-", Message::Decrement);

        // display
        let counter = text(self.value)
            // .style(|theme: &Theme| text::Style {
            //     color: Some(theme.palette().primary),
            // })
            .style(text::warning)
            .size(50)
            .width(Fill)
            .height(Fill)
            .center();

        let emoji = if self.value % 2 == 0 {
            "👨‍🔬‍"
        } else {
            "😎"
        };

        let showing_text = text!("Counter app {}", emoji)
            .size(20)
            .width(Fill)
            .height(Fill)
            .center();
        //layout
        let interface = column![showing_text, increment, counter, decrement,];
        interface
            .spacing(10)
            .align_x(Horizontal::Center)
            .width(Fill)
            .height(Fill)
    }
}
