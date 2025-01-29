use mlua;

#[derive(Debug, Clone)]
pub struct Array {
    pub data: Vec<mlua::Value>,
}

impl Array {
    pub fn new(data: Vec<mlua::Value>) -> Self {
        Self { data }
    }

    pub fn from_lua_table(table: mlua::Table) -> mlua::Result<Self> {
        let data = (1..=table.len()?) // 按 Lua 索引顺序访问表
            .map(|i| table.get(i)) // 如果值不存在（`nil`），返回 `None`
            .collect::<mlua::Result<Vec<mlua::Value>>>()?;
        Ok(Self::new(data))
    }

    pub fn map(&self, func: mlua::Function) -> mlua::Result<Array> {
        let new_data = self
            .data
            .iter()
            .map(|v| func.call(v.clone()))
            .collect::<mlua::Result<Vec<mlua::Value>>>()?;
        Ok(Array::new(new_data))
    }

    pub fn flatten(&self, level: Option<usize>) -> mlua::Result<Array> {
        let level = level.unwrap_or(1);
        let mut new_data: Vec<mlua::Value> = Vec::new();
        for v in &self.data {
            match v {
                mlua::Value::Table(table) if level > 0 => {
                    let array = Array::from_lua_table(table.clone())?;
                    let flattened = array.flatten(Some(level - 1))?;
                    new_data.extend(flattened.data);
                }
                mlua::Value::UserData(ud) =>{
                    if ud.borrow::<Array>().is_ok() {
                        let array = ud.borrow::<Array>()?;
                        let flattened = array.flatten(Some(level - 1))?;
                        new_data.extend(flattened.data);
                    } else {
                        new_data.push(v.clone());
                    }
                },
                _ => new_data.push(v.clone()),
            }
        }
        Ok(Array::new(new_data))
    }
}

impl mlua::FromLua for Array {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => {
                let data = (1..=table.len()?) // 按 Lua 索引顺序访问表
                    .map(|i| table.get(i)) // 如果值不存在（`nil`），返回 `None`
                    .collect::<mlua::Result<Vec<mlua::Value>>>()?;
                Ok(Self::new(data))
            }
            mlua::Value::UserData(ud) => ud.borrow::<Array>().map(|array| array.clone()),
            _ => Err(mlua::Error::external("Expected table")),
        }
    }
}

impl mlua::UserData for Array {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("len", |_, this, _: ()| Ok(this.data.len()));

        methods.add_method("map", |_, this, func: mlua::Function| {
            let new_array = this.map(func)?;
            Ok(new_array)
        });

        methods.add_method("filter", |_, this, func: mlua::Function| {
            let new_data: mlua::Result<Vec<mlua::Value>> = this
                .data
                .iter()
                .filter_map(|v| match func.call(v.clone()) {
                    Ok(keep) => {
                        if keep {
                            Some(Ok(v.clone()))
                        } else {
                            None
                        }
                    }
                    Err(err) => Some(Err(err)),
                })
                .collect();
            Ok(Array::new(new_data?))
        });

        methods.add_method(
            "reduce",
            |_, this, (func, init): (mlua::Function, mlua::Value)| {
                let mut acc = init;
                for v in &this.data {
                    acc = func.call((acc, v.clone()))?;
                }
                Ok(acc)
            },
        );

        methods.add_method("chain", |_, this, other: Array| {
            let mut data = this.data.clone();
            data.extend(other.data);
            Ok(Array::new(data))
        });

        methods.add_method("flatten", |_, this, level: Option<usize>| {
            let new_array = this.flatten(level);
            Ok(new_array)
        });

        methods.add_method("to_table", |lua, this, ()| {
            let table = lua.create_table()?;
            for (i, v) in this.data.iter().enumerate() {
                table.set(i + 1, v.clone())?;
            }
            Ok(table)
        });
        methods.add_method("to_list", |lua, this, ()| {
            let table = lua.create_table()?;
            for (_, v) in this.data.iter().enumerate() {
                if v == &mlua::Value::Nil {
                    continue;
                }
                table.push(v.clone())?;
            }
            Ok(table)
        });

        methods.add_method("print", |_, this, ()| {
            println!("{:?}", this.data);
            Ok(())
        });

        methods.add_meta_method(mlua::MetaMethod::Index, |_, this, index: i64| {
            let index = if index < 0 {
                this.data.len() as i64 + index
            } else {
                index
            } as usize;
            if index < this.data.len() {
                Ok(this.data[index].clone())
            } else {
                Err(mlua::Error::external("Index out of bounds"))
            }
        });

        methods.add_meta_method(mlua::MetaMethod::Concat, |_, this, other: mlua::Value| {
            let mut data = this.data.clone();
            match other {
                mlua::Value::Table(table) => {
                    let array = Array::from_lua_table(table)?;
                    data.extend(array.data);
                }
                mlua::Value::UserData(ud) => {
                    let array = ud.borrow::<Array>()?;
                    data.extend(array.data.clone());
                }
                _ => return Err(mlua::Error::external("Expected table or array")),
            }
            Ok(Array::new(data))
        });

        methods.add_meta_method_mut(
            mlua::MetaMethod::NewIndex,
            |_, this, (index, value): (i64, mlua::Value)| {
                let index = if index < 0 {
                    this.data.len() as i64 + index
                } else {
                    index
                } as usize;
                if index < this.data.len() {
                    this.data[index] = value;
                    Ok(())
                } else {
                    Err(mlua::Error::external("Index out of bounds"))
                }
            },
        );
    }
}

pub fn lua_generate(_: &mlua::Lua, data: mlua::Value) -> mlua::Result<Array> {
    let table = data
        .as_table()
        .ok_or_else(|| mlua::Error::external("Expected table"))?;
    Ok(Array::from_lua_table(table.clone())?)
}
