use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Name of your keyboard
    pub name: String,
}
