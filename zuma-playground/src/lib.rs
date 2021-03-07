#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use zumalib::ZumaCompiler;

const EMPTY_SVG: &str =
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500"></svg>"#;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        compiler: ZumaCompiler::new(),
        source_code: "".to_owned(),
        compiled: None,
        error: None,
    }
}

struct Model {
    compiler: ZumaCompiler,
    source_code: String,
    compiled: Option<String>,
    error: Option<String>,
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

            match compiled {
                Ok(res) => {
                    model.compiled = Some(res);
                    model.error = None;
                }
                Err(e) => model.error = Some(e.to_string()),
            }
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["container-fluid row align-items-start"],
            textarea![
                id!["source_input"],
                C!["col-4 element source_input"],
                input_ev(Ev::Input, Msg::SourceCodeChanged),
                attrs![At::SpellCheck => false]
            ],
            div![
                C!["col-6 element render_area"],
                raw![&model
                    .compiled
                    .clone()
                    .unwrap_or_else(|| EMPTY_SVG.to_owned())]
            ],
            div![
                C!["col-1"],
                model
                    .error
                    .clone()
                    .unwrap_or_else(|| "No errors.".to_owned())
            ]
        ],
        div![
            C!["container-fluid"],
            div![
                C!["element cheatsheet"],
                h2!("Cheatsheet"),
                md!(include_str!("../zuma_cheatsheet.md"))
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
