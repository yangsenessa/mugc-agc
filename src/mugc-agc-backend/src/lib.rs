mod mixcomfy_types;
mod mixcomfy_service;

use std::{cell::RefCell, result};
use std::mem;
use std::collections::BTreeMap;


use icrc_ledger_types::icrc1::transfer::{BlockIndex};


use mixcomfy_types::{ComfyUINode,MixComfyErr,MixComfy,
    WorkLoadInitParam,AGIWkFlowNode, WorkLoadLedger,ComfyUIPayload,
    WorkLoadLedgerItem};
use candid::{candid_method, export_service, Nat, Principal,CandidType, Deserialize,Encode};
use ic_cdk::{
    api::{self, call},
    storage,
    caller,
};

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}
type SubscriberStore = BTreeMap<Principal, Subscriber>;

#[derive(Clone, CandidType, Deserialize)]
struct Event0301008 {
    topic:String,
    payload:WorkLoadLedgerItem
}


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
}

#[derive(CandidType,Deserialize,Clone,Default)]
pub struct  State {
    mining_contract:WorkLoadInitParam,
    mixcomfy:MixComfy,
    agic_wk_node:Vec<AGIWkFlowNode>,
    work_load_ledger:WorkLoadLedger
}
#[derive(CandidType, Default,Deserialize,Clone)]
struct StableState {
    state: State,
}


#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|state: &RefCell<State>| mem::take(&mut *state.borrow_mut()));
    let stable_state: StableState = StableState { state };
    ic_cdk::println!("pre_upgrade");
    storage::stable_save((stable_state,)).unwrap();

}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade");
    let (StableState { state },) = storage::stable_restore()
                                              .expect("failed to restore stable state");
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
        Some(state.mining_contract.clone()) 
    })

}

#[ic_cdk::update]
fn subscribe(subscriber: Subscriber) {
    let subscriber_principal_id = ic_cdk::caller();
    SUBSCRIBERS.with(|subscribers| {
        subscribers
            .borrow_mut()
            .insert(subscriber_principal_id, subscriber)
    });
}


#[ic_cdk::update]
fn push_workload_record(record:ComfyUIPayload) ->Result<WorkLoadLedgerItem,MixComfyErr>{
    ic_cdk::println!("Push work load:{:?}", record);

    let result = STATE.with(|state|{
        let mut state = state.borrow_mut();
        let tokens = state.mining_contract.token_block.clone();
        ic_cdk::println!("{} tokens per block", tokens);
        state.mixcomfy.record_work_load(record,tokens)
       
    });
    match result {
        Ok(block) => {
            SUBSCRIBERS.with(|s|{
                let event = Event0301008 {
                    topic :  String::from("0301008"),
                    payload : block.clone()
                };
                for (k,v) in s.borrow().iter() {
                    if v.topic == event.topic {
                        let _call_result: Result<(), _> =
                        ic_cdk::notify(*k, "publish_0301008", (&event,));
                    }
                }
                Ok(block)
            }) 

       },
    Err(e) =>  Err(e),
        
    }


}


#[ic_cdk::query]
fn query_curr_workload() ->Option<Vec<WorkLoadLedgerItem>>{
    ic_cdk::println!("Query all workload");

    STATE.with(|state|{
        let  state = state.borrow();
        state.mixcomfy.query_all_workload()
    })
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();