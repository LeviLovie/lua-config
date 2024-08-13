fn main() {
    // let config = lua_config::LuaConfig::from_file(
    //     "config.lua",
    //     "function Config() return { value = 200 } end".as_bytes(),
    // );

    let config =
        lua_config::LuaConfig::from_file("config.lua", include_bytes!("../default_config.lua"));

    println!("Width: {}\n", config.get::<i32>("width").unwrap());
    println!("Height: {}\n", config.get::<i32>("height").unwrap());
    println!("Config:\n{}", config);
}
