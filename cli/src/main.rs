use std::{
    fs::{self, OpenOptions},
    io::{Result, Write},
    path::Path,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Scaffold {
        #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=25))]
        day: u32,
    },
}

// THIS SHOULD BE RUN RELATIVE TO THE WORKSPACE ROOT
// cargo run -p cli -- scaffold -d <day>
fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Scaffold { day } => {
            let workspace_root = std::env::current_dir()
                .expect("could not find workspace root")
                .to_path_buf();

            let path = workspace_root.join(format!("day-{:02}", day));

            scaffold_project(&path, day).expect("could not scaffold project")
        }
    }
}

fn scaffold_project(path: &Path, day: u32) -> Result<()> {
    let day_string = format!("day-{:02}", day);

    let part_template = read_template("part.txt").replace("<day>", &day_string);
    let cargo_toml_template = read_template("toml.txt").replace("<day>", &day_string);

    let bin_path = path.join("src/bin");
    fs::create_dir_all(&bin_path)?;

    for part_name in ["part-1.rs", "part-2.rs"] {
        let file_path = &bin_path.join(part_name);
        let template = part_template.replace("<part>", &part_name.replace(".rs", ""));

        create_and_write_file(file_path, template)?;
    }

    create_and_write_file(&path.join("Cargo.toml"), cargo_toml_template)?;

    create_and_write_file(&path.join("src/input.txt"), "Paste main input here".into())?;
    create_and_write_file(&path.join("src/test.txt"), "Paste test input here".into())?;

    Ok(())
}

fn read_template(name: &str) -> String {
    fs::read_to_string(format!("cli/src/templates/{name}"))
        .unwrap_or_else(|_| panic!("could not read templates/{name}"))
}

fn create_and_write_file(path: &Path, content: String) -> Result<()> {
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?
        .write_all(content.as_bytes())
}
