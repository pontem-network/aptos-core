script {
    use self::resource_test;

    fun main(_signer: &signer) {
        resource_test::get_resource();
    }
}
