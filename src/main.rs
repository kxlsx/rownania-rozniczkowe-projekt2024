mod phi;
mod cli;

fn main() -> anyhow::Result<()> {
    cli::process_args()
}

