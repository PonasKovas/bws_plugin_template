#![feature(backtrace)]

use async_ffi::{FfiFuture, FutureExt, LocalFfiFuture};
use bws_plugin::prelude::*;
use bws_plugin::register::RegPluginStruct;
use bws_plugin::vtable::BwsVTable;

#[no_mangle]
static BWS_ABI_VERSION: u64 =
    ((async_ffi::ABI_VERSION as u64) << 32) | (bws_plugin::ABI_VERSION as u64);

#[no_mangle]
unsafe extern "C" fn bws_library_init(register: unsafe extern "C" fn(RegPluginStruct)) {
    std::panic::set_hook(Box::new(|info| {
        let bt = std::backtrace::Backtrace::capture();
        println!("Plugin 'test_plugin' {}", info);
        println!("{}", bt);
    }));

    // Plugin::new("test-plugin", (1, 0, 0), entry)
    //     .add_dep("anti-grief", "=0.75.0")
    //     .register(register);

    Plugin::new("anti-grief", (0, 75, 1), entry).register(register);
}

extern "C" fn entry(
    _name: BwsString,
    vtable: BwsVTable,
    event_receiver: SendPtr<()>,
    // global_state: BwsGlobalState,
) -> FfiFuture<BwsUnit> {
    // initialize the vtable
    bws_plugin::vtable::init(vtable);

    async move {
        while let Some(event) = bws_plugin::receive_event(event_receiver).await {
            println!(
                "received event [{}]\n{:x}\n{:x}",
                event.0, event.1 .0 as usize, event.2 .0 as usize
            );
        }

        unit()
    }
    .into_ffi()
}
