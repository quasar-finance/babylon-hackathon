# Babylon liquidity tool (BLT)
## overview
The Babylon liquidity tool is a tool to accept user liquidity, distribute it throughout the wider ecosystem. The vault reads allocation weights from the gauge. The gauge interface is designed in such a way that it supports a wide range of datasources to calculate the allocations. Potential sources of allocations range from
- Shared security amount bought by BSNs
- amount of economic btc security by BVSes
- bbn voting
- bribing pool
- any combination of the above
- anything expressable in uints

The vault allocates liquidity to the different destinations through an adaptor interface. The adaptors are designed such that the vault works agnostic of the ecosystem it is allocating to. This allows the vault to support EVM, CosmWasm, MoveVM, SVM and bitVM. Each adaptor deployment supports one chain and one contract on that chain. For the allocation of the liquidity, a vault on the destination is needed.

## Mocks
The following components of this sytem are mocked in the live deployment:
- Ecosystem adaptor
- gauge
- dummy oracle

## future work
To make this system complete, next to implementing the mocked contracts, there are 2 main bodies of work that need to be completed. The ecosystem adaptors' bridges and the securing of the vaults on the destination chains. For the bridge stack, a lot already exists or is up and coming, such as Union, Axelar and Layerzero. The adaptors can easily be written to work with any of these. For securing of the vaults, a restaking stack with custom slashing conditions needs to be used. The slashing conditions work by giving operators access to an emergency button on the destination vaults. If the destination vaults start running at a major loss for a longer time period, the operators are obligated to pull the liquidity. The slashed security is then used reimburse the depositors. Effectively making this AVS a defacto insurance fund. This can be build through things like Satlayer, Eigenlayer or Symbiotic