use crate::amount::{Amount, BasisPoints};
use crate::error::{OrbitError, OrbitResult};
use crate::ids::{AccountId, AssetId, RouteId, VaultId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteConfig {
    pub id: RouteId,
    pub name: String,
    pub source_asset: AssetId,
    pub target_asset: AssetId,
    pub source_vault: VaultId,
    pub target_vault: VaultId,
    pub operator: AccountId,
    pub fee_bps: BasisPoints,
    pub relayer_bps: BasisPoints,
    pub max_unhedged: Amount,
    pub session_limit: Amount,
    pub min_confidence_bps: BasisPoints,
    pub enabled: bool,
}

impl RouteConfig {
    pub fn total_fee_bps(&self) -> OrbitResult<BasisPoints> {
        self.fee_bps
            .checked_add(self.relayer_bps)
            .ok_or(OrbitError::AmountOverflow)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouteBook {
    routes: BTreeMap<RouteId, RouteConfig>,
}

impl RouteBook {
    pub fn insert(&mut self, route: RouteConfig) {
        self.routes.insert(route.id, route);
    }

    pub fn get(&self, route_id: RouteId) -> OrbitResult<&RouteConfig> {
        let route = self
            .routes
            .get(&route_id)
            .ok_or(OrbitError::RouteNotFound)?;
        if !route.enabled {
            return Err(OrbitError::RouteDisabled);
        }

        Ok(route)
    }
}
