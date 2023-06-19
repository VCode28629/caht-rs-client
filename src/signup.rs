use iced::{
    widget::{button, column, row, text_input},
    Alignment, Sandbox,
};

#[derive(Default)]
pub struct SignUp {
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
pub enum SignUpMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    Cancel,
    SignUp,
}


impl Sandbox for SignUp {
    type Message = SignUpMessage;

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
            Self::Message::SignUp => {
                
            }
            Self::Message::Cancel => {
                ()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let username =
            text_input("username", &self.username).on_input(Self::Message::UsernameChanged);
        let password = text_input("password", &self.password)
            .on_input(Self::Message::PasswordChanged)
            .password();
        let login_button = button("Cancel").on_press(Self::Message::Cancel);
        let sign_up_button = button("Sign Up").on_press(Self::Message::SignUp);

        column![
            username,
            password,
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
