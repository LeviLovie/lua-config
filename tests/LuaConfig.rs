#[cfg(test)]
mod lua_config {
    #[cfg(test)]
    mod from_file {
        #[test]
        fn integer() {
            let config = lua_config::LuaConfig::from_file("./tests/data/integer.lua")
                .expect("Failed to load config")
                .with_default(include_bytes!("./data/integer.default.lua"))
                .expect("Failed to load default config")
                .execute()
                .expect("Failed to execute config");

            assert_eq!(config.get::<i32>("value"), Some(2));
            assert_eq!(config.get::<i32>("another_value"), Some(1));
        }

        #[test]
        fn number() {
            let config = lua_config::LuaConfig::from_file("./tests/data/number.lua")
                .expect("Failed to load config")
                .with_default(include_bytes!("./data/number.default.lua"))
                .expect("Failed to load default config")
                .execute()
                .expect("Failed to execute config");

            assert_eq!(config.get::<f64>("value"), Some(3.1415));
            assert_eq!(config.get::<f64>("another_value"), Some(3.56));
        }

        #[test]
        fn string() {
            let config = lua_config::LuaConfig::from_file("./tests/data/string.lua")
                .expect("Failed to load config")
                .with_default(include_bytes!("./data/string.default.lua"))
                .expect("Failed to load default config")
                .execute()
                .expect("Failed to execute config");

            assert_eq!(config.get::<String>("value"), Some("Hello!".to_string()));
            assert_eq!(
                config.get::<String>("another_value"),
                Some("Hello, Everyone!".to_string())
            );
        }

        #[test]
        fn boolean() {
            let config = lua_config::LuaConfig::from_file("./tests/data/boolean.lua")
                .expect("Failed to load config")
                .with_default(include_bytes!("./data/boolean.default.lua"))
                .expect("Failed to load default config")
                .execute()
                .expect("Failed to execute config");

            assert_eq!(config.get::<bool>("value"), Some(false));
            assert_eq!(config.get::<bool>("another_value"), Some(true));
        }
    }

    // function Default()
    //   return {
    //     value = 1,
    //     another_value = 2,
    //     second_level = {
    //       value = 3,
    //       another_value = 4,
    //       third_level = {
    //         value = 5,
    //         another_value = 6
    //       }
    //     }
    //   }
    // end
    #[test]
    fn complex() {
        let config = lua_config::LuaConfig::from_file("./tests/data/complex.lua")
            .expect("Failed to load config")
            .with_default(include_bytes!("./data/complex.default.lua"))
            .expect("Failed to load default config")
            .execute()
            .expect("Failed to execute config");

        assert_eq!(config.get::<i32>("value"), Some(2));
        assert_eq!(config.get::<i32>("another_value"), Some(2));
        let second_level = config.get::<lua_config::LuaTable>("second_level").unwrap();
        assert_eq!(second_level.get("value").unwrap().to::<i32>(), Some(4));
        assert_eq!(
            second_level.get("another_value").unwrap().to::<i32>(),
            Some(4)
        );
        let third_level = second_level
            .get("third_level")
            .unwrap()
            .to::<lua_config::LuaTable>()
            .unwrap();
        assert_eq!(third_level.get("value").unwrap().to::<i32>(), Some(6));
        assert_eq!(
            third_level.get("another_value").unwrap().to::<i32>(),
            Some(6)
        );
    }

    // #[test]
    // fn fetch_data() {
    //     let config = lua_config::LuaConfig::from_file("./tests/data/fetch_data.lua")
    //         .expect("Failed to load config")
    //         .execute()
    //         .expect("Failed to execute config");
    //
    //     assert_ne!(config.get::<String>("data/1"), None);
    // }
}
