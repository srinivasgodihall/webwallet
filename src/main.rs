mod account;
mod address;
mod app_config;
mod chain;
mod chain_adapter;
mod network;
mod secret;
mod solana_wallet;
mod wallet;
mod wallet_model;

use crate::wallet::WalletStatus;
use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (wallet_status, set_wallet_status) = signal(WalletStatus::NoWallet);
    view! {
        <style>
            "
            :root {
                color: #15201a;
                background: #f5f7f2;
                font-family:
                    Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
                    \"Segoe UI\", sans-serif;
            }

            * {
                box-sizing: border-box;
            }

            body {
                margin: 0;
                min-width: 320px;
                min-height: 100vh;
                background:
                    linear-gradient(135deg, rgba(43, 109, 79, 0.12), transparent 34%),
                    #f5f7f2;
            }

            button,
            input,
            select,
            textarea {
                font: inherit;
            }

            .app-shell {
                min-height: 100vh;
                display: grid;
                grid-template-columns: 260px minmax(0, 1fr);
            }

            .sidebar {
                padding: 28px 22px;
                background: #16251d;
                color: #f6fbf7;
                display: flex;
                flex-direction: column;
                gap: 28px;
            }

            .brand {
                display: grid;
                gap: 8px;
            }

            .brand-mark {
                width: 38px;
                height: 38px;
                border-radius: 8px;
                display: grid;
                place-items: center;
                background: #76d39a;
                color: #102017;
                font-weight: 800;
            }

            .brand h1 {
                margin: 0;
                font-size: 1.18rem;
                line-height: 1.25;
            }

            .brand p {
                margin: 0;
                color: #b8c9bf;
                font-size: 0.9rem;
                line-height: 1.45;
            }

            .nav-list {
                display: grid;
                gap: 8px;
            }

            .nav-item {
                padding: 10px 12px;
                border-radius: 8px;
                color: #d8e6dd;
                background: rgba(255, 255, 255, 0.05);
                font-size: 0.94rem;
            }

            .nav-item.active {
                color: #102017;
                background: #d8f4df;
                font-weight: 700;
            }

            .main {
                padding: 30px;
                display: grid;
                gap: 24px;
                align-content: start;
            }

            .topbar {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                gap: 18px;
                flex-wrap: wrap;
            }

            .page-title {
                display: grid;
                gap: 6px;
            }

            .page-title h2 {
                margin: 0;
                font-size: 1.8rem;
                line-height: 1.15;
            }

            .page-title p {
                margin: 0;
                color: #52645a;
                line-height: 1.5;
            }

            .network-pill {
                border: 1px solid #b9dbc3;
                border-radius: 999px;
                padding: 8px 12px;
                background: #eaf8ee;
                color: #245438;
                font-weight: 700;
                white-space: nowrap;
            }

            .summary-grid {
                display: grid;
                grid-template-columns: repeat(3, minmax(0, 1fr));
                gap: 16px;
            }

            .panel {
                border: 1px solid #d9e4dc;
                border-radius: 8px;
                background: rgba(255, 255, 255, 0.78);
                padding: 18px;
                box-shadow: 0 14px 40px rgba(24, 42, 31, 0.08);
            }

            .panel-label {
                margin: 0 0 8px;
                color: #65766c;
                font-size: 0.82rem;
                text-transform: uppercase;
                letter-spacing: 0.08em;
            }

            .panel-value {
                margin: 0;
                font-size: 1.35rem;
                font-weight: 800;
            }

            .workspace {
                display: grid;
                grid-template-columns: minmax(0, 1.5fr) minmax(280px, 0.85fr);
                gap: 16px;
            }

            .section-title {
                display: flex;
                justify-content: space-between;
                align-items: center;
                gap: 12px;
                margin-bottom: 16px;
            }

            .section-title h3 {
                margin: 0;
                font-size: 1rem;
            }

            .muted {
                color: #66766c;
            }

            .chain-list {
                display: grid;
                gap: 10px;
            }

            .chain-row {
                display: grid;
                grid-template-columns: 40px minmax(0, 1fr) auto;
                gap: 12px;
                align-items: center;
                padding: 12px;
                border: 1px solid #e2e9e4;
                border-radius: 8px;
                background: #fbfdfb;
            }

            .chain-icon {
                width: 40px;
                height: 40px;
                border-radius: 8px;
                display: grid;
                place-items: center;
                background: #edf3ef;
                font-weight: 800;
            }

            .chain-name {
                margin: 0;
                font-weight: 800;
            }

            .chain-description {
                margin: 2px 0 0;
                color: #68786e;
                font-size: 0.92rem;
            }

            .status {
                color: #8a5b00;
                background: #fff3cf;
                border-radius: 999px;
                padding: 6px 10px;
                font-size: 0.82rem;
                font-weight: 700;
                white-space: nowrap;
            }

            .learning-list {
                margin: 0;
                padding-left: 18px;
                display: grid;
                gap: 10px;
                color: #405148;
                line-height: 1.45;
            }

            .empty-state {
                border: 1px dashed #bacbc0;
                border-radius: 8px;
                padding: 18px;
                background: #f8fbf8;
                color: #56685e;
                line-height: 1.5;
            }

            @media (max-width: 840px) {
                .app-shell {
                    grid-template-columns: 1fr;
                }

                .sidebar {
                    padding: 20px;
                }

                .summary-grid,
                .workspace {
                    grid-template-columns: 1fr;
                }

                .main {
                    padding: 20px;
                }
            }
            "
        </style>

        <div class="app-shell">
            <aside class="sidebar">
                <div class="brand">
                    <div class="brand-mark">"W"</div>
                    <div>
                        <h1>"Educational Multi-Chain Wallet"</h1>
                        <p>"Learner-built wallet core with a static frontend shell."</p>
                    </div>
                </div>

                <nav class="nav-list" aria-label="Main navigation">
                    <div class="nav-item active">"Overview"</div>
                    <div class="nav-item">"Accounts"</div>
                    <div class="nav-item">"Networks"</div>
                    <div class="nav-item">"Learning Notes"</div>
                </nav>
            </aside>

            <main class="main">
                <section class="topbar">
                    <div class="page-title">
                        <h2>"Wallet Overview"</h2>
                        <p>"Frontend scaffold only. Wallet generation, storage, signing, and RPC logic come later."</p>
                    </div>
                    <div class="network-pill">"Sepolia testnet first"</div>
                </section>

                <section class="summary-grid" aria-label="Wallet summary">
                    <div class="panel">
                        <p class="panel-label">"Wallet status"</p>
                        <p class="panel-value">{move || wallet_status.get().label()}</p>
                        <button
                            type="button"
                            on:click=move |_| set_wallet_status.set(WalletStatus::Locked)
                            >
                              "Lock wallet"
                        </button>
                    </div>
                    <div class="panel">
                        <p class="panel-label">"Active account"</p>
                        <p class="panel-value">"None"</p>
                    </div>
                    <div class="panel">
                        <p class="panel-label">"Total balance"</p>
                        <p class="panel-value">"--"</p>
                    </div>
                </section>

                <section class="workspace">
                    <div class="panel">
                        <div class="section-title">
                            <h3>"Chain Adapters"</h3>
                            <span class="muted">"Planned"</span>
                        </div>

                        <div class="chain-list">
                            <div class="chain-row">
                                <div class="chain-icon">"E"</div>
                                <div>
                                    <p class="chain-name">"Ethereum"</p>
                                    <p class="chain-description">"First learning target: Sepolia account and balance flow."</p>
                                </div>
                                <span class="status">"Next"</span>
                            </div>

                            <div class="chain-row">
                                <div class="chain-icon">"S"</div>
                                <div>
                                    <p class="chain-name">"Solana"</p>
                                    <p class="chain-description">"Separate key type and transaction model for a later milestone."</p>
                                </div>
                                <span class="status">"Later"</span>
                            </div>

                            <div class="chain-row">
                                <div class="chain-icon">"B"</div>
                                <div>
                                    <p class="chain-name">"Bitcoin"</p>
                                    <p class="chain-description">"UTXO-based wallet design after account-based chains are clear."</p>
                                </div>
                                <span class="status">"Later"</span>
                            </div>
                        </div>
                    </div>

                    <aside class="panel">
                        <div class="section-title">
                            <h3>"Backend Learning Queue"</h3>
                        </div>

                        <ul class="learning-list">
                            <li>"Define wallet status types."</li>
                            <li>"Model Ethereum testnet networks."</li>
                            <li>"Generate wallet data in memory only."</li>
                            <li>"Add encrypted storage after the core model is reviewed."</li>
                        </ul>

                        <div class="empty-state">
                            "No secrets are generated, stored, logged, or transmitted by this frontend shell."
                        </div>
                    </aside>
                </section>
            </main>
        </div>
    }
}
