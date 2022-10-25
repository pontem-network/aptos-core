// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use backup_service::start_backup_service;
use pont_config::utils::get_available_port;
use pont_proptest_helpers::ValueGenerator;
use pont_temppath::TempPath;
use pont_types::{
    ledger_info::LedgerInfoWithSignatures,
    transaction::{TransactionToCommit, Version},
};
use pontdb::{
    test_helper::{arb_blocks_to_commit, update_in_memory_state},
    PontDB,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use storage_interface::DbWriter;
use tokio::runtime::Runtime;

pub fn tmp_db_empty() -> (TempPath, Arc<PontDB>) {
    let tmpdir = TempPath::new();
    let db = Arc::new(PontDB::new_for_test(&tmpdir));

    (tmpdir, db)
}

pub fn tmp_db_with_random_content() -> (
    TempPath,
    Arc<PontDB>,
    Vec<(Vec<TransactionToCommit>, LedgerInfoWithSignatures)>,
) {
    let (tmpdir, db) = tmp_db_empty();
    let mut cur_ver: Version = 0;
    let mut in_memory_state = db.buffered_state().lock().current_state().clone();
    let _ancestor = in_memory_state.base.clone();
    let blocks = ValueGenerator::new().generate(arb_blocks_to_commit());
    for (txns_to_commit, ledger_info_with_sigs) in &blocks {
        update_in_memory_state(&mut in_memory_state, txns_to_commit.as_slice());
        db.save_transactions(
            txns_to_commit,
            cur_ver, /* first_version */
            cur_ver.checked_sub(1),
            Some(ledger_info_with_sigs),
            true, /* sync_commit */
            in_memory_state.clone(),
        )
        .unwrap();
        cur_ver += txns_to_commit.len() as u64;
    }

    (tmpdir, db, blocks)
}

pub fn start_local_backup_service(db: Arc<PontDB>) -> (Runtime, u16) {
    let port = get_available_port();
    let rt = start_backup_service(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port), db);
    (rt, port)
}
