script {
    use pont_framework::pont_governance;
    use pont_framework::coin;
    use pont_framework::pont_coin::PontCoin;
    use pont_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = pont_governance::resolve(proposal_id, @pont_framework);
        let one_pont_coin_with_decimals = 10 ** (coin::decimals<PontCoin>() as u64);
        // Change min to 1000 and max to 1M Pont coins.
        let new_min_stake = 1000 * one_pont_coin_with_decimals;
        let new_max_stake = 1000000 * one_pont_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}
