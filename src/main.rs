use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Kumo {
    #[clap(subcommand)]
    cmd: KumoCmd,
}

#[derive(Parser, Debug)]
enum KumoCmd {
    #[clap(name = "build", about = "Builds the ami")]
    Build,

    #[clap(name = "up", about = "Deploy the ami")]
    Up,

    #[clap(name = "destroy", about = "Destroys the deployment")]
    Destroy,
}

fn main() {
    let kumo = Kumo::parse();

    match kumo.cmd {
        KumoCmd::Build => {
            // Handle "build" command
            println!("Executing build command");
        }
        KumoCmd::Up => {
            // Handle "up" command
            println!("Executing up command");
        }
        KumoCmd::Destroy => {
            // Handle "destroy" command
            println!("Executing destroy command");
        }
    }
}
