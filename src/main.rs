use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "aee", version = "0.1.0", about = "A command-line tool use to extract anime episodes from an Aniyomi structured folder")]
struct Cli {
    name: Option<String>,
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Hello {
        #[arg(default_value = "world")]
        target: String
    },
    Extract {
        #[arg(short, long, default_value = ".")]
        target: String,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    if cli.verbose {
        println!("Verbose mode is on");
    }

    match &cli.command {
        Some(Commands::Hello { target}) => {
            println!("Hello, {}!", target);
            Ok(())
        }
        Some(Commands::Extract { target }) => extract(target),
        None => {
            if let Some(name) = &cli.name.as_deref() {
                println!("Hello, {}!", name);
            } else {
                println!("No name provided. Use --help for more information.");
            }

            Ok(())
        }
    }
}

fn extract(target: &String) -> Result<(), std::io::Error> {
    let directory = std::path::Path::new(target);

    let mut to_be_moved = vec![];
    for entry in  std::fs::read_dir(directory)? {
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

    if !ask_confirm("\nThe following episodes will be processed... do you want to proceed?") {
        println!("Operation cancelled by user.");
        return Ok(())
    }

    for file_path in &to_be_moved {
        if let Some(file_name) = file_path.file_name() {
            let new_path = directory.join(file_name);
            std::fs::rename(&file_path, &new_path)?;
            std::fs::remove_dir_all(&file_path.parent().unwrap())?;
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
