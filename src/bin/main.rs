use p5::app::App;
use p5::canvas::Canvas;

#[derive(Clone)]
struct Context {
    blue: u8,
}

fn main() {
    let mut app = App::new(String::from("Hello from p5.rs"));
    let canvas = Canvas::from_size(400, 400);
    let context = Context { blue: 60 };

    app.set_canvas(canvas);
    app.set_context(context);
    app.draw(draw);
}

fn draw(canvas: &mut Canvas, context: &mut Context) {
    canvas.background([0x50, 0x60, context.blue, 0xff]);

    context.blue += 1;

    if context.blue > 254 {
        context.blue = 0x00;
    }
}
