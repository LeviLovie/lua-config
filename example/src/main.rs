fn main() {
    let config = lua_config::LuaConfig::from_file(
        "config.lua".to_string(),
        include_bytes!("../default_config.lua"),
    );

    println!("Height: {:?}\n", config.get("height"));

    println!("Config:\n{}", config);
}
