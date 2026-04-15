use crate::messages::Message;
use crate::states::Counter;
use iced::widget::{Column, button, column, text};

trait ColumnView<T> {
    // &'_ self means self is borrowed for some lifetime chosen by the compiler.
    // In practice, this expresses that the returned UI value cannot outlive the borrow of self.
    fn view(&'_ self) -> Column<'_, T>;
}

impl ColumnView<Message> for Counter {
    fn view(&'_ self) -> Column<'_, Message> {
        // buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // display
        let counter = text(self.value);

        //layout
        let interface = column![increment, counter, decrement,];
        interface
    }
}
