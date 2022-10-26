/// The chain id distinguishes between different chains (e.g., testnet and the main network).
/// One important role is to prevent transactions intended for one chain from being executed on another.
/// This code provides a container for storing a chain id and functions to initialize and get it.
module pont_framework::chain_id {
    use pont_framework::system_addresses;

    friend pont_framework::genesis;

    struct ChainId has key {
        id: u8
    }

    /// Only called during genesis.
    /// Publish the chain ID `id` of this instance under the SystemAddresses address
    public(friend) fun initialize(pont_framework: &signer, id: u8) {
        system_addresses::assert_pont_framework(pont_framework);
        move_to(pont_framework, ChainId { id })
    }

    /// Return the chain ID of this instance
    public fun get(): u8 acquires ChainId {
        borrow_global<ChainId>(@pont_framework).id
    }

    #[test_only]
    public fun initialize_for_test(pont_framework: &signer, id: u8) {
        initialize(pont_framework, id);
    }

    #[test(pont_framework = @0x1)]
    fun test_get(pont_framework: &signer) acquires ChainId {
        initialize_for_test(pont_framework, 1u8);
        assert!(get() == 1u8, 1);
    }
}
