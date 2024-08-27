use serde::{Deserialize, Serialize};
use super::nft_position::NftPosition;
use super::ft_position::FtPosition;
use super::lp_position::LpPosition;

#[derive(Deserialize, Serialize, Debug)]
pub struct PortfolioSummary {
    #[serde(rename = "adaBalance")]
    ada_balance: Option<f64>,
    #[serde(rename = "adaValue")]
    ada_value: Option<f64>,
    #[serde(rename = "liquidValue")]
    liquid_value: Option<f64>,
    #[serde(rename = "numFTs")]
    num_fts: Option<u32>, 
    #[serde(rename = "numNFTs")]
    num_nfts: Option<u32>,
    #[serde(rename = "positionsFt")]
    positions_ft: Option<Vec<FtPosition>>,
    #[serde(rename = "positionsLp")]
    positions_lp: Option<Vec<LpPosition>>,
    #[serde(rename = "positionsNft")]
    positions_nft: Option<Vec<NftPosition>>,
}