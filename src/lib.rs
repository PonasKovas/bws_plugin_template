#![feature(backtrace)]

use async_ffi::{FfiFuture, FutureExt};
use bws_plugin::prelude::*;
use bws_plugin::PluginEntrySignature;

const ABI_VERSION: u16 = 0;

#[no_mangle]
static BWS_ABI_VERSION: u64 = ((async_ffi::ABI_VERSION as u64) << 32)
    | ((bws_plugin::BWS_PLUGIN_ABI_VERSION as u64) << 16)
    | ABI_VERSION as u64;

#[repr(C)]
struct PluginStructure {
    name: BwsStr<'static>,
    version: BwsTuple3<u64, u64, u64>,
    dependencies: BwsSlice<'static, BwsTuple2<BwsStr<'static>, BwsStr<'static>>>,
    entry: PluginEntrySignature,
}

#[no_mangle]
unsafe extern "C" fn bws_library_init(register: unsafe extern "C" fn(PluginStructure)) {
    std::panic::set_hook(Box::new(|info| {
        let bt = std::backtrace::Backtrace::capture();
        println!("Plugin 'test_plugin' {}", info);
        println!("{}", bt);
    }));

    register(PluginStructure {
        name: BwsStr::from_str("test_plugin"),
        version: BwsTuple3(0, 1, 0),
        dependencies: BwsSlice::from_slice(&[]),
        entry: entry,
    });
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
