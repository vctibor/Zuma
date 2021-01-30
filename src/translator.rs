// This sits on top of both Zuma parser and SVG generator, and translates from Zuma IR to svg_model.

use crate::parsing::zuma_model;
use crate::svg_generator::*;

/// Translate from Zuma IR into SVG string.
pub fn translate(zuma_ir: zuma_model::Document) -> String {
    let mut doc = Document::new();
    
    doc.generate()
}