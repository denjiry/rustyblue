use rustyblue::parser::chartparser;

fn main() {
    let input = String::from("ソクラテスは死ぬ。");
    let node = chartparser::simple_parse(&input, 10);
    dbg!(node.unwrap());
}
