
use candid::{CandidType, Principal,Deserialize};
use serde::Serialize;
use icrc_ledger_types::icrc1::account::{Account,Subaccount,DEFAULT_SUBACCOUNT};
use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};

pub type ProposalState = String;
#[derive(Clone, Debug,CandidType, Deserialize,Default)]
pub struct WorkLoadInitParam {
    pub poll_account:String,
    pub token_block:NumTokens
}


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUIPayload {
    pub tx_id:u128,
    pub client_id:String,
    pub gen_ai_node:String,
    pub app_info:String,
    pub wk_id:String,
    pub promt_id:String,
    pub voice_key:String,
    pub deduce_asset_key:String
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct WorkLoadLedger {
    pub work_load :Vec<ComfyUIPayload>  
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
    pub comfy_node:Vec<ComfyUINode>   
}

//Smart contract related
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct GenaiProposal {
    pub id:u64,
    pub timestamp:u64,
    pub proposer:Principal,
    pub payload:ComfyUIPayload,
    pub state:ProposalState
}


#[derive(CandidType)]
pub enum MixComfyErr{
    NoneNodeVaild,
    RuntimeErr,
}
pub type OrderId = u32;
pub type WorkloadPlacementReceipt = Result<BlockIndex, MixComfyErr>;
#[derive(CandidType)]
pub enum OrderPlacementErr {
    InvalidOrder,
    OrderBookFull,
}

//Can be considered as mining trxjnl
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractCallInstance {
    pub caller:Principal,
    pub prompts_id:String,
    pub client_id:String,
    pub tokens:NumTokens,
    pub gmt_create:u64,
    pub agi_result:AGIAssetresult,
    pub state:ProposalState

}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AGIWkFlowNode {
    pub agi_id:String,
    pub wk_flow:String
}

#[derive(Clone, Debug, Default,CandidType, Deserialize)]
pub struct  AGIAssetresult {
    pub res_code:String,
    pub asset_key:String,
    pub res_message:String
}