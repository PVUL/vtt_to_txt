use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: vtt_to_txt <input.vtt> [output.txt]");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    // ✅ If no output path is provided, write to input file path with `.txt` extension
    let output_path: PathBuf = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        let stem = input_path.file_stem().unwrap().to_string_lossy();
        let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
        parent.join(format!("{}.txt", stem))
    };

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut result = String::new();
    let mut skip = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed == "WEBVTT" || trimmed.contains("-->") {
            continue;
        }

        if trimmed.starts_with("NOTE") {
            skip = true;
            continue;
        }

        if skip && trimmed.is_empty() {
            skip = false;
            continue;
        }

        if !skip {
            result.push_str(trimmed);
            result.push(' ');
        }
    }

    let mut output_file = File::create(&output_path)?;
    write!(output_file, "{}", result.trim())?;

    println!("✅ Output written to {}", output_path.display());
    Ok(())
}
