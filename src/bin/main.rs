use p5::app::App;
use p5::canvas::Canvas;
use p5::shape::line::Line;

#[derive(Clone)]
struct Context {
    blue: u8,
    x: f32,
}

fn main() {
    let mut app = App::new(String::from("Hello from p5.rs"));
    let canvas = Canvas::from_size(400, 400);
    let context = Context { blue: 60, x: 0. };

    app.set_canvas(canvas);
    app.set_context(context);
    app.draw(draw);
}

fn draw(canvas: &mut Canvas, context: &mut Context) {
    canvas.background([0x50, 0x60, context.blue, 0xff]);
    canvas.stroke([0xff, 0xff, 0, 0x88]);

    context.blue += 1;

    canvas.add(Line::new(10., 10., context.x, 100.));

    context.x += 0.8;

    if context.blue > 254 {
        context.blue = 0x00;
    }
}
