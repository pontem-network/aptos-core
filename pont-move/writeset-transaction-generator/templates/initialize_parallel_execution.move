script {
    use pont_framework::parallel_execution_config;
    fun main(pont_root: signer, _execute_as: signer) {
        parallel_execution_config::initialize_parallel_execution(&pont_root);
    }
}
