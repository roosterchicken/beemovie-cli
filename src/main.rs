/*  
 *  Copyright 2021 Rooster
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
mod help;
use std::env;
extern crate beemovie;

fn main() {
    let args: Vec<String> = env::args().collect();
    let barry_exec_name = &args[0];
    if args.len() == 1 {
        help::help(barry_exec_name.to_string());
    } else if args.len() >= 2 {
        if args[1] == "help" {
            help::help(barry_exec_name.to_string());
        } else if args[1] == "sentence" {
            if args.len() == 2 {
                println!("Incorrect arguments");
            } else if args.len() == 3 {
                let int: i32 = args[2].parse().unwrap();
                println!("{}", beemovie::sentence(int));
            } else if args.len() > 3 {
                println!("Incorrect arguments");
            }
        } else if args[1] == "word" {
            if args.len() == 2 {
                println!("Incorrect arguments");
            } else if args.len() == 3 {
                let int: i32 = args[2].parse().unwrap();
                println!("{}", beemovie::word(int));
            } else if args.len() > 3 {
                println!("Incorrect arguments");
            }
        } else if args[1] == "paragraph" {
            if args.len() == 2 {
                println!("Incorrect arguments");
            } else if args.len() == 3 {
                let int: i32 = args[2].parse().unwrap();
                println!("{}", beemovie::paragraph(int));
            } else if args.len() > 3 {
                println!("Incorrect arguments");
            }
        } else if args[1] == "script" {
            if args.len() == 2 {
                println!("{}", beemovie::script());
            } else if args.len() > 2 {
                println!("Incorrect arguments");
            }
        } else if args[1] == "version" {
            println!("beemovie-cli v0.1.2\nbeemovie v{}", beemovie::version());
        } else {
            println!("Incorrect arguments");
        }
    }
}
