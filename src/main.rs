use rustyblue::parser::chartparser;

fn main() {
    let mut input = String::new("ソクラテスは死ぬ。");
    let mut input = chartparser::Input { sentence: input };
    let node = chartparser::simple_parse(10);
    dbg!(node);
}
