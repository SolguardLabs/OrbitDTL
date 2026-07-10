use crate::amount::BasisPoints;
use crate::error::{OrbitError, OrbitResult};
use crate::ids::AssetId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub symbol: String,
    pub decimals: u8,
    pub risk_weight_bps: BasisPoints,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AssetRegistry {
    assets: BTreeMap<AssetId, Asset>,
}

impl AssetRegistry {
    pub fn insert(&mut self, asset: Asset) -> OrbitResult<()> {
        if asset.decimals > 18 {
            return Err(OrbitError::AmountOverflow);
        }

        self.assets.insert(asset.id, asset);
        Ok(())
    }

    pub fn get(&self, id: AssetId) -> OrbitResult<&Asset> {
        let asset = self.assets.get(&id).ok_or(OrbitError::AssetNotFound)?;
        if !asset.enabled {
            return Err(OrbitError::AssetDisabled);
        }

        Ok(asset)
    }
}
