use include_more::include_ts;

fn main() {
    let css = include_ts!("examples/ts.ts");

    println!("{}", css);
}
