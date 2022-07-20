//
// Copyright (C) 2019, 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: AGPL-3.0-or-later
//

fn main() {
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=static=kbupd_enclave_u");
    println!("cargo:rustc-link-lib=dylib=sgx_uae_service");
    println!("cargo:rustc-link-lib=dylib=sgx_urts");
    println!("cargo:rustc-link-lib=dylib=sgx_enclave_common");

    let mut cc = cc::Build::new();
    cc.file("c_src/kbupd_enclave_u.c").include("c_src").compile("kbupd_enclave_u");

    let mut protoc = prost_build::Config::new();
    protoc
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .field_attribute("node_id", "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]")
        .field_attribute(
            "EnclaveFrontendPartitionStatus.group_id",
            "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]",
        )
        .field_attribute(
            "EnclaveFrontendRangeStatus.group_id",
            "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]",
        )
        .field_attribute(
            "EnclaveReplicaPartitionStatus.group_id",
            "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]",
        )
        .field_attribute(
            "EnclaveReplicaPartitionStatus.service_id",
            "#[serde(with = \"serde_with::As::<Option<serde_with::hex::Hex>>\")]",
        )
        .field_attribute("BackupId.id", "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]")
        .field_attribute("ServiceId.id", "#[serde(with = \"serde_with::As::<serde_with::hex::Hex>\")]")
        .extern_path(".protobufs.kbupd_client", "kbupd_client")
        .compile_protos(&["src/kbupd.proto"], &["src/", "../kbupd_client/src/"])
        .expect("error compiling protobufs");
}
