use tergo_formatter::config::Config;

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn parses_the_fully_specified_config() {
    log_init();
    let full_spec = include_str!("./config_cases/full_spec.toml");
    let config: Result<Config, _> = toml::from_str(full_spec);

    assert!(config.is_ok());
}

#[test]
fn parses_the_partial_config() {
    log_init();
    let partial_spec = include_str!("./config_cases/partial_spec.toml");
    let config: Result<Config, _> = toml::from_str(partial_spec);

    assert!(config.is_ok(), "Error was {config:?}");
}
