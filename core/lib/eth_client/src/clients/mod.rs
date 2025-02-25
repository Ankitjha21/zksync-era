//! Various Ethereum client implementations.

mod generic;
mod http;
mod mock;

use serde::{Deserialize, Serialize};
use zksync_types::U256;

pub use self::{
    http::{PKSigningClient, QueryClient, SigningClient},
    mock::MockEthereum,
};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineaEstimateGas {
    pub base_fee_per_gas: U256,
    pub gas_limit: U256,
    pub priority_fee_per_gas: U256,
}
