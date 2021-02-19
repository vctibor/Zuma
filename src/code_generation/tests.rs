#![allow(unused_imports)]

use super::*;
use crate::interpretation::{Graphics, GraphicNode};

#[test]
fn test_blur() {

    let expected = r#"
<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000">
    <defs>
        <filter id="f1" x="0" y="0">
            <feGaussianBlur in="SourceGraphic" stdDeviation="15"></feGaussianBlur>
        </filter>
    </defs>
    <rect fill="yellow" filter="url(#f1)" height="90" stroke-width="3" stroke="green" width="90"></rect>
</svg>
    "#.trim();

    let g = Graphics::new()
        .add(
            GraphicNode::empty_element("defs")
                .insert_content(Graphics::new().add(
                    GraphicNode::empty_element("filter")
                        .add_attr("id", "f1")
                        .add_attr("x", "0")
                        .add_attr("y", "0")
                        .insert_content(Graphics::new().add(
                            GraphicNode::empty_element("feGaussianBlur")
                                .add_attr("in", "SourceGraphic")
                                .add_attr("stdDeviation", "15")
                        ))
                ))
        )
        .add(
            GraphicNode::empty_element("rect")
                .add_attr("width", "90")
                .add_attr("height", "90")
                .add_attr("stroke", "green")
                .add_attr("stroke-width", "3")
                .add_attr("fill", "yellow")
                .add_attr("filter", "url(#f1)")
        );

    let output = self::generate(&g);

    // println!("{}", output);

    assert_eq!(output, expected);
}