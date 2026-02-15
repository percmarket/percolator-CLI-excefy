const FEED_ENDPOINT = "http://localhost:8787/markets";

const marketGrid = document.getElementById("marketGrid");
const stats = document.getElementById("stats");
const feedStatus = document.getElementById("feedStatus");
const tpl = document.getElementById("marketCardTpl");

function fmtUsd(n) {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  }).format(n || 0);
}

function fmtSol(n) {
  return `${(n || 0).toFixed(1)} SOL`;
}

function pct(num, den) {
  if (!den) return 50;
  return Math.round((num / den) * 100);
}

function countdown(isoDate) {
  const ms = new Date(isoDate).getTime() - Date.now();
  if (ms <= 0) return "Closing now";
  const hrs = Math.floor(ms / 3_600_000);
  const mins = Math.floor((ms % 3_600_000) / 60_000);
  return `${hrs}h ${mins}m left`;
}

function renderStats(markets) {
  const totalMarkets = markets.length;
  const totalVolume = markets.reduce((s, m) => s + (m.volume_24h || 0), 0);
  const totalPools = markets.reduce((s, m) => s + (m.yes_pool || 0) + (m.no_pool || 0), 0);
  const avgMcap =
    totalMarkets > 0
      ? markets.reduce((s, m) => s + (m.market_cap || 0), 0) / totalMarkets
      : 0;

  const cards = [
    ["Active Markets", totalMarkets.toString()],
    ["24h Token Volume", fmtUsd(totalVolume)],
    ["Total Pools", fmtSol(totalPools)],
    ["Avg Market Cap", fmtUsd(avgMcap)],
  ];

  stats.innerHTML = cards
    .map(
      ([label, value]) =>
        `<div class="stat-card"><div class="stat-label">${label}</div><div class="stat-value">${value}</div></div>`
    )
    .join("");
}

function renderMarkets(markets) {
  marketGrid.innerHTML = "";
  if (!markets.length) {
    marketGrid.innerHTML = '<div class="empty">No migrated PumpSwap token markets found.</div>';
    return;
  }

  for (const m of markets) {
    const node = tpl.content.cloneNode(true);
    const totalPool = (m.yes_pool || 0) + (m.no_pool || 0);
    const yesOdds = pct(m.yes_pool || 0, totalPool);
    const noOdds = 100 - yesOdds;

    node.querySelector(".token-symbol").textContent = m.token_symbol;
    node.querySelector(".token-name").textContent = m.token_name;
    node.querySelector(".question").textContent = m.question;
    node.querySelector(".close-at").textContent = countdown(m.close_at);
    node.querySelector(".volume").textContent = `Vol: ${fmtUsd(m.volume_24h)}`;
    node.querySelector(".yes-btn").textContent = `YES ${yesOdds}%`;
    node.querySelector(".no-btn").textContent = `NO ${noOdds}%`;
    node.querySelector(".yes-pool").textContent = `YES pool: ${fmtSol(m.yes_pool)}`;
    node.querySelector(".no-pool").textContent = `NO pool: ${fmtSol(m.no_pool)}`;

    marketGrid.appendChild(node);
  }
}

async function loadMarkets() {
  try {
    const res = await fetch(FEED_ENDPOINT, { cache: "no-store" });
    if (!res.ok) throw new Error(`Feed responded ${res.status}`);
    const markets = await res.json();
    feedStatus.textContent = "Live feed connected";
    return markets;
  } catch {
    const fallback = await fetch("./data/sample-markets.json", { cache: "no-store" });
    const markets = await fallback.json();
    feedStatus.textContent = "Using local fallback data";
    return markets;
  }
}

async function boot() {
  const raw = await loadMarkets();
  const migratedOnly = raw.filter((m) => m.migrated_to_pumpswap === true);
  renderStats(migratedOnly);
  renderMarkets(migratedOnly);
}

boot();
setInterval(boot, 30_000);
