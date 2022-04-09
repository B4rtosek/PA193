use std::env;
mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    ui::ui("stdin", "stdout");
}
