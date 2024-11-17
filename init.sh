dfx stop
set -e
#trap 'dfx stop' EXIT
echo "===========SETUP========="
dfx start --background --clean


dfx deploy icrc1_ledger_canister --argument "(variant {
  Init = record {
    token_symbol = \"UNIVOICE\";
    token_name = \"L-UNIVOICE\";
    minting_account = record {
      owner = principal \"$(dfx identity --identity anonymous get-principal)\"
    };
    transfer_fee = 10_000;
    metadata = vec {};
    initial_balances = vec {
      record {
        record {
          owner = principal \"$(dfx identity --identity default get-principal)\";
        };
        10_000_000_000;
      };
    };
    archive_options = record {
      num_blocks_to_archive = 1000;
      trigger_threshold = 2000;
      controller_id = principal \"$(dfx identity --identity anonymous get-principal)\";
    };
    feature_flags = opt record {
      icrc2 = true;
    };
  }
})"
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx identity --identity default get-principal)\";
})"
echo "===========SETUP DONE========="
dfx deploy  mugc-agc-backend 
dfx canister call mugc-agc-backend update_minting_contract "(record{
       poll_account=  \"$(dfx canister id mugc-agc-backend)\";
       token_block=1000
   }
)"
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx identity --identity default get-principal)\";
})"
# approve the token_transfer_from_backend canister to spend 100 tokens
echo "===========icrc2_approve========="

dfx canister call --identity default icrc1_ledger_canister icrc2_approve "(
  record {
    spender= record {
      owner = principal \"$(dfx canister id mugc-agc-backend)\";
    };
    amount = 10_000_000_000: nat;
  }
)"
echo "===========icrc2_approve_end========="


dfx canister call --identity default mugc-agc-backend transfer "(record {
  amount = 100_000_000;
  to_account = record {
    owner = principal \"$(dfx canister id mugc-agc-backend)\";
  };
})"

echo "===========query balance========"

dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {
  owner = principal \"$(dfx canister id mugc-agc-backend)\";
})"

echo "===========query balance inner========"
dfx canister call  mugc-agc-backend query_poll_balance

echo "=========record_work_load========"
dfx canister call mugc-agc-backend push_workload_record "(
    record {
      promt_id = \"086daeb4-3795-486a-8d20-725866f4ded9\";
      client_id = \"1982027079\";
      ai_node = \"http://127.0.0.1:8188/prompt\";
      app_info = \"muse_talk\";
      wk_id = \"univoice-wk-local.json\";
      voice_key = \"2f4018e2-ed5e-4821-97ba-4873b431586f/tmp/tmprh7jbr_7.wav\";
      deduce_asset_key = \"AIGC_output_video_final_00116.mp4\";
      status = \"executed\" ;
      gmt_datatime=1731837234   
    })"

echo "==========query_curr_workload======="
dfx canister call mugc-agc-backend query_curr_workload