
use iced::widget::{button, column, text};
use iced::Element;

#[derive(Debug, Clone)]
enum Message {
    Tap,
    BuyUpgrade,
}

struct HamsterApp {
    coins: u64,
    coins_per_click: u64,
}

impl Default for HamsterApp {
    fn default() -> Self {
        Self {
            coins: 0,
            coins_per_click: 1,
        }
    }
}

impl HamsterApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tap => {
                self.coins += self.coins_per_click;
            }
            Message::BuyUpgrade => {
                if self.coins >= 10 {
                    self.coins -= 10;
                    self.coins_per_click += 1;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("Hamster Combat 4").size(32),
            text(format!("Balance: {} HMSTR", self.coins)),
            text(format!("Per click: {}", self.coins_per_click)),
            button("TAP").on_press(Message::Tap),
            button("UPGRADE (10 HMSTR)")
                .on_press_maybe(
                    (self.coins >= 10).then_some(Message::BuyUpgrade)
                ),
        ]
            .spacing(15)
            .padding(20)
            .into()
    }
}

fn main() -> iced::Result {
    iced::run("Hamster combat 4",HamsterApp::update, HamsterApp::view)
}
