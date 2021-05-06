use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;

/// input から1行ずつ UTF-8 文字列を読み込み、頻度を数える
///
/// 頻度を数える対象はオプションによって制御される
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicode の1文字ごと
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): \n または \r\n で区切られた1行ごと
///
/// # Exaples
/// 入力中の単語の出現頻度を数える例
///
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
/// 入力が UTF-8 でフォーマットされていない場合にパニックする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();

        use crate::CountOption::*;

        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                // 4. 単語で分割
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    // 5. 単語の出現頻度を数える
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}

/// ['count'](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度を数える
    Char,
    /// 単語ごとに頻度を数える
    Word,
    /// 行ごとに頻度を数える
    Line,
}

/// オプションのデフォルトは['Word'](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

