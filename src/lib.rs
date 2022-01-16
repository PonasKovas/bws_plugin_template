#![feature(backtrace)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]

use async_ffi::{FfiFuture, FutureExt};
use bws_plugin::prelude::*;
use bws_plugin::register::{Plugin, RegPluginStruct};
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
        while let Some(event) = unsafe { bws_plugin::receive_event(event_receiver).await } {
            bws_plugin::log(
                "template",
                format!(
                    "received event\n[{}]\n{:x}\n{:x}",
                    event.0, event.1 .0 as usize, event.2 .0 as usize
                ),
                bws_plugin::LogLevel::Info,
            );

            unsafe {
                bws_plugin::finish_event_handling(event.2, false);
            }
            return unit();
            bws_plugin::spawn_task(
                async move {
                    let (mut shutdown_receiver, shutdown_atomic) =
                        bws_plugin::shutdown::register_for_graceful_shutdown();
                    loop {
                        if futures_lite::future::poll_once(&mut shutdown_receiver)
                            .await
                            .is_some()
                        {
                            bws_plugin::log(
                                "template",
                                "Cleaning up plugin task",
                                bws_plugin::LogLevel::Info,
                            );
                            bws_plugin::shutdown::gracefully_exited(shutdown_atomic);
                            break;
                        }
                        bws_plugin::log("template", "task abuse", bws_plugin::LogLevel::Error);
                    }

                    unit()
                }
                .into_ffi(),
            );
        }

        unit()
    }
    .into_ffi()
}
