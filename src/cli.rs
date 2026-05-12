use std::env;

#[derive(Debug, PartialEq)]
pub enum CliAction {
    Run { config_path: String },
    Help,
}

pub fn parse_args() -> Result<CliAction, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    parse_args_from(&args)
}

pub fn parse_args_from(args: &[String]) -> Result<CliAction, String> {
    if args.is_empty() {
        return Ok(CliAction::Run {
            config_path: String::from("config.json"),
        });
    }

    match args[0].as_str() {
        "-h" | "--help" => Ok(CliAction::Help),
        "--config" => {
            if args.len() < 2 {
                Err(String::from("Nach --config fehlt der Dateiname."))
            } else if args.len() > 2 {
                Err(String::from("Zu viele Argumente nach --config."))
            } else {
                Ok(CliAction::Run {
                    config_path: args[1].clone(),
                })
            }
        }
        unknown => Err(format!("Unbekanntes Argument: {}", unknown)),
    }
}

pub fn print_help() {
    println!("Embedded Network Monitoring Framework");
    println!();
    println!("Verwendung:");
    println!("  cargo run");
    println!("  cargo run -- --config config.json");
    println!("  cargo run -- --help");
    println!();
    println!("Optionen:");
    println!("  --config <datei>   Pfad zur Konfigurationsdatei");
    println!("  -h, --help         Hilfe anzeigen");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_args_should_use_default_config() {
        let args: Vec<String> = vec![];

        let action = parse_args_from(&args).unwrap();

        assert_eq!(
            action,
            CliAction::Run {
                config_path: String::from("config.json")
            }
        );
    }

    #[test]
    fn config_arg_should_use_given_config_file() {
        let args = vec![String::from("--config"), String::from("test_config.json")];

        let action = parse_args_from(&args).unwrap();

        assert_eq!(
            action,
            CliAction::Run {
                config_path: String::from("test_config.json")
            }
        );
    }

    #[test]
    fn help_arg_should_return_help_action() {
        let args = vec![String::from("--help")];

        let action = parse_args_from(&args).unwrap();

        assert_eq!(action, CliAction::Help);
    }
}
