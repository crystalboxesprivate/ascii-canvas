mod display;

fn main() {
    let screen_buffer = display::ScreenBuffer::new(60, 30);
    display::clear_screen();
    display::draw(&screen_buffer, "█▇▆▅▄▃▂▁ ");
}
