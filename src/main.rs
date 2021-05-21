use nannou::prelude::*;

struct Model {
    _window: WindowId,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for i in 0..3 {
        let r = 100.0 - (i.to_f32().unwrap() * 20.0);
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(r)
            .stroke(WHITE)
            .stroke_weight(2.0)
            .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
