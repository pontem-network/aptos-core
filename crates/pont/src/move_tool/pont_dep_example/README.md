This is a small example of using the new `pont` dependency. This shall be removed once we have
documentation/tests.

`pack2` contains a package which is used by `pack1` as follows:

```
[dependencies]
Pack2 = { pont = "http://localhost:8080", address = "default" }
```

To see it working:

```shell
# Start a node with an account
pont node run-local-testnet --with-faucet &
pont account create --account default --use-faucet 
# Compile and publish pack2
cd pack2
pont move compile --named-addresses project=default     
pont move publish --named-addresses project=default
# Compile pack1 agains the published pack2
cd ../pack1
pont move compile --named-addresses project=default     
```
