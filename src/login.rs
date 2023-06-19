use iced::{
    window,
    widget::{button, column, row, text, text_input},
    Alignment, Sandbox,
};

#[derive(Default)]
pub struct Login {
    username: String,
    password: String,
    hint: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    Login,
    SignUp,
}

impl Sandbox for Login {
    type Message = LoginMessage;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::UsernameChanged(username) => {
                self.username = username;
            }
            Self::Message::PasswordChanged(password) => {
                self.password = password;
            }
            Self::Message::Login => {
                if self.username.len() > 32 {
                    self.hint = "Username too long".to_string();
                } else if self.password.len() > 32 {
                    self.hint = "Password too long".to_string();
                }
            },
            Self::Message::SignUp => {
                // window::resize<Self::Message>(600, 800);
                // super::signup::SignUp::run(Default::default());
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let username =
            text_input("username", &self.username).on_input(Self::Message::UsernameChanged);
        let password = text_input("password", &self.password)
            .on_input(Self::Message::PasswordChanged)
            .password();
        let hint = text("");
        let login_button = button("Login").on_press(Self::Message::Login);
        let sign_up_button = button("Sign Up").on_press(Self::Message::SignUp);

        column![
            username,
            password,
            hint,
            row![login_button, sign_up_button,]
                .spacing(10)
                .padding(20)
                .align_items(Alignment::Center)
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
