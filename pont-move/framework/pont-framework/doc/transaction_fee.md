
<a name="0x1_transaction_fee"></a>

# Module `0x1::transaction_fee`



-  [Resource `PontCoinCapabilities`](#0x1_transaction_fee_PontCoinCapabilities)
-  [Function `burn_fee`](#0x1_transaction_fee_burn_fee)
-  [Function `store_pont_coin_burn_cap`](#0x1_transaction_fee_store_pont_coin_burn_cap)


<pre><code><b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="pont_coin.md#0x1_pont_coin">0x1::pont_coin</a>;
<b>use</b> <a href="system_addresses.md#0x1_system_addresses">0x1::system_addresses</a>;
</code></pre>



<a name="0x1_transaction_fee_PontCoinCapabilities"></a>

## Resource `PontCoinCapabilities`



<pre><code><b>struct</b> <a href="transaction_fee.md#0x1_transaction_fee_PontCoinCapabilities">PontCoinCapabilities</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>&lt;<a href="pont_coin.md#0x1_pont_coin_PontCoin">pont_coin::PontCoin</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_transaction_fee_burn_fee"></a>

## Function `burn_fee`

Burn transaction fees in epilogue.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="transaction_fee.md#0x1_transaction_fee_burn_fee">burn_fee</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, fee: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="transaction_fee.md#0x1_transaction_fee_burn_fee">burn_fee</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, fee: u64) <b>acquires</b> <a href="transaction_fee.md#0x1_transaction_fee_PontCoinCapabilities">PontCoinCapabilities</a> {
    <a href="coin.md#0x1_coin_burn_from">coin::burn_from</a>&lt;PontCoin&gt;(
        <a href="account.md#0x1_account">account</a>,
        fee,
        &<b>borrow_global</b>&lt;<a href="transaction_fee.md#0x1_transaction_fee_PontCoinCapabilities">PontCoinCapabilities</a>&gt;(@pont_framework).burn_cap,
    );
}
</code></pre>



</details>

<a name="0x1_transaction_fee_store_pont_coin_burn_cap"></a>

## Function `store_pont_coin_burn_cap`

Only called during genesis.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="transaction_fee.md#0x1_transaction_fee_store_pont_coin_burn_cap">store_pont_coin_burn_cap</a>(pont_framework: &<a href="../../move-stdlib/doc/signer.md#0x1_signer">signer</a>, burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>&lt;<a href="pont_coin.md#0x1_pont_coin_PontCoin">pont_coin::PontCoin</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="transaction_fee.md#0x1_transaction_fee_store_pont_coin_burn_cap">store_pont_coin_burn_cap</a>(pont_framework: &<a href="../../move-stdlib/doc/signer.md#0x1_signer">signer</a>, burn_cap: BurnCapability&lt;PontCoin&gt;) {
    <a href="system_addresses.md#0x1_system_addresses_assert_pont_framework">system_addresses::assert_pont_framework</a>(pont_framework);
    <b>move_to</b>(pont_framework, <a href="transaction_fee.md#0x1_transaction_fee_PontCoinCapabilities">PontCoinCapabilities</a> { burn_cap })
}
</code></pre>



</details>


[move-book]: https://move-language.github.io/move/introduction.html
