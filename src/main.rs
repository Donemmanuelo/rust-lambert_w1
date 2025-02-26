mod commandline;

use anyhow::{anyhow, Ok, Result};
use log::{debug, info, warn};
use std::f64;

use commandline::LambertArgs;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = LambertArgs::from_args();

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
    let tolerance = 1e-10;

    let mut w = 256.0; // Initial guess
    let mut delta = f64::INFINITY;
    let mut w_old = 0.0;
    while delta.abs() > tolerance {
        delta = w - w_old;

        w_old = w;
        w = w - (w * f64::exp(w) - x) / (f64::exp(w) + w * f64::exp(w));
    }

    Ok(w)
}
#[cfg(test)]
mod test {

    #[test]
    fn test_placeholder() {
        use crate::lambert_w;
        let x = -1.0;
        assert!(matches!(lambert_w(x), Err(_)));
    }
}
#[cfg(test)]
mod tests {
    use log::info;

    use crate::lambert_w;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn lambert_w_pos_input() {
        init();

        info!("lambert for values positive values");
        assert_eq!(lambert_w(1.0).unwrap(), 0.5671432904097838);
    }

    #[test]
    fn lambert_w_principal_branch() {
        init();

        info!("The principle branch w(0)");
        assert_eq!(0.0, 0.0);
    }
    #[test]
    fn lambert_w_neq_val() {
        init();

        info!("lambert a value less than -1/e");
        assert_eq!(lambert_w(-0.2).unwrap(), -0.25917110181907377);
    }

    #[test]
    fn lambert_w_larger_val() {
        init();

        info!("lambert for large values");
        assert_eq!(lambert_w(10000000000000000000000000000.0).unwrap(), 60.371859509617295);
    }
}
