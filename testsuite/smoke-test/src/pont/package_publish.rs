// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::pont::move_test_helpers;
use crate::smoke_test_environment::new_local_swarm_with_pont;
use forge::Swarm;

#[tokio::test]
async fn test_package_publish() {
    let mut swarm = new_local_swarm_with_pont(1).await;
    let mut info = swarm.pont_public_info();

    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let base_path_v1 = base_dir.join("src/pont/package_publish_modules_v1/");
    let base_path_v2 = base_dir.join("src/pont/package_publish_modules_v2/");
    let base_path_v3 = base_dir.join("src/pont/package_publish_modules_v3/");

    move_test_helpers::publish_package(&mut info, base_path_v1)
        .await
        .unwrap();
    // v2 is downwards compatible to v1
    move_test_helpers::publish_package(&mut info, base_path_v2)
        .await
        .unwrap();
    // v3 is not downwards compatible to v2
    move_test_helpers::publish_package(&mut info, base_path_v3)
        .await
        .unwrap_err();
}
