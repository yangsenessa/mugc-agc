use candid::{CandidType, Principal,Deserialize};
use std::ops::{Add, AddAssign, Mul, SubAssign};
use std::collections::{HashMap, HashSet};


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



//NFT miner
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct NFTDetail {
    pub tokenid:u64,
    pub owner:Principal,
    pub contract:ContractInfo
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct  ContractInfo {
    pub constractid:String,
    pub relate_nft_token_id:u64,
    pub poll_account:Principal,
    pub token_sum:u64
}
#[derive(CandidType, Deserialize, PartialEq)]
enum MetadataPurpose {
    Preview,
    Rendered,
}
#[allow(clippy::enum_variant_names)]
#[derive(CandidType, Deserialize)]
enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}
#[derive( CandidType, Deserialize)]
pub struct MetadataPart {
    purpose: MetadataPurpose,
    key_val_data: HashMap<String, MetadataVal>,
    data: Vec<u8>,
}













