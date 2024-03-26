use std::collections::VecDeque;

use iced::{
    widget::{scrollable, text, Column},
    Alignment, Application, Element, Length,
};

use crate::{reel::Reel, Stop};

pub(crate) struct Front {
    queue: Reel,
}

#[derive(Debug)]
pub enum Message {
    Up,
    Down,
    Chosen,
    Typing(String),
}

impl Application for Front {
    fn view(&self) -> Element<Message> {
        let colum = self
            .queue
            .to_vec_deque()
            .into_iter()
            .map(|s| text(s.title()))
            .fold(Column::new(), |c, s| c.push(s));

        let content = scrollable(
            colum
                .spacing(5)
                .width(Length::Fill)
                .align_items(Alignment::Start),
        );

        iced::widget::container(content).padding(20).into()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Chosen => iced::Command::none(),
            Message::Up => todo!(),
            Message::Down => todo!(),
            Message::Typing(_) => todo!(),
        }
    }

    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Reel;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Front { queue: flags }, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::CatppuccinMocha
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }
}
