use ansi_term::Color::Red;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
struct Cli {
    /// Highlight matching string
    #[clap(long)]
    color: bool,
    #[clap(short, long)]
    ignore_case: bool,
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let _enabled = ansi_term::enable_ansi_support();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read `{}`", &args.path.display()))?;

    for line in content.lines() {
        let index_found;

        if args.ignore_case {
            index_found = line.to_lowercase().find(&args.pattern.to_lowercase());
        } else {
            index_found = line.find(&args.pattern);
        }

        if index_found == None {
            continue;
        }

        if args.color {
            let found = &line[index_found.unwrap()..args.pattern.len()];

            println!("{}", line.replace(found, &Red.paint(found).to_string()));
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
