#[cfg(test)]
mod lua_type {
    #[test]
    fn integer() {
        let lua_type = lua_config::LuaType::Integer(42);
        assert_eq!(lua_type.get::<i32>(), Some(42));
        assert_eq!(lua_type.get::<i64>(), Some(42));
        assert_eq!(lua_type.get::<f64>(), Some(42.0));
        assert_eq!(lua_type.get::<bool>(), None);
        assert_eq!(lua_type.get::<String>(), None);
    }

    #[test]
    fn number() {
        let lua_type = lua_config::LuaType::Number(42.0);
        assert_eq!(lua_type.get::<i32>(), Some(42));
        assert_eq!(lua_type.get::<i64>(), Some(42));
        assert_eq!(lua_type.get::<f64>(), Some(42.0));
        assert_eq!(lua_type.get::<bool>(), None);
        assert_eq!(lua_type.get::<String>(), None);
    }

    #[test]
    fn boolean() {
        let lua_type = lua_config::LuaType::Boolean(true);
        assert_eq!(lua_type.get::<i32>(), None);
        assert_eq!(lua_type.get::<i64>(), None);
        assert_eq!(lua_type.get::<f64>(), None);
        assert_eq!(lua_type.get::<bool>(), Some(true));
        assert_eq!(lua_type.get::<String>(), None);
    }

    #[test]
    fn string() {
        let lua_type = lua_config::LuaType::String("Hello, World!".to_string());
        assert_eq!(lua_type.get::<i32>(), None);
        assert_eq!(lua_type.get::<i64>(), None);
        assert_eq!(lua_type.get::<f64>(), None);
        assert_eq!(lua_type.get::<bool>(), None);
        assert_eq!(lua_type.get::<String>(), Some("Hello, World!".to_string()));
    }
}
