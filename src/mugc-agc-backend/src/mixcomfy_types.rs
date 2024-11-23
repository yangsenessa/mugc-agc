
use std::default;

use candid::{CandidType, Principal,Deserialize,Nat};
use serde::Serialize;
use icrc_ledger_types::icrc1::account::{Account,Subaccount,DEFAULT_SUBACCOUNT};
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens};



#[derive(CandidType,Deserialize,Clone)]
pub enum MinerTxState {
    Prepared(String),
    Claimed(String)
}

impl Default  for MinerTxState {
    fn default() -> Self {
       MinerTxState::Prepared(String::from("prepared"))
   }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct WorkLoadInitParam {
    pub poll_account:String,
    pub nft_collection_id:String,
    pub token_block:NumTokens
}

pub type Timestamp = u64;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUIPayload {
    pub promt_id:String,
    pub client_id:String,
    pub ai_node:String,
    pub app_info:String,
    pub wk_id:String,
    pub voice_key:String,
    pub deduce_asset_key:String,
    pub status:String,
    //Not the time of AI node, but the time on chain
    pub gmt_datatime:Timestamp
}
#[derive(Clone, Default, CandidType, Deserialize)]
pub struct WorkLoadLedgerItem {
    pub wkload_id:BlockIndex,
    pub work_load :ComfyUIPayload,
    pub block_tokens:NumTokens,
    pub mining_status:MinerTxState
}


#[derive(Clone, Default, CandidType, Deserialize)]
pub struct WorkLoadLedger {
    pub work_load_record:Vec<WorkLoadLedgerItem>
}

#[derive(Clone, Debug, Default, CandidType, Deserialize,Serialize)]
pub struct PromtsVecParams {
    pub client_id:String,
    pub param_name:String,
    pub param_val:String,
    pub related_wk_node:String //Related to workflow node's params for inputing
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUINode {
    pub node_id:u32,
    pub ori_url:String,
    pub ws_url:String,
    pub weight:i32
}

#[derive(Default,CandidType,Deserialize,Clone)]
pub struct MixComfy{
    pub comfy_node:Vec<ComfyUINode>,
    pub workload_records:WorkLoadLedger,
    pub miner_ledger:MinerRecordLedger
}

//Smart contract related
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GenaiProposal {
    pub id:u64,
    pub timestamp:u64,
    pub proposer:Principal,
    pub payload:ComfyUIPayload,
    pub state:String
}


#[derive(CandidType)]
pub enum MixComfyErr{
    NoneNodeVaild (String),
    RuntimeErr (String),
}
pub type OrderId = u32;
pub type WorkloadPlacementReceipt = Result<BlockIndex, MixComfyErr>;
#[derive(CandidType)]
pub enum OrderPlacementErr {
    InvalidOrder,
    OrderBookFull,
}



#[derive(CandidType, Deserialize,Clone)]
pub struct MinerRecordItem {
    pub minner_claim_id:BlockIndex,
    pub minner:Principal,
    pub wkload_id:BlockIndex,
    pub client_id:String,
    pub tokens:NumTokens,
    pub gmt_create:u64,
    pub agi_result:AGIAssetresult,
    pub state:MinerTxState
}

#[derive(Default,CandidType, Deserialize,Clone)]
pub struct MinerRecordLedger {
    record:Vec<MinerRecordItem>
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AGIWkFlowNode {
    pub agi_id:String,
    pub wk_flow:String
}

#[derive(Clone, Debug, Default,CandidType, Deserialize)]
pub struct  AGIAssetresult {
    pub res_code:String,
    pub asset_key:String,  // prompt and file input
    pub prc_dataset_key:String, //Training data
    pub aigc_asset_key:String, //AI Gen result S3 key
    pub res_message:String
}