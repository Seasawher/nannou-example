/* 白い背景の中に青い正方形を表示する */

use nannou::prelude::*;

struct Model {}

fn main() {
  // 600x400 のウィンドウを用意
  nannou::app(model).simple_window(view).size(600, 400).run();
}

fn model(_app: &App) -> Model {
  Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
  // キャンバスを取得
  let draw = app.draw();

  // 背景色を設定
  draw.background().color(WHITE);

  // 1辺100の正方形を原点に表示
  draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

  // フレームに書き出し
  draw.to_frame(app, &frame).unwrap();
}
