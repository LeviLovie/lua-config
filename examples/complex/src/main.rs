use lua_config::{LuaConfig, LuaTable};
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
            config
                .get::<LuaTable>("window")
                .unwrap()
                .get("width")
                .unwrap()
                .to::<i32>()
                .unwrap(),
            config
                .get::<LuaTable>("window")
                .unwrap()
                .get("height")
                .unwrap()
                .to::<i32>()
                .unwrap(),
        )
        .title(
            config
                .get::<LuaTable>("window")
                .unwrap()
                .get("title")
                .unwrap()
                .to::<String>()
                .unwrap()
                .as_str(),
        )
        .build();

    let x = config
        .get::<LuaTable>("graphics")
        .unwrap()
        .get("x")
        .unwrap()
        .to::<i32>()
        .unwrap();
    let y = config
        .get::<LuaTable>("graphics")
        .unwrap()
        .get("y")
        .unwrap()
        .to::<i32>()
        .unwrap();
    let width = config
        .get::<LuaTable>("graphics")
        .unwrap()
        .get("width")
        .unwrap()
        .to::<i32>()
        .unwrap();
    let height = config
        .get::<LuaTable>("graphics")
        .unwrap()
        .get("height")
        .unwrap()
        .to::<i32>()
        .unwrap();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_rectangle(x, y, width, height, Color::RED);
    }
}
