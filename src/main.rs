use semproject::{data_to_int, hrp_expand};


fn main() {
    let text = "suyash";
    let resp = data_to_int(text);
    println!("{:?}",resp);
    println!(" ANOTHER TEST: ");
    let c = 'a';
    println!("{}", (c as u8) << 1);
    println!("{}", (c as u8) & 31);
    let textexp = hrp_expand(text);
    println!("Text is {}\nExpanded to {:?}",text,textexp);
}
