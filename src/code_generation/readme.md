Code for generating SVG from Rust. I would like to split this into separate crate eventually.

Expected API:

    Document::new()
        .add(Line::new(0.0, 0.0, 1.0, 10.0)
            .color(Color::new(128, 25, 45))
            .width(3.0)
        )
    .generate()