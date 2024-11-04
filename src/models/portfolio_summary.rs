use std::fmt;
use serde::{Deserialize, Serialize};
use super::nft_position::NftPosition;
use super::ft_position::FtPosition;
use super::lp_position::LpPosition;

#[derive(Deserialize, Serialize, Debug)]
pub struct PortfolioSummary {
    #[serde(rename = "adaBalance")]
    pub ada_balance: f64,
    #[serde(rename = "adaValue")]
    pub ada_value: f64,
    #[serde(rename = "liquidValue")]
    pub liquid_value: f64,
    #[serde(rename = "numFTs")]
    pub num_fts: u32,
    #[serde(rename = "numNFTs")]
    pub num_nfts: u32,
    #[serde(rename = "positionsFt")]
    pub positions_ft: Vec<FtPosition>,
    #[serde(rename = "positionsLp")]
    pub positions_lp: Vec<LpPosition>,
    #[serde(rename = "positionsNft")]
    pub positions_nft: Vec<NftPosition>,
}

impl fmt::Display for PortfolioSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PortfolioSummary {{ ada_balance: {:?}, ada_value: {:?}, liquid_value: {:?}, num_fts: {:?}, num_nfts: {:?} }}",
            self.ada_balance, self.ada_value, self.liquid_value, self.num_fts, self.num_nfts)
    }
}
