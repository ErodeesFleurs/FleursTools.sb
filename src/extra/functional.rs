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
    let start = start.unwrap();
    let stop = stop.unwrap();
    let step = step.unwrap_or(1);
    let data = (start..stop)
        .step_by(step as usize)
        .map(mlua::Value::Integer)
        .collect();
    Ok(array::Array::new(data))
}

pub fn map(
    _: &mlua::Lua,
    (func, array): (mlua::Function, array::Array),
) -> mlua::Result<array::Array> {
    let new_array = array.map(func)?;
    Ok(new_array)
}

pub fn chain(
    _: &mlua::Lua,
    (array1, array2): (array::Array, array::Array),
) -> mlua::Result<array::Array> {
    let mut data = array1.data.clone();
    data.extend(array2.data);
    Ok(array::Array::new(data))
}
