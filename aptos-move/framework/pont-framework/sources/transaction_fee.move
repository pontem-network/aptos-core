module pont_framework::transaction_fee {
    use pont_framework::coin::{Self, BurnCapability};
    use pont_framework::aptos_coin::AptosCoin;
    use pont_framework::system_addresses;

    friend pont_framework::genesis;
    friend pont_framework::transaction_validation;

    struct AptosCoinCapabilities has key {
        burn_cap: BurnCapability<AptosCoin>,
    }

    /// Burn transaction fees in epilogue.
    public(friend) fun burn_fee(account: address, fee: u64) acquires AptosCoinCapabilities {
        coin::burn_from<AptosCoin>(
            account,
            fee,
            &borrow_global<AptosCoinCapabilities>(@pont_framework).burn_cap,
        );
    }

    /// Only called during genesis.
    public(friend) fun store_aptos_coin_burn_cap(pont_framework: &signer, burn_cap: BurnCapability<AptosCoin>) {
        system_addresses::assert_pont_framework(pont_framework);
        move_to(pont_framework, AptosCoinCapabilities { burn_cap })
    }
}
