#![windows_subsystem = "windows"]

use iced::Application;
mod parser;

struct State {
    cur_eqn: Vec<Chars>,
    showing_res: bool,
    history: Vec<Vec<Chars>>,
    res: Option<f64>
}

fn eqn_to_str(eqn: Vec<Chars>) -> String {
    let mut out = String::new();

    for ch in eqn {
        out.push(ch.as_char())
    }

    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Chars {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    RParen,
    Div,
    C,
    Del,
    Mul,
    Sub,
    Add,
    LParen,
    Dot,
    Eq
}

impl Chars {
    fn as_char(&self) -> char {
        use Chars::*;
        match self {
            Zero => '0',
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            RParen => ')',
            Div => '÷',
            Mul => '×',
            Sub => '-',
            Add => '+',
            LParen => '(',
            Dot => '.',
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ButtonClick(Chars),
    Event(iced::Event)
}

impl iced::Application for State {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            State {
                cur_eqn: Vec::new(),
                showing_res: true,
                history: Vec::new(),
                res: Some(0.0)
            },
            iced::Command::none()
        )
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::Event)
    }

    fn title(&self) -> String {
        "Calculator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ButtonClick(btn) => {
                if self.showing_res {
                    self.showing_res = false;
                    self.res = None;
                }
                match btn {
                    Chars::C => {
                        self.cur_eqn = Vec::new();
                        self.showing_res = true
                    },
                    Chars::Del => {
                        self.cur_eqn.pop();
                    },
                    Chars::Eq => {
                        let mut tokens = chars_to_tokens(self.cur_eqn.clone());
                        tokens.push(parser::Token::End);
                        self.res = Some(parser::eval(tokens).unwrap());

                        self.history.push(self.cur_eqn.drain(..).collect());
                        self.showing_res = true;
                    },
                    _ => self.cur_eqn.push(btn)
                }
            },
            Message::Event(ev) => match ev {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, modifiers }) => {
                    if self.showing_res {
                        self.showing_res = false;
                        self.res = None;
                    }
                    match key_code {
                        iced::keyboard::KeyCode::Plus | iced::keyboard::KeyCode::NumpadAdd => {
                            self.cur_eqn.push(Chars::Add);
                        },
                        iced::keyboard::KeyCode::Minus | iced::keyboard::KeyCode::NumpadSubtract => {
                            self.cur_eqn.push(Chars::Sub);
                        },
                        iced::keyboard::KeyCode::Asterisk | iced::keyboard::KeyCode::NumpadMultiply => {
                            self.cur_eqn.push(Chars::Mul);
                        },
                        iced::keyboard::KeyCode::Slash | iced::keyboard::KeyCode::NumpadDivide => {
                            self.cur_eqn.push(Chars::Div);
                        },
                        iced::keyboard::KeyCode::Equals if modifiers.shift() => {
                            self.cur_eqn.push(Chars::Add);
                        },
                        iced::keyboard::KeyCode::Key8 if modifiers.shift() => {
                            self.cur_eqn.push(Chars::Mul);
                        },
                        iced::keyboard::KeyCode::Key9 if modifiers.shift() => {
                            self.cur_eqn.push(Chars::LParen);
                        },
                        iced::keyboard::KeyCode::Key0 if modifiers.shift() => {
                            self.cur_eqn.push(Chars::RParen);
                        },
                        iced::keyboard::KeyCode::Equals | iced::keyboard::KeyCode::Enter | iced::keyboard::KeyCode::NumpadEnter | iced::keyboard::KeyCode::NumpadEquals => {
                            if !self.cur_eqn.is_empty() {
                                let mut tokens = chars_to_tokens(self.cur_eqn.clone());
                                tokens.push(parser::Token::End);
                                self.res = Some(parser::eval(tokens).unwrap());
                                self.history.push(self.cur_eqn.drain(..).collect());
                                self.showing_res = true;
                            }
                        }
                        iced::keyboard::KeyCode::Key0 | iced::keyboard::KeyCode::Numpad0 => {
                            self.cur_eqn.push(Chars::Zero);
                        },
                        iced::keyboard::KeyCode::Key1 | iced::keyboard::KeyCode::Numpad1 => {
                            self.cur_eqn.push(Chars::One);
                        },
                        iced::keyboard::KeyCode::Key2 | iced::keyboard::KeyCode::Numpad2 => {
                            self.cur_eqn.push(Chars::Two);
                        },
                        iced::keyboard::KeyCode::Key3 | iced::keyboard::KeyCode::Numpad3 => {
                            self.cur_eqn.push(Chars::Three)
                        },
                        iced::keyboard::KeyCode::Key4 | iced::keyboard::KeyCode::Numpad4 => {
                            self.cur_eqn.push(Chars::Four);
                        },
                        iced::keyboard::KeyCode::Key5 | iced::keyboard::KeyCode::Numpad5 => {
                            self.cur_eqn.push(Chars::Five);
                        },
                        iced::keyboard::KeyCode::Key6 | iced::keyboard::KeyCode::Numpad6 => {
                            self.cur_eqn.push(Chars::Six)
                        },
                        iced::keyboard::KeyCode::Key7 | iced::keyboard::KeyCode::Numpad7 => {
                            self.cur_eqn.push(Chars::Seven);
                        },
                        iced::keyboard::KeyCode::Key8 | iced::keyboard::KeyCode::Numpad8 => {
                            self.cur_eqn.push(Chars::Eight);
                        },
                        iced::keyboard::KeyCode::Key9 | iced::keyboard::KeyCode::Numpad9 => {
                            self.cur_eqn.push(Chars::Nine);
                        },
                        iced::keyboard::KeyCode::Backspace => {
                            self.cur_eqn.pop();
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let contents = iced::widget::column![
            iced::widget::text(if let Some(res) = self.res { res.to_string() } else if self.cur_eqn.is_empty() { "0".to_string() } else { eqn_to_str(self.cur_eqn.clone()) }).width(iced::Length::Units(200)).height(iced::Length::Units(50)),
            iced::widget::row![
                iced::widget::button("(").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::LParen)),
                iced::widget::button(")").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::RParen)),
                iced::widget::button("C").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::C)),
                iced::widget::button("⌫").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Del))
            ].spacing(5),
            iced::widget::row![
                iced::widget::button("7").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Seven)),
                iced::widget::button("8").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Eight)),
                iced::widget::button("9").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Nine)),
                iced::widget::button("×").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Mul))
            ].spacing(5),
            iced::widget::row![
                iced::widget::button("4").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Four)),
                iced::widget::button("5").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Five)),
                iced::widget::button("6").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Six)),
                iced::widget::button("-").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Sub))
            ].spacing(5),
            iced::widget::row![
                iced::widget::button("1").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::One)),
                iced::widget::button("2").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Two)),
                iced::widget::button("3").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Three)),
                iced::widget::button("+").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Add))
            ].spacing(5),
            iced::widget::row![
                iced::widget::button("0").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Zero)),
                iced::widget::button(".").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Dot)),
                iced::widget::button("=").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Eq)),
                iced::widget::button("÷").width(iced::Length::Units(50)).height(iced::Length::Units(50)).on_press(Self::Message::ButtonClick(Chars::Div))
            ].spacing(5)
        ].spacing(5);

        iced::widget::container(contents)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

