use alloy_primitives::U256;
use serde::{Deserialize, Serialize};

/// Internal struct to calculate reward percentiles
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxGasAndReward {
    /// Gas used by the transaction
    pub gas_used: u64,
    /// The effective gas tip by the transaction
    pub reward: u128,
}

impl PartialOrd for TxGasAndReward {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TxGasAndReward {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare only the reward
        // see:
        // <https://github.com/ethereum/go-ethereum/blob/ee8e83fa5f6cb261dad2ed0a7bbcde4930c41e6c/eth/gasprice/feehistory.go#L85>
        self.reward.cmp(&other.reward)
    }
}

/// Response type for `eth_feeHistory`
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeHistory {
    /// An array of block base fees per gas.
    /// This includes the next block after the newest of the returned range,
    /// because this value can be derived from the newest block. Zeroes are
    /// returned for pre-EIP-1559 blocks.
    ///
    /// # Note
    ///
    /// Empty list is skipped only for compatibility with Erigon and Geth.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub base_fee_per_gas: Vec<U256>,
    /// An array of block gas used ratios. These are calculated as the ratio
    /// of `gasUsed` and `gasLimit`.
    ///
    /// # Note
    ///
    /// The `Option` is only for compatability with Erigon and Geth.
    pub gas_used_ratio: Vec<f64>,
    /// Lowest number block of the returned range.
    pub oldest_block: U256,
    /// An (optional) array of effective priority fee per gas data points from a single
    /// block. All zeroes are returned if the block is empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<Vec<Vec<U256>>>,
}
