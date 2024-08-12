use std::str::from_utf8;

pub struct LuaConfig {
    pub path: String,
}

impl LuaConfig {
    pub fn from_string(file: String, default: &[u8]) -> Self {
        let default: String = from_utf8(default).unwrap().to_string();
        let mut lua_config = LuaConfig {
            path: "File from String".to_string(),
        };
        lua_config.init(file, default);
        lua_config
    }

    pub fn from_file(path: String, default: &[u8]) -> Self {
        let file = std::fs::read_to_string(path).unwrap();
        let default: String = from_utf8(default).unwrap().to_string();
        let mut lua_config = LuaConfig {
            path: "File from String".to_string(),
        };
        lua_config.init(file, default);
        lua_config
    }

    pub fn init(&mut self, config: String, default: String) {
        let lua = rlua::Lua::new();

        let default_values = LuaConfig::get_hashmap_by_function(&lua, &default, "Default");
        let config_values = LuaConfig::get_hashmap_by_function(&lua, &config, "Config");

        println!("Default values:");
        for (key, value) in default_values.iter() {
            println!("{}: {:?}", key, value);
        }

        println!("\nConfig values:");
        for (key, value) in config_values.iter() {
            println!("{}: {:?}", key, value);
        }

        // Check if any config values are missing from the default values
        for (key, _value) in config_values.iter() {
            if !default_values.contains_key(key) {
                println!(
                    "Config value \"{}\" is not in the default values, value will be skipped :(",
                    key
                );
            }
        }

        let mut resulting_config_values = std::collections::HashMap::new();
        for (key, value) in default_values.iter() {
            let config_value = config_values.get(key);
            match config_value {
                Some(conf_value) => {
                    resulting_config_values.insert(key, conf_value);
                }
                None => {
                    resulting_config_values.insert(key, value);
                }
            }
        }

        println!("\nResulting values:");
        for (key, value) in resulting_config_values.iter() {
            println!("{}: {:?}", key, value);
        }
    }

    fn get_hashmap_by_function<'lua>(
        lua: &'lua rlua::Lua,
        code: &str,
        function_name: &str,
    ) -> std::collections::HashMap<String, rlua::Value<'lua>> {
        let ctx = lua.load(code);
        ctx.exec().unwrap();
        let globals = lua.globals();
        let func = globals.get::<_, rlua::Function>(function_name).unwrap();
        let table = func.call::<_, rlua::Table>(()).unwrap();

        if table.is_empty() {
            eprintln!("Default function return table is empty");
            std::process::exit(1);
        }

        let mut values = std::collections::HashMap::new();
        for pair in table.pairs::<String, rlua::Value>() {
            let (key, value) = pair.unwrap();
            values.insert(key, value);
        }

        values
    }
}
