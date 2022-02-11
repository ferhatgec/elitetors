// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_cpp {
    use elite::ast::EliteKeywords;

    fn replace(data: String) -> String {
        data.replace('\"', "\\'").replace("\0", "")
    }

    pub fn parse(data: elite::parser::EliteParser) -> String {
        let mut regenerated_code = String::from("\
// taken from https://github.com/ferhatgec/systemf/blob/master/src/lib.rs
macro_rules! systemf {
    ($arg: expr) => {
        let args: Vec<_> = ($arg).split(' ').collect();
        let command = args.get(0).unwrap();
        let mut __command = std::process::Command::new(command);

        for __arg in args.iter().skip(1) {
           __command.arg(__arg);
        }

        __command.status()
    };
}

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_must_use)]
#[allow(non_snake_case)]
fn main() {\n\
    let cli_arguments: Vec<_> = std::env::args().collect();\n");

        let mut line = 0u32;
        let mut is_for = false;

        for x in data.ast_nodes.data {
            match x.__type {
                EliteKeywords::Set => {
                    regenerated_code.push_str(
                        format!("{}let mut {} = \"{}\";\n", " ".repeat(line as usize), x.__name, replace(x.__data)).as_str());
                }
                EliteKeywords::Print => {
                    regenerated_code.push_str(
                        format!("{}print!(\"{}\");\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Println => {
                    regenerated_code.push_str(
                        format!("{}println!(\"{}\");\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Use => {}
                EliteKeywords::RequiredVersion => {
                    regenerated_code.push_str(format!("if \"{}\" != \"{}\"\n{{\n{}",
                                                            replace(x.__name),
                                                            replace(x.__data),
                                                            " println!(\"elite: Required higher version\");\
                                                            \n std::process::exit(1);\n}\n").as_str());
                }
                EliteKeywords::Change => {}
                EliteKeywords::IfArg => {
                    regenerated_code.push_str(format!("{}if \"{}\"", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::LeftParenthese => {}
                EliteKeywords::RightParenthese => {}
                EliteKeywords::LeftSqBracket => {
                    regenerated_code.push_str("{\n");
                    if is_for { is_for = false; continue; } line += 1;
                }
                EliteKeywords::RightSqBracket => {
                    regenerated_code.push_str("}\n");
                    if line < 1 { continue } line -= 1;
                }
                EliteKeywords::Eq => {
                    regenerated_code.push_str(format!(" == \"{}\"\n", replace(x.__data)).as_str());
                }
                EliteKeywords::UnEq => {
                    regenerated_code.push_str(format!(" != \"{}\"\n", replace(x.__data)).as_str());
                }
                EliteKeywords::Signal => {
                    if x.__name == "exit" {
                        regenerated_code.push_str(format!("{}std::process::exit(1);\n", " ".repeat(line as usize)).as_str());
                    } else if x.__name == "start" {
                        is_for = true;
                    }
                }
                EliteKeywords::Exec => {
                    regenerated_code.push_str(format!("{}systemf!(\"{}\");\n", " ".repeat(line as usize), replace(x.__name)).as_str());
                }
                EliteKeywords::AddSource => {}
                EliteKeywords::Append => {}
                EliteKeywords::Exit => {
                    regenerated_code.push_str(format!("{}std::process::exit(1);\n", " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Specific => {
                    match x.__data.as_str() {
                        "x86" |
                        "amd64" => regenerated_code.push_str(
                                format!("{}if std::env::consts::ARCH == \"{}\"\n", " ".repeat(line as usize), x.__data).as_str()),
                        "windows" |
                        "macos" |
                        "linux" |
                        "freebsd" |
                        "netbsd" |
                        "android" => regenerated_code.push_str(
                            format!("{}if std::env::consts::OS == \"{}\"\n", " ".repeat(line as usize), x.__data).as_str()),
                        _ =>
                            // other platforms are not directly supported but this is may be TODO.
                            regenerated_code.push_str(
                            format!("{} // not supported\n", " ".repeat(line as usize)).as_str())

                    }

                }
                EliteKeywords::Argument => {
                    regenerated_code.push_str(
                        format!("{}if cli_arguments.len() >= 2 && cli_arguments.last().unwrap() == \"{}\"\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Exists => {
                    regenerated_code.push_str(
                        format!("{}if std::path::Path::new(\"{}\").exists()\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Undefined => {},
                _ => {}
            }
        }

        regenerated_code.push_str("}\n");
        regenerated_code
    }
}