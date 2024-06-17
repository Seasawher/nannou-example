/* 青い正方形を回転させるアニメージョン */

use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).simple_window(view).size(600, 400).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

// アプリケーションが起動している間、ループ
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    // アプリケーションが起動してからの秒数を t に格納
    let t = app.time;

    // sin、cos を使って円運動を表現
    let center = pt2(t.cos(), t.sin()) * 100.0;

    draw.rect()
        .x_y(center.x, center.y)
        .w_h(100.0, 100.0)
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}