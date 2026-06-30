//! Stone Gaming Extension v1.0.2
//!
//! Bietet:
//! - Firmen-Registrierung (Company)
//! - Spiele-Registrierung (Game) mit Key-Generation
//! - Item-Trading & Marktplatz
//! - Game-ID-Validierung
//!
//! Wird als WASM-Modul vom Stone Dashboard geladen.

use serde::{Deserialize, Serialize};

// ─── Init / Shutdown ────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn init() {}
#[no_mangle]
pub extern "C" fn shutdown() {}

#[no_mangle]
pub extern "C" fn name() -> *const u8 {
    b"Gaming Extension\0".as_ptr()
}
#[no_mangle]
pub extern "C" fn version() -> *const u8 {
    b"1.0.2\0".as_ptr()
}

// ─── Firmen-Registrierung ──────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct CompanyRegistration {
    pub owner_wallet: String,
    pub name: String,
    pub country: String,    // ISO-3166-1 alpha-2, z.B. "DE"
    pub website: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyInfo {
    pub owner_wallet: String,
    pub name: String,
    pub country: String,
    pub website: String,
    pub verified: bool,
    pub status: String,
    pub registered_at: i64,
}

/// Validiert eine Firmen-Registrierung (vom Dashboard vor dem Submit aufgerufen).
#[no_mangle]
pub extern "C" fn validate_company(json_ptr: *const u8, json_len: usize) -> *const u8 {
    let json = unsafe { std::slice::from_raw_parts(json_ptr, json_len) };
    let json_str = std::str::from_utf8(json).unwrap_or("{}");

    match serde_json::from_str::<CompanyRegistration>(json_str) {
        Ok(c) => {
            let mut errors: Vec<String> = Vec::new();
            if c.name.len() < 2 || c.name.len() > 64 {
                errors.push("Name muss 2-64 Zeichen lang sein".into());
            }
            if c.country.len() != 2 {
                errors.push("Land muss ISO-3166-1 alpha-2 sein (z.B. DE)".into());
            }
            if c.owner_wallet.len() != 64 {
                errors.push("Ungültige Wallet-Adresse".into());
            }
            let result = if errors.is_empty() {
                format!("{{\"ok\": true, \"name\": \"{}\"}}\0", c.name)
            } else {
                format!("{{\"ok\": false, \"errors\": {}}}\0", serde_json::to_string(&errors).unwrap_or_default())
            };
            result.into_bytes().leak().as_ptr()
        }
        Err(e) => {
            let err = format!("{{\"ok\": false, \"error\": \"{}\"}}\0", e);
            err.into_bytes().leak().as_ptr()
        }
    }
}

// ─── Spiele-Registrierung ──────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct GameRegistration {
    pub game_id: String,
    pub owner_company: String,
    pub name: String,
    pub version: String,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub game_id: String,
    pub owner_company: String,
    pub name: String,
    pub version: String,
    pub genres: Vec<String>,
    pub verified: bool,
    pub status: String,
    pub registered_at: i64,
}

/// Validiert eine Game-ID (3-64 Zeichen, nur a-z, 0-9, _, -).
#[no_mangle]
pub extern "C" fn validate_game_id(ptr: *const u8, len: usize) -> *const u8 {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let game_id = std::str::from_utf8(bytes).unwrap_or("");

    let valid = game_id.len() >= 3
        && game_id.len() <= 64
        && game_id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');

    let result: &[u8] = if valid {
        b"{\"ok\": true}\0"
    } else {
        b"{\"ok\": false, \"error\": \"Game-ID muss 3-64 Zeichen: a-z, 0-9, _, -\"}\0"
    };
    result.as_ptr()
}

/// Validiert eine Spiele-Registrierung.
#[no_mangle]
pub extern "C" fn validate_game(json_ptr: *const u8, json_len: usize) -> *const u8 {
    let json = unsafe { std::slice::from_raw_parts(json_ptr, json_len) };
    let json_str = std::str::from_utf8(json).unwrap_or("{}");

    match serde_json::from_str::<GameRegistration>(json_str) {
        Ok(g) => {
            let mut errors: Vec<String> = Vec::new();
            if g.game_id.len() < 3 || g.game_id.len() > 64 {
                errors.push("Game-ID muss 3-64 Zeichen lang sein".into());
            }
            if g.name.len() < 2 || g.name.len() > 96 {
                errors.push("Name muss 2-96 Zeichen lang sein".into());
            }
            if g.owner_company.len() != 64 {
                errors.push("Ungültige Company-Wallet".into());
            }
            let result = if errors.is_empty() {
                format!("{{\"ok\": true, \"game_id\": \"{}\"}}\0", g.game_id)
            } else {
                format!("{{\"ok\": false, \"errors\": {}}}\0", serde_json::to_string(&errors).unwrap_or_default())
            };
            result.into_bytes().leak().as_ptr()
        }
        Err(e) => {
            let err = format!("{{\"ok\": false, \"error\": \"{}\"}}\0", e);
            err.into_bytes().leak().as_ptr()
        }
    }
}

// ─── Item-Trading ──────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct TradeOffer {
    pub offer_id: String,
    pub from_wallet: String,
    pub to_wallet: String,
    pub item_id: String,
    pub amount: String,
}

#[no_mangle]
pub extern "C" fn create_trade_offer(json_ptr: *const u8, json_len: usize) -> *const u8 {
    let json = unsafe { std::slice::from_raw_parts(json_ptr, json_len) };
    let json_str = std::str::from_utf8(json).unwrap_or("{}");

    match serde_json::from_str::<TradeOffer>(json_str) {
        Ok(offer) => {
            let resp = format!("{{\"ok\": true, \"offer_id\": \"{}\"}}\0", offer.offer_id);
            resp.into_bytes().leak().as_ptr()
        }
        Err(_) => b"{\"ok\": false, \"error\": \"invalid offer\"}\0".as_ptr(),
    }
}

// ─── Marktplatz ────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct MarketplaceItem {
    pub item_id: String,
    pub name: String,
    pub game_id: String,
    pub price: String,
    pub seller_wallet: String,
}

#[no_mangle]
pub extern "C" fn list_marketplace_items() -> *const u8 {
    let items: Vec<MarketplaceItem> = vec![];
    let json = serde_json::to_string(&items).unwrap_or_else(|_| "[]".into());
    format!("{json}\0").into_bytes().leak().as_ptr()
}

