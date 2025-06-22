use std::{any, ffi};

use anyhow::Ok;
use libffi::middle;

// 通过parms字符串来解析参数列表，比如说parms为"ptr ptr ptr -> i32" 那么就会通过addri地址调用fn(*mut std::ffi:c_void, *mut std::ffi:c_void, *mut std::ffi:c_void) -> std::ffi::c_int
// 然后将mlua::MultiValue解析传入, 并返

#[derive(Debug, Clone, Copy)]
enum ArgType {
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    Ptr,
}

fn parse_types(parms: &str) -> anyhow::Result<(Vec<ArgType>, ArgType)> {
    let tokens = parms.split_whitespace().collect::<Vec<&str>>();
    let arrow_pos = tokens.iter().position(|&t| t == "->").ok_or_else(|| anyhow::anyhow!("Invalid parameter format: missing '->'"))?;

    if tokens.len() != arrow_pos + 2 {
        anyhow::bail!("Invalid parameter format")
    }

    let arg_types_str = &tokens[..arrow_pos];
    let ret_type_str = tokens[arrow_pos + 1];

    let mut arg_types = Vec::new();

    let match_type = |t: &str|{
        Ok(match t {
            "u32" => ArgType::U32,
            "u64" => ArgType::U64,
            "i32" => ArgType::I32,
            "i64" => ArgType::I64,
            "f32" => ArgType::F32,
            "f64" => ArgType::F64,
            "ptr" => ArgType::Ptr,
            other => anyhow::bail!("Unsupported parameter type: {}", other)
        })
    };

    for t in arg_types_str {
        let t_type = match_type(t)?;
        arg_types.push(t_type);
    }

    let ret_type = match_type(ret_type_str)?;

    Ok((arg_types, ret_type))
}

fn arg_type_to_ffi_type(t: ArgType) -> middle::Type {
    match t {
        ArgType::U32 => middle::Type::u32(),
        ArgType::U64 => middle::Type::u64(),
        ArgType::I32 => middle::Type::i32(),
        ArgType::I64 => middle::Type::i64(),
        ArgType::F32 => middle::Type::f32(),
        ArgType::F64 => middle::Type::f64(),
        ArgType::Ptr => middle::Type::pointer(),
    }
}

fn arg_vec_to_ffi_type_vec(types: Vec<ArgType>) -> Vec<middle::Type> {
    let mut ffi_types = Vec::new();

    for t in types {
        ffi_types.push(arg_type_to_ffi_type(t));
    }

    ffi_types
}

fn lua_val_to_arg(t_type: ArgType, lua_val: &mlua::Value, arg_storage: &mut Vec<Box<dyn any::Any>>) -> anyhow::Result<middle::Arg>{
    Ok(match t_type {
        ArgType::U32 => {
            if let mlua::Value::Integer(num) = lua_val {
                let mut boxed = Box::new((*num) as u32);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::U64 => {
            if let mlua::Value::Integer(num) = lua_val {
                let mut boxed = Box::new((*num) as u64);
                let ptr = Box::as_mut_ptr(&mut boxed);

                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::I32 => {
            if let mlua::Value::Integer(num) = lua_val {
                let mut boxed = Box::new((*num) as i32);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::I64 => {
            if let mlua::Value::Integer(num) = lua_val {
                let mut boxed = Box::new((*num) as i64);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::F32=> {
            if let mlua::Value::Number(num) = lua_val {
                let mut boxed = Box::new((*num) as f32);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::F64=> {
            if let mlua::Value::Number(num) = lua_val {
                let mut boxed = Box::new((*num) as f64);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
        ArgType::Ptr => {
            if let mlua::Value::Integer(num) = lua_val {
                let mut boxed = Box::new((*num) as u64);
                let ptr = Box::as_mut_ptr(&mut boxed);
                
                arg_storage.push(boxed);
                middle::Arg::new(&ptr)
            } else {
                anyhow::bail!("Cannot convert Lua value to the expected type {:?}", t_type)
            }
        },
    })
}

pub fn call_fn(addr: u64, parms: &str, val: mlua::MultiValue) -> anyhow::Result<mlua::Value> {
    let (arg_types, ret_type) = parse_types(parms)?;
    if arg_types.len() < val.len() {
        anyhow::bail!("Number of passed arguments ({}) does not match parameter list ({})", val.len(), arg_types.len())
    }

    let cif = middle::Cif::new(arg_vec_to_ffi_type_vec(arg_types.clone()), arg_type_to_ffi_type(ret_type));

    let mut args: Vec<middle::Arg> = Vec::new();
    let mut arg_storage: Vec<Box<dyn any::Any>> = Vec::new();

    for (i, t_type) in arg_types.iter().enumerate() {
        let lua_val = val.get(i).ok_or_else(|| anyhow::anyhow!("Missing argument at position {}", i))?;
        let arg = lua_val_to_arg(*t_type, lua_val, &mut arg_storage)?;
        args.push(arg);
    }

    let code_ptr = middle::CodePtr(addr as *mut ffi::c_void);

    let result_ptr: *const ffi::c_void = unsafe { cif.call(code_ptr, &args) };

    let ret_value = match ret_type {
        ArgType::U32 => {
            let res = unsafe { *(result_ptr as *const u32) };
            mlua::Value::Integer(res as i64)
        },
        ArgType::U64 => {
            let res = unsafe { *(result_ptr as *const u64) };
            mlua::Value::Integer(res as i64)
        },
        ArgType::I32 => {
            let res = unsafe { *(result_ptr as *const i32) };
            mlua::Value::Integer(res as i64)
        },
        ArgType::I64 => {
            let res = unsafe { *(result_ptr as *const i64) };
            mlua::Value::Integer(res as i64)
        },
        ArgType::F32 => {
            let res = unsafe { *(result_ptr as *const f32) };
            mlua::Value::Number(res as f64)
        },
        ArgType::F64 => {
            let res = unsafe { *(result_ptr as *const f64) };
            mlua::Value::Number(res as f64)
        },
        ArgType::Ptr => {
            let res = unsafe { *(result_ptr as *const u64) };
            mlua::Value::Integer(res as i64)
        },
    };

    Ok(ret_value)
}