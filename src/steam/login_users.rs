use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::SubAssign;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LoginUsers {
    #[allow(dead_code)]
    pub users: HashMap<String, HashMap<String, String>>,
}

impl Default for LoginUsers {
    fn default() -> Self {
        Self {
            users: HashMap::with_capacity(1),
        }
    }
}

// enum ParserState {
//     Start,
//     Users,
//     User,
// }

impl LoginUsers {
    pub(crate) fn get_first_user_id(vdf_file: &PathBuf) -> Option<String> {
        let file = File::open(vdf_file).ok()?;
        let reader = BufReader::new(file);
        let mut deep = 0;
        let mut rtn = None;
        for line_result in reader.lines() {
            let line = line_result.ok()?;
            let trim = line.trim();

            let mut split = trim.split_ascii_whitespace();
            if deep == 0 {
                if split.next()? == "{" {
                    deep = 1;
                }
            } else if deep == 1 {
                let item = split.next()?;
                if item.len() == 19 {
                    let mut number: u64 = item[1..18].parse().ok()?;
                    number.sub_assign(0x110000100000000);
                    rtn = Some(number.to_string());
                    break;
                }
            }
        }
        rtn
    }

    // pub fn from_vdf_file(vdf_file: &Path) -> anyhow::Result<LoginUsers> {
    //     let mut rtn = Self::default();
    //     let mut state = ParserState::Start;
    //     let file = File::open(vdf_file)?;
    //     let reader = BufReader::new(file);
    //     let mut deeps = 0;
    //
    //     for line_result in reader.lines() {
    //         let line = line_result?;
    //         let trim = line.trim();
    //
    //         let tokens = find_tokens(&line)?;
    //
    //         if let Some(&first) = tokens.first() {
    //             match first {
    //                 "{" => deeps += 1,
    //                 "}" => deeps -= 1,
    //                 _ => {}
    //             }
    //         }
    //     }
    //
    //     if deeps != 0 {
    //         return Err(anyhow!("VDF File is not correctly aligned"));
    //     }
    //
    //     Ok(rtn)
    // }
}

// fn find_tokens(line: &str) -> anyhow::Result<Vec<&str>> {
//     let mut rtn = Vec::with_capacity(2);
//     let bytes = line.as_bytes();
//     let mut pos = 0;
//     let mut in_token = false;
//     let mut start = 0;
//
//     while pos < line.len() {
//         let c = bytes[pos];
//
//         if in_token {
//             if c == b'"' {
//                 in_token = false;
//                 rtn.push(&line[start..pos])
//             }
//         } else {
//             match c {
//                 b'"' => {
//                     in_token = true;
//                     start = pos;
//                 }
//                 b'{' => {
//                     start = pos;
//                     rtn.push(&line[start..pos + 1]);
//                 }
//                 b'}' => {
//                     start = pos;
//                     rtn.push(&line[start..pos + 1]);
//                 }
//                 _ => {}
//             }
//         }
//
//         pos += 1;
//     }
//
//     if in_token {
//         return Err(anyhow!("Token is not closed"));
//     }
//
//     Ok(rtn)
// }
