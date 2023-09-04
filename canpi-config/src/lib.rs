enum LogLevel {
    NOTSET,
    INFO,
    WARN,
    NOTICE,
    DEBUG,
}

struct ConfigItem {
    name: str,
    prompt: str,
    value: str,
    type:

}
struct CapConfig {
    grid_enabled: bool,
    log_level: LogLevel,
    tcp_port: int,
    grid_port: int,
    turnout_file: str,
    can_id: int,
    can_device: str,
    node_number: int
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
