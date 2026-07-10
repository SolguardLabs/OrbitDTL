use crate::accounts::Account;
use crate::amount::Amount;
use crate::asset::{Asset, AssetRegistry};
use crate::error::{OrbitError, OrbitResult};
use crate::events::Event;
use crate::ids::{AccountId, AssetId, IntentId, RouteId, SessionId, TxId, VaultId};
use crate::oracle::{Oracle, Price};
use crate::orders::{IntentStatus, QueuedIntent, TransferIntent};
use crate::risk::RiskEngine;
use crate::routes::{RouteBook, RouteConfig};
use crate::session::SettlementSession;
use crate::vault::{Vault, VaultKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DemoReport {
    pub accounts: BTreeMap<AccountId, Account>,
    pub vaults: BTreeMap<VaultId, Vault>,
    pub sessions: BTreeMap<SessionId, SettlementSession>,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ledger {
    pub assets: AssetRegistry,
    pub accounts: BTreeMap<AccountId, Account>,
    pub vaults: BTreeMap<VaultId, Vault>,
    pub oracle: Oracle,
    pub routes: RouteBook,
    pub intents: BTreeMap<IntentId, QueuedIntent>,
    pub sessions: BTreeMap<SessionId, SettlementSession>,
    pub risk: RiskEngine,
    pub events: Vec<Event>,
    slot: u64,
    next_account: u64,
    next_asset: u64,
    next_vault: u64,
    next_route: u64,
    next_intent: u64,
    next_session: u64,
    next_tx: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            assets: AssetRegistry::default(),
            accounts: BTreeMap::new(),
            vaults: BTreeMap::new(),
            oracle: Oracle::default(),
            routes: RouteBook::default(),
            intents: BTreeMap::new(),
            sessions: BTreeMap::new(),
            risk: RiskEngine::new(
                Amount::new(25_000_000_000_000),
                Amount::new(80_000_000_000_000),
            ),
            events: Vec::new(),
            slot: 1,
            next_account: 1,
            next_asset: 1,
            next_vault: 1,
            next_route: 1,
            next_intent: 1,
            next_session: 1,
            next_tx: 1,
        }
    }

    pub fn create_account(&mut self, label: impl Into<String>) -> AccountId {
        let id = AccountId::new(self.next_account);
        self.next_account += 1;
        self.accounts.insert(id, Account::new(id, label));
        id
    }

    pub fn register_asset(
        &mut self,
        symbol: impl Into<String>,
        decimals: u8,
        risk_weight_bps: u16,
    ) -> OrbitResult<AssetId> {
        let id = AssetId::new(self.next_asset);
        self.next_asset += 1;
        let symbol = symbol.into();
        self.assets.insert(Asset {
            id,
            symbol: symbol.clone(),
            decimals,
            risk_weight_bps,
            enabled: true,
        })?;
        self.events
            .push(Event::AssetRegistered { asset: id, symbol });
        Ok(id)
    }

    pub fn set_price(
        &mut self,
        asset: AssetId,
        price_e8: u128,
        confidence_bps: u16,
    ) -> OrbitResult<()> {
        self.assets.get(asset)?;
        self.oracle.set_price(Price {
            asset,
            price_e8,
            confidence_bps,
            slot: self.slot,
        });
        Ok(())
    }

    pub fn create_vault(
        &mut self,
        asset: AssetId,
        controller: AccountId,
        kind: VaultKind,
    ) -> OrbitResult<VaultId> {
        self.assets.get(asset)?;
        self.account(controller)?;
        let id = VaultId::new(self.next_vault);
        self.next_vault += 1;
        self.vaults
            .insert(id, Vault::new(id, asset, controller, kind));
        self.events.push(Event::VaultCreated {
            vault: id,
            asset,
            controller,
        });
        Ok(id)
    }

    pub fn deposit_vault(&mut self, vault: VaultId, amount: Amount) -> OrbitResult<()> {
        self.vault_mut(vault)?.deposit(amount)
    }

    pub fn credit_account(
        &mut self,
        account: AccountId,
        asset: AssetId,
        amount: Amount,
    ) -> OrbitResult<()> {
        self.assets.get(asset)?;
        self.account_mut(account)?.credit(asset, amount)
    }

    pub fn add_route(&mut self, mut route: RouteConfig) -> OrbitResult<RouteId> {
        self.assets.get(route.source_asset)?;
        self.assets.get(route.target_asset)?;
        let source_vault = self.vault(route.source_vault)?;
        let target_vault = self.vault(route.target_vault)?;
        if source_vault.asset != route.source_asset || target_vault.asset != route.target_asset {
            return Err(OrbitError::InvalidVaultAsset);
        }
        self.account(route.operator)?;

        let id = RouteId::new(self.next_route);
        self.next_route += 1;
        route.id = id;
        self.events.push(Event::RouteAdded {
            route: id,
            source_vault: route.source_vault,
            target_vault: route.target_vault,
        });
        self.routes.insert(route);
        Ok(id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn submit_intent(
        &mut self,
        owner: AccountId,
        recipient: AccountId,
        route_id: RouteId,
        amount_in: Amount,
        min_amount_out: Amount,
        deadline_slot: u64,
        nonce: u64,
    ) -> OrbitResult<IntentId> {
        self.account(owner)?;
        self.account(recipient)?;
        let route = self.routes.get(route_id)?.clone();
        let source_price = self.oracle.price(route.source_asset)?;
        let target_price = self.oracle.price(route.target_asset)?;
        if source_price.confidence_bps < route.min_confidence_bps
            || target_price.confidence_bps < route.min_confidence_bps
        {
            return Err(OrbitError::PriceConfidence);
        }

        let gross_out = self.oracle.quote(
            &self.assets,
            route.source_asset,
            route.target_asset,
            amount_in,
        )?;
        let fee = gross_out.mul_bps_floor(route.total_fee_bps()?)?;
        let net_out = gross_out.checked_sub(fee)?;
        if net_out < min_amount_out {
            return Err(OrbitError::MinimumOutput);
        }

        self.vault_mut(route.source_vault)?.reserve(amount_in)?;
        let id = IntentId::new(self.next_intent);
        self.next_intent += 1;
        let intent = TransferIntent {
            id,
            owner,
            recipient,
            source_asset: route.source_asset,
            target_asset: route.target_asset,
            source_vault: route.source_vault,
            target_vault: route.target_vault,
            route: route_id,
            amount_in,
            min_amount_out,
            deadline_slot,
            nonce,
        };
        self.intents
            .insert(id, QueuedIntent::new(intent, gross_out, fee, self.slot));
        self.events.push(Event::IntentQueued {
            intent: id,
            route: route_id,
            amount_in,
            quoted_out: gross_out,
        });
        Ok(id)
    }

    pub fn open_session(&mut self) -> SessionId {
        let id = SessionId::new(self.next_session);
        self.next_session += 1;
        self.sessions
            .insert(id, SettlementSession::new(id, self.slot));
        self.events.push(Event::SessionOpened {
            session: id,
            slot: self.slot,
        });
        id
    }

    pub fn record_counterflow(
        &mut self,
        session_id: SessionId,
        route_id: RouteId,
        amount: Amount,
    ) -> OrbitResult<()> {
        self.routes.get(route_id)?;
        self.session_mut(session_id)?
            .add_counterflow(route_id, amount)?;
        self.events.push(Event::CounterflowRecorded {
            session: session_id,
            route: route_id,
            amount,
        });
        Ok(())
    }

    pub fn settle_intent(
        &mut self,
        session_id: SessionId,
        intent_id: IntentId,
    ) -> OrbitResult<TxId> {
        let queued = self.intent(intent_id)?.clone();
        if queued.status != IntentStatus::Pending {
            return Err(OrbitError::IntentNotPending);
        }
        let intent = queued.intent.clone();
        let route = self.routes.get(intent.route)?.clone();
        if intent.deadline_slot < self.slot {
            return Err(OrbitError::IntentNotPending);
        }

        let gross_out = self.oracle.quote(
            &self.assets,
            intent.source_asset,
            intent.target_asset,
            intent.amount_in,
        )?;
        let fee = gross_out.mul_bps_floor(route.total_fee_bps()?)?;
        let net_out = gross_out.checked_sub(fee)?;
        if net_out < intent.min_amount_out {
            return Err(OrbitError::MinimumOutput);
        }

        let counterflow_credit = {
            let session = self.session_mut(session_id)?;
            session.counterflow_allowance(route.id, gross_out)?
        };

        self.risk
            .authorize(&route, intent.owner, gross_out, counterflow_credit)?;

        self.vault_mut(intent.source_vault)?
            .consume_reserved(intent.amount_in)?;
        self.vault_mut(intent.target_vault)?.pay_out(gross_out)?;
        self.account_mut(intent.recipient)?
            .credit(intent.target_asset, net_out)?;
        self.account_mut(route.operator)?
            .credit(intent.target_asset, fee)?;

        let tx = TxId::new(self.next_tx);
        self.next_tx += 1;
        if let Some(queued) = self.intents.get_mut(&intent_id) {
            queued.status = IntentStatus::Settled;
        }
        self.session_mut(session_id)?.include(intent_id);
        self.events.push(Event::IntentSettled {
            tx,
            session: session_id,
            intent: intent_id,
            recipient: intent.recipient,
            target_asset: intent.target_asset,
            gross_out,
            net_out,
            fee,
            counterflow_credit,
        });
        self.slot += 1;
        Ok(tx)
    }

    pub fn cancel_intent(&mut self, intent_id: IntentId) -> OrbitResult<()> {
        let queued = self.intent(intent_id)?.clone();
        if queued.status != IntentStatus::Pending {
            return Err(OrbitError::IntentNotPending);
        }
        self.vault_mut(queued.intent.source_vault)?
            .release_reserved(queued.intent.amount_in)?;
        if let Some(intent) = self.intents.get_mut(&intent_id) {
            intent.status = IntentStatus::Cancelled;
        }
        self.events
            .push(Event::IntentCancelled { intent: intent_id });
        Ok(())
    }

    pub fn demo() -> OrbitResult<DemoReport> {
        let mut ledger = Self::new();
        let operator = ledger.create_account("orbit-operator");
        let alice = ledger.create_account("market-maker-a");
        let bob = ledger.create_account("settlement-recipient-b");

        let usd = ledger.register_asset("oUSD", 6, 1_000)?;
        let eur = ledger.register_asset("oEUR", 6, 1_050)?;
        ledger.set_price(usd, 100_000_000, 9_950)?;
        ledger.set_price(eur, 108_000_000, 9_930)?;

        let usd_vault = ledger.create_vault(usd, operator, VaultKind::Settlement)?;
        let eur_vault = ledger.create_vault(eur, operator, VaultKind::Settlement)?;
        ledger.deposit_vault(usd_vault, Amount::new(600_000_000_000))?;
        ledger.deposit_vault(eur_vault, Amount::new(520_000_000_000))?;
        ledger
            .vault_mut(eur_vault)?
            .accrue(Amount::new(5_000_000_000))?;
        ledger.credit_account(alice, usd, Amount::new(35_000_000_000))?;

        let route = ledger.add_route(RouteConfig {
            id: RouteId::new(0),
            name: "usd-eur-primary".to_string(),
            source_asset: usd,
            target_asset: eur,
            source_vault: usd_vault,
            target_vault: eur_vault,
            operator,
            fee_bps: 18,
            relayer_bps: 2,
            max_unhedged: Amount::new(16_000_000_000),
            session_limit: Amount::new(60_000_000_000),
            min_confidence_bps: 9_800,
            enabled: true,
        })?;

        let intent_a = ledger.submit_intent(
            alice,
            bob,
            route,
            Amount::new(12_000_000_000),
            Amount::new(11_050_000_000),
            100,
            1,
        )?;
        let intent_b = ledger.submit_intent(
            alice,
            bob,
            route,
            Amount::new(10_000_000_000),
            Amount::new(9_200_000_000),
            100,
            2,
        )?;
        let intent_c = ledger.submit_intent(
            alice,
            bob,
            route,
            Amount::new(2_000_000_000),
            Amount::new(1_800_000_000),
            100,
            3,
        )?;
        ledger.cancel_intent(intent_c)?;

        let session = ledger.open_session();
        ledger.record_counterflow(session, route, Amount::new(10_500_000_000))?;
        ledger.settle_intent(session, intent_a)?;
        ledger.settle_intent(session, intent_b)?;
        ledger.session_mut(session)?.close();

        Ok(DemoReport {
            accounts: ledger.accounts,
            vaults: ledger.vaults,
            sessions: ledger.sessions,
            events: ledger.events,
        })
    }

    fn account(&self, id: AccountId) -> OrbitResult<&Account> {
        self.accounts.get(&id).ok_or(OrbitError::AccountNotFound)
    }

    fn account_mut(&mut self, id: AccountId) -> OrbitResult<&mut Account> {
        self.accounts
            .get_mut(&id)
            .ok_or(OrbitError::AccountNotFound)
    }

    fn vault(&self, id: VaultId) -> OrbitResult<&Vault> {
        self.vaults.get(&id).ok_or(OrbitError::VaultNotFound)
    }

    fn vault_mut(&mut self, id: VaultId) -> OrbitResult<&mut Vault> {
        self.vaults.get_mut(&id).ok_or(OrbitError::VaultNotFound)
    }

    fn intent(&self, id: IntentId) -> OrbitResult<&QueuedIntent> {
        self.intents.get(&id).ok_or(OrbitError::IntentNotFound)
    }

    fn session_mut(&mut self, id: SessionId) -> OrbitResult<&mut SettlementSession> {
        self.sessions
            .get_mut(&id)
            .ok_or(OrbitError::SessionNotFound)
    }
}
