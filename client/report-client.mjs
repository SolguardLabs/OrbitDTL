function amount(value, field) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(`${field} must be a non-negative safe integer`);
  }
  return BigInt(value);
}

function record(value, field) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${field} must be an object`);
  }
  return value;
}

export function summarizeReport(report) {
  const source = record(report, "report");
  const vaults = record(source.vaults, "vaults");
  const sessions = record(source.sessions, "sessions");
  if (!Array.isArray(source.events)) throw new TypeError("events must be an array");

  const assetVaults = new Map();
  for (const [id, raw] of Object.entries(vaults)) {
    const vault = record(raw, `vaults.${id}`);
    const asset = String(amount(vault.asset, `vaults.${id}.asset`));
    const reserve = amount(vault.reserve, `vaults.${id}.reserve`);
    const locked = amount(vault.locked, `vaults.${id}.locked`);
    if (locked > reserve) throw new RangeError(`vaults.${id} locked exceeds reserve`);
    const current = assetVaults.get(asset) ?? { vaults: 0, reserve: 0n, locked: 0n, available: 0n };
    current.vaults += 1;
    current.reserve += reserve;
    current.locked += locked;
    current.available += reserve - locked;
    assetVaults.set(asset, current);
  }

  const payouts = new Map();
  for (const [index, raw] of source.events.entries()) {
    const event = record(raw, `events.${index}`);
    if (event.type !== "intent_settled") continue;
    const asset = String(amount(event.target_asset, `events.${index}.target_asset`));
    const gross = amount(event.gross_out, `events.${index}.gross_out`);
    const net = amount(event.net_out, `events.${index}.net_out`);
    const fee = amount(event.fee, `events.${index}.fee`);
    if (gross !== net + fee) throw new RangeError(`events.${index} settlement does not reconcile`);
    const current = payouts.get(asset) ?? { settlements: 0, gross: 0n, net: 0n, fees: 0n };
    current.settlements += 1;
    current.gross += gross;
    current.net += net;
    current.fees += fee;
    payouts.set(asset, current);
  }

  return {
    vaultsByAsset: Object.fromEntries(
      [...assetVaults]
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([asset, value]) => [
          asset,
          {
            vaults: value.vaults,
            reserve: value.reserve.toString(),
            locked: value.locked.toString(),
            available: value.available.toString(),
          },
        ]),
    ),
    settlementsByAsset: Object.fromEntries(
      [...payouts]
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([asset, value]) => [
          asset,
          {
            settlements: value.settlements,
            gross: value.gross.toString(),
            net: value.net.toString(),
            fees: value.fees.toString(),
          },
        ]),
    ),
    sessions: Object.keys(sessions).length,
    events: source.events.length,
  };
}
