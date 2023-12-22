use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
  #[arg(short, long)]
  pub action: String,

  #[arg(short, long)]
  pub file: String,
}