use anyhow::Result;

use clap::Parser;

use crate::phi::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(help_template(
    "\
{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}
"
))]
struct Cli {
    /// Specify output plot filename.
    #[arg(
        short = 'o', long, 
        value_name = "FILE", 
        default_value = "a.png",
    )]
    output: String,

    /// Specify the number of elements.
    #[arg(short = 'n', long,
        default_value_t = 20,
    )]
    number_of_elements: usize,

    /// Specify the value of the gravitational constant G.
    #[arg(short = 'G', long,
        default_value_t = 6.6743e-11,
    )]
    grav_const: f64,

    /// Specify the number of samples used for plotting
    #[arg(short = 's', long,
        default_value_t = 1000,
    )]
    sampling: usize,

    /// Do not output output a plot.
    #[arg(short = 'p', long)]
    plot_skip: bool,

    /// Print license
    #[arg(short = 'L', long)]
    license: bool,
}

pub fn process_args() -> Result<()> {
    let cli = Cli::parse();

    if cli.license {
        print_license();
        return Ok(());
    }

    let n = cli.number_of_elements;

    let bilins = phi_bilinear_matrix(n);
    let lins = phi_linear_matrix(n, cli.grav_const);

    println!("B matrix:\n{:#}", bilins);
    println!("L matrix:\n{:#}", lins);

    let phi_ext = solve_for_phi_ext(&bilins, &lins);
    println!("W matrix:\n{:#}", phi_ext);

    let phi = construct_phi(&phi_ext, n);

    if !cli.plot_skip {
        plot_phi_into_file(phi, &cli.output, cli.sampling)?;
    }

    Ok(())
}


fn print_license() {
    const LICENSE: &str =
        "This is free software. You may redistribute copies of it under the terms of
the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.
There is NO WARRANTY, to the extent permitted by law.";
    // just in case
    debug_assert!(
        env!("CARGO_PKG_LICENSE").starts_with("GPL-3.0"),
        "LICENSE message needs to be updated."
    );

    println!(
        "{} {}\n{}\n\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        LICENSE
    );
}