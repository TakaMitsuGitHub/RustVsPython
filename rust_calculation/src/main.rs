
// use plotters::prelude::*;
// use chrono::offset::Local;
// use chrono::Date;





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


use std::time::Instant;
mod prime_number_calculation;

const LIMIT:usize = 1000000;
const NUM:usize = 10;
fn main() {
    // 型注釈は必要ない場合、自動的に推論される
    let mut v = Vec::new();
    for _ in 0..NUM {
        let start = Instant::now();
        prime_number_calculation::sieve_of_eratosthenes(LIMIT);
        let duration = start.elapsed();
        
        println!("処理時間: {:?}", duration);
        v.push(duration.as_secs_f64());
    }

    let sum: f64 = v.iter().sum();
    let average = sum / v.len() as f64;
    println!("平均処理時間: {:?}s", average);

}
