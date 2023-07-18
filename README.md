# An Texa Holdem contract for WASM
Card's Valid ranks: one of [23456789TJQKA]
Card's Valid suits: one of [chsd]

Like: "Tc Jc Qc Kc Ac" is Royal flush

# Example of using texa holdem for WASM
wacthing ./build_deploy.sh 

# If you need use vmbridge to call Texa Holdem contract of Wasm
1. you must complie and deploy wasm
> ./optimize-wasm.sh
> 
> ./build_deploy.sh 

then you will follow log: 

``` 
store--------------
codeid:3
contract--------------
0x8F12F712176bb3e5926D012D0eA72FA2BBb85051
gas estimate: 339238
result--------------
{ type: wasm, attributes: [ { key: _contract_address, value: 0x8F12F712176bb3e5926D012D0eA72FA2BBb85051 }, { key: winner, value: 3 }, { key: cards, value: Flush, ace-high }, { key: hands, value: Tc Ac } ] }
```

the wasmconrtact is `0x8F12F712176bb3e5926D012D0eA72FA2BBb85051`

2. you must deploy evm contract 

> 合约代码为 ./evmcontract/contractpoker.sol
> >调用方法为: getWiner(string memory wasmAddr, string memory msgData)
> > wasmAddr为wasm合约地址
> >msgData为hex编码的调用wasm合约的输入 例如 hex.encode("{"poker":{"user_hands":["Kc 4c","3s 3h","5s 5h","3d Ah","Tc Ac","Th Ad","Kh Th","2s 2c","7c 6c"],"board":"3c 5c As Jc Qh"}}")