

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn lambert_w() {
        init();

        info!("can log from the test too");
        assert_eq!(1.0, 0.56714329);
    }

  
}/* 
use text_placeholder::Template;
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;

let default_template = Template::new("Hello {{first}} {{second}}!");

let mut table = HashMap::new();
table.insert("first", "text");
table.insert("second", "placeholder");

assert_eq!(default_template.fill_with_hashmap(&table), "Hello text placeholder!");
*/