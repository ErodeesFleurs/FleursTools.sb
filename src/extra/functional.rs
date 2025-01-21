use super::array;

pub fn bind(
    _: &mlua::Lua,
    (func, args): (mlua::Function, mlua::Variadic<mlua::Value>),
) -> mlua::Result<mlua::Function> {
    Ok(func.bind(args)?)
}

pub fn range(
    _: &mlua::Lua,
    (start, stop, step): (Option<i64>, Option<i64>, Option<i64>),
) -> mlua::Result<array::Array> {
    let start = start.unwrap_or(1);
    let stop = stop.ok_or_else(|| mlua::Error::RuntimeError("`stop` is required".into()))?;
    let step = step.unwrap_or(1);

    if step == 0 {
        return Err(mlua::Error::RuntimeError("`step` cannot be zero".into()));
    }

    // 生成序列
    let mut data = Vec::new();
    let mut current = start;

    if step > 0 {
        while current <= stop {
            data.push(mlua::Value::Integer(current));
            current += step;
        }
    } else {
        while current >= stop {
            data.push(mlua::Value::Integer(current));
            current += step;
        }
    }

    Ok(array::Array::new(data))
}

pub fn map(
    _: &mlua::Lua,
    (func, array): (mlua::Function, array::Array),
) -> mlua::Result<array::Array> {
    let data = array.map(func)?;
    Ok(array::Array::new(data))
}

pub fn chain(
    _: &mlua::Lua,
    (array1, array2): (array::Array, array::Array),
) -> mlua::Result<array::Array> {
    let mut data = array1.data.clone();
    data.extend(array2.data);
    Ok(array::Array::new(data))
}
