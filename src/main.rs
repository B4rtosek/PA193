use semproject::decode_hex;
mod ui;

fn main() {
    dbg!(decode_hex("bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqzk5jj0"));
    /*let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    ui::ui("stdin", "stdout");*/
}
