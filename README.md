[![Rust](https://github.com/LeviLovie/lua-config/actions/workflows/rust.yml/badge.svg)](https://github.com/LeviLovie/lua-config/actions)
[![Documentation](https://docs.rs/lua-config/badge.svg)](https://docs.rs/lua-config)
[![Latest version](https://img.shields.io/crates/v/lua-config.svg)](https://crates.io/crates/lua-config)
![License](https://img.shields.io/crates/l/lua-config.svg)

# lua-config
A simple rust configuration tool using lua.

## Idea
I always have trouble configuring my rust applications. I wanted to try out lua as a tool for configurations as I could also do some logic while the config is being read. For example:
```lua
function Config()
	local config = {
		width = 800,
	}
	config.height = config.width * 3 / 4

	return config
end
```
As you can see the `height` value is calculated on the fly depending on the `width`.

## How to use
Install using `cargo add lua_config`.
Create a file for the default configuration, for example is:
```lua
function Default()
	return {
		name = "My Game",
		width = 800,
		height = 600,
	}
end
```
Than create a file for your configuration. This file should be alongside the application, if you are distributing your project, you will want to also distribute this file alongside the app. This can be done manually or automated with `build.rs`. For example:
```lua
function Config()
	local config = {
		width = 1280,
	}
	config.height = config.width * 3 / 4

	return config
end
```
Now you can use the configuration in your rusty code:
```rust
fn main() {
    let config = lua_config::LuaConfig::from_file("config.lua")
        .expect("Failed to load config")
        .with_default(include_bytes!("../default_config.lua"))
        .expect("Failed to load default config")
        .execute()
        .expect("Failed to execute config");

    println!("Width: {}\n", config.get("width").unwrap().to::<i32>().unwrap());
    println!("Height: {}\n", config.get("height").unwrap().to::<i32>().unwrap());
    println!("Config:\n{}", config);
}
```
The terminal output is:
```
Width: 800

Height: 600

Config:
height = Number(600)
name = String("My Game")
width = Integer(800)
```

## Features

### `crash_on_none`
To avoid execive `.unwrap()` you can add `crash_on_none` feature. That way `get()` and `to<T>()` functions are gonna return `LuaType` and `T` without `Option` respectivly. I case of an error the library will panic. This is added to avoid `.unwrap()` everywere, and gives the same result of crashing.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
