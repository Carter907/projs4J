mod archetypes;
mod archetype_builder;

use std::env::args;
use std::os::unix::process::CommandExt;
use std::process::Command;
use clap::{arg, Args, Parser, Subcommand};
use strum::IntoEnumIterator;
use crate::archetype_builder::ArchetypeBuilder;
use crate::archetypes::{MavenArchetypeInfo, MavenArchetypes};

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Projs4J {

    #[command(subcommand)]
    command: Commands

}
#[derive(Subcommand)]
enum Commands {
    /// creates a new java project based on custom maven archetypes
    NewProject {
        #[arg(short, long)]
        project_name: String,
        #[arg(short, long, default_value_t=String::from("com.example"))]
        group_id: String,
        #[arg(short, long, required = true)]
        archetype: MavenArchetypes,

    }
}


fn main() -> std::io::Result<()> {
    let args = Projs4J::parse();

    match &args.command {

        Commands::NewProject { project_name, group_id, archetype} => {
            let archetype_info = MavenArchetypeInfo(*archetype);
            ArchetypeBuilder::init_projects();



            Command::new("mvn")
                .current_dir(archetype_info.path()?)
                .arg("install")
                .output()?;

            Command::new("mvn")
                .arg("archetype:generate")
                .args(
                    [
                        format!("-DarchetypeGroupId={}",archetype_info.group_id()),
                        format!("-DarchetypeArtifactId={}", archetype_info),
                        "-DarchetypeVersion=1.0-SNAPSHOT".to_string(),
                        format!("-DgroupId={}", *group_id),
                        format!("-DartifactId={}", *project_name),
                        "-DinteractiveMode=false".to_string()
                    ]
                ).output()?;
        }
    }

    Ok(())

}
