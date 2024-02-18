use std::time::Instant;

mod prime_number_calculation;

const NUM:usize = 10000;
fn main() {
    // 型注釈は必要ない場合、自動的に推論される
    let mut v = Vec::new();
    for _ in 0..10 {
        let start = Instant::now();
        prime_number_calculation::sieve_of_eratosthenes(NUM);
        let duration = start.elapsed();
        
        println!("処理時間: {:?}", duration);
        v.push(duration.as_secs_f64());
    }

    let sum: f64 = v.iter().sum();
    let average = sum / v.len() as f64;
    println!("平均処理時間: {:?}s", average);

}
