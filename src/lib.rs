use abi_stable::std_types::{RSlice, RStr, RVec, Tuple2};
use bws_plugin_interface::{global_state::GState, BwsPlugin};

#[no_mangle]
static BWS_ABI: u32 = bws_plugin_interface::ABI;

#[no_mangle]
static BWS_PLUGIN_ROOT: BwsPlugin = BwsPlugin {
    name: RStr::from_str("plugin_template"),
    version: RStr::from_str(env!("CARGO_PKG_VERSION")),
    dependencies: RSlice::from_slice(&[Tuple2(RStr::from_str("plugin1"), RStr::from_str("*"))]),
    enable,
    disable,
};

fn enable(gstate: &GState) {
    println!("Plugin template enabled");
    gstate
        .read()
        .plugins
        .iter()
        .find(|p| p.name() == "plugin1")
        .expect("plugin1 not found")
        .disable(gstate)
        .expect("plugin1 already disabled?");
}
fn disable(_gstate: &GState) {
    println!("Plugin template disabled");
}
