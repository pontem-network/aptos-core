script {
    use pont_framework::pont_governance;
    use pont_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = pont_governance::resolve(proposal_id, @pont_framework);
        // Update voting power increase limit to 10%.
        staking_config::update_voting_power_increase_limit(&framework_signer, 10);
    }
}
