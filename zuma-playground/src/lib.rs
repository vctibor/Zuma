#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use zumalib::ZumaCompiler;

const EMPTY_SVG: &'static str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1000"></svg>"#;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        compiler: ZumaCompiler::new(),
        source_code: "".to_owned(),
        compiled: None,
    }
}

struct Model {
    compiler: ZumaCompiler,
    source_code: String,
    compiled: Option<String>
}

#[derive(Clone)]
enum Msg {
    SourceCodeChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SourceCodeChanged(updated_source) => {
            model.source_code = updated_source;
            let compiled = model.compiler.compile(model.source_code.clone());
            if let Ok(res) = compiled {
                model.compiled = Some(res)
            };
        } 
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![

        textarea![
            C!["element source_input"],
            input_ev(Ev::Input, Msg::SourceCodeChanged)
        ],
                
        div![
            C!["element render_area"],
            raw![&model.compiled.clone().unwrap_or(EMPTY_SVG.to_owned())]
        ],

        div![
            C!["element cheatsheet"],
            h2!("Cheatsheet"),
            md!(include_str!("../zuma_cheatsheet.md"))
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
