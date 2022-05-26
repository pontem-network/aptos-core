module AptosFramework::ModulePublisherConfig {
    // use Std::Vector;
    use Std::Errors;
    use AptosFramework::Timestamp;
    use AptosFramework::SystemAddresses;

    /// Error with config
    const ECONFIG: u64 = 0;

    struct ModulePublisherConfig has key {
        publisher_allowlist: vector<address>
    }

    public fun initialize(
        core_resource_account: &signer,
        publisher_allowlist: vector<address>,
    ) {
        Timestamp::assert_genesis();
        SystemAddresses::assert_core_resource(core_resource_account);
        assert!(!exists<ModulePublisherConfig>(@CoreResources), Errors::already_published(ECONFIG));

        move_to(
            core_resource_account,
            ModulePublisherConfig {
                publisher_allowlist,
            }
        );
    }
}
