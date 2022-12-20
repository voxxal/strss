pub struct State {
    pub scroll: i32,
}

impl State {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }

    pub fn on_tick(&self) {}

    pub fn scroll_up(&mut self) {
        self.scroll = (self.scroll - 1).max(0);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }
}
