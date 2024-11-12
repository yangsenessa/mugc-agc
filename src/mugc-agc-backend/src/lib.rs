mod types;
mod mixcomfy_types;
mod mixcomfy_service;

use std::cell::RefCell;
use std::mem;


use mixcomfy_types::{ComfyUINode,MixComfyErr,AGIWkFlowNode, AGIAssetresult};
use mixcomfy_service::MixComfy;
use candid::{candid_method, export_service, Nat, Principal,CandidType, Encode};
use ic_cdk::{
    api::{self, call},
    export::candid,
    storage,
    caller,
};

use ic_cdk_macros::*;
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct  State {
    owner:Option<Principal>,
    mixcomfy:MixComfy,
    agic_wk_node:vec<AGIWkFlowNode>
}

#[pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|state: &RefCell<State>| mem::take(&mut *state.borrow_mut()));

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
