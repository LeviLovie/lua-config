#[cfg(test)]
mod lua_config_tests {
    #[test]
    fn test_integer() {
        let config = lua_config::LuaConfig::from_file(
            "./tests/data/integer.lua",
            include_bytes!("./data/integer.default.lua"),
        );

        assert_eq!(config.get::<i32>("value"), Some(2));
        assert_eq!(config.get::<i32>("another_value"), Some(1));
    }

    #[test]
    fn test_number() {
        let config = lua_config::LuaConfig::from_file(
            "./tests/data/number.lua",
            include_bytes!("./data/number.default.lua"),
        );

        assert_eq!(config.get::<f64>("value"), Some(3.1415));
        assert_eq!(config.get::<f64>("another_value"), Some(3.56));
    }

    #[test]
    fn test_boolean() {
        let config = lua_config::LuaConfig::from_file(
            "./tests/data/boolean.lua",
            include_bytes!("./data/boolean.default.lua"),
        );

        assert_eq!(config.get::<bool>("value"), Some(false));
        assert_eq!(config.get::<bool>("another_value"), Some(true));
    }

    #[test]
    fn test_string() {
        let config = lua_config::LuaConfig::from_file(
            "./tests/data/string.lua",
            include_bytes!("./data/string.default.lua"),
        );

        assert_eq!(config.get::<String>("value"), Some("Hello!".to_string()));
        assert_eq!(
            config.get::<String>("another_value"),
            Some("Hello, Everyone!".to_string())
        );
    }
}
