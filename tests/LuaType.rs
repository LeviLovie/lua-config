use lua_config::LuaType;

#[test]
fn test_lua_type_integer() {
    let lua_type = LuaType::Integer(42);
    assert_eq!(lua_type.get::<i32>(), Some(42));
    assert_eq!(lua_type.get::<i64>(), Some(42));
    assert_eq!(lua_type.get::<f64>(), Some(42.0));
    assert_eq!(lua_type.get::<bool>(), None);
    assert_eq!(lua_type.get::<String>(), None);
}

#[test]
fn test_lua_type_number() {
    let lua_type = LuaType::Number(42.0);
    assert_eq!(lua_type.get::<i32>(), Some(42));
    assert_eq!(lua_type.get::<i64>(), Some(42));
    assert_eq!(lua_type.get::<f64>(), Some(42.0));
    assert_eq!(lua_type.get::<bool>(), None);
    assert_eq!(lua_type.get::<String>(), None);
}

#[test]
fn test_lua_type_boolean() {
    let lua_type = LuaType::Boolean(true);
    assert_eq!(lua_type.get::<i32>(), None);
    assert_eq!(lua_type.get::<i64>(), None);
    assert_eq!(lua_type.get::<f64>(), None);
    assert_eq!(lua_type.get::<bool>(), Some(true));
    assert_eq!(lua_type.get::<String>(), None);
}

#[test]
fn test_lua_type_string() {
    let lua_type = LuaType::String("Hello, World!".to_string());
    assert_eq!(lua_type.get::<i32>(), None);
    assert_eq!(lua_type.get::<i64>(), None);
    assert_eq!(lua_type.get::<f64>(), None);
    assert_eq!(lua_type.get::<bool>(), None);
    assert_eq!(lua_type.get::<String>(), Some("Hello, World!".to_string()));
}
