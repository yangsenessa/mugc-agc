
use candid::{CandidType, Principal,Deserialize};
use serde::Serialize;
use crate::types::{Tokens,Account};


#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUIPayload {
    pub gen_ai_node:ComfyUINode,
    pub wk_info_id:String,
    pub client_id:String,
    pub promts:Vec<PromtsVecParams>,
    pub voice_base64:String,
    pub category:String
}
#[derive(Clone, Debug, Default, CandidType, Deserialize,Serialize)]
pub struct PromtsVecParams {
    pub param_name:String,
    pub param_val:String,
    pub related_wk_node:String //Related to workflow node's params for inputing
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUINode {
    pub host:String,
    pub url_suffix:String,
    pub port:String
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

// The state of a Proposal
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum ProposalState {
    // The proposal is open for voting
    Open,

    // Enough "yes" votes have been cast to accept the proposal, and it will soon be executed
    Accepted,

    // Enough "no" votes have been cast to reject the proposal, and it will not be executed
    Rejected,

    // The proposal is currently being executed
    Executing,

    // The proposal has been successfully executed
    Succeeded,

    // A failure occurred while executing the proposal
    Failed(String),
}
#[derive(CandidType)]
pub enum MixComfyErr{
    NoneNodeVaild,
    RuntimeErr,
}

//Can be considered as mining trxjnl
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ContractCallInstance {
    pub caller:Principal,
    pub contractid:String,
    pub tokens:Tokens,
    pub state:ProposalState,
    pub enable_timestamp:u64,
    pub disable_timestamp:u64,
    pub work_load_cnt:u64
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct AGIWkFlowNode {
    pub agi_id:String,
    pub wk_flow:String
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct  AGIAssetresult {
    pub res_code:String,
    pub res_message:String
}