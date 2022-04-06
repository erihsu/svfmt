use std::{collections::HashMap, io::Write, path::PathBuf, fs::OpenOptions};
use sv_parser::parse_sv;
use svfmt::FormatStatus;
use clap::Parser;
use verilog_filelist_parser;
use termcolor::{Color,ColorChoice,ColorSpec,StandardStream,WriteColor};

/// svfmt CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct SvFmtArgs {
    /// format all files in the filelist
    #[clap(short = 'f')]
    pub filelist: Option<PathBuf>,
    /// path to the files that need to be formatted
    #[clap(short = 'p')]
    pub path: Option<PathBuf>,
    /// Additional including path
    pub include: Vec<PathBuf>,
    /// excluded path, files under excluded path will not be formatted
    #[clap(long)]
    pub exclude: Vec<PathBuf>,
    /// format file inplace, else svfmt will automatically generate formatted code under <svfmt_outdir>
    #[clap(long, short = 'i')]
    pub inplace: bool,
    /// generate formatted code in <outdir>, if not specified, will use default path
    #[clap(long)]
    pub outdir:Option<PathBuf>,
    /// Debug svfmt result
    #[clap(long,short = 'd')]
    pub debug: bool,
    /// recursive format, which means will format file that's included in current source code
    #[clap(long, short = 'r')]
    pub recursive: bool,    
}

impl Default for SvFmtArgs {
    fn default() -> Self {
        Self {
            filelist: None,
            path: None,
            include: vec![],
            exclude: vec![],
            inplace: true,
            outdir: None,
            debug: false,
            recursive: false,
        }

    }
}


fn main() -> std::io::Result<()>{
    // set termcolor
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;


    let args = SvFmtArgs::parse();
    let mut svfmt_outdir = PathBuf::new();



    if !args.inplace {
        svfmt_outdir = if let Some(outdir) = args.outdir {
            outdir
        } else {
            println!("Non-inplace format must specify outdir for generated code");
            return Ok(());
        }
    }

    // format file from filelist
    if let Some(f) = args.filelist {
        let filelist = verilog_filelist_parser::parse_file(f).expect("Cannot read filelist, parse error");
        let defines = HashMap::new(); // discard defines in filelist
        let includes = filelist.incdirs;
        let paths = filelist.files;
        for path in paths {
            if let Ok((syntax_tree, _)) = parse_sv(&path, &defines, &includes, false, false) {
                writeln!(&mut stdout, "start to format {:?}",path)?;
                stdout.reset()?;
                let mut status = FormatStatus::new(&syntax_tree);
                status.exec_format();
                if args.debug {
                    println!("[Output]");
                    println!("{}",status.buffer);
                }
                // generate formatted code inplace or in outdir
                if args.inplace {
                    let mut opened_f = OpenOptions::new().write(true).truncate(true).open(path)?;
                    opened_f.write_all(status.buffer.as_bytes())?;
                    opened_f.flush()?;
                } else {
                    // FIXME
                    svfmt_outdir.push(path.file_name().unwrap());
                    let mut file = std::fs::File::create(svfmt_outdir.clone())?;
                    file.write_all(status.buffer.as_bytes())?;
                }
            } else {
                println!("{:?} parse failed",path);
                return Ok(());
            }
        }
    } 
    // format from file
    else if let Some(path) = args.path {
        let defines = HashMap::new();
        let includes:Vec<PathBuf> = Vec::new();        
        if let Ok((syntax_tree, _)) = parse_sv(&path, &defines, &includes, false, false) {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut stdout, "start to format {:?}",path)?;
            stdout.reset()?;
            let mut status = FormatStatus::new(&syntax_tree);
            status.exec_format();
            if args.debug {
                println!("[Output]");
                println!("{}",status.buffer);
            }
            // generate formatted code inplace or in outdir
            if args.inplace {
                let mut opened_f = OpenOptions::new().write(true).truncate(true).open(path)?;
                opened_f.write_all(status.buffer.as_bytes())?;
                opened_f.flush()?;
            } else {
                // FIXME
                svfmt_outdir.push(path.file_name().unwrap());
                let mut file = std::fs::File::create(svfmt_outdir.clone())?;
                file.write_all(status.buffer.as_bytes())?;
            }
        } else {
            println!("{:?} parse failed",path);
            return Ok(());
        }
    } else {
        println!("You should specify file to format");
    }

    return Ok(());
}
