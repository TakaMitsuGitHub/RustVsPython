// グローバル変数
const X:isize = 10;

pub fn variable() {
    println!("X = {}", X);

    // 変数定義
    let x:isize = 5;
    println!("x = {}", x);

    // 変数の再定義
    let mut x:isize = x + 1;
    println!("x = {}", x);

    // 変数に再代入
    x = 1;
    println!("x = {}", x);


}