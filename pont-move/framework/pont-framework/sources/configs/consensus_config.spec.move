spec pont_framework::consensus_config {
    spec set {
        use pont_framework::chain_status;
        use pont_framework::timestamp;
        requires chain_status::is_operating();
        requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
    }
}
