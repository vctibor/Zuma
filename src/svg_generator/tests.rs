#![allow(unused_imports)]

use crate::svg_generator::Document;
use crate::svg_generator::Line;

#[test]
fn svg_gen_test_line() {
    let actual = Document::new()
        .add(Line::new(0.0, 0.0, 1.0, 10.0)
            .color(128, 25, 45)
            .width(3.0)
            .into()
        )
        .generate();

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <line style="stroke-width:3;stroke:rgb(128,25,45)" x1="0" x2="1" y1="0" y2="10"/>
</svg>
    "#.trim();

    assert_eq!(actual, expected);
}