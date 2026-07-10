use crate::amount::Amount;
use crate::error::OrbitResult;
use crate::ids::{AccountId, AssetId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub label: String,
    balances: BTreeMap<AssetId, Amount>,
}

impl Account {
    pub fn new(id: AccountId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            balances: BTreeMap::new(),
        }
    }

    pub fn balance(&self, asset: AssetId) -> Amount {
        self.balances.get(&asset).copied().unwrap_or_default()
    }

    pub fn credit(&mut self, asset: AssetId, amount: Amount) -> OrbitResult<()> {
        let current = self.balance(asset);
        self.balances.insert(asset, current.checked_add(amount)?);
        Ok(())
    }

    pub fn balances(&self) -> &BTreeMap<AssetId, Amount> {
        &self.balances
    }
}
