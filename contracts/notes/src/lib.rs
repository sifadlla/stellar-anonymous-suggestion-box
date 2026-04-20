#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Suggestion {
    pub id: u64,
    pub content: String,
    pub timestamp: u64,
}

const SUGGESTION_DATA: Symbol = symbol_short!("SUGG_DATA");

#[contract]
pub struct AnonymousBox;

#[contractimpl]
impl AnonymousBox {
    // Mengambil semua saran yang masuk
    pub fn get_suggestions(env: Env) -> Vec<Suggestion> {
        env.storage().instance().get(&SUGGESTION_DATA).unwrap_or(Vec::new(&env))
    }

    // Mengirim saran baru secara anonim
    pub fn submit_suggestion(env: Env, content: String) -> String {
        let mut suggestions: Vec<Suggestion> = env.storage().instance().get(&SUGGESTION_DATA).unwrap_or(Vec::new(&env));
        
        let new_suggestion = Suggestion {
            id: env.prng().gen::<u64>(),
            content: content,
            timestamp: env.ledger().timestamp(), // Mencatat waktu saran
        };
        
        suggestions.push_back(new_suggestion);
        
        env.storage().instance().set(&SUGGESTION_DATA, &suggestions);
        
        String::from_str(&env, "Saran berhasil dikirim secara anonim")
    }

    // Menghapus saran (Bisa dibatasi untuk admin jika diperlukan)
    pub fn delete_suggestion(env: Env, id: u64) -> String {
        let mut suggestions: Vec<Suggestion> = env.storage().instance().get(&SUGGESTION_DATA).unwrap_or(Vec::new(&env));

        for i in 0..suggestions.len() {
            if suggestions.get(i).unwrap().id == id {
                suggestions.remove(i);
                env.storage().instance().set(&SUGGESTION_DATA, &suggestions);
                return String::from_str(&env, "Saran berhasil dihapus");
            }
        }

        String::from_str(&env, "Saran tidak ditemukan")
    }
}

mod test;