fn chars_to_tokens(chars: Vec<Chars>) -> Vec<parser::Token> {
    let mut tokens = Vec::new();
    let mut depth = 0;
    let mut in_dec = false;

    for char in chars {
        match char {
            Chars::Zero | Chars::One | Chars::Two | Chars::Three | Chars::Four | Chars::Five | Chars::Six | Chars::Seven | Chars::Eight | Chars::Nine => {
                if !in_dec {
                    if let Some(parser::Token::Number(val)) = tokens.last_mut() {
                        *val = val.mul_add(10.0, char as u8 as f64);
                    } else {
                        tokens.push(parser::Token::Number(char as u8 as f64))
                    }
                } else {
                    if let Some(parser::Token::Number(val)) = tokens.last_mut() {
                        *val = *val + ((char as u8 as f64) / 10.0_f64.powi(depth));
                        depth += 1;
                    } else {
                        tokens.push(parser::Token::Number(char as u8 as f64 / 10.0))
                    }                    
                }
            },
            Chars::Dot => {
                in_dec = true;
                depth = 1;
            },
            Chars::Add if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::Plus);
            },
            Chars::Add => tokens.push(parser::Token::Plus),
            Chars::Mul if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::Star);
            },
            Chars::Mul => tokens.push(parser::Token::Star),
            Chars::Div if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::Slash);
            },
            Chars::Div => tokens.push(parser::Token::Slash),
            Chars::Sub if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::Dash);
            },
            Chars::Sub => tokens.push(parser::Token::Dash),
            Chars::LParen if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::LeftParen);
            },
            Chars::LParen => tokens.push(parser::Token::LeftParen),
            Chars::RParen if in_dec => {
                in_dec = false;
                tokens.push(parser::Token::RightParen);
            },
            Chars::RParen => tokens.push(parser::Token::RightParen),
            _ => unreachable!()
        }
    }

    tokens
}

fn main() -> iced::Result {
    let mut settings = iced::Settings::default();
    let icon = include_bytes!("../icon.png");
    settings.window = iced::window::Settings {
        size: (280, 400),
        position: iced::window::Position::Centered,
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,
        decorations: true,
        transparent: false,
        always_on_top: false,
        icon: Some(iced::window::icon::Icon::from_file_data(icon, Some(image::ImageFormat::Png)).unwrap()),
    };
    State::run(settings)
}
