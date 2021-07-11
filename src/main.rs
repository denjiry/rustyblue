use rustyblue::parser::chartparser;

fn main() {
    let mut input = String::from("ソクラテスは死ぬ。");
    let mut input = chartparser::Input { sentence: input };
    let node = input.simple_parse(10);
    dbg!(node);
}
