use ansi_term::Color::Red;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    #[clap(long)]
    color: bool,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let _enabled = ansi_term::enable_ansi_support();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read `{}`", &args.path.display()))?;

    for line in content.lines() {
        let index_found = line.find(&args.pattern);

        if index_found == None {
            continue;
        }

        if args.color {
            println!(
                "{}",
                line.replace(&args.pattern, &Red.paint(&args.pattern).to_string())
            );
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
