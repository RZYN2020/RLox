use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rox", about = "A rust implementation of lox")]
pub struct Options {
    #[structopt(short, long)]
    /// class search path of directories and zip/jar files
    pub filename: Option<String>,
}
