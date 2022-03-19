use std::{collections::HashMap, env, io::Write, path::PathBuf};
use sv_parser::parse_sv;
use svfmt::FormatStatus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: PathBuf = PathBuf::from(&args[1]);
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = Vec::new();

    if let Ok((syntax_tree, _)) = parse_sv(&path, &defines, &includes, false, false) {
        let mut status = FormatStatus::new(&syntax_tree);
        status.exec_format();
        println!("{}", status.syntax_tree);
        let mut file = std::fs::File::create("test.sv").unwrap();
        file.write_all(status.buffer.as_bytes()).unwrap();
    } else {
        println!("parse failed");
    }
}
