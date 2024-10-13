use std::fmt;
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

impl fmt::Display for PortfolioSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PortfolioSummary {{ ada_balance: {:?}, ada_value: {:?}, liquid_value: {:?}, num_fts: {:?}, num_nfts: {:?} }}",
            self.ada_balance, self.ada_value, self.liquid_value, self.num_fts, self.num_nfts)
    }
}
