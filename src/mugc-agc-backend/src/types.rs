use candid::{CandidType, Deserialize, Principal};
use std::ops::{Add, AddAssign, Mul, SubAssign};

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Tokens {
    pub amount_e8s: u64,
}
impl Add for Tokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tokens {
            amount_e8s: self.amount_e8s + other.amount_e8s,
        }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Self) {
        self.amount_e8s += other.amount_e8s;
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Self) {
        self.amount_e8s -= other.amount_e8s;
    }
}

impl Mul<u64> for Tokens {
    type Output = Tokens;
    fn mul(self, rhs: u64) -> Self {
        Tokens {
            amount_e8s: self.amount_e8s * rhs,
        }
    }
}
// Struct for candid
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ComfyUIPayload {
    pub gen_ai_node:ComfyUINode,
    pub wk_info_id:String,
    pub client_id:String,
    pub promts:PromtsVecParams,
    pub voice_base64:String,
    pub category:String
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub tokens: Tokens,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: Principal,
    pub amount: Tokens,
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

//NFT miner
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct NFTDetail {
    pub tokenid:u64,
    pub miner:Principal,
    pub contract:ContractInfo
}
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct  ContractInfo {
    pub constractid:String,
    pub poll_account:Principal,
    pub token_global:u64
}


#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct ContractCallInstance {
    pub caller:Principal,
    pub contractid:String,
    pub tokens:Tokens,
    pub state:ProposalState,
    pub enable_timestamp:u64,
    pub disable_timestamp:u64,
    pub work_load_cnt:u64
}







