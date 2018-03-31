//! ## About Fuzztest
//!
//! Easily test your software using powerful evolutionary, feedback-driven fuzzing technology.

#![feature(use_extern_macros, decl_macro)]
extern crate lazy_static;

pub use lazy_static::*;

use std::process::{self, Command};
use std::env;
use std::fs;
use std::path::Path;
use std::io;
use std::io::Write;

fn cd_to_crate_root() {
    let mut path = env::current_dir().unwrap();

    while !path.join("Cargo.toml").is_file() {
        // move to parent path
        path = match path.parent() {
            Some(parent) => parent.into(),
            None => {
                eprintln!("error: could not find `Cargo.toml` in current directory or any parent directory");
                process::exit(1);
            }
        };
    }

    env::set_current_dir(path).unwrap();
}

/// This macro indicates a point which should be accessible by the fuzzer
///
/// The goal of setting those markers is to ensure that the fuzzing targets are well
/// set up and capable of reaching important parts of the code.
///
/// This macro takes one argument: a `marker` whose type is convertible to `&str`. 
///
/// ```rust
/// # #[macro_use] extern crate fuzztest;
/// # fn main() {
/// fuzz_marker!("point_a");
/// # }
/// ```
///
/// ## Performance
/// 
/// Under normal compilation this macro expands to nothing and therefore can't affect
/// in any way performance.
///
/// When compiled from the `check_target_with_marker` function (which sets `--cfg fuzztest`
/// in `RUSTFLAGS`), it will expand to a very carefully optimized piece of code that should 
/// have almost no impact on performance.

pub macro fuzz_marker($marker:expr) {
    #[cfg(fuzztest)]
    {
        use std::env;
        use std::fs::File;

        lazy_static! {
            static ref MARKER_SET: bool = {
                let marker: &str = {$marker}.into();
                let env_marker = env::var("FUZZTEST_MARKER")
                    .expect("fuzztest error: environment variable FUZZTEST_MARKER not set");
            
                marker == env_marker
            };
        }
        if *MARKER_SET {
            // touch marker file
            File::create(format!("fuzztest/{}.marker", {$marker}))
                .expect("fuzztest error: impossible to create marker file");

            panic!("fuzztest: the marker has been successfully hit!");
        }
    }
}

/// Check that a fuzzing target can hit a pre-defined marker
///
/// See the `fuzz_marker!` macro.
pub fn check_target_with_marker(target: &str, marker: &str) {
    cd_to_crate_root();

    let fuzztest_path = Path::new("fuzztest");
    if fuzztest_path.is_dir() {
        panic!("The fuzztest directory already exists. It might contain important crash data. Aborting... ");
    }
    fs::create_dir(fuzztest_path).expect("failed to create `fuzztest` directory");

    // TODO: check that honggfuzz is installed and at the correct version
    let output = Command::new("cargo")
        .args(&["hfuzz", "run", target])
        .env("HFUZZ_RUN_ARGS", "-W fuzztest --run_time 5 --exit_upon_crash")
        .env("RUSTFLAGS", "--cfg fuzztest")
        .env("FUZZTEST_MARKER", marker)
        .output()
        .expect("failed to launch fuzzer")
        ;

    if !output.status.success() {
        // clean work directory
        fs::remove_dir_all(fuzztest_path).expect("failed to remove `fuzztest` directory");

        // print log
        io::stdout().write(&output.stdout).unwrap();
        io::stderr().write(&output.stderr).unwrap();

        panic!("fuzztest: fuzzer exited unsuccessfully")
    }

    let has_crashfile = fs::read_dir(fuzztest_path)
        .expect("failed to read `fuzztest` directory")
        .map(|f|{f.unwrap().file_name()})
        .any(|f|{f.to_string_lossy().ends_with(".fuzz")});

    // check that the marker has been hit and created the corresponding file
    let has_marker_file = fuzztest_path.join(marker).with_extension("marker").is_file();

    match (has_marker_file, has_crashfile) {
        (true, true) => { // success: we hit the marker
            // clean work directory
            fs::remove_dir_all(fuzztest_path).expect("failed to remove `fuzztest` directory");
        },
        (true, false) => unreachable!("fuzztest: We got a marker file without a crash file. It's unexpected. Please file a bug report."),
        (false, true) => panic!("fuzztest: Ouch, we hit a crash which was not the marker! Please check out the crash file. (`cargo hfuzz run-debug {} fuzztest/*.fuzz`)", target),
        (false, false) => {
            // clean work directory
            fs::remove_dir_all(fuzztest_path).expect("failed to remove `fuzztest` directory");

            panic!("fuzztest: The fuzzer couldn't find the expected crash marked with `{}`", marker)
        },
    }
}
