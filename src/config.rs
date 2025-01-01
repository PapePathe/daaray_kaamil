#[derive(Debug, PartialEq)]
pub struct Config {
    pub source_path: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("you must specify file path");
        }

        let source_path = args[1].clone();

        Ok(Self { source_path })
    }
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    fn test_config_new() {
        assert_eq!(
            Err::<Config, &str>("you must specify file path"),
            Err(Config::new(&[]).unwrap_err())
        );
    }
    #[test]
    fn test_config_new_2() {
        assert_eq!(
            Err::<Config, &str>("you must specify file path"),
            Err(Config::new(&["xasida".to_string()]).unwrap_err())
        );
    }
    #[test]
    fn test_config_new_3() {
        assert_eq!(
            Ok(Config {
                source_path: "/tmp/404.json".to_string()
            }),
            Config::new(&["xasida".to_string(), "/tmp/404.json".to_string()])
        );
    }
}
