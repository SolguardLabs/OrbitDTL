use crate::amount::Amount;
use crate::ids::{AccountId, AssetId, IntentId, RouteId, VaultId};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentStatus {
    Pending,
    Settled,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferIntent {
    pub id: IntentId,
    pub owner: AccountId,
    pub recipient: AccountId,
    pub source_asset: AssetId,
    pub target_asset: AssetId,
    pub source_vault: VaultId,
    pub target_vault: VaultId,
    pub route: RouteId,
    pub amount_in: Amount,
    pub min_amount_out: Amount,
    pub deadline_slot: u64,
    pub nonce: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedIntent {
    pub intent: TransferIntent,
    pub quoted_amount_out: Amount,
    pub quoted_fee: Amount,
    pub created_slot: u64,
    pub status: IntentStatus,
}

impl QueuedIntent {
    pub fn new(
        intent: TransferIntent,
        quoted_amount_out: Amount,
        quoted_fee: Amount,
        created_slot: u64,
    ) -> Self {
        Self {
            intent,
            quoted_amount_out,
            quoted_fee,
            created_slot,
            status: IntentStatus::Pending,
        }
    }
}
