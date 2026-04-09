# 🚀 DApp: **NebulaStore (Permissionless Cloud Storage on Stellar)**

---
<img width="1903" height="856" alt="image" src="https://github.com/user-attachments/assets/b0804e8c-acc8-4eab-99a7-351088ae8912" />
http://stellar.expert/explorer/testnet/contract/CAY3F54DJOLPCR3AXVSZZG5S73SAAZVLGYTTEHNZ4QPSQKENDN6GSJQ2

## 🔥 Core Philosophy

* Anyone can upload
* Anyone can access (unless encrypted by user)
* No “owner-only” functions
* No centralized control
* Privacy is **user-controlled via encryption**, not contract permissions

---

# 🧠 Architecture Overview

### ⚠️ Important Insight

Stellar (even with Soroban) is **not suitable for storing large files directly**.

So we use:

* **On-chain (Stellar/Soroban):**

  * File metadata
  * Hash (CID)
  * Timestamp
  * Optional payment / escrow logic

* **Off-chain (Decentralized storage):**

  * IPFS / Arweave

---

# 🏗️ Smart Contract Design (Soroban)

### 📦 Data Structure

```rust
struct FileData {
    uploader: Address,
    cid: Bytes,        // IPFS CID
    name: Bytes,
    timestamp: u64,
    encrypted: bool,
}
```

---

## ⚙️ Functions (Fully Permissionless)

### 1. Upload File Metadata

```rust
fn upload(env: Env, cid: Bytes, name: Bytes, encrypted: bool)
```

✅ Anyone can call
✅ No restrictions
✅ Stores metadata on-chain

---

### 2. Get File

```rust
fn get_file(env: Env, cid: Bytes) -> FileData
```

✅ Public read
✅ No permission needed

---

### 3. List Files (Optional pagination)

```rust
fn list_files(env: Env) -> Vec<FileData>
```

---

### ❌ What we DO NOT include (as per your rule)

* ❌ delete_file (ownership = permission)
* ❌ edit_file
* ❌ admin controls
* ❌ allow/deny access lists

---

### 🔐 Privacy Model (User-side)

* If user wants privacy:

  * Encrypt file before uploading
  * Share key off-chain

👉 Smart contract stays permissionless
👉 Privacy handled cryptographically

---

# 🌐 Storage Flow

1. User uploads file → IPFS
2. Gets CID (Content Identifier)
3. Calls smart contract → stores CID + metadata
4. Anyone can retrieve CID → fetch file from IPFS

---

# 🎨 Frontend Design

## 🧭 Navbar

**Logo:** `NebulaStore`

* Home
* Upload
* Explore
* My Files (client-side filtered, not permission-based)

---

## 🏠 Hero Section

**Headline:**

> "Store Anything. Own Nothing. Control Everything."

**Subtext:**

> A fully permissionless cloud where your data lives beyond control — powered by Stellar & IPFS.

**CTA Buttons:**

* Upload File
* Explore Files

---

# 📄 Pages

### 1. Upload Page

* File picker
* Encrypt toggle 🔐
* Upload to IPFS
* Submit CID to contract

---

### 2. Explore Page

* Public file feed
* Search by CID / name
* Download button

---

### 3. My Files (Frontend Filter)

* Filter by wallet address
* No contract restriction

---

# 🔌 Integration (Step-by-Step)

## 1. Upload to IPFS

Use:

* `web3.storage` or `nft.storage`

```js
const cid = await client.put([file]);
```

---

## 2. Call Soroban Contract

Using Stellar SDK:

```js
await contract.call(
  "upload",
  cid,
  fileName,
  encrypted
);
```

---

## 3. Wallet Connection

Use:

* Freighter Wallet

```js
import { getUserInfo } from "@stellar/freighter-api";
```

---

## 4. Fetch Data

```js
const files = await contract.call("list_files");
```

---

# 🔐 Optional Feature (User-Triggered Permission)

## 💰 Paid Storage (Escrow-based)

ONLY if user wants:

```rust
fn upload_paid(env: Env, cid: Bytes, payment: i128)
```

* User attaches payment
* No admin involved
* Fully trustless

---

# 🛡️ Security Considerations

* No central failure point
* Data integrity via CID (hash)
* Privacy via encryption
* Spam risk → can be reduced via optional upload fee

---

# ⚡ Hackathon Pitch (Short)

> NebulaStore is a fully permissionless decentralized cloud storage platform built on Stellar, where anyone can upload, access, and share data without restrictions. It combines IPFS for storage and Soroban for trustless metadata, ensuring censorship resistance and user-controlled privacy.

---

# 💡 Why This is Strong

* Aligns with **true Web3 ethos**
* No hidden permissions
* Scalable (off-chain storage)
* Easy to demo
* Judges LOVE “permissionless + real-world use”

---
