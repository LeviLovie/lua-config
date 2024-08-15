fn main() {
    let config = lua_config::LuaConfig::from_file("config.lua")
        .expect("Failed to load config")
        .execute()
        .expect("Failed to execute config");

    println!(
        "A random fact about kittens: {}",
        config.get("data").get("1").to::<String>()
    );
}
