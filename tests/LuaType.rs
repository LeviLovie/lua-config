#[cfg(test)]
mod lua_type {
    #[test]
    fn integer() {
        let lua_type = lua_config::LuaType::Integer(42);
        assert_eq!(lua_type.to::<i32>(), Some(42));
        assert_eq!(lua_type.to::<i64>(), Some(42));
        assert_eq!(lua_type.to::<f64>(), Some(42.0));
        assert_eq!(lua_type.to::<bool>(), None);
        assert_eq!(lua_type.to::<String>(), None);
    }

    #[test]
    fn number() {
        let lua_type = lua_config::LuaType::Number(42.0);
        assert_eq!(lua_type.to::<i32>(), Some(42));
        assert_eq!(lua_type.to::<i64>(), Some(42));
        assert_eq!(lua_type.to::<f64>(), Some(42.0));
        assert_eq!(lua_type.to::<bool>(), None);
        assert_eq!(lua_type.to::<String>(), None);
    }

    #[test]
    fn boolean() {
        let lua_type = lua_config::LuaType::Boolean(true);
        assert_eq!(lua_type.to::<i32>(), None);
        assert_eq!(lua_type.to::<i64>(), None);
        assert_eq!(lua_type.to::<f64>(), None);
        assert_eq!(lua_type.to::<bool>(), Some(true));
        assert_eq!(lua_type.to::<String>(), None);
    }

    #[test]
    fn string() {
        let lua_type = lua_config::LuaType::String("Hello, World!".to_string());
        assert_eq!(lua_type.to::<i32>(), None);
        assert_eq!(lua_type.to::<i64>(), None);
        assert_eq!(lua_type.to::<f64>(), None);
        assert_eq!(lua_type.to::<bool>(), None);
        assert_eq!(lua_type.to::<String>(), Some("Hello, World!".to_string()));
    }
}
