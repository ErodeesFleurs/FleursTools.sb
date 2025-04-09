use std::ffi::c_void;

use retour::static_detour;

use super::symbol::platform_base_addr;

type AddChatMessageFn =
    extern "C" fn(this_: *mut c_void, message: *const c_void, config: *const c_void);

static_detour! {
    static AddChatMessageHook: extern "C" fn(*mut c_void, *const c_void, *const c_void);
}

pub fn hook_func(addr: u64, name: &str) -> anyhow::Result<bool> {
    println!(
        "target addr: {:#x}, base addr: {:#x}",
        addr,
        platform_base_addr(name)?
    );
    let addr = addr + platform_base_addr(name)?;

    unsafe {
        println!("Making page writable for address: {:#x}", addr);
        // make_page_writable(addr as *mut c_void);
        println!("Hooking function at address: {:#x}", addr);
        AddChatMessageHook.initialize(
            std::mem::transmute::<u64, AddChatMessageFn>(addr),
            |arg1, arg2, arg3| hook_func_impl(arg1, arg2, arg3),
        )?;
        println!("Hook initialized for function at address: {:#x}", addr);
        AddChatMessageHook.enable()?;
        println!("Hook enabled for function at address: {:#x}", addr);
    };

    Ok(true)
}

fn hook_func_impl(this: *mut c_void, msg: *const c_void, config: *const c_void) {
    println!(
        "hook_func_impl: this={:?}, msg={:?}, config={:?}",
        this, msg, config
    );
}
