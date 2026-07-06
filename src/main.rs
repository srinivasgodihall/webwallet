mod account;
mod address;
mod app_config;
mod chain;
mod chain_adapter;
mod network;
mod secret;
mod solana_balance;
mod solana_rpc;
mod solana_wallet;
mod wallet;
mod wallet_model;
mod wallet_session;

use crate::solana_balance::SolanaBalance;
use crate::solana_rpc::fetch_solana_devnet_balance;
use crate::solana_wallet::generate_solana_wallet;
use crate::wallet::WalletStatus;
use crate::wallet_session::{NamedWallet, WalletSession};
use gloo_timers::future::TimeoutFuture;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

fn main() {
    mount_to_body(App);
}

#[cfg(target_arch = "wasm32")]
async fn copy_public_address_to_clipboard(address: String) -> Result<(), ()> {
    let window = web_sys::window().ok_or(())?;
    let clipboard = window.navigator().clipboard();

    JsFuture::from(clipboard.write_text(&address))
        .await
        .map_err(|_| ())?;

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn copy_public_address_to_clipboard(_address: String) -> Result<(), ()> {
    Err(())
}

fn shorten_address(address: &str) -> String {
    if address.len() <= 12 {
        address.to_string()
    } else {
        format!("{}...{}", &address[..4], &address[address.len() - 4..])
    }
}

#[component]
fn App() -> impl IntoView {
    let (wallet_status, set_wallet_status) = signal(WalletStatus::NoWallet);
    let (wallet_session, set_wallet_session) = signal(WalletSession::new_empty());
    let (balance, set_balance) = signal::<Option<SolanaBalance>>(None);
    let (balance_error, set_balance_error) = signal::<Option<String>>(None);
    let (is_fetching_balance, set_is_fetching_balance) = signal(false);
    let (copy_status, set_copy_status) = signal("Copy");
    let (new_wallet_name, set_new_wallet_name) = signal(String::new());

    view! {
        <style>
            "
            :root {
                color: #f8f7ff;
                background: #090a12;
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
                overflow-x: hidden;
                font-size: 16px;
                line-height: 1.5;
                background:
                    radial-gradient(circle at 16% 0%, rgba(128, 83, 255, 0.26), transparent 34%),
                    radial-gradient(circle at 92% 12%, rgba(33, 224, 198, 0.2), transparent 30%),
                    linear-gradient(180deg, #10111d 0%, #090a12 48%, #05060b 100%);
            }

            button,
            input,
            select,
            textarea {
                font: inherit;
            }

            button {
                border: 0;
            }

            .app-shell {
                min-height: 100vh;
                padding: clamp(24px, 3vw, 48px);
                position: relative;
            }

            .app-shell::before {
                content: \"\";
                position: fixed;
                inset: 0;
                pointer-events: none;
                background:
                    linear-gradient(rgba(255, 255, 255, 0.035) 1px, transparent 1px),
                    linear-gradient(90deg, rgba(255, 255, 255, 0.035) 1px, transparent 1px);
                background-size: 74px 74px;
                mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 0.5), transparent 72%);
            }

            .wallet-app {
                width: min(1540px, 100%);
                margin: 0 auto;
                display: grid;
                gap: clamp(26px, 2.4vw, 38px);
                position: relative;
                z-index: 1;
            }

            .topbar {
                display: grid;
                grid-template-columns: minmax(260px, auto) minmax(320px, 1fr) auto;
                align-items: center;
                gap: clamp(18px, 2vw, 30px);
            }

            .brand {
                display: flex;
                align-items: center;
                gap: 13px;
                min-width: 0;
            }

            .brand-mark {
                width: 54px;
                height: 54px;
                border-radius: 18px;
                display: grid;
                place-items: center;
                color: #090a12;
                font-size: 1rem;
                font-weight: 900;
                background: linear-gradient(135deg, #9b7cff 0%, #38d9ff 48%, #55f6a8 100%);
                box-shadow: 0 18px 44px rgba(56, 217, 255, 0.2);
            }

            .brand h1 {
                margin: 0;
                font-size: 1.22rem;
                line-height: 1.15;
                font-weight: 850;
            }

            .brand p {
                margin: 4px 0 0;
                color: #a6a5bb;
                font-size: 1rem;
                line-height: 1.35;
            }

            .search-wrap {
                display: flex;
                align-items: center;
                gap: 10px;
                min-height: 56px;
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 28px;
                padding: 0 20px;
                background: rgba(255, 255, 255, 0.055);
                backdrop-filter: blur(18px);
            }

            .search-dot {
                width: 14px;
                height: 14px;
                border: 2px solid #a6a5bb;
                border-radius: 999px;
                position: relative;
                flex: 0 0 auto;
            }

            .search-dot::after {
                content: \"\";
                width: 6px;
                height: 2px;
                border-radius: 999px;
                background: #a6a5bb;
                position: absolute;
                right: -5px;
                bottom: -2px;
                transform: rotate(45deg);
            }

            .search-input {
                width: 100%;
                border: 0;
                outline: 0;
                color: #f8f7ff;
                background: transparent;
                font-size: 1.06rem;
            }

            .search-input::placeholder {
                color: #7e7d91;
            }

            .top-actions {
                display: flex;
                align-items: center;
                justify-content: flex-end;
                gap: 10px;
            }

            .network-toggle,
            .profile-pill,
            .circle-button {
                min-height: 52px;
                border-radius: 999px;
                color: #f8f7ff;
                background: rgba(255, 255, 255, 0.07);
                border: 1px solid rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(18px);
            }

            .network-toggle,
            .profile-pill {
                display: inline-flex;
                align-items: center;
                gap: 9px;
                padding: 0 16px;
                font-size: 1rem;
                font-weight: 760;
                white-space: nowrap;
            }

            .circle-button {
                width: 52px;
                display: grid;
                place-items: center;
                font-weight: 800;
            }

            .status-dot {
                width: 8px;
                height: 8px;
                border-radius: 999px;
                background: #55f6a8;
                box-shadow: 0 0 0 6px rgba(85, 246, 168, 0.11);
                flex: 0 0 auto;
            }

            .main-layout {
                display: grid;
                grid-template-columns: minmax(0, 1fr) minmax(380px, 440px);
                gap: clamp(24px, 2.6vw, 40px);
                align-items: start;
            }

            .wallet-column {
                display: grid;
                gap: 24px;
                min-width: 0;
            }

            .wallet-card {
                min-height: clamp(600px, 58vh, 760px);
                border-radius: 30px;
                padding: 1px;
                background:
                    linear-gradient(135deg, rgba(255, 255, 255, 0.38), rgba(255, 255, 255, 0.04)),
                    linear-gradient(135deg, rgba(155, 124, 255, 0.76), rgba(56, 217, 255, 0.26), rgba(85, 246, 168, 0.2));
                box-shadow:
                    0 34px 110px rgba(0, 0, 0, 0.45),
                    0 0 0 1px rgba(255, 255, 255, 0.05);
            }

            .wallet-card-inner {
                min-height: calc(clamp(600px, 58vh, 760px) - 2px);
                border-radius: 29px;
                padding: clamp(30px, 3vw, 46px);
                display: grid;
                align-content: space-between;
                gap: 28px;
                overflow: hidden;
                position: relative;
                background:
                    radial-gradient(circle at 14% 12%, rgba(255, 255, 255, 0.18), transparent 24%),
                    radial-gradient(circle at 92% 6%, rgba(85, 246, 168, 0.2), transparent 26%),
                    linear-gradient(145deg, rgba(43, 37, 84, 0.95), rgba(11, 13, 25, 0.98) 58%, rgba(14, 18, 30, 0.98));
            }

            .wallet-card-inner::after {
                content: \"\";
                position: absolute;
                width: 420px;
                height: 420px;
                right: -170px;
                bottom: -180px;
                border-radius: 999px;
                background: rgba(56, 217, 255, 0.12);
                filter: blur(6px);
            }

            .wallet-card-top,
            .wallet-card-main,
            .wallet-card-bottom {
                position: relative;
                z-index: 1;
            }

            .wallet-card-top {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                gap: 18px;
            }

            .wallet-title {
                min-width: 0;
            }

            .wallet-label {
                margin: 0;
                color: #b7b5cb;
                font-size: 0.96rem;
                font-weight: 800;
                letter-spacing: 0;
                text-transform: uppercase;
            }

            .wallet-title h2 {
                margin: 8px 0 0;
                color: #ffffff;
                font-size: clamp(3rem, 5.4vw, 5.8rem);
                line-height: 0.98;
                font-weight: 900;
            }

            .wallet-state {
                display: inline-flex;
                align-items: center;
                gap: 8px;
                min-height: 46px;
                border-radius: 999px;
                padding: 0 16px;
                color: #dfffea;
                background: rgba(85, 246, 168, 0.12);
                border: 1px solid rgba(85, 246, 168, 0.2);
                white-space: nowrap;
                font-size: 1rem;
                font-weight: 780;
            }

            .balance-block {
                display: grid;
                gap: 11px;
            }

            .balance-caption {
                margin: 0;
                color: #b7b5cb;
                font-size: 1.12rem;
                line-height: 1.4;
            }

            .balance-amount {
                margin: 0;
                color: #ffffff;
                font-size: clamp(4.2rem, 8.2vw, 9rem);
                line-height: 0.9;
                font-weight: 900;
                display: flex;
                align-items: baseline;
                flex-wrap: wrap;
                gap: 12px;
            }

            .balance-unit {
                color: #a6a5bb;
                font-size: clamp(1.25rem, 2.2vw, 1.8rem);
                font-weight: 850;
            }

            .balance-loader {
                display: inline-flex;
                align-items: center;
                gap: 6px;
                width: 46px;
                opacity: 1;
                transform: translateY(-4px);
                transition:
                    opacity 160ms ease,
                    width 160ms ease;
            }

            .balance-loader.hidden {
                width: 0;
                opacity: 0;
                overflow: hidden;
            }

            .balance-loader span {
                width: 9px;
                height: 9px;
                border-radius: 999px;
                background: #55f6a8;
                animation: balance-bounce 900ms ease-in-out infinite;
                box-shadow: 0 0 18px rgba(85, 246, 168, 0.45);
            }

            .balance-loader span:nth-child(2) {
                animation-delay: 120ms;
            }

            .balance-loader span:nth-child(3) {
                animation-delay: 240ms;
            }

            .wallet-card-bottom {
                display: grid;
                gap: 22px;
            }

            .address-section {
                display: grid;
                gap: 8px;
            }

            .address-capsule {
                display: grid;
                grid-template-columns: minmax(0, 1fr) auto;
                align-items: center;
                gap: 14px;
                border-radius: 24px;
                padding: 18px 20px;
                background: rgba(255, 255, 255, 0.08);
                border: 1px solid rgba(255, 255, 255, 0.1);
            }

            .address-capsule p {
                margin: 0;
            }

            .address-label {
                color: #a6a5bb;
                font-size: 0.9rem;
                font-weight: 800;
                text-transform: uppercase;
                letter-spacing: 0;
            }

            .copy-address-button {
                width: 42px;
                height: 42px;
                min-height: 42px;
                border-radius: 999px;
                padding: 0;
                display: grid;
                place-items: center;
                color: #08090f;
                background: rgba(85, 246, 168, 0.92);
                cursor: pointer;
                transition:
                    opacity 160ms ease,
                    transform 160ms ease,
                    background 160ms ease;
            }

            .copy-address-icon {
                width: 19px;
                height: 19px;
                display: block;
            }

            .copy-address-icon.hidden {
                display: none;
            }

            .copy-address-button:not(:disabled):hover {
                background: #55f6a8;
                transform: translateY(-1px);
            }

            .copy-address-button:disabled {
                opacity: 0.44;
                cursor: not-allowed;
            }

            .sr-only {
                position: absolute;
                width: 1px;
                height: 1px;
                padding: 0;
                margin: -1px;
                overflow: hidden;
                clip: rect(0, 0, 0, 0);
                white-space: nowrap;
                border: 0;
            }

            .address-value {
                color: #f8f7ff;
                font-family:
                    ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
                    \"Liberation Mono\", monospace;
                font-size: 1.04rem;
                line-height: 1.45;
                overflow-wrap: anywhere;
            }

            .primary-actions {
                display: grid;
                grid-template-columns: 1.2fr 1fr;
                gap: 12px;
            }

            .wallet-name-field {
                grid-column: 1 / -1;
                display: grid;
                gap: 8px;
            }

            .wallet-name-field label {
                color: #a6a5bb;
                font-size: 0.9rem;
                font-weight: 800;
                text-transform: uppercase;
                letter-spacing: 0;
            }

            .wallet-name-input {
                width: 100%;
                min-height: 58px;
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 20px;
                padding: 0 18px;
                color: #f8f7ff;
                background: rgba(255, 255, 255, 0.075);
                outline: none;
                font-size: 1.05rem;
                transition:
                    border-color 160ms ease,
                    background 160ms ease,
                    box-shadow 160ms ease;
            }

            .wallet-name-input::placeholder {
                color: #7e7d91;
            }

            .wallet-name-input:focus {
                border-color: rgba(85, 246, 168, 0.38);
                background: rgba(255, 255, 255, 0.095);
                box-shadow: 0 0 0 4px rgba(85, 246, 168, 0.1);
            }

            .primary-button,
            .secondary-button {
                min-height: 64px;
                border-radius: 22px;
                padding: 0 22px;
                cursor: pointer;
                font-size: 1.06rem;
                font-weight: 850;
                transition:
                    transform 160ms ease,
                    box-shadow 160ms ease,
                    background 160ms ease,
                    border-color 160ms ease;
            }

            .primary-button {
                color: #08090f;
                background: linear-gradient(135deg, #55f6a8 0%, #38d9ff 100%);
                box-shadow: 0 20px 48px rgba(56, 217, 255, 0.24);
            }

            .secondary-button {
                color: #f8f7ff;
                background: rgba(255, 255, 255, 0.09);
                border: 1px solid rgba(255, 255, 255, 0.14);
            }

            .primary-button:hover,
            .secondary-button:hover {
                transform: translateY(-1px);
            }

            .primary-button:active,
            .secondary-button:active {
                transform: translateY(0);
            }

            .error-text {
                min-height: 1.35rem;
                margin: 0;
                color: #ffb8aa;
                font-size: 1rem;
                line-height: 1.45;
            }

            .quick-action-row {
                display: grid;
                grid-template-columns: repeat(4, minmax(0, 1fr));
                gap: 16px;
            }

            .quick-action {
                min-height: 122px;
                border-radius: 26px;
                padding: 22px;
                display: grid;
                gap: 12px;
                align-content: center;
                text-align: left;
                color: #f8f7ff;
                background: rgba(255, 255, 255, 0.065);
                border: 1px solid rgba(255, 255, 255, 0.09);
                backdrop-filter: blur(18px);
            }

            .quick-action[disabled] {
                cursor: not-allowed;
                opacity: 0.74;
            }

            .action-glyph {
                width: 44px;
                height: 44px;
                border-radius: 16px;
                display: grid;
                place-items: center;
                color: #08090f;
                background: linear-gradient(135deg, #9b7cff, #38d9ff);
                font-size: 0.95rem;
                font-weight: 900;
            }

            .quick-action strong {
                font-size: 1.12rem;
            }

            .quick-action span:last-child {
                color: #a6a5bb;
                font-size: 0.95rem;
            }

            .content-grid {
                display: grid;
                grid-template-columns: minmax(0, 1.1fr) minmax(280px, 0.9fr);
                gap: 24px;
            }

            .surface {
                border-radius: 30px;
                padding: 24px;
                background: rgba(255, 255, 255, 0.06);
                border: 1px solid rgba(255, 255, 255, 0.09);
                backdrop-filter: blur(18px);
            }

            .section-head {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: 14px;
                margin-bottom: 18px;
            }

            .section-head h3 {
                margin: 0;
                color: #ffffff;
                font-size: 1.18rem;
                font-weight: 850;
            }

            .section-head p {
                margin: 4px 0 0;
                color: #a6a5bb;
                font-size: 1rem;
            }

            .small-pill {
                border-radius: 999px;
                padding: 8px 12px;
                color: #dcd9ff;
                background: rgba(155, 124, 255, 0.14);
                border: 1px solid rgba(155, 124, 255, 0.22);
                font-size: 0.9rem;
                font-weight: 800;
                white-space: nowrap;
            }

            .small-pill.active {
                color: #dfffea;
                background: rgba(85, 246, 168, 0.13);
                border-color: rgba(85, 246, 168, 0.22);
            }

            .asset-list,
            .activity-list,
            .wallet-list,
            .dapp-list {
                display: grid;
                gap: 13px;
            }

            .asset-row,
            .activity-row,
            .wallet-row,
            .dapp-row {
                border-radius: 22px;
                background: rgba(255, 255, 255, 0.052);
                border: 1px solid rgba(255, 255, 255, 0.07);
            }

            .asset-row {
                display: grid;
                grid-template-columns: 54px minmax(0, 1fr) auto;
                align-items: center;
                gap: 16px;
                padding: 16px;
            }

            .token-icon {
                width: 54px;
                height: 54px;
                border-radius: 18px;
                display: grid;
                place-items: center;
                color: #ffffff;
                background: linear-gradient(135deg, #7d5cff, #34d7ff);
                font-weight: 900;
            }

            .asset-name,
            .asset-balance,
            .activity-title,
            .wallet-row strong,
            .dapp-row strong {
                margin: 0;
                color: #ffffff;
                font-weight: 850;
            }

            .asset-meta,
            .asset-value,
            .activity-copy,
            .wallet-row span,
            .dapp-row span {
                margin: 4px 0 0;
                color: #a6a5bb;
                font-size: 1rem;
            }

            .asset-name,
            .asset-balance,
            .activity-title,
            .wallet-row strong,
            .dapp-row strong {
                font-size: 1.06rem;
            }

            .asset-values {
                text-align: right;
            }

            .activity-row {
                display: grid;
                grid-template-columns: 12px minmax(0, 1fr) auto;
                gap: 15px;
                align-items: center;
                padding: 16px;
            }

            .timeline-dot {
                width: 10px;
                height: 10px;
                border-radius: 999px;
                background: #55f6a8;
                box-shadow: 0 0 0 6px rgba(85, 246, 168, 0.11);
            }

            .activity-time {
                color: #7e7d91;
                font-size: 0.94rem;
                white-space: nowrap;
            }

            .right-rail {
                display: grid;
                gap: 18px;
            }

            .wallet-row,
            .dapp-row {
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: 14px;
                padding: 16px;
            }

            .wallet-row-button {
                width: 100%;
                color: inherit;
                text-align: left;
                cursor: pointer;
                transition:
                    background 160ms ease,
                    border-color 160ms ease,
                    transform 160ms ease;
            }

            .wallet-row-button:not(.active):hover {
                background: rgba(255, 255, 255, 0.08);
                border-color: rgba(255, 255, 255, 0.13);
                transform: translateY(-1px);
            }

            .wallet-row-button.active {
                background: rgba(85, 246, 168, 0.1);
                border-color: rgba(85, 246, 168, 0.24);
            }

            .wallet-row .small-pill {
                margin: 0;
                color: #dfffea;
            }

            .wallet-chip {
                display: flex;
                align-items: center;
                gap: 13px;
                min-width: 0;
            }

            .wallet-avatar {
                width: 50px;
                height: 50px;
                border-radius: 16px;
                display: grid;
                place-items: center;
                color: #08090f;
                background: linear-gradient(135deg, #55f6a8, #38d9ff);
                font-weight: 900;
                flex: 0 0 auto;
            }

            .chain-cloud {
                display: flex;
                flex-wrap: wrap;
                gap: 10px;
            }

            .chain-pill {
                border-radius: 999px;
                padding: 10px 13px;
                color: #b7b5cb;
                background: rgba(255, 255, 255, 0.055);
                border: 1px solid rgba(255, 255, 255, 0.08);
                font-size: 0.96rem;
                font-weight: 760;
            }

            .chain-pill.active {
                color: #08090f;
                background: linear-gradient(135deg, #55f6a8, #38d9ff);
                border-color: transparent;
            }

            .security-grid {
                display: grid;
                gap: 10px;
            }

            .security-item {
                display: grid;
                grid-template-columns: 1fr auto;
                gap: 14px;
                align-items: start;
                padding: 16px 0;
                border-top: 1px solid rgba(255, 255, 255, 0.08);
            }

            .security-item:first-child {
                border-top: 0;
                padding-top: 0;
            }

            .security-item p {
                margin: 0;
                color: #ffffff;
                font-size: 1.05rem;
                font-weight: 800;
            }

            .security-item span {
                display: block;
                margin-top: 4px;
                color: #a6a5bb;
                font-size: 0.98rem;
                line-height: 1.4;
            }

            @keyframes balance-bounce {
                0% {
                    opacity: 0.38;
                    transform: translateY(0);
                }
                50% {
                    opacity: 1;
                    transform: translateY(-6px);
                }
                100% {
                    opacity: 0.38;
                    transform: translateY(0);
                }
            }

            @media (max-width: 980px) {
                .topbar,
                .main-layout,
                .content-grid {
                    grid-template-columns: 1fr;
                }

                .top-actions {
                    justify-content: flex-start;
                    flex-wrap: wrap;
                }

                .wallet-card,
                .wallet-card-inner {
                    min-height: auto;
                }
            }

            @media (max-width: 640px) {
                .app-shell {
                    padding: 14px;
                }

                .brand-mark {
                    width: 46px;
                    height: 46px;
                }

                .brand h1 {
                    font-size: 1.08rem;
                }

                .brand p,
                .search-input {
                    font-size: 0.94rem;
                }

                .network-toggle,
                .profile-pill,
                .circle-button {
                    min-height: 44px;
                }

                .circle-button {
                    width: 44px;
                }

                .wallet-card-inner {
                    padding: 20px;
                    gap: 22px;
                }

                .wallet-title h2 {
                    font-size: 2.6rem;
                }

                .wallet-state {
                    min-height: 40px;
                    font-size: 0.9rem;
                }

                .balance-caption {
                    font-size: 1rem;
                }

                .balance-amount {
                    font-size: 3.35rem;
                    gap: 9px;
                }

                .balance-unit {
                    font-size: 1.1rem;
                }

                .wallet-card-top,
                .section-head {
                    flex-wrap: wrap;
                }

                .primary-actions,
                .quick-action-row,
                .asset-row {
                    grid-template-columns: 1fr;
                }

                .asset-values {
                    text-align: left;
                }

                .surface {
                    padding: 18px;
                }

                .quick-action {
                    min-height: 104px;
                    padding: 18px;
                }

                .activity-row {
                    grid-template-columns: 12px minmax(0, 1fr);
                }

                .activity-time {
                    grid-column: 2;
                }
            }
            "
        </style>

        <div class="app-shell">
            <div class="wallet-app">
                <header class="topbar">
                    <div class="brand">
                        <div class="brand-mark">"W"</div>
                        <div>
                            <h1>"WebWallet"</h1>
                            <p>"Multi-chain learning wallet"</p>
                        </div>
                    </div>

                    <label class="search-wrap">
                        <span class="search-dot"></span>
                        <input
                            class="search-input"
                            type="search"
                            placeholder="Search wallet, token, or app"
                            aria-label="Search wallet, token, or app"
                        />
                    </label>

                    <div class="top-actions">
                        <div class="network-toggle" aria-label="Current network">
                            <span class="status-dot"></span>
                            <span>"Solana Devnet"</span>
                        </div>
                        <div class="circle-button" aria-label="Notifications">"N"</div>
                        <div class="profile-pill">
                            <span>"Primary"</span>
                        </div>
                    </div>
                </header>

                <main class="main-layout">
                    <div class="wallet-column">
                        <section class="wallet-card">
                            <div class="wallet-card-inner">
                                <div class="wallet-card-top">
                                    <div class="wallet-title">
                                        <p class="wallet-label">"Active wallet"</p>
                                        <h2>
                                            {move || {
                                                wallet_session.with(|session| {
                                                    session
                                                        .selected_wallet()
                                                        .map(|wallet| wallet.name().to_string())
                                                        .unwrap_or_else(|| "No wallet".to_string())
                                                    })
                                            }}
                                        </h2>
                                    </div>
                                    <div class="wallet-state">
                                        <span class="status-dot"></span>
                                        <span>{move || wallet_status.get().label()}</span>
                                    </div>
                                </div>

                                <div class="wallet-card-main">
                                    <div class="balance-block">
                                        <p class="balance-caption">
                                            {move || {
                                                if wallet_session.with(|session| session.selected_wallet().is_some()) {
                                                    "Solana Devnet balance".to_string()
                                                } else {
                                                    "Create a wallet to begin".to_string()
                                                }
                                            }}
                                        </p>
                                        <p class="balance-amount">
                                            {move || {
                                                balance
                                                    .get()
                                                    .map(|balance| format!("{:.4}", balance.sol()))
                                                    .unwrap_or_else(|| "0.0000".to_string())
                                            }}
                                            <span class="balance-unit">"SOL"</span>
                                            <span
                                                class=move || {
                                                    if is_fetching_balance.get() {
                                                        "balance-loader"
                                                    } else {
                                                        "balance-loader hidden"
                                                    }
                                                }
                                                aria-label="Refreshing balance"
                                            >
                                                <span></span>
                                                <span></span>
                                                <span></span>
                                            </span>
                                        </p>
                                    </div>
                                </div>

                                <div class="wallet-card-bottom">
                                    <div class="address-section">
                                        <p class="address-label">"Public address"</p>
                                        <div class="address-capsule">
                                            <p class="address-value">
                                                {move || {
                                                    wallet_session.with(|session| {
                                                        session
                                                            .selected_public_address()
                                                            .map(|address| address.as_str().to_string())
                                                            .unwrap_or_else(|| "No address generated".to_string())
                                                    })
                                                }}
                                            </p>
                                            <button
                                                class="copy-address-button"
                                                type="button"
                                                disabled=move || {
                                                    wallet_session
                                                        .with(|session| session.selected_public_address().is_none())
                                                        || copy_status.get() == "Copying"
                                                }
                                                aria-label=move || copy_status.get()
                                                title=move || copy_status.get()
                                                on:click=move |_| {
                                                    let Some(public_address) = wallet_session.with(|session| {
                                                        session
                                                            .selected_public_address()
                                                            .map(|address| address.as_str().to_string())
                                                    }) else {
                                                        set_copy_status.set("No address");
                                                        return;
                                                    };

                                                    set_copy_status.set("Copying");

                                                    spawn_local(async move {
                                                        match copy_public_address_to_clipboard(public_address).await {
                                                            Ok(()) => {
                                                                set_copy_status.set("Copied");
                                                                TimeoutFuture::new(1_400).await;

                                                                if copy_status.get_untracked() == "Copied" {
                                                                    set_copy_status.set("Copy");
                                                                }
                                                            }
                                                            Err(()) => set_copy_status.set("Copy failed"),
                                                        }
                                                    });
                                                }
                                            >
                                                <svg
                                                    class=move || {
                                                        if copy_status.get() == "Copied" {
                                                            "copy-address-icon hidden"
                                                        } else {
                                                            "copy-address-icon"
                                                        }
                                                    }
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    aria-hidden="true"
                                                >
                                                    <rect x="8" y="8" width="11" height="11" rx="2"></rect>
                                                    <path d="M5 15H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v1"></path>
                                                </svg>
                                                <svg
                                                    class=move || {
                                                        if copy_status.get() == "Copied" {
                                                            "copy-address-icon"
                                                        } else {
                                                            "copy-address-icon hidden"
                                                        }
                                                    }
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2.4"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    aria-hidden="true"
                                                >
                                                    <path d="M20 6 9 17l-5-5"></path>
                                                </svg>
                                                <span class="sr-only">{move || copy_status.get()}</span>
                                            </button>
                                        </div>
                                    </div>

                                    <div class="primary-actions">
                                        <div class="wallet-name-field">
                                            <label for="wallet-name">"Wallet name"</label>
                                            <input
                                                id="wallet-name"
                                                class="wallet-name-input"
                                                type="text"
                                                maxlength="28"
                                                placeholder="Trading wallet, savings, devnet..."
                                                prop:value=move || new_wallet_name.get()
                                                on:input=move |event| {
                                                    set_new_wallet_name.set(event_target_value(&event));
                                                }
                                            />
                                        </div>
                                        <button
                                            class="primary-button"
                                            type="button"
                                            on:click=move |_| {
                                                let wallet = generate_solana_wallet();
                                                let next_index =
                                                    wallet_session.with(|session| session.wallet_count());
                                                let wallet_name = new_wallet_name.with(|name| {
                                                    let trimmed_name = name.trim();

                                                    if trimmed_name.is_empty() {
                                                        format!("Wallet {}", next_index + 1)
                                                    } else {
                                                        trimmed_name.to_string()
                                                    }
                                                });

                                                set_wallet_status.set(WalletStatus::Unlocked);
                                                set_wallet_session.update(|session| {
                                                    session.add_wallet(NamedWallet::new(wallet_name, wallet));
                                                    let _ = session.select_wallet(next_index);
                                                });
                                                set_balance.set(None);
                                                set_balance_error.set(None);
                                                set_copy_status.set("Copy");
                                                set_new_wallet_name.set(String::new());
                                            }
                                        >
                                            "Create Wallet"
                                        </button>
                                        <button
                                            class="secondary-button"
                                            type="button"
                                            on:click=move |_| {
                                                let Some(public_address) = wallet_session.with(|session| {
                                                    session
                                                        .selected_public_address()
                                                        .cloned()
                                                }) else {
                                                    set_balance_error
                                                        .set(Some("Create a Solana wallet first".to_string()));
                                                    return;
                                                };

                                                set_is_fetching_balance.set(true);
                                                set_balance_error.set(None);

                                                spawn_local(async move {
                                                    match fetch_solana_devnet_balance(&public_address).await {
                                                        Ok(next_balance) => {
                                                            set_balance.set(Some(next_balance));
                                                            set_balance_error.set(None);
                                                        }
                                                        Err(error) => {
                                                            set_balance.set(None);
                                                            set_balance_error.set(Some(error.message().to_string()));
                                                        }
                                                    }

                                                    set_is_fetching_balance.set(false);
                                                });
                                            }
                                        >
                                            "Refresh Balance"
                                        </button>
                                    </div>

                                    <p class="error-text">
                                        {move || balance_error.get().unwrap_or_default()}
                                    </p>
                                </div>
                            </div>
                        </section>

                        <div class="quick-action-row">
                            <button class="quick-action" type="button" disabled=true>
                                <span class="action-glyph">"S"</span>
                                <strong>"Send"</strong>
                                <span>"Coming later"</span>
                            </button>
                            <button class="quick-action" type="button" disabled=true>
                                <span class="action-glyph">"R"</span>
                                <strong>"Receive"</strong>
                                <span>"Coming later"</span>
                            </button>
                            <button class="quick-action" type="button" disabled=true>
                                <span class="action-glyph">"X"</span>
                                <strong>"Swap"</strong>
                                <span>"Coming later"</span>
                            </button>
                            <button class="quick-action" type="button" disabled=true>
                                <span class="action-glyph">"D"</span>
                                <strong>"Apps"</strong>
                                <span>"Coming later"</span>
                            </button>
                        </div>

                        <div class="content-grid">
                            <section class="surface">
                                <div class="section-head">
                                    <div>
                                        <h3>"Assets"</h3>
                                        <p>"Simple token list, not a trading table."</p>
                                    </div>
                                    <span class="small-pill active">"1 live"</span>
                                </div>

                                <div class="asset-list">
                                    <div class="asset-row">
                                        <div class="token-icon">"S"</div>
                                        <div>
                                            <p class="asset-name">"Solana"</p>
                                            <p class="asset-meta">"Native asset on Devnet"</p>
                                        </div>
                                        <div class="asset-values">
                                            <p class="asset-balance">
                                                {move || {
                                                    balance
                                                        .get()
                                                        .map(|balance| format!("{:.4} SOL", balance.sol()))
                                                        .unwrap_or_else(|| "-- SOL".to_string())
                                                }}
                                            </p>
                                            <p class="asset-value">"Devnet"</p>
                                        </div>
                                    </div>
                                </div>
                            </section>

                            <section class="surface">
                                <div class="section-head">
                                    <div>
                                        <h3>"Recent activity"</h3>
                                        <p>"A timeline belongs here later."</p>
                                    </div>
                                    <span class="small-pill">"Empty"</span>
                                </div>

                                <div class="activity-list">
                                    <div class="activity-row">
                                        <span class="timeline-dot"></span>
                                        <div>
                                            <p class="activity-title">"No transactions yet"</p>
                                            <p class="activity-copy">
                                                "Signing and transaction history are future milestones."
                                            </p>
                                        </div>
                                        <span class="activity-time">"Now"</span>
                                    </div>
                                </div>
                            </section>
                        </div>
                    </div>

                    <aside class="right-rail">
                        <section class="surface">
                            <div class="section-head">
                                <div>
                                    <h3>"Wallets"</h3>
                                    <p>"Session-only wallet switcher."</p>
                                </div>
                                <span class="small-pill active">
                                    {move || {
                                        format!(
                                            "{} saved",
                                            wallet_session.with(|session| session.wallet_count()),
                                        )
                                    }}
                                </span>
                            </div>

                            <div class="wallet-list">
                                <Show
                                    when=move || {
                                        wallet_session.with(|session| session.wallet_count() > 0)
                                    }
                                    fallback=move || {
                                        view! {
                                            <div class="wallet-row">
                                                <div class="wallet-chip">
                                                    <div class="wallet-avatar">"0"</div>
                                                    <div>
                                                        <strong>"No wallet yet"</strong>
                                                        <span>"Create one to start switching."</span>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                >
                                    <For
                                        each=move || {
                                            wallet_session.with(|session| {
                                                session
                                                    .wallets()
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(index, wallet)| {
                                                        (
                                                            index,
                                                            wallet.name().to_string(),
                                                            wallet
                                                                .public_address()
                                                                .as_str()
                                                                .to_string(),
                                                        )
                                                    })
                                                    .collect::<Vec<_>>()
                                            })
                                        }
                                        key=|(index, _, _)| *index
                                        children=move |(index, wallet_name, address)| {
                                            let short_address = shorten_address(&address);

                                            view! {
                                                <button
                                                    class=move || {
                                                        if wallet_session
                                                            .with(|session| session.selected_wallet_index() == Some(index))
                                                        {
                                                            "wallet-row wallet-row-button active"
                                                        } else {
                                                            "wallet-row wallet-row-button"
                                                        }
                                                    }
                                                    type="button"
                                                    aria-pressed=move || {
                                                        wallet_session
                                                            .with(|session| session.selected_wallet_index() == Some(index))
                                                    }
                                                    on:click=move |_| {
                                                        set_wallet_session.update(|session| {
                                                            let _ = session.select_wallet(index);
                                                        });
                                                        set_balance.set(None);
                                                        set_balance_error.set(None);
                                                        set_copy_status.set("Copy");
                                                    }
                                                >
                                                    <div class="wallet-chip">
                                                        <div class="wallet-avatar">{format!("{}", index + 1)}</div>
                                                        <div>
                                                            <strong>{wallet_name}</strong>
                                                            <span>{short_address}</span>
                                                        </div>
                                                    </div>
                                                </button>
                                            }
                                        }
                                    />
                                </Show>
                            </div>
                        </section>

                        <section class="surface">
                            <div class="section-head">
                                <div>
                                    <h3>"Networks"</h3>
                                    <p>"Solana works now. Others stay visual until built."</p>
                                </div>
                            </div>

                            <div class="chain-cloud">
                                <span class="chain-pill">"Ethereum"</span>
                                <span class="chain-pill">"Bitcoin"</span>
                                <span class="chain-pill active">"Solana"</span>
                                <span class="chain-pill">"Base"</span>
                                <span class="chain-pill">"Arbitrum"</span>
                                <span class="chain-pill">"Optimism"</span>
                                <span class="chain-pill">"Polygon"</span>
                                <span class="chain-pill">"BNB"</span>
                                <span class="chain-pill">"Avalanche"</span>
                                <span class="chain-pill">"Sui"</span>
                                <span class="chain-pill">"Aptos"</span>
                                <span class="chain-pill">"Cosmos"</span>
                            </div>
                        </section>

                        <section class="surface">
                            <div class="section-head">
                                <div>
                                    <h3>"Connected apps"</h3>
                                    <p>"dApp permissions will be added later."</p>
                                </div>
                                <span class="small-pill">"0"</span>
                            </div>

                            <div class="dapp-list">
                                <div class="dapp-row">
                                    <div>
                                        <strong>"No apps connected"</strong>
                                        <span>"Your wallet is isolated in this milestone."</span>
                                    </div>
                                </div>
                            </div>
                        </section>

                        <section class="surface">
                            <div class="section-head">
                                <div>
                                    <h3>"Security"</h3>
                                    <p>"Learning-safe wallet lifecycle."</p>
                                </div>
                                <span class="small-pill active">"Local"</span>
                            </div>

                            <div class="security-grid">
                                <div class="security-item">
                                    <div>
                                        <p>"Secret visibility"</p>
                                        <span>"Private material is not rendered in the UI."</span>
                                    </div>
                                    <span class="small-pill active">"OK"</span>
                                </div>
                                <div class="security-item">
                                    <div>
                                        <p>"Session"</p>
                                        <span>"Generated wallet stays in memory only."</span>
                                    </div>
                                    <span class="small-pill active">"Memory"</span>
                                </div>
                                <div class="security-item">
                                    <div>
                                        <p>"Network"</p>
                                        <span>"Balance reads use Solana Devnet RPC."</span>
                                    </div>
                                    <span class="small-pill">"Devnet"</span>
                                </div>
                            </div>
                        </section>
                    </aside>
                </main>
            </div>
        </div>
    }
}
