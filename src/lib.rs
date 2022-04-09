use std::{io::Error, io::ErrorKind};

const BECH32M_CONST: usize = 0x2bc830a3;
const DATA_LUT: [&'static str; 4] = ["qpzry9x8", "gf2tvdw0", "s3jn54kh", "ce6mua7l"];
const CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub struct ValidationResponse {
    result: bool,
    reason: String,
}

pub struct DecodedData {
    pub hrp: String,
    pub data: Vec<usize>,
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
    let gen: Vec<usize> = vec![0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    let mut chk = 1 as usize;

    for i in values {
        let b = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ (*i as usize);
        for j in 0..5 {
            chk ^= if ((b >> j) & 1) != 0 {
                gen[j]
            } else {
                0 as usize
            };
        }
    }

    chk
}

pub fn verify_data_checksum(hrp: &str, mut data: Vec<usize>) -> bool {
    let mut hrp = hrp_expand(hrp);
    hrp.append(&mut data);
    let res = polymod(&hrp);
    res == BECH32M_CONST
}

pub fn verify_checksum(hrp: &str, data: &str) -> bool {
    let mut hrp = hrp_expand(hrp);
    let mut data = data_to_int(data);
    hrp.append(&mut data);
    let res = polymod(&hrp);
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
    let mut hrpx: Vec<usize> = Vec::new();

    for c in hrp.chars() {
        hrpx.push((c as usize) >> 5);
    }

    hrpx.push(0);

    for c in hrp.chars() {
        hrpx.push((c as usize) & 31);
    }

    return hrpx;
}

fn convert_bits(data: Vec<usize>, from: usize, to: usize, pad: bool) -> Result<Vec<usize>, Error> {
    let mut result: Vec<usize> = Vec::new();

    let mut acc = 0;
    let mut bits = 0;
    let max_v = (1 << to) - 1;
    let max_acc = (1 << (from + to - 1)) - 1;

    for c in data {
        if (c >> from) > 0 {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("Invalid character: {}", c),
            ));
        }

        acc = ((acc << from) | c) & max_acc;
        bits += from;

        while bits >= to {
            bits -= to;
            result.push((acc >> bits) & max_v);
        }
    }

    if pad {
        if bits > 0 {
            result.push((acc << (to - bits)) & max_v);
        }
    } else if bits >= from || ((acc << (to - bits)) & max_v) > 0 {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("Bit conversion failed!"),
        ));
    }

    Ok(result)
}

pub fn decode_hex(bech_string: &str) -> Result<String, Error> {
    let decode_result = decode(bech_string);

    if decode_result.is_err() {
        return Err(decode_result.err().unwrap());
    }

    let data = decode_result.unwrap().data;
    let convert_bits = convert_bits(data[1..data.len()].to_vec(), 5, 8, false);

    if convert_bits.is_err() {
        return Err(convert_bits.err().unwrap());
    }

    let converted_bits = convert_bits.unwrap();

    let decoded: String = converted_bits
        .iter()
        .map(|c| format!("{:02x?}", c))
        .collect();
    Ok(decoded.to_lowercase())
}

