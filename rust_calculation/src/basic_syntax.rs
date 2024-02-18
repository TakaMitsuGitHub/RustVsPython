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

    // 静的配列
    const V:[isize; 3]= [1, 2, 3];
    println!("V = {:?}", V);

    // 動的配列
    let mut v = Vec::<isize>::new();
    println!("空の配列 v = {:?}", v);
    v.push(X);
    v.push(X);
    println!("値が追加された配列 v = {:?}", v);

    integer_for(v);

    // 1文字
    let c:char = 'c';
    println!("c = {}", c);

    //  文字列
    let mut s:&str = "a";
    println!("s = {}", s);
    s = "abc";
    println!("s = {}", s);

    intangible_for(s.chars());

}

pub fn integer_for(v: Vec<isize>) {
    // for文で値を取り出す為の変数に型指定する必要は無い（型推論）
    for i in v {
        println!("i = {}", i);
    }
}

pub fn intangible_for<T>(v: T)
where
T: IntoIterator,
T::Item: std::fmt::Display,
{
    for i in v.into_iter() {
        println!("i = {}", i);
    }
}

