extern crate scml;

fn main() {
    let scml_str = r##"
        <scml>
            <stroke type="s">
                <anchor id="0" at="begin" />
                <anchor id="1" at="inside" />
                <anchor id="2" at="inside" />
            </stroke>
            <stroke type="hz">
                <anchor id="0" at="begin" />
                <anchor id="3" at="inside1" />
                <anchor id="4" at="end" />
            </stroke>
            <stroke type="h">
                <anchor id="1" at="begin" />
                <anchor id="3" at="end" />
            </stroke>
            <stroke type="h">
                <anchor id="2" at="begin" />
                <anchor id="4" at="end" />
            </stroke>
        </scml>
    "##;

    println!("{:#?}", scml::parse::read(scml_str));
}
