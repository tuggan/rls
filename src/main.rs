/*
   Copyright 2017 Dennis Vesterlund

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

extern crate getopts;

use std::fs;
use std::env;
use getopts::Options;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut all: bool = false;

    let mut opts = Options::new();

    opts.optflag("a", "", "Print all files, even those starting with \".\"");
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Err(e) => panic!("  {}", e),
        Ok(o) => o,
    };

    if matches.opt_present("a") {
        all = true;
    }

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    match fs::read_dir(".") {
        Err(e) => panic!("Error while reading directory {}", e),
        Ok(paths) => {
            for path in paths {
                match path {
                    Err(e) => eprintln!("Failed to unwrap path: {}", e),
                    Ok(p) => {
                        let st = p.file_name().to_os_string().into_string().unwrap();
                        if !st.starts_with('.') || all {
                            print!("{}\t", st);
                        };
                    }
                }
            }
        }
    }
}
