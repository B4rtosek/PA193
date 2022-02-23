// use std::fs;

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn isvalid() {

        let validstrings = fs::read_to_string(".\\validbech32m.json").expect(" Error parsing test file as string");
        // The current directory for testing would be the root directory (which contains the src folder)
        // Thus the file has been copied there as well.
        
        assert_eq!(2+2,4);

    }
}