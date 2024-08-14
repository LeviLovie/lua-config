fn main() {
    let config = lua_config::LuaConfig::from_file("config.lua")
        .expect("Failed to load config")
        .with_default(include_bytes!("../default_config.lua"))
        .expect("Failed to load default config")
        .execute()
        .expect("Failed to execute config");

    println!("Config:\n{}", config);
}
