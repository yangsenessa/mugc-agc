dfx stop
set -e
#trap 'dfx stop' EXIT
echo "===========SETUP========="
dfx start --background --clean
YOU=$(dfx identity get-principal)
dfx deploy mugc-agc-backend --argument 'record{
   poll_account= "aaaa-aaaaa";
   token_block=1000
}'
dfx canister call mugc-agc-backend export_minting_contract