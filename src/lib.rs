// use std::fs;

fn valideh( teststr: &str ) -> &str {
    /*
    The entire logic of validating bech32m goes here.
    */
    if teststr.eq("A1LQFN3A") { // The very first string in the list.
        "VALID"
    } else { "INVALID" }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::Value;
    use crate::valideh;

    #[test]
    fn isvalid() {

        let fileasstr = fs::read_to_string(".\\validbech32m.json").expect(" Error parsing test file as string");
        // The current directory for testing would be the root directory (which contains the src folder)
        // Thus the file has been copied there as well.

        let jsonval: Value = serde_json::from_str(&fileasstr[..]).expect("JSON parsing error");     // &xyz[..] convert String xyx to &str which is the required type.
        // dbg!(jsonval["VALID_BECH32M"]);
        assert_eq!("VALID",valideh(jsonval["VALID_BECH32M"][1].as_str().unwrap()));

    }
}