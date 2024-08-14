use std::error::Error;
use std::str::from_utf8;

#[derive(Debug, Clone)]
pub enum LuaType {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
    Table(LuaTable),
}

pub type LuaTable = std::collections::HashMap<String, LuaType>;

impl LuaType {
    pub fn to<T>(&self) -> Option<T>
    where
        T: LuaConvert,
    {
        T::from_lua_type(self)
    }

    pub fn get(&self, key: &str) -> Option<&LuaType> {
        if let LuaType::Table(table) = self {
            table.get(key)
        } else {
            None
        }
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

impl LuaConvert for f32 {
    fn from_lua_type(lua_type: &LuaType) -> Option<Self> {
        match lua_type {
            LuaType::Integer(i) => Some(*i as f32),
            LuaType::Number(n) => Some(*n as f32),
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

impl LuaConvert for LuaTable {
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
    pub data: LuaTable,
    config: String,
    default: Option<String>,
}

impl LuaConfig {
    pub fn from_string(file: String) -> Self {
        LuaConfig {
            data: std::collections::HashMap::new(),
            config: file,
            default: None,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = std::fs::read_to_string(path)?;
        Ok(LuaConfig::from_string(file))
    }

    pub fn with_default(mut self, default: &[u8]) -> Result<Self, Box<dyn Error>> {
        self.default = Some(from_utf8(default)?.to_string());
        Ok(self)
    }

    pub fn execute(mut self) -> Result<Self, Box<dyn Error>> {
        let lua = rlua::Lua::new();
        let config_values = LuaConfig::get_hashmap_by_function(&lua, &self.config, "Config")?;

        if self.default.is_some() {
            let default_values = LuaConfig::get_hashmap_by_function(
                &lua,
                &self.default.clone().unwrap(),
                "Default",
            )?;

            // Recursivly return error if any value in the config_values is not present in the default_values
            fn check_config_table(
                config: &std::collections::HashMap<String, LuaType>,
                default: &std::collections::HashMap<String, LuaType>,
            ) -> Result<(), Box<dyn Error>> {
                for (key, value) in config.iter() {
                    if let Some(default_value) = default.get(key) {
                        match value {
                            LuaType::Table(table) => {
                                if let LuaType::Table(default_table) = default_value {
                                    check_config_table(table, default_table)?;
                                } else {
                                    return Err(format!("Key {} is not a table", key).into());
                                }
                            }
                            _ => {}
                        }
                    } else {
                        return Err(format!("Key {} not found in default config", key).into());
                    }
                }
                Ok(())
            }
            check_config_table(&config_values, &default_values)?;

            // Recursivly merge the default_values into the config_values
            fn merge_tables(
                config: std::collections::HashMap<String, LuaType>,
                default: std::collections::HashMap<String, LuaType>,
            ) -> std::collections::HashMap<String, LuaType> {
                let mut result = default.clone();
                for (key, value) in config {
                    match value {
                        LuaType::Table(table) => {
                            if let LuaType::Table(default_table) = default.get(&key).unwrap() {
                                result.insert(
                                    key,
                                    LuaType::Table(merge_tables(table, default_table.clone())),
                                );
                            }
                        }
                        _ => {
                            result.insert(key, value);
                        }
                    }
                }
                result
            }

            self.data = merge_tables(config_values, default_values);
        } else {
            self.data = config_values;
        }

        Ok(self)
    }

    pub fn get(&self, key: &str) -> Option<&LuaType> {
        self.get_lua_type(key)
    }

    pub fn get_lua_type(&self, key: &str) -> Option<&LuaType> {
        self.data.get(key)
    }

    fn declare_lua_functions(ctx: &rlua::Context) -> Result<(), rlua::Error> {
        let _globals = ctx.globals();

        let fetch_data = ctx.create_function(|lua_ctx, url: String| {
            let response = reqwest::blocking::get(url).expect("Failed to fetch data");
            let table = LuaConfig::lua_table_from_json(lua_ctx, &response.text().unwrap())
                .expect("Failed to convert JSON to Lua table");
            Ok(table)
        })?;
        _globals.set("fetch_data", fetch_data)?;

        Ok(())
    }

    fn lua_table_from_json<'lua>(
        lua: &'lua rlua::Lua,
        json: &str,
    ) -> Result<rlua::Table<'lua>, Box<dyn Error>> {
        let json = json::parse(json)?;

        fn convert_json_to_lua<'lua>(
            lua: &'lua rlua::Lua,
            json_value: &json::JsonValue,
        ) -> Result<rlua::Value<'lua>, Box<dyn Error>> {
            match json_value {
                json::JsonValue::Null => Ok(rlua::Value::Nil),
                json::JsonValue::String(s) => Ok(rlua::Value::String(lua.create_string(s)?)),
                json::JsonValue::Number(n) => Ok(rlua::Value::Number(
                    n.as_fixed_point_i64(0).unwrap_or_default() as f64,
                )),
                json::JsonValue::Boolean(b) => Ok(rlua::Value::Boolean(*b)),
                json::JsonValue::Object(obj) => {
                    let table = lua.create_table()?;
                    for (key, value) in obj.iter() {
                        table.set(key, convert_json_to_lua(lua, value)?)?;
                    }
                    Ok(rlua::Value::Table(table))
                }
                json::JsonValue::Array(arr) => {
                    let table = lua.create_table()?;
                    for (i, value) in arr.iter().enumerate() {
                        table.set(i + 1, convert_json_to_lua(lua, value)?)?;
                    }
                    Ok(rlua::Value::Table(table))
                }
                _ => unimplemented!("This datatype is not implemented yet"),
            }
        }

        let lua_value = convert_json_to_lua(lua, &json)?;

        if let rlua::Value::Table(table) = lua_value {
            Ok(table)
        } else {
            Err("Root element is not a table".into())
        }
    }

    fn get_hashmap_by_function<'lua>(
        lua: &'lua rlua::Lua,
        code: &str,
        function_name: &str,
    ) -> Result<std::collections::HashMap<String, LuaType>, Box<dyn Error>> {
        let ctx = lua.load(code);
        LuaConfig::declare_lua_functions(&lua).unwrap();

        ctx.exec()?;
        let globals = lua.globals();
        let func = match globals.get::<_, rlua::Function>(function_name) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("Error getting function {}: {}", function_name, e).into());
            }
        };
        let table = match func.call::<_, rlua::Table>(()) {
            Ok(t) => t,
            Err(e) => {
                return Err(format!("Error calling function {}: {}", function_name, e).into());
            }
        };

        if table.is_empty() {
            return Err(format!("Function {} returned an empty table", function_name).into());
        }

        let mut values = std::collections::HashMap::new();
        for pair in table.pairs::<String, rlua::Value>() {
            let (key, value) = pair?;
            let value = LuaConfig::value_to_lua_type(&value);
            values.insert(key, value);
        }

        Ok(values)
    }

    fn value_to_lua_type(value: &rlua::Value) -> LuaType {
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
                        map.insert(key, LuaConfig::value_to_lua_type(&value));
                    }
                }
                LuaType::Table(map)
            }
            _ => unimplemented!("Conversion for this Lua type is not implemented yet"),
        }
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
