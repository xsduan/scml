extern crate scml;

fn main() {
    println!("{:#?}", scml::read("src/bin/examples/scml_65e5.json").unwrap());
}
