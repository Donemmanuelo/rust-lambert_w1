mod commandline;

use anyhow::{anyhow, Result};
use std::f64;
use log::{debug, info, warn};

use commandline::LambertArgs;
use structopt::StructOpt;

fn main()-> Result<()>{

let  args = LambertArgs::from_args();

let x: f64 = args.x;
 
    env_logger::init();
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is a warning message");
    let w = lambert_w(x)?;
    println!("Lambert W({}) = {}", x, w);
    Ok(())
}

fn lambert_w(x: f64) -> Result<f64> {
    if x < -1.0 / f64::consts::E {
        return Err(anyhow!("Input must be greater than or equal to -1/e"));
    }

    let mut w = x;
    for _ in 0..100 {
        let w_next = w - (w * f64::exp(w) - x) / (f64::exp(w) + w * f64::exp(w));
        if (w_next - w).abs() < 1e-10 {
            return Ok(w_next);
        }
        w = w_next;
    }
    Err(anyhow!("Failed to converge"))
}
