module pont_framework::transaction_fee {
    use pont_framework::coin::{Self, BurnCapability};
    use pont_framework::pont_coin::PontCoin;
    use pont_framework::system_addresses;

    friend pont_framework::genesis;
    friend pont_framework::transaction_validation;

    struct PontCoinCapabilities has key {
        burn_cap: BurnCapability<PontCoin>,
    }

    /// Burn transaction fees in epilogue.
    public(friend) fun burn_fee(account: address, fee: u64) acquires PontCoinCapabilities {
        coin::burn_from<PontCoin>(
            account,
            fee,
            &borrow_global<PontCoinCapabilities>(@pont_framework).burn_cap,
        );
    }

    /// Only called during genesis.
    public(friend) fun store_pont_coin_burn_cap(pont_framework: &signer, burn_cap: BurnCapability<PontCoin>) {
        system_addresses::assert_pont_framework(pont_framework);
        move_to(pont_framework, PontCoinCapabilities { burn_cap })
    }
}
