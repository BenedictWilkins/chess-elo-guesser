
// data available here: https://database.lichess.org/


use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;
use std::process::{Command, Output};
use itertools::Itertools;


use std::fs::File;

use zzz::ProgressBarIterExt as _;

mod disambiguation;
mod pgniterator;
mod filenamechunker;

use filenamechunker::FileNameChunker;

use pgniterator::{records, write_binary_record_to_file,BinaryParser, StringParser, BinaryRecord, Record};

use glob::glob;
use clap::Parser;

/// Convert .pgn to .tnot. 
/// Can currently convert ~100k games per min
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// pgn files (or glob patterns) to process
    #[arg(short, long, num_args(1..), required=true)]
    files: Vec<String>,
    /// whether to unzip .zst files before processing, note that these files must have the proper extention `.pgn.zst`
    #[arg(short, long)]
    unzip: bool,
    /// force overwrite when unzipping
    #[arg(long)]
    forceunzip: bool,
    /// force overwrite when converting
    #[arg(long)]
    force: bool,
    /// when files are very large, use this to chunk the resulting conversion
    #[arg(short, long, default_value="10000000")] // 10 million = a couple of gig, this is ok
    chunk : usize,
}

// bytes so that it can be compared directly with OSStr
static PGN_EXTENSION : &[u8] = "pgn".as_bytes();
static ZST_EXTENSION : &[u8] = "zst".as_bytes();
static TNOT_EXTENSION : &[u8] = "tnot".as_bytes();

fn main() {
    let args = Args::parse();
    let paths = get_paths(args.files);

    // Iterate over matched file paths
    for mut path in paths {
        if path.extension().is_none() {
            continue;
        }
        if args.unzip && path.extension().unwrap().as_bytes() == ZST_EXTENSION { // should unzip?
            let success; 
            (path, success) = unzip(path.clone(), args.forceunzip);
            if !success { 
                continue;
            }
        }
        let ext = path.extension().unwrap().as_bytes();
        if ext == PGN_EXTENSION {
            // convert the file!
            println!("INFO: Converting file {:?}", path);
            convert(&path, args.chunk);
        }
    }
}

fn convert(path : &PathBuf, chunk : usize) {
    let path_tnot = path.with_extension(OsStr::from_bytes(TNOT_EXTENSION));
    let records = records(&path, BinaryParser).unwrap();
    let iter = records.into_iter().chunks(chunk);
    let chunker = FileNameChunker::new(iter.into_iter(), path_tnot);
    for (chunk, file) in chunker.progress() {
        let mut file = File::create(file).unwrap(); // TODO check errors...
        for record in chunk.filter_map(Result::ok).progress() {
            write_binary_record_to_file(&record, &mut file);
        }
    }  
}

fn unzip(path : PathBuf, force : bool) -> (PathBuf, bool) {
    let pgn_path = path.with_extension(""); 
    //println!("{:?} {:?}", pgn_path, pgn_path.extension());

    // check that path has the proper extension before unzipping
    if pgn_path.extension().is_none() {
        return (pgn_path, false);
    }
    let ext = pgn_path.extension().unwrap().as_bytes();
    if ext != PGN_EXTENSION {
        println!("INFO: Skipping file {:?} as it is not a .pgn file.", path);
        return (pgn_path, false); // don't bother unzipping, its not a pgn file
    }
    // check pgn_path already exists, skip it unless forced.
    if pgn_path.exists() && !force {
        println!("INFO: Skipping file {:?} as it has already been unzipped.", path);
        return (pgn_path, false);
    }
    // pzstd must be installed!
    let unzip_result = Command::new("pzstd")
                            .arg("-df")
                            .arg(&path)
                            .output()
                            .expect("Failed to execute command.");
    if !unzip_result.status.success() {
        eprintln!("ERROR: Failed to unzip file {:?} : {:?}", path, String::from_utf8_lossy(&unzip_result.stderr));
        // failed to unzip for some reason...
        return (pgn_path, false);
    }
    // successfully unzipped
    return (pgn_path, true);
}



fn get_paths(paths: Vec<String>) -> impl Iterator<Item = PathBuf> {
    paths.into_iter()
        .flat_map(|file_arg| {
            let mut matches = glob(&file_arg).expect("Failed to read glob pattern").peekable();
            if matches.peek().is_none() {
                eprintln!("WARNING: No files were found for glob pattern: {}", &file_arg);
            }
            matches.filter_map(|path| {
                match path {
                    Ok(path) => Some(path),
                    Err(e) => { eprintln!("ERROR: Error while processing file: {}", e); None}
                }
            }).filter_map(|path| resolve_path(path))
        })
}

fn resolve_path(path : PathBuf) -> Option<PathBuf> {
    match path.canonicalize() {
        Ok(path) => Some(path),
        Err(e) => { eprintln!("ERROR: Failed to resolve path {:?}, {}", path, e); None}
    }
}

