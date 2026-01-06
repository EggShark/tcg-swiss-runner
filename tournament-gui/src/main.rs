fn main() {
    println!("Hello World!");
    let _ = iced::run(TournamentApp::update, TournamentApp::view);
}

type Message = ();

#[derive(Default)]
struct TournamentApp;

impl TournamentApp {
    fn update(&mut self, _message: Message) {

    }

    fn view(&self) -> iced::Element<Message> {
        "hello world!".into()
    }
}
