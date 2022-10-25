script {
    use pont_framework::pont_governance;

    fun main(proposal_id: u64) {
        let framework_signer = pont_governance::resolve(proposal_id, @pont_framework);
        // Update voting duration of Pont governance proposals to 1 day. Other params don't change.
        let updated_voting_duration_secs = 24 * 60 * 60;
        let unchanged_min_voting_threshold = pont_governance::get_min_voting_threshold();
        let unchanged_required_proposer_stake = pont_governance::get_required_proposer_stake();
        pont_governance::update_governance_config(
            &framework_signer,
            unchanged_min_voting_threshold,
            unchanged_required_proposer_stake,
            updated_voting_duration_secs,
        );
    }
}
