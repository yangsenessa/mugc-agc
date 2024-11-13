mod types;
mod mixcomfy_types;
mod mixcomfy_service;

use std::{cell::RefCell, result};
use std::mem;

use mixcomfy_types::{ComfyUINode,MixComfyErr,MixComfy,WorkLoadInitParam,AGIWkFlowNode, WorkLoadLedger};
use candid::{candid_method, export_service, Nat, Principal,CandidType, Deserialize,Encode};
use ic_cdk::{
    api::{self, call},
    storage,
    caller,
};

use ic_cdk_macros::*;
use serde::Serialize;
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

#[ic_cdk::init]
fn init(args:WorkLoadInitParam) {
    STATE.with(|state|{
        let mut state = state.borrow_mut();
        state.mining_contract = args.clone();
        state.owner = Some(ic_cdk::api::caller());
        ic_cdk::println!("Owner:{:?}", state.owner);
            
    });

}
