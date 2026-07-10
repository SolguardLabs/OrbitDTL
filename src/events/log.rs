use crate::amount::Amount;
use crate::ids::{AccountId, AssetId, IntentId, RouteId, SessionId, TxId, VaultId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    AssetRegistered {
        asset: AssetId,
        symbol: String,
    },
    VaultCreated {
        vault: VaultId,
        asset: AssetId,
        controller: AccountId,
    },
    RouteAdded {
        route: RouteId,
        source_vault: VaultId,
        target_vault: VaultId,
    },
    IntentQueued {
        intent: IntentId,
        route: RouteId,
        amount_in: Amount,
        quoted_out: Amount,
    },
    SessionOpened {
        session: SessionId,
        slot: u64,
    },
    CounterflowRecorded {
        session: SessionId,
        route: RouteId,
        amount: Amount,
    },
    IntentSettled {
        tx: TxId,
        session: SessionId,
        intent: IntentId,
        recipient: AccountId,
        target_asset: AssetId,
        gross_out: Amount,
        net_out: Amount,
        fee: Amount,
        counterflow_credit: Amount,
    },
    IntentCancelled {
        intent: IntentId,
    },
}
