script {
    use pont_framework::validator_set;
    fun main(pont_root: signer) {
        {{#each addresses}}
        validator_set::remove_validator(&pont_root, @0x{{this}});
        {{/each}}
    }
}
