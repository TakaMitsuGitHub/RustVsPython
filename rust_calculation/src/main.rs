
use plotters::prelude::*;
use chrono::offset::Local;
use chrono::Date;

mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  /* (1) プロット用データの準備 */

  // データを取得。この時点では(日付,値)のタプルのVector型になっている
  let data = data::get_data();

  /* x軸とy軸で個別のVector型にする */

  // x軸 : 日付のVector
  let xs: Vec<Date<Local>> = data.iter()
                                 .map(|(x, _)| data::parse_time(*x))
                                 .collect();
  // y軸: 値のVector
  let ys: Vec<f32> = data.iter()
                         .map(|(_, y)| *y)
                         .collect();


  /* (2) 描画先の情報を設定 */

  let image_width = 1080;
  let image_height = 720;
  // 描画先を指定。画像出力する場合はBitMapBackend
  let root = BitMapBackend::new
    ("plot.png", (image_width, image_height)).into_drawing_area();

  // 背景を白にする
  root.fill(&WHITE)?;


  /* (3) グラフ全般の設定 */

  /* y軸の最大最小値を算出
     f32型はNaNが定義されていてys.iter().max()等が使えないので工夫が必要
     参考サイト
     https://qiita.com/lo48576/items/343ca40a03c3b86b67cb */
  let (y_min, y_max) = ys.iter()
                         .fold(
                           (0.0/0.0, 0.0/0.0),
                           |(m,n), v| (v.min(m), v.max(n))
                          );

  let caption = "Sample Plot";
  let font = ("sans-serif", 20);

  let mut chart = ChartBuilder::on(&root)
    .caption(caption, font.into_font()) // キャプションのフォントやサイズ
    .margin(10)                         // 上下左右全ての余白
    .x_label_area_size(16)              // x軸ラベル部分の余白
    .y_label_area_size(42)              // y軸ラベル部分の余白
    .build_cartesian_2d(                // x軸とy軸の数値の範囲を指定する
      *xs.first().unwrap()..*xs.last().unwrap(), // x軸の範囲
      y_min..y_max                               // y軸の範囲
    )?;


  /* (4) グラフの描画 */

  // x軸y軸、グリッド線などを描画
  chart.configure_mesh().draw()?;

  // 折れ線グラフの定義＆描画
  let line_series = LineSeries::new(
                    xs.iter()
                      .zip(ys.iter())
                      .map(|(x, y)| (*x, *y)),
                    &RED
                   );
  chart.draw_series(line_series)?;

  Ok(())
}



// mod polars_calculation;
// fn main() {
//     match polars_calculation::read_csv() {
//         Ok(_) => {
//             // 成功した場合の処理
//             println!("CSVの読み込みに成功しました");
//         },
//         Err(e) => {
//             // エラーの場合の処理、例えばエラーメッセージを表示する
//             eprintln!("CSVの読み込みに失敗しました: {}", e);
//         }
//     }
// }


// use std::time::Instant;
// mod prime_number_calculation;

// const LIMIT:usize = 1000000;
// const NUM:usize = 10;
// fn main() {
//     // 型注釈は必要ない場合、自動的に推論される
//     let mut v = Vec::new();
//     for _ in 0..NUM {
//         let start = Instant::now();
//         prime_number_calculation::sieve_of_eratosthenes(LIMIT);
//         let duration = start.elapsed();
        
//         println!("処理時間: {:?}", duration);
//         v.push(duration.as_secs_f64());
//     }

//     let sum: f64 = v.iter().sum();
//     let average = sum / v.len() as f64;
//     println!("平均処理時間: {:?}s", average);

// }
