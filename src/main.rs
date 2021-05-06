//! wordcount はシンプルな文字、単語、行の出現頻度の計測機能を提供します。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見て下さい。
#![warn(missing_docs)]

// 1. 引数の読み込み
// 2. ファイルを開く
// 3. ファイルから1行ずつ読む
// 4. 単語で分割
// 5. 単語の出現頻度を数える

use std::io::BufReader;
use std::fs::File;
use std::env;
use wordcount::count;

fn main() {
    // 1. 引数の読み込み
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2. ファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読む
    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
