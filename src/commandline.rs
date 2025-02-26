use structopt::StructOpt;
#[derive(StructOpt)]
pub struct LambertArgs {
    #[structopt(short, long)]
    pub x: f64,
}
