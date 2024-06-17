/* Coding Train より. Bubble Sort の可視化 */

// nannouの基本的なツール群を使えるようにする
use nannou::prelude::*;

// 実行される関数のルート
fn main() {
  nannou::app(model).simple_window(view).size(600, 600).run();
}

// Model は，状態を管理するところ
// ここでは状態はなく，静的な表示を行うだけなので，空で構わない
struct Model {
  values: Vec<u32>,
}

fn model(_app: &App) -> Model {
  Model {
    values: initialize_values(),
  }
}

fn initialize_values() -> Vec<u32> {
  let mut values = vec![];
  for _i in 0..600 {
    values.push(random_range(0, 600));
  }
  values
}

// fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  // 背景色を単に黒色にする
  draw.background().color(BLACK);

  // 現在の Vec の内容を描画する
  // Model.values のそれぞれの要素に対して，それぞれの要素の値に応じた白い線を描画する
  for (i, value) in model.values.iter().enumerate() {
    let start_x = (i as f32) - 300.0;
    let start_y = -300.0 as f32;
    draw
      .line()
      .start(pt2(start_x, start_y))
      .end(pt2(start_x as f32, (start_y as f32) + *value as f32))
      .color(WHITE);
  }

  draw.to_frame(app, &frame).unwrap();
}
