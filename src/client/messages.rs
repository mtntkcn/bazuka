use crate::blockchain::ZkBlockchainPatch;
use crate::consensus::pow::Difficulty;
use crate::core::{
    Account, Address, Block, ContractId, ContractPayment, Header, Money, MpnPayment,
    TransactionAndDelta,
};
use crate::zk;
use std::collections::HashMap;

use super::{
    explorer::{ExplorerBlock, ExplorerMpnAccount},
    Peer, PeerAddress,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SocialProfiles {
    pub discord: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetStatsRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetStatsResponse {
    pub social_profiles: SocialProfiles,
    pub height: u64,
    pub power: u128,
    pub next_reward: Money,
    pub timestamp: u32,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetAccountRequest {
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetAccountResponse {
    pub account: Account,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMpnAccountRequest {
    pub index: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMpnAccountResponse {
    pub account: zk::MpnAccount,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetExplorerMpnAccountsRequest {
    pub page: usize,
    pub page_size: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetExplorerMpnAccountsResponse {
    pub accounts: HashMap<u32, ExplorerMpnAccount>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostMinerSolutionRequest {
    pub nonce: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostMinerSolutionResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMinerPuzzleRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMinerPuzzleResponse {
    pub puzzle: Option<Puzzle>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Puzzle {
    pub key: String,        // Puzzle key encoded in hex
    pub blob: String,       // Blob encoded in hex
    pub offset: usize,      // From which byte the nonce starts?
    pub size: usize,        // How big is the nonce? (Bytes)
    pub target: Difficulty, // Difficulty target
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HandshakeRequest {
    Node(Peer),
    Client,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HandshakeResponse {
    pub peer: Peer,
    pub timestamp: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetPeersRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetPeersResponse {
    pub peers: Vec<PeerAddress>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostBlockRequest {
    pub block: Block,
    pub patch: ZkBlockchainPatch,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PostBlockResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetBlocksRequest {
    pub since: u64,
    pub count: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetBlocksResponse {
    pub blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetExplorerBlocksRequest {
    pub since: u64,
    pub count: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetExplorerBlocksResponse {
    pub pow_hashes: Vec<String>,
    pub blocks: Vec<ExplorerBlock>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetOutdatedHeightsRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetOutdatedHeightsResponse {
    pub outdated_heights: HashMap<ContractId, u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetStatesRequest {
    pub outdated_heights: HashMap<ContractId, u64>,
    pub to: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetStatesResponse {
    pub patch: ZkBlockchainPatch,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetHeadersRequest {
    pub since: u64,
    pub count: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetHeadersResponse {
    pub headers: Vec<Header>,
    pub pow_keys: Vec<Vec<u8>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetBalanceRequest {
    pub addr: Address,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetBalanceResponse {
    pub amount: Money,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactRequest {
    pub tx_delta: TransactionAndDelta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactZeroRequest {
    pub tx: zk::MpnTransaction,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactZeroResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShutdownRequest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShutdownResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetZeroMempoolRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetZeroMempoolResponse {
    pub updates: Vec<zk::MpnTransaction>,
    pub payments: Vec<MpnPayment>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactMpnPaymentRequest {
    pub tx: MpnPayment,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactMpnPaymentResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMempoolRequest {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMempoolResponse {
    pub tx: Vec<TransactionAndDelta>,
    pub tx_zk: Vec<MpnPayment>,
    pub zk: Vec<zk::MpnTransaction>,
}
