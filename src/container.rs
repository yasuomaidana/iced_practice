use clap::{Parser, ValueEnum};
use iced::widget::container;
use iced::{Fill, padding};

#[derive(Parser, Debug)]
struct Mode {
    #[arg(value_enum, default_value_t = Container::SimplePadding)]
    widget: Container,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Container {
    SimplePadding,
    CustomPadding,
    Aligned,
}

fn view(selected: &Container) -> iced::Element<'_, ()> {
    match selected {
        Container::SimplePadding => container("This is a bordered box!")
            .padding(10)
            .style(container::bordered_box),
        Container::CustomPadding => container("Independent padding!")
            .padding(padding::vertical(30).left(20).right(80))
            .style(container::bordered_box),
        Container::Aligned => {
            let my_box = container("Bottom right!")
                .padding(10)
                .style(container::primary);

            container(my_box)
                .width(Fill)
                .height(Fill)
                .padding(10)
                .align_bottom(Fill)
                .align_right(Fill)
                .style(container::bordered_box)
        }
    }
    .into()
}

fn main() -> iced::Result {
    let args = Mode::parse();

    iced::application(
        move || args.widget,
        |_state: &mut Container, _message: ()| {},
        view,
    )
    .title("Container")
    .run()
}
