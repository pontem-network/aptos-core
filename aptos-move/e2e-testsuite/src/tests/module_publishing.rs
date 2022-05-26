// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_keygen::KeyGen;
use aptos_types::{
    on_chain_config::{ModulePublisherConfig, VMPublishingOption},
    transaction::{ExecutionStatus, TransactionStatus},
};
use language_e2e_tests::{
    account::{Account, AccountData, AccountRoleSpecifier}, compile::compile_module, current_function_name, executor::FakeExecutor,
    transaction_status_eq,
};
use move_deps::move_core_types::vm_status::StatusCode;

// A module with an address different from the sender's address should be rejected
#[test]
fn bad_module_address() {
    let mut executor = FakeExecutor::from_genesis_with_options(VMPublishingOption::open(), None);
    executor.set_golden_file(current_function_name!());

    // create a transaction trying to publish a new module.
    let account1 = executor.create_raw_account_data(1_000_000, 10);
    let account2 = executor.create_raw_account_data(1_000_000, 10);

    executor.add_account_data(&account1);
    executor.add_account_data(&account2);

    let program = format!(
        "
        module 0x{}.M {{
        }}
        ",
        account1.address()
    );

    // compile with account 1's address
    let compiled_module = compile_module(&program).1;
    // send with account 2's address
    let txn = account2
        .account()
        .transaction()
        .module(compiled_module)
        .sequence_number(10)
        .gas_unit_price(1)
        .sign();

    // TODO: This is not verified for now.
    // verify and fail because the addresses don't match
    // let vm_status = executor.verify_transaction(txn.clone()).status().unwrap();
    // assert!(vm_status.is(StatusType::Verification));
    // assert!(vm_status.major_status == StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER);

    // execute and fail for the same reason
    let output = executor.execute_transaction(txn);
    match output.status() {
        TransactionStatus::Keep(status) => {
            assert!(
                status
                    == &ExecutionStatus::MiscellaneousError(Some(
                        StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER
                    ))
            );
            // assert!(status.status_code() == StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER);
        }
        vm_status => panic!("Unexpected verification status: {:?}", vm_status),
    };
}

macro_rules! module_republish_test {
    ($name:ident, $prog1:literal, $prog2:literal) => {
        #[test]
        fn $name() {
            let mut executor = FakeExecutor::from_genesis_with_options(VMPublishingOption::open(), None);
            executor.set_golden_file(current_function_name!());

            let sequence_number = 2;
            let account = executor.create_raw_account_data(1_000_000, sequence_number);
            executor.add_account_data(&account);

            let program1 = String::from($prog1).replace("##ADDRESS##", &account.address().to_hex());
            let compiled_module1 = compile_module(&program1).1;

            let txn1 = account
                .account()
                .transaction()
                .module(compiled_module1.clone())
                .sequence_number(sequence_number)
                .sign();

            let program2 = String::from($prog2).replace("##ADDRESS##", &account.address().to_hex());
            let compiled_module2 = compile_module(&program2).1;

            let txn2 = account
                .account()
                .transaction()
                .module(compiled_module2)
                .sequence_number(sequence_number + 1)
                .sign();

            let output1 = executor.execute_transaction(txn1);
            executor.apply_write_set(output1.write_set());
            // first tx should allways succeed
            assert!(transaction_status_eq(
                &output1.status(),
                &TransactionStatus::Keep(ExecutionStatus::Success),
            ));

            let output2 = executor.execute_transaction(txn2);
            println!("{:?}", output2.status());
            // second tx should always fail, module republish is not allowed
            assert!(transaction_status_eq(
                &output2.status(),
                &TransactionStatus::Keep(ExecutionStatus::MiscellaneousError(Some(
                    StatusCode::DUPLICATE_MODULE_NAME
                ))),
            ));
        }
    };
}

// Publishing a module named M under the same address twice is OK (a module is self-compatible)
module_republish_test!(
    duplicate_module,
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
        public f() { label b0: return; }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
        public f() { label b0: return; }
    }
    " // ExecutionStatus::Success
);

