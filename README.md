# Babylon liquidity tool (BLT)
## overview
The Babylon liquidity tool accepts user liquidity and distributes it throughout the wider ecosystem. 


## Design
The vault allocates liquidity to the different destinations through an adaptor interface. The adaptors are designed such that the vault works agnostic of the ecosystem it is allocating to. This allows the vault to support EVM, CosmWasm, MoveVM, SVM and bitVM. Each adaptor deployment supports one chain and one contract on that chain. For the allocation of the liquidity, a vault on the destination is needed.

The vault reads allocation weights from the gauge. The gauge interface is designed in such a way that it supports a wide range of datasources to calculate the allocations. This includes
- Shared security amount bought by BSNs
- amount of economic btc security by BVSes
- bbn voting
- bribing pool
- ...

The following components of this system are mocked in the live deployment:
- ecosystem adaptor
- gauge
- dummy oracle