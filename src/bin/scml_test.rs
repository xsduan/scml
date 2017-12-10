extern crate scml;

fn main() {
    scml::transform(scml::read("src/bin/examples/scml_65e5.json").unwrap())
}
