mod types;
mod mixcomfy_types;
mod mixcomfy_service;

use std::cell::RefCell;
use std::convert::TryInto;

use mixcomfy_types::{ComfyUINode,MixComfyErr};
use mixcomfy_service::MixComfy;
use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct  State {
    owner:Option<Principal>,
    mixcomfy:MixComfy
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
