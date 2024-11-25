use structopt::StructOpt;

#[derive(StructOpt)]
pub(crate) struct Args {
    /// The file to take code-to-interpret from. `-` means Stdin
    #[structopt()]
    pub code_file: String,
}
