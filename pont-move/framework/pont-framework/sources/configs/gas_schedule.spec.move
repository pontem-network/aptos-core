spec pont_framework::gas_schedule {
    spec set_gas_schedule {
        use pont_framework::chain_status;
        requires chain_status::is_operating();
    }

    spec set_storage_gas_config {
        use pont_framework::chain_status;
        requires chain_status::is_operating();
    }
}
