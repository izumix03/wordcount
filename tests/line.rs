use std::io::Cursor;
use wordcount::{count, CountOption};
use indoc::indoc;

#[macro_use]
mod utils;

#[test]
fn line_count_works() {
    let input = Cursor::new(
        indoc!("Tokyo, Japan
        Kyoto, Japan
        Tokyo, Japan
        Shanghai, China")
    );
    let freqs = count(input, CountOption::Line);

    assert_map!(freqs, {
        "Tokyo, Japan" => 2;
        "Kyoto, Japan" => 1;
        "Shanghai, China" => 1
    });
}

#[test]
fn linc_count_ifcr() {
    let input = Cursor::new("aa\r\nbb\r\nc\r\nbb");
    let freqs = count(input, CountOption::Line);
    assert_map!(freqs, {"aa" => 1; "bb" => 2; "c" => 1});
}