pub fn decode(bech_string: &str) -> Result<DecodedData, Error> {
    let mut has_lowercase_char = false;
    let mut has_uppercase_char = false;
    for c in bech_string.chars() {
        let ascii_value = c as usize;

        if ascii_value < 33 || ascii_value > 126 {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("ERROR: Invalid character {}", c),
            ));
        }

        if ascii_value >= 97 && ascii_value <= 122 {
            has_lowercase_char = true;
        }

        if ascii_value >= 65 && ascii_value <= 90 {
            has_uppercase_char = true;
        }
    }

    if has_lowercase_char && has_uppercase_char {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("ERROR: Use only lowercase or uppercase characters, not both!"),
        ));
    }

    let bech = bech_string.to_lowercase();
    let one_locations: Vec<_> = bech.match_indices("1").collect();
    if one_locations.len() < 1 {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("ERROR: Separator was not found!"),
        ));
    }

    let start_index = one_locations[one_locations.len() - 1].0;
    let string_len = bech.chars().count();
    if start_index < 1 || start_index + 7 > string_len || string_len > 90 {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("ERROR: Invalid input!"),
        ));
    }

    let mut result: Vec<usize> = Vec::new();
    for p in start_index + 1..string_len {
        let charset_char = CHARSET.find(bech.chars().nth(p).unwrap());

        if charset_char == None {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                format!("ERROR: Invalid character in input!"),
            ));
        }

        result.push(charset_char.unwrap());
    }

    let hrp = &bech[0..start_index];

    if !verify_data_checksum(hrp, result.clone()) {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("ERROR: Checksum validation failed!"),
        ));
    }

    result.truncate(result.len().saturating_sub(6));

    Ok(DecodedData {
        hrp: hrp.to_owned(),
        data: result,
    })
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
        let valid_vectors: [&str; 7] = [
            "A1LQFN3A", 
            "a1lqfn3a", 
            "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6",
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx", 
            "11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8", 
            "split1checkupstagehandshakeupstreamerranterredcaperredlc445v", 
            "?1v759aa"
        ];

        for vector in &valid_vectors {
            let validation = valideh(vector);

            assert_eq!(validation.result, true);
        }
    }

    #[test]
    fn isinvalid() {
        let invalid_vectors: [&str; 13] = [
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

        for vector in &invalid_vectors {
            let validation = valideh(vector);

            assert_eq!(validation.result, false);
        }
    }

    #[test]
    fn decode_valid() {
        let mut valid_results: Vec<(&str, DecodedData)> = Vec::new();

        valid_results.push((
            "A1LQFN3A",
            DecodedData {
                hrp: "a".to_owned(),
                data: vec![],
            },
        ));

        valid_results.push((
            "a1lqfn3a",
            DecodedData {
                hrp: "a".to_owned(),
                data: vec![],
            },
        ));

        valid_results.push((
            "bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7kt5nd6y",
            DecodedData {
                hrp: "bc".to_owned(),
                data: vec![
                    1, 14, 20, 15, 7, 13, 26, 0, 25, 18, 6, 11, 13, 8, 21, 4, 20, 3, 17, 2, 29, 3,
                    12, 29, 3, 4, 15, 24, 20, 6, 14, 30, 22, 14, 20, 15, 7, 13, 26, 0, 25, 18, 6,
                    11, 13, 8, 21, 4, 20, 3, 17, 2, 29, 3, 12, 29, 3, 4, 15, 24, 20, 6, 14, 30, 22,
                ],
            },
        ));

        valid_results.push((
            "BC1SW50QGDZ25J",
            DecodedData {
                hrp: "bc".to_owned(),
                data: vec![16, 14, 20, 15, 0],
            },
        ));

        valid_results.push((
            "bc1zw508d6qejxtdg4y5r3zarvaryvaxxpcs",
            DecodedData {
                hrp: "bc".to_owned(),
                data: vec![
                    2, 14, 20, 15, 7, 13, 26, 0, 25, 18, 6, 11, 13, 8, 21, 4, 20, 3, 17, 2, 29, 3,
                    12, 29, 3, 4, 12,
                ],
            },
        ));

        valid_results.push((
            "tb1pqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesf3hn0c",
            DecodedData {
                hrp: "tb".to_owned(),
                data: vec![
                    1, 0, 0, 0, 0, 1, 17, 5, 5, 25, 11, 10, 6, 4, 8, 13, 18, 20, 6, 3, 25, 0, 23,
                    18, 18, 12, 24, 27, 2, 23, 6, 14, 21, 29, 4, 14, 6, 25, 24, 18, 13, 2, 25, 14,
                    26, 23, 4, 31, 8, 12, 16, 25, 16,
                ],
            },
        ));

        valid_results.push((
            "bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqzk5jj0",
            DecodedData {
                hrp: "bc".to_owned(),
                data: vec![
                    1, 15, 6, 31, 6, 12, 31, 23, 25, 27, 18, 29, 26, 24, 21, 13, 0, 12, 10, 10, 28,
                    29, 1, 24, 11, 0, 28, 1, 9, 23, 31, 6, 27, 5, 23, 7, 2, 17, 22, 10, 25, 30, 10,
                    0, 21, 22, 5, 23, 24, 2, 30, 12, 0,
                ],
            },
        ));

        valid_results.push((
            "?1v759aa",
            DecodedData {
                hrp: "?".to_owned(),
                data: vec![],
            },
        ));

        valid_results.push((
            "split1checkupstagehandshakeupstreamerranterredcaperredlc445v",
            DecodedData {
                hrp: "split".to_owned(),
                data: vec![
                    24, 23, 25, 24, 22, 28, 1, 16, 11, 29, 8, 25, 23, 29, 19, 13, 16, 23, 29, 22,
                    25, 28, 1, 16, 11, 3, 25, 29, 27, 25, 3, 3, 29, 19, 11, 25, 3, 3, 25, 13, 24,
                    29, 1, 25, 3, 3, 25, 13,
                ],
            },
        ));

        valid_results.push(("11llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllludsr8", DecodedData{
            hrp: "1".to_owned(),
            data: vec![31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31],
        }));

        valid_results.push((
            "abcdef1l7aum6echk45nj3s0wdvt2fg8x9yrzpqzd3ryx",
            DecodedData {
                hrp: "abcdef".to_owned(),
                data: vec![
                    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12,
                    11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
                ],
            },
        ));

        valid_results.push(("an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber11sg7hg6", DecodedData{
            hrp: "an83characterlonghumanreadablepartthatcontainsthetheexcludedcharactersbioandnumber1".to_owned(),
            data: vec![],
        }));

        for valid_result in valid_results {
            let decode_result = decode(valid_result.0);

            assert_eq!(decode_result.is_ok(), true);

            let decoded_data = decode_result.unwrap();
            assert_eq!(decoded_data.hrp, valid_result.1.hrp);
            assert_eq!(decoded_data.data, valid_result.1.data);
        }
    }

    #[test]
    fn decode_valid_hex() {
        let mut valid_results: Vec<(&str, &str)> = Vec::new();

        valid_results.push((
            "bc1pp3tck3286zd8gcvf4wv2lryakan245qth4sp8zxld0vfcfnk9uwqpdl8nf",
            "0c578b4547d09a746189ab98af8c9db766aad00bbd601388df6bd89c26762f1c",
        ));

        valid_results.push((
            "bc1ptfep7c3n3addv44m44pwua0ud5hg9f0fvqv3ssx8gvh",
            "5a721f62338f5ad656bbad42ee75fc6d2e82a5e9601918",
        ));

        valid_results.push((
            "bc1p0wvtajlj680zy40cq2rug3qmzmpk90wg0urc9a7u8hvhf6xal5cg83rj",
            "7b98becbf2d1de2255f80287c4441b16c362bdc87f0782f7dc3dd974e8ddfd",
        ));

        valid_results.push((
            "bc1pltv8xm6r3fcazgtaluuve9fd73h5d7kqqtzuvp240k7tjylt584qjmruzp",
            "fad8736f438a71d1217dff38cc952df46f46fac002c5c605557dbcb913eba1ea",
        ));

        for valid_result in valid_results {
            let decode_result = decode_hex(valid_result.0);

            assert_eq!(decode_result.is_ok(), true);
            assert_eq!(decode_result.unwrap(), valid_result.1);
        }
    }

    #[test]
    fn decode_invalid() {
        let invalid_results: [&str; 18] = [
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
            "1p2gdwpf",
            "bc1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqh2y7hd",
            "tb1z0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vqglt7rf",
            "BC1S0XLXVLHEMJA6C4DQV22UAPCTQUPFHLXM9H8Z3K2E72Q4K9HCZ7VQ54WELL",
            "bc1p38j9r5y49hruaue7wxjce0updqjuyyx0kh56v8s25huc6995vvpql3jow4",
            "tb1p0xlxvlhemja6c4dqv22uapctqupfhlxm9h8z3k2e72q4k9hcz7vq47Zagq"
        ];
        for invalid_result in invalid_results {
            let decode_result = decode(invalid_result);

            assert_eq!(decode_result.is_ok(), false);
            assert_eq!(decode_result.is_err(), true);
        }
    }
}
