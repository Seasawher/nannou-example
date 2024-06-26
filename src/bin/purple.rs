/* 単なる紫色のウィンドウを表示する */

// nannouの基本的なツール群を使えるようにする
use nannou::prelude::*;

// 実行される関数のルート
// 実行されるのはこの関数のみで，ほかの関数はこの関数内で呼び出される
// nannou においては短い関数になっていることが多い
fn main() {
  nannou::app(model).update(update).simple_window(view).run();
}

// Model は，状態を管理するところ
// ここでは状態はなく，静的な表示を行うだけなので，空で構わない
struct Model {}

fn model(_app: &App) -> Model {
  Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
  // 背景色を単に紫色にする
  frame.clear(PURPLE);
}
