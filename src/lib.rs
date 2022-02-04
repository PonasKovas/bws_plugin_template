use abi_stable::std_types::{RSlice, RStr, RVec, Tuple2};
use bws_plugin_interface::BwsPlugin;

#[no_mangle]
static BWS_ABI: u32 = bws_plugin_interface::ABI;

#[no_mangle]
static BWS_PLUGIN_ROOT: BwsPlugin = BwsPlugin {
    name: RStr::from_str("plugin_template"),
    version: RStr::from_str(env!("CARGO_PKG_VERSION")),
    dependencies: RSlice::from_slice(&[]),
};
