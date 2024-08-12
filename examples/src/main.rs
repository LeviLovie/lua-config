fn main() {
    let _lua_config = lua_config::LuaConfig::from_file(
        "config.lua".to_string(),
        include_bytes!("../default_config.lua"),
    );
}
