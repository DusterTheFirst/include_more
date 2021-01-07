use include_more::include_sass;

fn main() {
    let css = include_sass!("examples/scss.scss");

    println!("{}", css);
}
