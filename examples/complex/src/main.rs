use lua_config::LuaConfig;
use raylib::prelude::*;

fn main() {
    let config = LuaConfig::from_file("./data/config.lua")
        .expect("Failed to load config")
        .with_default(include_bytes!("../include/config.default.lua"))
        .expect("Failed to load default config")
        .execute()
        .expect("Failed to execute config");

    let (mut rl, thread) = raylib::init()
        .size(
            config.get("window").get("width").to::<i32>(),
            config.get("window").get("height").to::<i32>(),
        )
        .title(config.get("window").get("title").to::<String>().as_str())
        .build();

    let x = config.get("graphics").get("x").to::<i32>();
    let y = config.get("graphics").get("y").to::<i32>();
    let width = config.get("graphics").get("width").to::<i32>();
    let height = config.get("graphics").get("height").to::<i32>();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_rectangle(x, y, width, height, Color::RED);
    }
}
