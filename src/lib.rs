#![feature(backtrace)]

use async_ffi::{FfiFuture, FutureExt};
use bws_plugin::prelude::*;
use bws_plugin::register::{PluginEntrySignature, RegPluginStruct};

const ABI_VERSION: u16 = 0;

#[no_mangle]
static BWS_ABI_VERSION: u64 = ((async_ffi::ABI_VERSION as u64) << 32)
    | ((bws_plugin::BWS_PLUGIN_ABI_VERSION as u64) << 16)
    | ABI_VERSION as u64;

#[no_mangle]
unsafe extern "C" fn bws_library_init(register: unsafe extern "C" fn(RegPluginStruct)) {
    std::panic::set_hook(Box::new(|info| {
        let bt = std::backtrace::Backtrace::capture();
        println!("Plugin 'test_plugin' {}", info);
        println!("{}", bt);
    }));

    Plugin::new("test-plugin", (1, 0, 0), entry)
        .add_dep("anti-grief", "=0.75.0")
        .register(register);

    Plugin::new("anti-grief", (0, 75, 1), entry).register(register);
}

extern "C" fn entry(
    _name: BwsStr,
    // mut gate: BwsPluginGate,
    // global_state: BwsGlobalState,
) -> FfiFuture<BwsUnit> {
    async move {
        // loop {
        //     match (&mut gate).await {
        //         Some(mut guard) => {
        //             let event = guard.event();
        //             println!(
        //                 "Received {:?} event. port: {}",
        //                 event,
        //                 global_state.get_port()
        //             );
        //             match event.id {
        //                 0 => unsafe {
        //                     event.data.cast::<bool>().write(true);
        //                 },
        //                 _ => {}
        //             }
        //             guard.finish();
        //         }
        //         None => {
        //             // Channel dead... :(
        //             break;
        //         }
        //     }
        // }

        unit()
    }
    .into_ffi()
}
