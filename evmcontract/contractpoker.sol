// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;

 
contract ContratPoker {

    address precomplieContarct = 0x0000000000000000000000000000000000000100;
    event wasmResult(string data);

    function getWiner(string memory wasmAddr, string memory msgData) public payable returns (bytes memory response){
        (bool success, bytes memory data) = precomplieContarct.call{value: msg.value}(
            abi.encodeWithSignature("callToWasm(string,string)", wasmAddr,msgData)
        );
        require(success);
        string memory res = abi.decode(data,(string));
        emit wasmResult(res);
        
        return data;
    }
}
