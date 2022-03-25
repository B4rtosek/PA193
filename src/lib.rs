use std::fs;
use serde_json::Value;      // Hardcode the test vectors in the code and remove this.


const BECH32M_CONST : usize = 0x2bc830a3;
const DATA_LUT : [&'static str; 4] = ["qpzry9x8","gf2tvdw0","s3jn54kh","ce6mua7l"];

pub fn data_to_int(data: &str) -> Vec<usize> {
    let dataiter = data.chars();
    let mut dataint: Vec<usize> = Vec::new();
    for i in dataiter {
        for j in 0 .. 4 {
            if let Some(v) = DATA_LUT[j].find(i) {
                let val = (8*j + v) as usize;
                dataint.push(val);
            };
        }
    }
    dataint
}

pub fn polymod(values: &Vec<usize>) -> usize {
    let GEN: Vec<usize> = vec![0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    let mut chk = 1 as usize;

    for i in values {
        let b = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ (*i as usize);
        for j in 0 .. 5 {
            chk ^= if ((b >> j) & 1) != 0 { GEN[j] } else { 0 as usize };
        }
    }

    chk
}

pub fn verify_checksum(hrp: &str, data: &str) -> bool {
    
    println!("💀 Received {} as hrp and {} as data", hrp, data);
    let mut hrp = hrp_expand( hrp );
    println!("💀 HRP expanded to: {:?}", &hrp);
    let mut data = data_to_int(data);
    println!("💀 Data to int looks like: {:?}", &data);
    hrp.append(&mut data);
    println!("💀 Sending {:?} to polymod", &hrp);
    let res = polymod( &hrp );
    println!("💀 Result of the polymod is => {}", &res);
    res == BECH32M_CONST
}

pub fn create_checksum( hrp: &str, data: &str) -> Vec<usize> {
    let mut values: Vec<usize> = hrp_expand(hrp);
    let mut data = data_to_int(data);
    values.append(&mut data);
    let mut zerosvec: Vec<usize> = vec![0,0,0,0,0,0];
    values.append(&mut zerosvec);
    let polymod_res = polymod(&values) ^ BECH32M_CONST;
    let mut checksum_vec: Vec<usize> = Vec::with_capacity(6);
    for i in (0..6).rev() {
        checksum_vec.push((polymod_res >> 5 * i) & 31)
    }
    checksum_vec
}

pub fn hrp_expand( hrp: &str) -> Vec<usize> {
    let hrpiter = hrp.chars();
    let hrpiter2 = hrpiter.clone();     // A better way would be to borrow the iterator into the loop but I am not in the mood to do it rn.
    // The clone is required as the iterator is consumed upon use. More specifically, it moves into the for loop when being called as it is.
    let mut hrpx: Vec<usize> = Vec::new();
    for c in hrpiter {
        hrpx.push((c as usize) >> 5);
    }
    hrpx.push(0 as usize);
    for c in hrpiter2 {
        hrpx.push((c as usize) & 31);
    }
    hrpx
}

pub fn encode(hrp: &str, data: &str) -> String {
    let checksum = create_checksum(hrp, data);
    let dataint = data_to_int(data);
    let mut combined: Vec<usize> = Vec::new();
    let mut dataiter = dataint.iter();
    let mut dataindex = dataiter.next(); // Note the iterator provides &usize return despite arraw being of usize elements.
    while dataindex != None {
        combined.push(*dataindex.unwrap());
        dataindex = dataiter.next();
    }
    let mut checksum_iter = checksum.iter();
    let mut checksumindex = checksum_iter.next();
    while checksumindex != None {
        combined.push(*checksumindex.unwrap());
        checksumindex = checksum_iter.next();
    }

    let mut encoded_string = String::new();
    encoded_string.push_str(hrp);
    encoded_string.push('1');
    
    for i in combined {
        let string_index = (i/8) as usize;
        let holder_string = DATA_LUT[string_index];
        encoded_string.push(holder_string.chars().nth(i - 8*string_index).unwrap());
    }

    encoded_string
}



pub fn valideh( teststr: &str ) -> &str {
    /*
    ===THIS IS PART OF THE MAIN CODE. THIS FUNCTION WILL BE USED INSIDE MAIN (main.rs) FILE
    FOR FUNCTIONALITY.===

    - The entire logic of validating bech32m goes here.
    */

    let response;

    // Separator check.
    // THis test should actually follow the length check. It gets tested implicitly when bifurcating the code into data and hrp.
    if teststr.contains("1") {

        // Length check.
        let teststrlen = teststr.len();
        if teststrlen <= 90 && teststrlen > 0 {

        // let teststrparts: Vec<&str> = teststr.split("1").collect();
        // let hrp = teststrparts[..teststrparts.len()-1].join("");
        // let datapart = String::from(*teststrparts.last().unwrap());
        
        let hrp: &str;
        let datapart: &str;
        
        /*
        Better logic exists using &str::rfind()
        Maybe even rsplit_once()
        */
        let mut separator: usize = 0;
        let teststriter = teststr.chars().enumerate();
        for i in teststriter {
            let (ind, val) = i;
            if val == '1' {
                separator = ind;
            }
        }


        hrp = &teststr[..separator];
        datapart = &teststr[separator+1..];

        // dbg!(&hrp);
        // dbg!(&datapart);
        println!("🔛 {} is split into {} as hrp and {} as data", teststr, hrp, datapart);

        if hrp.len() > 0 {

            let hrpiter = hrp.chars();
            let mut casecheckflag: usize = 0;      //0 = not set. 1 = uppercase chars. 2 = lowercase chars.
            for i in hrpiter{
                // dbg!(&i);
                // Valid character range check
                if i >= '\u{0021}' && i <= '\u{007E}' {     // ASCII 33 - 126 translation into UTF-8
                    // println!("valid char range test pass")
                    //Mix case check
                    if i.is_lowercase(){
                        if casecheckflag == 1 {
                            response = "INVALID: MIX CASE HRP";
                            println!("{}",response);
                            return response;
                        }
                        casecheckflag = 2;
                    } else if i.is_uppercase(){
                        if casecheckflag == 2 {
                            response = "INVALID: MIX CASE HRP";
                            println!("{}",response);
                            return response;
                        }
                        casecheckflag = 1;
                    }

                    /*
                    I believe the HRP checking ends here. Hereafter we need to verify the data(checksum) section.

                    The data part, which is at least 6 characters long and only consists of alphanumeric characters excluding "1", "b", "i", and "o"[4].
                    */
                    let hrp = hrp.to_ascii_lowercase();
                    let hrp = hrp.as_str();
                    let datapart = datapart.to_ascii_lowercase();
                    let datapart = datapart.as_str();
                    if datapart.len() < 6 {
                        response = "INVALID: DATAPART LENGTH TOO SMALL";
                        println!("{}",response);
                        return response;
                    } else {
                        let datachars = datapart.chars();
                        for i in datachars {
                            if i.is_ascii_alphanumeric() && i != '1' && i != 'b' && i != 'i' && i != 'o' {
                                
                                /*
                                Data character validity testing ends here. The tests to compute and test the checksums should follow.
                                */
                                if verify_checksum(hrp, datapart) { 
                                    response = "VALID";
                                } else {
                                    response = "INVALID: CHECKSUM VALIDATION FAILED";
                                }

                                return response;
                            } else {
                                response = "INVALID: INVALID CHARACTERS IN DATA";
                                println!("{}",response);
                                return response;                 
                            }
                        }
                    }
                    


                } else {
                    response = "INVALID: HRP INVALID CHARACTER";
                    println!("{}",response);
                    return response;
                }
            }

        } else {
            response = "INVALID: HRP EMPTY";
            println!("{}",response);
            return response;
        }



        } else {
            response = "INVALID: LENGTH OUT OF RANGE";
            println!("{}",response);
            return response;
        }
    } else {
        response = "INVALID: SEPARATOR NOT FOUND";
        println!("{}",response);
        return response;
    }
    


    // Dummy test for testing tests. Remove before submission.
    // if teststr.eq("A1LQFN3A") { // The very first string in the list.
    //     "VALID"
    // } else { "INVALID" }
    "YOU SHOULDNT BE HERE"
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

        let fileasstr = fs::read_to_string("./validbech32m.json").expect(" Error parsing test file as string");
        // The current directory for testing would be the root directory (which contains the src folder)
        // Thus the file has been copied there as well.

        let jsonval: Value = serde_json::from_str(&fileasstr[..]).expect("JSON parsing error");     // &xyz[..] convert String xyx to &str which is the required type &str.

        let mut iter = 0;

        while iter < jsonval["VALID_BECH32M"].as_array().unwrap().len() {
            let theword = jsonval["VALID_BECH32M"][iter].as_str().unwrap();
            let theresult = valideh(&theword);
            if theresult.ne("VALID") {
                println!("\n ==== {} : DID NOT RECEIVE VALIDATION ====\n\n",theword);
                println!("{}",theresult);
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