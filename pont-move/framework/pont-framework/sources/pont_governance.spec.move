spec pont_framework::pont_governance {
    spec reconfigure {
        use pont_framework::chain_status;
        requires chain_status::is_operating();
        requires timestamp::spec_now_microseconds() >= reconfiguration::last_reconfiguration_time();
    }
}
