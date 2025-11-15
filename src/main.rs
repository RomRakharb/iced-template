use iced::{
    Element,
    Length::Fill,
    Subscription, Task,
    keyboard::{Key, key, on_key_press},
    widget::{button, column, container, text},
};

fn main() -> iced::Result {
    iced::application(|| State::default(), State::update, State::view)
        .title("iced template")
        .subscription(subscription)
        .run()
}

#[derive(Default)]
struct State {
    count: isize,
}

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        let tasks = Vec::new();

        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }

        Task::batch(tasks)
    }

    fn view(&self) -> Element<'_, Message> {
        container(column![
            button("+").on_press(Message::Increment),
            text(self.count.to_string()),
            button("-").on_press(Message::Decrement)
        ])
        .center(Fill)
        .into()
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    on_key_press(|key, modifiers| match (key, modifiers) {
        (Key::Named(key::Named::ArrowUp), _) => Some(Message::Increment),
        (Key::Named(key::Named::ArrowDown), _) => Some(Message::Decrement),
        _ => None,
    })
}
