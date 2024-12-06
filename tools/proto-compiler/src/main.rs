use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!("root: {}", root.display());

    // We build the proto files for the main penumbra_proto crate
    // and for the cnidarium crate separately, because the
    // cnidarium crate is supposed to be independent of the
    // rest of the Penumbra codebase and its proto structures.
    // Unfortunately, this means duplicating a lot of logic, because
    // we can't share the prost_build::Config between the two.

    let target_dir = root
        .join("..")
        .join("..")
        .join("crates")
        .join("proto")
        .join("src")
        .join("gen");
    let cnidarium_target_dir = root
        .join("..")
        .join("..")
        .join("crates")
        .join("cnidarium")
        .join("src")
        .join("gen");

    println!("target_dir: {}", target_dir.display());
    println!("cnidarium_target_dir: {}", cnidarium_target_dir.display());

    // https://github.com/penumbra-zone/penumbra/issues/3038#issuecomment-1722534133
    // Using the "no_lfs" suffix prevents matching a catch-all LFS rule.
    let descriptor_file_name = "proto_descriptor.bin.no_lfs";

    // prost_build::Config isn't Clone, so we need to make two.
    let mut config = prost_build::Config::new();
    let mut cnidarium_config = prost_build::Config::new();

    config.compile_well_known_types();
    // As recommended in pbjson_types docs.
    config.extern_path(".google.protobuf", "::pbjson_types");
    // NOTE: we need this because the rust module that defines the IBC types is external, and not
    // part of this crate.
    // See https://docs.rs/prost-build/0.5.0/prost_build/struct.Config.html#method.extern_path
    config.extern_path(".ibc", "::ibc_proto::ibc");
    // TODO: which of these is the right path?
    config.extern_path(".ics23", "::ics23");
    config.extern_path(".cosmos.ics23", "::ics23");

    cnidarium_config.compile_well_known_types();
    cnidarium_config.extern_path(".google.protobuf", "::pbjson_types");
    cnidarium_config.extern_path(".ibc", "::ibc_proto::ibc");
    cnidarium_config.extern_path(".ics23", "::ics23");
    cnidarium_config.extern_path(".cosmos.ics23", "::ics23");

    config
        .out_dir(&target_dir)
        .file_descriptor_set_path(&target_dir.join(descriptor_file_name))
        .enable_type_names();
    cnidarium_config
        .out_dir(&cnidarium_target_dir)
        .file_descriptor_set_path(&cnidarium_target_dir.join(descriptor_file_name))
        .enable_type_names();

    let rpc_doc_attr = r#"#[cfg(feature = "rpc")]"#;

    tonic_build::configure()
        .out_dir(&cnidarium_target_dir)
        .emit_rerun_if_changed(false)
        .server_mod_attribute(".", rpc_doc_attr)
        .client_mod_attribute(".", rpc_doc_attr)
        .compile_with_config(
            cnidarium_config,
            &["../../proto/penumbra/penumbra/cnidarium/v1/cnidarium.proto"],
            &["../../proto/penumbra/", "../../proto/rust-vendored/"],
        )?;

    tonic_build::configure()
        .out_dir(&target_dir)
        .emit_rerun_if_changed(false)
        // Only in Tonic 0.10
        //.generate_default_stubs(true)
        // We need to feature-gate the RPCs.
        .server_mod_attribute(".", rpc_doc_attr)
        .client_mod_attribute(".", rpc_doc_attr)
        .compile_with_config(
            config,
            &[
                "../../proto/penumbra/penumbra/core/app/v1/app.proto",
                "../../proto/penumbra/penumbra/core/asset/v1/asset.proto",
                "../../proto/penumbra/penumbra/core/txhash/v1/txhash.proto",
                "../../proto/penumbra/penumbra/core/component/auction/v1/auction.proto",
                "../../proto/penumbra/penumbra/core/component/compact_block/v1/compact_block.proto",
                "../../proto/penumbra/penumbra/core/component/community_pool/v1/community_pool.proto",
                "../../proto/penumbra/penumbra/core/component/dex/v1/dex.proto",
                "../../proto/penumbra/penumbra/core/component/distributions/v1/distributions.proto",
                "../../proto/penumbra/penumbra/core/component/funding/v1/funding.proto",
                "../../proto/penumbra/penumbra/core/component/fee/v1/fee.proto",
                "../../proto/penumbra/penumbra/core/component/governance/v1/governance.proto",
                "../../proto/penumbra/penumbra/core/component/ibc/v1/ibc.proto",
                "../../proto/penumbra/penumbra/core/component/sct/v1/sct.proto",
                "../../proto/penumbra/penumbra/core/component/shielded_graph/v1/shielded_graph.proto",
                "../../proto/penumbra/penumbra/core/component/shielded_pool/v1/shielded_pool.proto",
                "../../proto/penumbra/penumbra/core/component/stake/v1/stake.proto",
                "../../proto/penumbra/penumbra/core/keys/v1/keys.proto",
                "../../proto/penumbra/penumbra/core/num/v1/num.proto",
                "../../proto/penumbra/penumbra/core/transaction/v1/transaction.proto",
                "../../proto/penumbra/penumbra/crypto/decaf377_fmd/v1/decaf377_fmd.proto",
                "../../proto/penumbra/penumbra/crypto/decaf377_frost/v1/decaf377_frost.proto",
                "../../proto/penumbra/penumbra/crypto/decaf377_rdsa/v1/decaf377_rdsa.proto",
                "../../proto/penumbra/penumbra/crypto/tct/v1/tct.proto",
                "../../proto/penumbra/penumbra/custody/v1/custody.proto",
                "../../proto/penumbra/penumbra/custody/threshold/v1/threshold.proto",
                // Also included in the cnidarium crate directly.
                "../../proto/penumbra/penumbra/cnidarium/v1/cnidarium.proto",
                "../../proto/penumbra/penumbra/tools/summoning/v1/summoning.proto",
                "../../proto/penumbra/penumbra/util/tendermint_proxy/v1/tendermint_proxy.proto",
                "../../proto/penumbra/penumbra/view/v1/view.proto",
                "../../proto/rust-vendored/tendermint/abci/types.proto",
                "../../proto/rust-vendored/tendermint/types/validator.proto",
                "../../proto/rust-vendored/tendermint/p2p/types.proto",
                "../../proto/rust-vendored/cosmos/bank/v1beta1/query.proto",
                "../../proto/rust-vendored/cosmos/tx/v1beta1/service.proto",
                "../../proto/rust-vendored/cosmos/tx/v1beta1/tx.proto",
                "../../proto/rust-vendored/cosmos/tx/config/v1/config.proto",
                "../../proto/rust-vendored/cosmos/tx/signing/v1beta1/signing.proto",
                "../../proto/rust-vendored/cosmos/base/abci/v1beta1/abci.proto",
                "../../proto/rust-vendored/cosmos/crypto/multisig/v1beta1/multisig.proto",
                "../../proto/rust-vendored/ibc/applications/transfer/v1/query.proto",
                "../../proto/rust-vendored/ibc/core/channel/v1/query.proto",
                "../../proto/rust-vendored/ibc/core/client/v1/query.proto",
                "../../proto/rust-vendored/ibc/core/connection/v1/query.proto",
                "../../proto/rust-vendored/noble/forwarding/v1/account.proto",
                "../../proto/rust-vendored/noble/forwarding/v1/genesis.proto",
                "../../proto/rust-vendored/noble/forwarding/v1/packet.proto",
                "../../proto/rust-vendored/noble/forwarding/v1/query.proto",
                "../../proto/rust-vendored/noble/forwarding/v1/tx.proto",
            ],
            &["../../proto/penumbra/", "../../proto/rust-vendored/"],
        )?;

    // Finally, build pbjson Serialize, Deserialize impls:
    let descriptor_set = std::fs::read(target_dir.join(descriptor_file_name))?;
    let cnidarium_descriptor_set = std::fs::read(cnidarium_target_dir.join(descriptor_file_name))?;

    pbjson_build::Builder::new()
        .register_descriptors(&cnidarium_descriptor_set)?
        .ignore_unknown_fields()
        .out_dir(&cnidarium_target_dir)
        .build(&[".penumbra"])?;

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .ignore_unknown_fields()
        .out_dir(&target_dir)
        // These are all excluded because they're part of the Tendermint proxy,
        // so they use `tendermint` types that may not be Serialize/Deserialize,
        // and we don't need to serialize them with Serde anyways.
        .exclude([
            ".penumbra.util.tendermint_proxy.v1.ABCIQueryResponse".to_owned(),
            ".penumbra.util.tendermint_proxy.v1.GetBlockByHeightResponse".to_owned(),
            ".penumbra.util.tendermint_proxy.v1.GetStatusResponse".to_owned(),
        ])
        .build(&[".penumbra"])?;

    Ok(())
}
