/*  */

// nannouの基本的なツール群を使えるようにする
use nannou::prelude::*;

/* 定数 */
// 点の初期位置
const X: f32 = 0.01;
const Y: f32 = 0.0;
const Z: f32 = 0.0;

// 微分方程式の係数
const A: f32 = 10.0;
const B: f32 = 28.0;
const C: f32 = 8.0 / 3.0;

const DT : f32 = 0.01;

const SCALE: f32 = 10.0;
const MAX_POINTS: usize = 5000;
const COLOR_SPEED : f32 = 10.0;

fn main() {
  nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
  // 現在の x, y, z の値
  x: f32,
  y: f32,
  z: f32,
  // 軌跡を保存するためのベクタ
  traj: Vec<Point3>,
}

fn model(_app: &App) -> Model {
  Model {
    x: X,
    y: Y,
    z: Z,
    traj: vec![],
  }
}

fn update(_app: &App, model: &mut Model, _update: Update) -> () {
  let (x, y, z) = euler_method(model.x, model.y, model.z, DT);
  model.x = x;
  model.y = y;
  model.z = z;

  // 新しい点を軌跡に追加する
  model.traj.push(pt3(model.x, model.y, model.z));

  // 必要に応じて古い位置を削除してメモリを節約
  if model.traj.len() > MAX_POINTS {
    model.traj.remove(0);
  }
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  // 背景色を半透明の黒色にする
  let bg_color = Rgba::new(0.0, 0.0, 0.0, 0.3);
  draw.background().color(bg_color);

  // ベクターの各点を線でつなぎながら描画する
  for (i, window) in model.traj.windows(2).enumerate() {
    let x0 = window[0].x;
    let y0 = window[0].y;
    let z0 = window[0].z;
    let x1 = window[1].x;
    let y1 = window[1].y;
    let z1 = window[1].z;

    let hue = (i as f32) / (MAX_POINTS as f32) * COLOR_SPEED;
    let color = hsla(hue,0.7, 0.7, 1.0);

    draw
      .scale(SCALE)
      .line()
      .start(pt3(x0, y0, z0).into())
      .end(pt3(x1, y1, z1).into())
      .weight(0.1)
      .color(color);
  }

  // 描画を完了
  draw.to_frame(app, &frame).unwrap();
}

/// Euler 法を使って Lorenz 方程式の初期値問題を解く.
/// 次のステップの x, y, z を返す.
fn euler_method(x: f32, y: f32, z: f32, dt: f32) -> (f32, f32, f32) {
  let dx = (A * (y - x)) * dt;
  let dy = (x * (B - z) - y) * dt;
  let dz = (x * y - C * z) * dt;

  (x + dx, y + dy, z + dz)
}
