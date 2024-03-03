
// use plotters::prelude::*;
// use chrono::offset::Local;
// use chrono::Date;

// mod data;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("stacked_bar_chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Stacked Bar Chart", ("sans-serif", 40))
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(0..10, 0..30)?;

    chart.configure_mesh().draw()?;

    let data = vec![
        (0, 10, BLUE),
        (1, 15, RED),
        (2, 8, GREEN),
        // ... 他のデータ ...
    ];

    for (idx, value, color) in data {
        chart.draw_series(
            std::iter::once(Rectangle::new([(idx, 0), (idx + 1, value)], color.filled())),
        )?;
    }

    // 積み上げるデータの追加
    let data2 = vec![
        (0, 5, YELLOW),
        (1, 10, MAGENTA),
        (2, 12, CYAN),
        // ... 他のデータ ...
    ];

    for (idx, value, color) in data2 {
        chart.draw_series(
            std::iter::once(Rectangle::new([(idx, value), (idx + 1, value * 2)], color.filled())),
        )?;
    }

    root.present()?;
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
