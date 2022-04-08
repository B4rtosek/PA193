use semproject::*;

fn main() {
    let hrp = "bc";
    let data = "947927b70496f32c9d6c35835ed714459965403137";
    dbg!(encode(hrp, data));

    dbg!(decode("BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4"));
}
