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

    let mut paths: Vec<String> = Vec::new();

    for path in matches.free {
        paths.push(path);
    }

    let mut print_header = false;

    if paths.len() > 1 {
        print_header = true
    }

    for path in paths {
        if print_header {
            println!("{}:", path);
        }

        match fs::read_dir(path) {
            Err(e) => eprintln!("Error while reading directory {}", e),
            Ok(dirents) => {
                for dirent in dirents {
                    match dirent {
                        Err(e) => eprintln!("Failed to unwrap path: {}", e),
                        Ok(p) => {
                            print!{"{:?}\t", p.file_name()};
                            match p.file_name().into_string() {
                                Err(e) => eprintln!("Failed while converting string {:?}", e),
                                Ok(st) => {
                                    if !st.starts_with('.') || all {
                                        print!("{}\t", st);
                                    };
                                }
                            };
                        }
                    }
                }
            }
        }
        print!("\n\n");
    }
}
