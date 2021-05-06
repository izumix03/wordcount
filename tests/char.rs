use std::collections::HashMap;
use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn word_count_works() {
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}


#[test]
fn word_count_works2() {
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);
    assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    count(
        Cursor::new([
            b'a', // a
            0xf0, 0x90, 0x80, // でたらめなバイト列
            0xe3, 0x81, 0x82, // あ
        ]),
        CountOption::Word,
    );
}

#[test]
fn word_count_works3() {
    let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);

    assert_eq!(freqs.len(), 3);
    assert_map!(freqs, {"aa" => 1; "cc" => 1; "dd" => 1});
}
