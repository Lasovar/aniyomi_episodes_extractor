use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "aee", version = "0.2.0", about = "A command-line tool use to extract anime episodes from an Aniyomi structured folder")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// Extract anime episodes from the specified library folder
    Extract {
        #[arg(short, long, default_value = ".", help = "the anime library folder to extract from")]
        target: String,
        #[arg(short, long, help = "extract episodes from subfolders rather the main folder")]
        sub: bool,
        #[arg(short, long, help = "automatically confirm all prompts")]
        yes: bool,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    if cli.verbose {
        println!("Verbose mode is on");
    }

    match &cli.command {
        Some(Commands::Extract { target, sub, yes }) =>{
            if !*sub {
                return extract_from_string(target, *yes);
            }

            let directory_path = std::path::Path::new(target);

            let mut to_be_moved = vec![];
            for entry in std::fs::read_dir(directory_path)? {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }

                println!("Found `{}`", path.display());
                to_be_moved.push(path);
            }

            println!("Found {} directories.", to_be_moved.len());

            if !*yes && !ask_confirm("\nThe following directories will be processed... do you want to proceed?") {
                println!("Operation cancelled by user.");
                return Ok(())
            }

            for path in to_be_moved {
                extract(&path, true)?;
                println!("Extracted `{}`\n", path.display());
            }

            Ok(())
        },
        None => {
            println!("No name provided. Use --help for more information.");

            Ok(())
        }
    }
}

fn extract_from_string(target: &String, auto_confirm: bool) -> Result<(), std::io::Error> {
    let directory = std::path::Path::new(target);
    extract(directory, auto_confirm)
}

fn extract(target: &std::path::Path, auto_confirm: bool) -> Result<(), std::io::Error> {
    let mut to_be_moved = vec![];
    for entry in  std::fs::read_dir(target)? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        for file in std::fs::read_dir(path)? {
            let file_path = file?.path();
            if let Some(extension) = file_path.extension() {
                if extension == "mp4" || extension == "mkv" || extension == "avi" {
                    println!("Episode file: {:?}", file_path);
                    to_be_moved.push(file_path);
                }
            }
        }
    }

    if to_be_moved.len() < 1 {
        println!("No episode files found to process.");
        return Ok(())
    }

    if !auto_confirm && !ask_confirm("\nThe following episodes will be processed... do you want to proceed?")
    {
        println!("Operation cancelled by user.");
        return Ok(())
    }

    for file_path in &to_be_moved {
        if let Some(file_name) = file_path.file_name() {
            let new_path = target.join(file_name);
            std::fs::rename(&file_path, &new_path)?;
            if let Some(parent) = file_path.parent() {
                std::fs::remove_dir_all(&parent)?;
            }
        }
    }

    println!("\n{} Episodes moved!", to_be_moved.len());

    Ok(())
}

fn ask_confirm(prompt: &str) -> bool {
    use std::io::Write;

    print!("{} (y/N): ", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false, // default = N
    }
}
