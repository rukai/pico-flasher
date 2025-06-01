use clap::Parser;
use miette::Result;

pub mod elf;
pub mod flash;

#[derive(Parser)]
struct Args {
    /// File to flash
    #[clap()]
    pub file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let firmware_bytes_raw = std::fs::read(args.file).unwrap();
    let firmware_bytes = elf::elf_to_bin(&firmware_bytes_raw)?;

    flash::flash_device(&firmware_bytes)?;

    println!("Successfully flashed!");
    Ok(())
}
