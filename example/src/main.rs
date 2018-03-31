#[macro_use] extern crate honggfuzz;
#[macro_use] extern crate fuzztest;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            fuzz_marker!("beginning");

            if data.len() == 0 {
                fuzz_marker!("data_len_0");
            }

            if data.len() != 10 {return}
            fuzz_marker!("data_len_10");

            if data[0] != b'q' {return}
            if data[1] != b'w' {return}
            if data[2] != b'e' {return}
            if data[3] != b'r' {return}
            if data[4] != b't' {return}
            if data[5] != b'y' {return}

            fuzz_marker!("qwerty");

            if data[6] != b'u' {return}
            if data[7] != b'i' {return}
            if data[8] != b'o' {return}
            if data[9] != b'p' {return}

            fuzz_marker!("qwertyuiop");

            if data.len() == 0 { // impossible
                fuzz_marker!("impossible");
            }
        });
    }
}

// TODO: test that the fuzzer is really reading the input corpus
// by inserting a marker at an edge undiscoverable by the fuzzer but
// covered in the input corpus.

#[cfg(test)]
mod tests {
    use fuzztest::*;

    #[test]
    fn fuzzer_can_find_beginning_marker() {
        check_target_with_marker("example", "beginning");
    }

    #[test]
    fn fuzzer_can_find_data_len_0_marker() {
        check_target_with_marker("example", "data_len_0");
    }

    #[test]
    fn fuzzer_can_find_data_len_10_marker() {
        check_target_with_marker("example", "data_len_10");
    }

    #[test]
    fn fuzzer_can_find_qwerty_marker() {
        check_target_with_marker("example", "qwerty");
    }

    #[test]
    fn fuzzer_can_find_qwertyuiop_marker() {
        check_target_with_marker("example", "qwertyuiop");
    }

    #[test]
    #[should_panic]
    fn fuzzer_cant_find_impossible_marker() {
        check_target_with_marker("example", "impossible");
    }
}