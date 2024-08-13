use std::str::from_utf8;

#[derive(Debug, Clone)]
pub enum LuaType {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(std::collections::HashMap<String, LuaType>),
}

impl LuaType {
    pub fn get<T>(&self) -> Option<T>
    where
        T: LuaConvert,
    {
        T::from_lua_type(self)
    }
}

pub trait LuaConvert: Sized {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self>;
}

impl LuaConvert for i32 {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        match lua_type {
            LuaType::Integer(i) => i32::try_from(*i).ok(),
            LuaType::Number(n) => Some(*n as i32),
            _ => None,
        }
    }
}

impl LuaConvert for i64 {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        match lua_type {
            LuaType::Integer(i) => Some(*i),
            LuaType::Number(n) => Some(*n as i64),
            _ => None,
        }
    }
}

impl LuaConvert for f64 {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        match lua_type {
            LuaType::Number(n) => Some(*n),
            LuaType::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
}

impl LuaConvert for bool {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        if let LuaType::Boolean(b) = lua_type {
            Some(*b)
        } else {
            None
        }
    }
}

impl LuaConvert for String {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        if let LuaType::String(s) = lua_type {
            Some(s.clone())
        } else {
            None
        }
    }
}

impl LuaConvert for std::collections::HashMap<String, LuaType> {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        if let LuaType::Table(table) = lua_type {
            Some(table.clone())
        } else {
            None
        }
    }
}

fn print_lua_type(value: LuaType, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
    match value {
        LuaType::Nil => write!(f, "nil"),
        LuaType::Boolean(b) => write!(f, "Boolean({})", b),
        LuaType::Integer(n) => write!(f, "Integer({})", n),
        LuaType::Number(n) => write!(f, "Number({})", n),
        LuaType::String(s) => write!(f, "String(\"{}\")", s),
        LuaType::Table(map) => {
            write!(f, "{{")?;
            for (key, value) in map.iter() {
                write!(f, "\n{}{} = ", " ".repeat(depth * 4), key)?;
                print_lua_type(value.clone(), f, depth + 1)?;
            }
            write!(f, "\n{}}}", " ".repeat((depth - 1) * 4))
        }
    }
}

impl std::fmt::Display for LuaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        print_lua_type(self.clone(), f, 1)
    }
}

pub struct LuaConfig {
    pub path: String,
    pub data: std::collections::HashMap<String, LuaType>,
}

impl LuaConfig {
    pub fn from_string(file: String, default: &[u8]) -> Self {
        let default: String = from_utf8(default).unwrap().to_string();
        let mut lua_config = LuaConfig {
            path: "File from String".to_string(),
            data: std::collections::HashMap::new(),
        };
        lua_config.init(file, default);
        lua_config
    }

    pub fn from_file(path: &str, default: &[u8]) -> Self {
        let file = std::fs::read_to_string(path).unwrap();
        let default: String = from_utf8(default).unwrap().to_string();
        let mut lua_config = LuaConfig {
            path: "File from String".to_string(),
            data: std::collections::HashMap::new(),
        };
        lua_config.init(file, default);
        lua_config
    }

    pub fn init(&mut self, config: String, default: String) {
        let lua = rlua::Lua::new();

        let default_values = LuaConfig::get_hashmap_by_function(&lua, &default, "Default");
        let config_values = LuaConfig::get_hashmap_by_function(&lua, &config, "Config");

        // for (key, _value) in config_values.iter() {
        //     if !default_values.contains_key(key) {
        //         println!(
        //             "Config value \"{}\" is not in the default values, it will be skipped :(",
        //             key
        //        );
        //     }
        // }

        let mut resulting_config_values: std::collections::HashMap<String, rlua::Value> =
            std::collections::HashMap::new();
        for (key, value) in default_values.iter() {
            let config_value = config_values.get(key);
            match config_value {
                Some(conf_value) => {
                    resulting_config_values.insert(key.to_string(), conf_value.clone());
                }
                None => {
                    resulting_config_values.insert(key.to_string(), value.clone());
                }
            }
        }

        self.data = self.convert_map(resulting_config_values);
    }

    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: LuaConvert,
    {
        let data = self.data.get(key);
        match data {
            Some(value) => value.get(),
            None => None,
        }
    }

    pub fn get_lua_type<T>(&self, key: &str) -> Option<&LuaType> {
        self.data.get(key)
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

    fn value_to_lua_type(&self, value: &rlua::Value) -> LuaType {
        match value {
            rlua::Value::Nil => LuaType::Nil,
            rlua::Value::Boolean(b) => LuaType::Boolean(*b),
            rlua::Value::Integer(n) => LuaType::Integer(*n),
            rlua::Value::Number(n) => LuaType::Number(*n),
            rlua::Value::String(s) => LuaType::String(s.to_str().unwrap_or_default().to_owned()),
            rlua::Value::Table(table) => {
                let mut map = std::collections::HashMap::new();
                for pair in table.clone().pairs::<String, rlua::Value>() {
                    if let Ok((key, value)) = pair {
                        map.insert(key, self.value_to_lua_type(&value));
                    }
                }
                LuaType::Table(map)
            }
            _ => unimplemented!("Conversion for this Lua type is not implemented yet"), // Handle other types as needed
        }
    }

    fn convert_map(
        &self,
        lua_map: std::collections::HashMap<String, rlua::Value>,
    ) -> std::collections::HashMap<String, LuaType> {
        let mut result = std::collections::HashMap::new();
        for (key, value) in lua_map {
            result.insert(key, self.value_to_lua_type(&value));
        }
        result
    }
}

impl std::fmt::Display for LuaConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            write!(f, "{} = {}\n", key, value)?;
        }
        Ok(())
    }
}
