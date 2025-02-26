#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn lambert_w_pos_input() {
        init();

        info!("lambert for values positive values");
        assert_eq!(1.0, 0.56714329);
    }

    #[test]
    fn lambert_w_principal_branch() {
        init();

        info!("The principle branch w(0)");
        assert_eq!(0.0, 0.0);
    }
    #[test]
    fn lambert_w() {
        init();

        info!("lambert a value less than -1/e");
        assert_eq!(-0.2, -0.25917110181907377);
    }

    #[test]
    fn lambert_w_larger_val() {
        init();

        info!("lambert for large values");
        assert_eq!(10000000000000000000000000000.0, 60.371859509617295);
    }
}
