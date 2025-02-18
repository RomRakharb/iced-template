use iced::{
    widget::{button, column, container, text},
    Element,
    Length::Fill,
};

pub fn main() -> iced::Result {
    iced::run("A cool counter", State::update, State::view)
}

#[derive(Default)]
struct State {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ])
        .center(Fill)
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_counts_properly() {
        let mut counter = State::default();

        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);

        assert_eq!(counter.value, 1);
    }
}
