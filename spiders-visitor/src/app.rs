pub struct App<'a> {
    pub title: &'a str,
    pub quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title: title,
            quit: false,
        }
    }

    pub fn on_up(&mut self) {}
    pub fn on_down(&mut self) {}
    pub fn on_left(&mut self) {}
    pub fn on_right(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => { self.quit = true; }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
    }
}
