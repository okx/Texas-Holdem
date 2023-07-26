res=$(exchaincli tx wasm store ./artifacts/texa_poker.wasm --fees 0.01okt --from lifei --gas=20000000 -b block -y)
echo "store--------------$res"
code_id=$(echo "$res" | jq '.logs[0].events[1].attributes[0].value' | sed 's/\"//g')
echo "codeid:"$code_id
res=$(exchaincli tx wasm instantiate "$code_id" '{"name":"my test token"}' --label test1 --admin ex1gwcgl6g8k5ncyanuhk4pz9js9mrdqt657xxa8z --fees 0.001okt --from lifei -b block -y)
contractAddr=$(echo "$res" | jq '.logs[0].events[0].attributes[0].value' | sed 's/\"//g')
echo "contract--------------"
echo $contractAddr

res=$(exchaincli tx wasm execute $contractAddr '{"poker":{"user_hands":["Kc 4c","3s 3h","5s 5h","Tc Ac","3d Ah","Th Ad","Kh Th","2s 2c","7c 6c"],"board":"3c 5c As Jc Qh"}}' --fees 0.001okt --gas auto --from lifei -b block -y)
result=$(echo "$res" | jq -r '.logs[0].events[2]' | sed 's/\"//g')
echo "result--------------"
echo $result