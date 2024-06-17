/* クリックすると正方形と背景の色がそれぞれランダムに変わる

ただし画面のどこをクリックしても色が変わる
*/

use nannou::prelude::*;

// 現在設定している色の状態を保持
struct Model {
  rec_color: Rgb8,
  back_color: Rgb8,
}

fn main() {
  nannou::app(model).run();
}

fn model(app: &App) -> Model {
  // イベントハンドラなどを設定
  app
    .new_window()
    .size(600, 400)
    .mouse_pressed(mouse_pressed)
    .view(view)
    .build()
    .unwrap();

  // 色の初期値を指定してインスタンス生成
  Model {
    rec_color: gen_random_color(),
    back_color: gen_random_color(),
  }
}

// クリック時のイベントハンドラ
fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
  model.rec_color = gen_random_color();
  model.back_color = gen_random_color();
}

// ランダムな色を返す自作関数
fn gen_random_color() -> Rgb8 {
  let r = random::<u8>();
  let g = random::<u8>();
  let b = random::<u8>();
  let random_color = rgb8(r, g, b);
  random_color
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(model.back_color);

  draw
    .rect()
    // 設定した色で塗りつぶす
    .color(model.rec_color)
    .x_y(0.0, 0.0)
    .w_h(100.0, 100.0);

  draw.to_frame(app, &frame).unwrap();
}
