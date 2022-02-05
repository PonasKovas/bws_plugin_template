use abi_stable::std_types::{RSlice, RStr, RVec, Tuple2};
use bws_plugin_interface::{
    debug, error,
    extra::Extra,
    global_state::{vtable::LogLevel, GState},
    info, trace, warn, BwsPlugin,
};

#[no_mangle]
static BWS_ABI: u32 = bws_plugin_interface::ABI;

#[no_mangle]
static BWS_PLUGIN_ROOT: BwsPlugin = BwsPlugin {
    name: RStr::from_str(env!("CARGO_PKG_NAME")),
    version: RStr::from_str(env!("CARGO_PKG_VERSION")),
    dependencies: RSlice::from_slice(&[]),
    enable,
    disable,
    extra: Extra::new(),
};

fn enable(gstate: &GState) {
    info!(@gstate, "Started");
}
fn disable(gstate: &GState) {
    info!(@gstate, "Stopped");
}