// Republishing a module named M under the same address with a superset of the structs is OK
module_republish_test!(
    layout_compatible_module,
    "
    module 0x##ADDRESS##.M {
    }
    ",
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
    }
    " // ExecutionStatus::Success
);

// Republishing a module named M under the same address with a superset of public functions is OK
module_republish_test!(
    linking_compatible_module,
    "
    module 0x##ADDRESS##.M {
    }
    ",
    "
    module 0x##ADDRESS##.M {
        public f() { label b0: return; }
    }
    " // ExecutionStatus::Success
);

// Republishing a module named M under the same address that breaks data layout should be rejected
module_republish_test!(
    layout_incompatible_module_with_new_field,
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64, g: bool }
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

module_republish_test!(
    layout_incompatible_module_with_changed_field,
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        struct T { f: bool }
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

module_republish_test!(
    layout_incompatible_module_with_removed_field,
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        struct T {}
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

module_republish_test!(
    layout_incompatible_module_with_removed_struct,
    "
    module 0x##ADDRESS##.M {
        struct T { f: u64 }
    }
    ",
    "
    module 0x##ADDRESS##.M {
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

// Republishing a module named M under the same address that breaks linking should be rejected
module_republish_test!(
    linking_incompatible_module_with_added_param,
    "
    module 0x##ADDRESS##.M {
        public f() { label b0: return; }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        public f(_a: u64) { label b0: return; }
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

module_republish_test!(
    linking_incompatible_module_with_changed_param,
    "
    module 0x##ADDRESS##.M {
        public f(_a: u64) { label b0: return; }
    }
    ",
    "
    module 0x##ADDRESS##.M {
        public f(_a: bool) { label b0: return; }
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

module_republish_test!(
    linking_incompatible_module_with_removed_pub_fn,
    "
    module 0x##ADDRESS##.M {
        public f() { label b0: return; }
    }
    ",
    "
    module 0x##ADDRESS##.M {
    }
    " // ExecutionStatus::MiscellaneousError(Some(StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE))
);

#[test]
pub fn test_publishing_modules_proper_sender() {
    // create a FakeExecutor with a genesis from file
    let mut executor = FakeExecutor::allowlist_genesis();
    executor.set_golden_file(current_function_name!());

    // create a transaction trying to publish a new module.
    let sender = Account::new_aptos_root();

    let program = format!(
        "
        module 0x{}.M {{
        }}
        ",
        sender.address(),
    );

    let random_script = compile_module(&program).1;
    let txn = sender
        .transaction()
        .module(random_script)
        .sequence_number(0)
        .sign();
    assert_eq!(executor.verify_transaction(txn.clone()).status(), None);
    assert_eq!(
        executor.execute_transaction(txn).status(),
        &TransactionStatus::Keep(ExecutionStatus::Success)
    );
}

#[test]
pub fn test_publishing_modules_invalid_sender() {
    // create a FakeExecutor with a genesis from file
    let mut executor = FakeExecutor::allowlist_genesis();
    executor.set_golden_file(current_function_name!());

    // create a transaction trying to publish a new module.
    let sender = executor.create_raw_account_data(1_000_000, 10);
    executor.add_account_data(&sender);

    let program = String::from(
        "
        module 0x1.M {
        }
        ",
    );

    let random_script = compile_module(&program).1;
    let txn = sender
        .account()
        .transaction()
        .module(random_script)
        .sequence_number(10)
        .sign();
    assert_eq!(
        executor.execute_transaction(txn).status(),
        // this should be MODULE_ADDRESS_DOES_NOT_MATCH_SENDER but we don't do this check in prologue now
        &TransactionStatus::Keep(ExecutionStatus::MiscellaneousError(Some(
            StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER
        ))),
    );
}

#[test]
pub fn test_publishing_allow_modules() {
    // create a FakeExecutor with a genesis from file
    let mut executor = FakeExecutor::from_genesis_with_options(VMPublishingOption::open(), None);
    executor.set_golden_file(current_function_name!());

    // create a transaction trying to publish a new module.
    let sender = executor.create_raw_account_data(1_000_000, 10);
    executor.add_account_data(&sender);

    let program = format!(
        "
        module 0x{}.M {{
        }}
        ",
        sender.address(),
    );

    let random_script = compile_module(&program).1;
    let txn = sender
        .account()
        .transaction()
        .module(random_script)
        .sequence_number(10)
        .sign();
    assert_eq!(executor.verify_transaction(txn.clone()).status(), None);
    assert_eq!(
        executor.execute_transaction(txn).status(),
        &TransactionStatus::Keep(ExecutionStatus::Success)
    );
}

#[test]
fn check_module_update_allow_list() {
    let (privkey, pubkey) = KeyGen::from_os_rng().generate_keypair();
    let allowed_module_publisher = aptos_types::account_address::from_public_key(&pubkey);

    let publisher_allowlist = vec![allowed_module_publisher.clone()];

    let mut executor = FakeExecutor::from_genesis_with_options(VMPublishingOption::open(), Some(ModulePublisherConfig::from(publisher_allowlist)));

    let sequence_number = 2;
    let account = Account::new_validator(allowed_module_publisher, privkey, pubkey);
    let sender = AccountData::with_account(account, 1_000_000, sequence_number, AccountRoleSpecifier::ParentVASP);
    // let sender = executor.create_raw_account_data(1_000_000, sequence_number);
    executor.add_account_data(&sender);
    let non_allowed_sender = executor.create_raw_account_data(1_000_000, sequence_number);
    executor.add_account_data(&non_allowed_sender);

    // create a transaction trying to publish a new module.
    let module = format!(
        "
        module 0x{}.M {{
            struct T {{ f: address }}
        }}
        ",
        sender.address(),
    );

    let random_script = compile_module(&module).1;
    let txn = sender
        .account()
        .transaction()
        .module(random_script)
        .sequence_number(sequence_number)
        .sign();
    assert_eq!(executor.verify_transaction(txn.clone()).status(), None);
    let output = executor.execute_transaction(txn);
    executor.apply_write_set(output.write_set());
    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(ExecutionStatus::Success)
    );

    // update module from address that contains in allow list
    let updated_module = format!(
        "
        module 0x{}.M {{
            struct T {{ f: address }}
            struct U {{ f: u64 }}
        }}
        ",
        sender.address(),
    );

    let updated_script = compile_module(&updated_module).1;
    let update_txn = sender
        .account()
        .transaction()
        .module(updated_script)
        .sequence_number(sequence_number + 1)
        .sign();
    assert_eq!(
        executor.verify_transaction(update_txn.clone()).status(),
        None
    );
    let output = executor.execute_transaction(update_txn);
    executor.apply_write_set(output.write_set());
    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(ExecutionStatus::Success)
    );

    // publish new module from second address
    let second_module = format!(
        "
        module 0x{}.M {{
            struct T {{ f: address }}
        }}
        ",
        non_allowed_sender.address(),
    );

    let second_script = compile_module(&second_module).1;
    let txn = non_allowed_sender
        .account()
        .transaction()
        .module(second_script)
        .sequence_number(sequence_number)
        .sign();
    assert_eq!(executor.verify_transaction(txn.clone()).status(), None);
    let output = executor.execute_transaction(txn);
    executor.apply_write_set(output.write_set());
    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(ExecutionStatus::Success)
    );

    // try update module from second address that not contains in allow list
    let reupdated_module = format!(
        "
        module 0x{}.M {{
            struct T {{ f: address }}
            struct U {{ f: u64 }}
        }}
        ",
        non_allowed_sender.address(),
    );

    let reupdated_script = compile_module(&reupdated_module).1;
    let disallow_txn = non_allowed_sender
        .account()
        .transaction()
        .module(reupdated_script)
        .sequence_number(sequence_number + 1)
        .sign();
    assert_eq!(
        executor.verify_transaction(disallow_txn.clone()).status(),
        None
    );
    let output = executor.execute_transaction(disallow_txn);
    executor.apply_write_set(output.write_set());
    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(ExecutionStatus::MiscellaneousError(Some(
            StatusCode::DUPLICATE_MODULE_NAME
        )))
    );
}
