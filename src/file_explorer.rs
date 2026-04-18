use iced::Length;
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, text};
use rfd::FileDialog;

#[derive(Default)]
struct State {
    file_name: String,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFileExplorer,
}

impl State {
    fn view(&self) -> iced::Element<'_, Message> {
        let file_name = if self.file_name.is_empty() {
            "No file selected"
        } else {
            &self.file_name
        };

        let content = column![
            button("Open file").on_press(Message::OpenFileExplorer),
            text(file_name).size(24)
        ]
        .spacing(16)
        .align_x(Horizontal::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenFileExplorer => {
                self.file_name = FileDialog::new()
                    .pick_file()
                    .and_then(|path| {
                        path.file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                    })
                    .unwrap_or_else(|| "No file selected".to_owned());
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}
