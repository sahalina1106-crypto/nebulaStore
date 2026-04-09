#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Bytes, Address, Vec};

#[contract]
pub struct NebulaStore;

#[contracttype]
#[derive(Clone)]
pub struct FileData {
    pub uploader: Address,
    pub cid: Bytes,
}

#[contracttype]
pub enum DataKey {
    File(Bytes),
    List,
}

#[contractimpl]
impl NebulaStore {

    // 📤 Upload (permissionless)
    pub fn upload(env: Env, uploader: Address, cid: Bytes) {
        let file = FileData {
            uploader: uploader.clone(),
            cid: cid.clone(),
        };

        // store file
        env.storage().instance().set(&DataKey::File(cid.clone()), &file);

        // store list
        let mut list: Vec<Bytes> = env
            .storage()
            .instance()
            .get(&DataKey::List)
            .unwrap_or(Vec::new(&env));

        list.push_back(cid);

        env.storage().instance().set(&DataKey::List, &list);
    }

    // 📥 Get file
    pub fn get(env: Env, cid: Bytes) -> Option<FileData> {
        env.storage().instance().get(&DataKey::File(cid))
    }

    // 📜 List all files
    pub fn list(env: Env) -> Vec<Bytes> {
        env.storage()
            .instance()
            .get(&DataKey::List)
            .unwrap_or(Vec::new(&env))
    }
}