extern crate lalrpop;

fn main() {
    //lalrpop::process_root().unwrap();



    lalrpop::Configuration::new()
        .set_in_dir("src/parsing")
        .process()
        .unwrap();
}