use std::io::Result;
use std::process::Command;

pub fn get_ast(file: &str) -> Result<Vec<u8>> {
    Command::new("swiftc")
        .arg("-dump-parse")
        .arg(file)
        .output()
        .map(|i| i.stdout)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn strip_one_line_comment() {
//         let swift = "\
//         // comment
//         let a = 1
//         ";

//         assert_eq!(
//             "\
//         let a = 1
//         ",
//             strip(swift)
//         );
//     }
// }
