use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct LambertArgs {
    #[structopt(short, long)]
    pub x: f64,
}
