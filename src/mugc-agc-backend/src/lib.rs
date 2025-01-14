mod mixcomfy_types;
mod mixcomfy_service;

use std::{cell::RefCell, result};
use std::mem;
use std::collections::BTreeMap;
use std::time::Duration;

use icrc_ledger_types::icrc1::transfer::{BlockIndex};
use mixcomfy_types::{ComfyUINode, MixComfyErr, MixComfy,
                     WorkLoadInitParam, AGIWkFlowNode, WorkLoadLedger, ComfyUIPayload,
                     WorkLoadLedgerItem};
use candid::{candid_method, export_service, Nat, Principal, CandidType, Deserialize, Encode};
use ic_cdk::{
    api::{self, call},
    storage,
    caller,
};

use std::fs::File;
use rustpotter::{DetectionConfig, Rustpotter};

const TIMER_INTERVAL_SEC: u64 = 60 * 2;


#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

type SubscriberStore = BTreeMap<Principal, Subscriber>;

#[derive(Clone, CandidType, Deserialize)]
struct Event0301008 {
    topic: String,
    payload: WorkLoadLedgerItem,
}


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
    static RUSTPOTTER: RefCell<Option<Rustpotter>> = RefCell::new(None);
}

#[derive(CandidType, Deserialize, Clone, Default)]
pub struct State {
    mining_contract: WorkLoadInitParam,
    mixcomfy: MixComfy,
    agic_wk_node: Vec<AGIWkFlowNode>,
    work_load_ledger: WorkLoadLedger,
}

#[derive(CandidType, Default, Deserialize, Clone)]
struct StableState {
    state: State,
}

struct DetectionResult {
    detected: bool,
    timestamp: Option<u64>,
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|state: &RefCell<State>| mem::take(&mut *state.borrow_mut()));
    let stable_state: StableState = StableState { state };
    ic_cdk::println!("pre_upgrade");
    storage::stable_save((stable_state, )).unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade");
    let (StableState { state }, ) = storage::stable_restore()
        .expect("failed to restore stable state");
    STATE.with(|state0| *state0.borrow_mut() = state);
    ic_cdk::println!("post_upgrade");
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn query_comfy_nodes() -> Option<Vec<ComfyUINode>> {
    STATE.with(|s| {
        s.borrow_mut().mixcomfy.get_comfy_nodes()
    })
}

#[ic_cdk::update]
fn reg_comfy_nodes(nodes: Vec<ComfyUINode>) -> Option<Vec<ComfyUINode>> {
    let result = STATE.with(|s| {
        s.borrow_mut().mixcomfy.reg_comfy_nodes(nodes)
    });
    match result {
        Err(e) => {
            eprintln!("Runtime Error ");
            None
        }
        Ok(()) => STATE.with(|s| {
            s.borrow_mut().mixcomfy.get_comfy_nodes()
        })
    }
}

#[ic_cdk::query]
fn gen_ai_node_router() -> Option<ComfyUINode> {
    STATE.with(|s| {
        s.borrow_mut().mixcomfy.decide_comfy_node()
    })
}

#[ic_cdk::query]
fn export_minting_contract() -> Option<WorkLoadInitParam> {
    STATE.with(|s| {
        Some(s.borrow().mining_contract.clone())
    })
}


#[ic_cdk::update]
fn update_minting_contract(args: WorkLoadInitParam) -> Option<WorkLoadInitParam> {
    STATE.with(|state| {
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
fn push_workload_record(record: ComfyUIPayload) -> Result<WorkLoadLedgerItem, MixComfyErr> {
    ic_cdk::println!("Push work load:{:?}", record);

    let result = STATE.with(|state| {
        let mut state = state.borrow_mut();
        let tokens = state.mining_contract.token_block.clone();
        let token_pool = state.mining_contract.poll_account.clone();
        let nft_pool = state.mining_contract.nft_collection_id.clone();
        ic_cdk::println!("{} tokens per block", tokens);
        state.mixcomfy.record_work_load(record, tokens, token_pool, nft_pool)
    });
    match result {
        Ok(block) => {
            SUBSCRIBERS.with(|s| {
                let event = Event0301008 {
                    topic: String::from("0301008"),
                    payload: block.clone(),
                };
                for (k, v) in s.borrow().iter() {
                    if v.topic == event.topic {
                        let _call_result: Result<(), _> =
                            ic_cdk::notify(*k, "publish_0301008", (&event, ));
                    }
                }
                Ok(block)
            })
        }
        Err(e) => Err(e),
    }
}


#[ic_cdk::query]
fn query_curr_workload() -> Option<Vec<WorkLoadLedgerItem>> {
    ic_cdk::println!("Query all workload");

    STATE.with(|state| {
        let state = state.borrow();
        state.mixcomfy.query_all_workload()
    })
}

fn setup_timer() {
    ic_cdk_timers::set_timer_interval(Duration::from_secs(TIMER_INTERVAL_SEC), || {
        ic_cdk::print("Creating block");
        let work_load: ComfyUIPayload = ComfyUIPayload {
            promt_id: String::from("086daeb4-3795-486a-8d20-725866f4ded9"),
            client_id: String::from("1982027079"),
            ai_node: String::from("http://127.0.0.1:8188/prompt"),
            app_info: String::from("miner_test"),
            wk_id: String::from("test.json"),
            voice_key: String::from("2f4018e2-ed5e-4821-97ba-4873b431586f/tmp/tmprh7jbr_7.wav"),
            deduce_asset_key: String::from("testkey"),
            status: String::from("executed"),
            gmt_datatime: ic_cdk::api::time(),
        };
        let res = push_workload_record(work_load);
        match res {
            Result::Ok(ledger) => {
                ic_cdk::println!("Create block ok")
            }
            Result::Err(e) => {
                ic_cdk::println!("Create block error")
            }
        }
    });
}

#[update]
#[candid_method(update)]
async fn initialize_rustpotter() -> String {
    RUSTPOTTER.with(|mut rustpotter| {
        let mut instance = rustpotter.borrow_mut();
        if instance.is_none() {
            let mut rustpotter_instance = Rustpotter::new(DetectionConfig::default());
            let model_path = "";
            let mut model_file = File::open(model_path).expect("Could not open model file");
            rustpotter_instance.add_hotword(&mut model_file).expect("Failed to load hotword model");
            *instance = Some(rustpotter_instance);
        }
    });

    "Rustpotter initialized".to_string()
}

async fn detect_hotword(audio: Vec<u8>) -> DetectionResult {
    RUSTPOTTER.with (|rustpotter| {
        let instance = rustpotter.borrow();
        if let Some(rustpotter_instance) = instance.as_ref() {
            let detections = rustpotter_instance.process_audio_bytes(&audio).unwrap_or_default();

            if !detections.is_empty() {
                let detection = &detections[0];
                return DetectionResult {
                    detected: true,
                    timestamp: Some(detection.timestamp),
                };
            }
        }

        DetectionResult {
            detected: false,
            timestamp: None,
        }
    });
}

#[ic_cdk::init]
fn init() {
    setup_timer();
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();