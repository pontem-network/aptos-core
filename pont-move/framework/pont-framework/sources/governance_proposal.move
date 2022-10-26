/// Define the GovernanceProposal that will be used as part of on-chain governance by PontGovernance.
///
/// This is separate from the PontGovernance module to avoid circular dependency between PontGovernance and Stake.
module pont_framework::governance_proposal {
    friend pont_framework::pont_governance;

    struct GovernanceProposal has store, drop {}

    /// Create and return a GovernanceProposal resource. Can only be called by PontGovernance
    public(friend) fun create_proposal(): GovernanceProposal {
        GovernanceProposal {}
    }

    /// Useful for PontGovernance to create an empty proposal as proof.
    public(friend) fun create_empty_proposal(): GovernanceProposal {
        create_proposal()
    }

    #[test_only]
    public fun create_test_proposal(): GovernanceProposal {
        create_empty_proposal()
    }
}
