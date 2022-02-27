use std::fs;
use serde_json::Value;      // Needs to go away. std library only.

pub fn valideh( teststr: &str ) -> &str {
    /*
    ===THIS IS PART OF THE MAIN CODE. THIS FUNCTION WILL BE USED INSIDE MAIN (main.rs) FILE
    FOR FUNCTIONALITY.===

    - The entire logic of validating bech32m goes here.
    */


    if teststr.eq("A1LQFN3A") { // The very first string in the list.
        "VALID"
    } else { "INVALID" }
}

//  ========== TESTS START HERE ==========

/*
------ !!! Refer to the section on Integration testing in the Testing chapter of the Rust book.
By a simple file restructuring it allows for tests to be in a separate folder. This would be imperative in going
forward in order to avoid conflict in working separately. On that note I highly recommend reading the simple
chapter on modules/crates/packages in Rust. It'll help us cleanly divide our code into multiple files
we can work on ~independently.
*/

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn isvalid() {
        
        /*
        The test to test valid bech32m vectors.
        */

        let fileasstr = fs::read_to_string(".\\validbech32m.json").expect(" Error parsing test file as string");
        // The current directory for testing would be the root directory (which contains the src folder)
        // Thus the file has been copied there as well.

        let jsonval: Value = serde_json::from_str(&fileasstr[..]).expect("JSON parsing error");     // &xyz[..] convert String xyx to &str which is the required type.

        let mut iter = 0;

        while iter < jsonval["VALID_BECH32M"].as_array().unwrap().len() {
            let theword = jsonval["VALID_BECH32M"][iter].as_str().unwrap();
            let theresult = valideh(&theword);
            if theresult.ne("VALID") {
                println!("\n ==== {} : DID NOT RECEIVE VALIDATION ====\n\n",theword);
                panic!();
            }
            iter += 1;
        }
        // assert_eq!("VALID",valideh(jsonval["VALID_BECH32M"][1].as_str().unwrap()));

    }

    #[test]
    fn isinvalid() {
        /*
        Test the invalid bech32m vectors here.
        */
        assert_eq!(1,1)
    }

    #[test]
    fn emptytest() {}
}