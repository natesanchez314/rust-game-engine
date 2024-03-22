pub trait Game {
    fn resize(&mut self, new_size: (u32, u32));
    fn update(&mut self, delta: instant::Duration);
    fn render();
}