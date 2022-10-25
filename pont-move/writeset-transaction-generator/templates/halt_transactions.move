script {
    use pont_framework::transaction_publishing_option;
    fun main(pont_root: signer) {
        transaction_publishing_option::halt_all_transactions(&pont_root);
    }
}
