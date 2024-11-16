mod types;
mod mixcomfy_types;
mod mixcomfy_service;

use std::{cell::RefCell, result};
use std::mem;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use serde::Serialize;


use mixcomfy_types::{ComfyUINode,MixComfyErr,MixComfy,
    WorkLoadInitParam,AGIWkFlowNode, WorkLoadLedger,ComfyUIPayload};
use candid::{candid_method, export_service, Nat, Principal,CandidType, Deserialize,Encode};
use ic_cdk::{
    api::{self, call},
    storage,
    caller,
};

use ic_cdk_macros::*;

#[derive(CandidType, Deserialize, Serialize)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType,Deserialize,Clone,Default)]
pub struct  State {
    owner:Option<Principal>,
    mining_contract:WorkLoadInitParam,
    mixcomfy:MixComfy,
    agic_wk_node:Vec<AGIWkFlowNode>,
    work_load_ledger:WorkLoadLedger
}
#[derive(CandidType, Default,Deserialize,Clone)]
struct StableState {
    state: State,
}


#[pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|state: &RefCell<State>| mem::take(&mut *state.borrow_mut()));
    let stable_state: StableState = StableState { state };
    ic_cdk::println!("pre_upgrade");
    storage::stable_save((stable_state,)).unwrap();

}

#[post_upgrade]
fn post_upgrade() {
    let (StableState { state },) = storage::stable_restore().unwrap();
    STATE.with(|state0| *state0.borrow_mut() = state);
    ic_cdk::println!("post_upgrade");

}



#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn query_comfy_nodes() -> Option<Vec<ComfyUINode>>{
     STATE.with(|s|{
        s.borrow_mut().mixcomfy.get_comfy_nodes()
    })
}

#[ic_cdk::update]
fn reg_comfy_nodes(nodes:Vec<ComfyUINode>) -> Option<Vec<ComfyUINode>> {
    let result = STATE.with(|s|{
               s.borrow_mut().mixcomfy.reg_comfy_nodes(nodes)
    });
    match result {
        Err(e) =>{eprintln!("Runtime Error ");None},
        Ok(()) =>STATE.with(|s|{
            s.borrow_mut().mixcomfy.get_comfy_nodes()
        })
    }
    
}

#[ic_cdk::query]
fn gen_ai_node_router() ->Option<ComfyUINode> {
    STATE.with(|s| {
        s.borrow_mut().mixcomfy.decide_comfy_node()
    })
}

#[ic_cdk::query]
fn export_minting_contract()->Option<WorkLoadInitParam> {
    STATE.with(|s|{
       Some(s.borrow().mining_contract.clone())
    })
}

#[ic_cdk::update]
fn update_minting_contract(args:WorkLoadInitParam)->Option<WorkLoadInitParam> {
    STATE.with(|state|{
        let mut state = state.borrow_mut();
        state.mining_contract = args.clone();
        state.owner = Some(ic_cdk::api::caller());
        ic_cdk::println!("Owner:{:?}", state.owner);
        Some(state.mining_contract.clone()) 
    })

}

#[ic_cdk::update]
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_from_args = TransferFromArgs {
        // the account we want to transfer tokens from (in this case we assume the caller approved the canister to spend funds on their behalf)
        from: Account::from(ic_cdk::caller()),
        // can be used to distinguish between transactions
        memo: None,
        // the amount we want to transfer
        amount: args.amount,
        // the subaccount we want to spend the tokens from (in this case we assume the default subaccount has been approved)
        spender_subaccount: None,
        // if not specified, the default fee for the canister is used
        fee: None,
        // the account we want to transfer tokens to
        to: args.to_account,
        // a timestamp indicating when the transaction was created by the caller; if it is not specified by the caller then this is set to the current ICP time
        created_at_time: None,
    };

    // 1. Asynchronously call another canister function using `ic_cdk::call`.
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        // 2. Convert a textual representation of a Principal into an actual `Principal` object. The principal is the one we specified in `dfx.json`.
        //    `expect` will panic if the conversion fails, ensuring the code does not proceed with an invalid principal.
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        // 3. Specify the method name on the target canister to be called, in this case, "icrc1_transfer".
        "icrc2_transfer_from",
        // 4. Provide the arguments for the call in a tuple, here `transfer_args` is encapsulated as a single-element tuple.
        (transfer_from_args,),
    )
    .await // 5. Await the completion of the asynchronous call, pausing the execution until the future is resolved.
    // 6. Apply `map_err` to transform any network or system errors encountered during the call into a more readable string format.
    //    The `?` operator is then used to propagate errors: if the result is an `Err`, it returns from the function with that error,
    //    otherwise, it unwraps the `Ok` value, allowing the chain to continue.
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    // 7. Access the first element of the tuple, which is the `Result<BlockIndex, TransferError>`, for further processing.
    .0
    // 8. Use `map_err` again to transform any specific ledger transfer errors into a readable string format, facilitating error handling and debugging.
    .map_err(|e: TransferFromError| format!("ledger transfer error {:?}", e))
}

#[ic_cdk::query]
async fn query_poll_balance()->Result<NumTokens,String> {
    ic_cdk::println!(
        "Query balance of mining pool {}",
        ic_cdk::id(),
    );

    let balance = ic_cdk::call::<(Account,),(Nat,)> (
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
                       .expect("Could not decode the principal."),
        "icrc1_balance_of",
        (Account::from(ic_cdk::id()),)

    ).await
    .map_err(|e| format!("fail to call ledger:{:?}",e))?
    .0.clone();

    Ok(balance)
     
}

#[ic_cdk::update]
fn push_workload_recore(record:ComfyUIPayload) ->Result<BlockIndex,MixComfyErr>{
    ic_cdk::println!("Push work load:{:?}", record);

    STATE.with(|state|{
        let mut state = state.borrow_mut();
        state.mixcomfy.record_work_load(record)
    })
}



// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();