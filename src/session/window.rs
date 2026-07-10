use crate::amount::Amount;
use crate::error::OrbitResult;
use crate::ids::{IntentId, RouteId, SessionId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementSession {
    pub id: SessionId,
    pub opened_slot: u64,
    pub closed: bool,
    pub counterflow: BTreeMap<RouteId, Amount>,
    pub accounted_counterflow: BTreeMap<RouteId, Amount>,
    pub included_intents: Vec<IntentId>,
}

impl SettlementSession {
    pub fn new(id: SessionId, opened_slot: u64) -> Self {
        Self {
            id,
            opened_slot,
            closed: false,
            counterflow: BTreeMap::new(),
            accounted_counterflow: BTreeMap::new(),
            included_intents: Vec::new(),
        }
    }

    pub fn add_counterflow(&mut self, route: RouteId, amount: Amount) -> OrbitResult<()> {
        let current = self.counterflow.get(&route).copied().unwrap_or_default();
        self.counterflow.insert(route, current.checked_add(amount)?);
        Ok(())
    }

    pub fn counterflow_allowance(
        &mut self,
        route: RouteId,
        requested: Amount,
    ) -> OrbitResult<Amount> {
        let available = self.counterflow.get(&route).copied().unwrap_or_default();
        let allowance = available.min(requested);
        let accounted = self
            .accounted_counterflow
            .get(&route)
            .copied()
            .unwrap_or_default()
            .checked_add(allowance)?;
        self.accounted_counterflow.insert(route, accounted);
        Ok(allowance)
    }

    pub fn include(&mut self, intent: IntentId) {
        self.included_intents.push(intent);
    }

    pub fn close(&mut self) {
        self.closed = true;
    }
}
