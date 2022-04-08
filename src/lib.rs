use serde_json::Value;
use std::{fs, io::Error, io::ErrorKind, result}; // Hardcode the test vectors in the code and remove this.

const BECH32M_CONST: usize = 0x2bc830a3;
const DATA_LUT: [&'static str; 4] = ["qpzry9x8", "gf2tvdw0", "s3jn54kh", "ce6mua7l"];

pub struct ValidationResponse {
    result: bool,
    reason: String,
}

pub fn data_to_int(data: &str) -> Vec<usize> {
    let dataiter = data.chars();
    let mut dataint: Vec<usize> = Vec::new();
    for i in dataiter {
        for j in 0..4 {
            if let Some(v) = DATA_LUT[j].find(i) {
                let val = (8 * j + v) as usize;
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
        for j in 0..5 {
            chk ^= if ((b >> j) & 1) != 0 {
                GEN[j]
            } else {
                0 as usize
            };
        }
    }

    chk
}

pub fn verify_checksum(hrp: &str, data: &str) -> bool {
    println!("💀 Received {} as hrp and {} as data", hrp, data);
    let mut hrp = hrp_expand(hrp);
    println!("💀 HRP expanded to: {:?}", &hrp);
    let mut data = data_to_int(data);
    println!("💀 Data to int looks like: {:?}", &data);
    hrp.append(&mut data);
    println!("💀 Sending {:?} to polymod", &hrp);
    let res = polymod(&hrp);
    println!("💀 Result of the polymod is => {}", &res);
    res == BECH32M_CONST
}

pub fn create_checksum(hrp: &str, data: &str) -> Vec<usize> {
    let mut values: Vec<usize> = hrp_expand(hrp);
    let mut data = data_to_int(data);
    values.append(&mut data);
    let mut zerosvec: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    values.append(&mut zerosvec);
    let polymod_res = polymod(&values) ^ BECH32M_CONST;
    let mut checksum_vec: Vec<usize> = Vec::with_capacity(6);
    for i in (0..6).rev() {
        checksum_vec.push((polymod_res >> 5 * i) & 31)
    }
    checksum_vec
}

pub fn hrp_expand(hrp: &str) -> Vec<usize> {
    let hrpiter = hrp.chars();
    let hrpiter2 = hrpiter.clone(); // A better way would be to borrow the iterator into the loop but I am not in the mood to do it rn.
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

pub fn encode(hrp: &str, data: &str) -> Result<String, Error> {
    /*
    Unsafe function. Doesn't check the validity of hrp and data strings.
    */
    let hrpvalres = hrp_valideh(hrp);
    if !hrpvalres.result {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("Error with HRP: {}", hrpvalres.reason),
        ));
    }
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
        let string_index = (i / 8) as usize;
        let holder_string = DATA_LUT[string_index];
        encoded_string.push(holder_string.chars().nth(i - 8 * string_index).unwrap());
    }

    Ok(encoded_string)
}

fn hrp_valideh(hrp: &str) -> ValidationResponse {
    let hrp_len = hrp.len();

    let mut response = String::new();
    if hrp_len >= 1 && hrp_len <= 83 {
        let hrpiter = hrp.chars();
        let mut casecheckflag: usize = 0; //0 = not set. 1 = uppercase chars. 2 = lowercase chars.
        for i in hrpiter {
            // dbg!(&i);
            // Valid character range check
            if i >= '\u{0021}' && i <= '\u{007E}' {
                // ASCII 33 - 126 translation into UTF-8
                //Mix case check
                if i.is_lowercase() {
                    if casecheckflag == 1 {
                        response = "INVALID: MIX CASE HRP".to_owned();
                        return ValidationResponse {
                            result: false,
                            reason: response,
                        };
                    }
                    casecheckflag = 2;
                } else if i.is_uppercase() {
                    if casecheckflag == 2 {
                        response = "INVALID: MIX CASE HRP".to_owned();
                        return ValidationResponse {
                            result: false,
                            reason: response,
                        };
                    }
                    casecheckflag = 1;
                }
            } else {
                response = "INVALID: HRP CHARACTERS INVALID".to_owned();
                return ValidationResponse {
                    result: false,
                    reason: response,
                };
            }
        }
    } else {
        response = "INVALID: HRP LENGTH OUT OF RANGE".to_owned();
        return ValidationResponse {
            result: false,
            reason: response,
        };
    }

    ValidationResponse {
        result: true,
        reason: response,
    }
}

pub fn valideh(teststr: &str) -> ValidationResponse {
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
            let hrp: &str;
            let datapart: &str;

            /*
            Better logic exists using rsplit_once()
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
            datapart = &teststr[separator + 1..];

            // dbg!(&hrp);
            // dbg!(&datapart);
            println!(
                "🔛 {} is split into {} as hrp and {} as data",
                teststr, hrp, datapart
            );

            let hrp_res = hrp_valideh(hrp);
            if hrp_res.result == false {
                return hrp_res;
            }

            /*
            The data part, which is at least 6 characters long and only consists of alphanumeric characters excluding "1", "b", "i", and "o"[4].
            */
            let hrp = hrp.to_ascii_lowercase();
            let hrp = hrp.as_str();
            let datapart = datapart.to_ascii_lowercase();
            let datapart = datapart.as_str();
            if datapart.len() < 6 {
                response = "INVALID: DATAPART LENGTH TOO SMALL".to_owned();
                println!("{}", response);
                return ValidationResponse {
                    result: false,
                    reason: response,
                };
            } else {
                let datachars = datapart.chars();
                for i in datachars {
                    if i.is_ascii_alphanumeric() && i != '1' && i != 'b' && i != 'i' && i != 'o' {
                        /*
                        Data character validity testing ends here. The tests to compute and test the checksums should follow.
                        */
                        let _res;
                        if verify_checksum(hrp, datapart) {
                            response = "VALID".to_owned();
                            _res = true;
                        } else {
                            response = "INVALID: CHECKSUM VALIDATION FAILED".to_owned();
                            _res = false;
                        }

                        return ValidationResponse {
                            result: _res,
                            reason: response,
                        };
                    } else {
                        response = "INVALID: INVALID CHARACTERS IN DATA".to_owned();
                        println!("{}", response);
                        return ValidationResponse {
                            result: false,
                            reason: response,
                        };
                    }
                }
            }
        } else {
            response = "INVALID: LENGTH OUT OF RANGE".to_owned();
            println!("{}", response);
            return ValidationResponse {
                result: false,
                reason: response,
            };
        }
    } else {
        response = "INVALID: SEPARATOR NOT FOUND".to_owned();
        println!("{}", response);
        return ValidationResponse {
            result: false,
            reason: response,
        };
    }
    ValidationResponse {
        result: false,
        reason: "YOU SHOULDNT BE HERE".to_owned(),
    }
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
        let validVectors: [&str; 7] = [
            "A1LQFN3A", 
            "a1lqfn3a", 
            "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6",
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx", 
            "11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8", 
            "split1checkupstagehandshakeupstreamerranterredcaperredlc445v", 
            "?1v759aa"
        ];

        for vector in &validVectors {
            let validation = valideh(vector);

            assert_eq!(validation.result, true);
        }
    }

    #[test]
    fn isinvalid() {
        let invalidVectors: [&str; 13] = [
            " 1xj0phk", 
            "\x7F1g6xzxy", 
            "\x701vctc34",
            "an84characterslonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11d6pts4", 
            "qyrz8wqd2c9m", 
            "y1b0jsk6g", 
            "lt1igcx5c0", 
            "in1muywd", 
            "mm1crxm3i", 
            "au1s5cgom", 
            "M1VUXWEZ", 
            "16plkw9", 
            "1p2gdwpf"
        ];

        for vector in &invalidVectors {
            let validation = valideh(vector);

            assert_eq!(validation.result, false);
        }
    }
}
