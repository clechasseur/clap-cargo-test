use clap::{Args, Parser};
use clap_cargo::{Manifest, Workspace};

#[derive(Debug, Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cli {
    ClapCargoTest(ClapCargoTestArgs),
}

#[derive(Debug, Args)]
struct ClapCargoTestArgs {
    #[command(flatten)]
    manifest: Manifest,
    #[command(flatten)]
    workspace: Workspace,
}

fn main() {
    let Cli::ClapCargoTest(args) = Cli::parse();

    let metadata = args.manifest.metadata().exec().unwrap();
    let (selected, unselected) = args.workspace.partition_packages(&metadata);

    println!(
        "Selected packages: {:?}\nUnselected packages: {:?}",
        selected.iter().map(|&p| &p.name).collect::<Vec<_>>(),
        unselected.iter().map(|&p| &p.name).collect::<Vec<_>>(),
    );
}
