# Subscription Payment System (Soroban Smart Contract)

## 📌 Project Description
This project implements a basic Subscription Payment System using Soroban smart contracts on the Stellar blockchain. It allows users to subscribe to recurring payments and enables merchants to receive automated payments at fixed intervals.

---

## ⚙️ What it does
The smart contract allows:

- Users to create subscription plans with a merchant
- Automatic recurring payments based on time intervals
- Secure authorization using Stellar accounts
- Cancellation of subscriptions anytime

The system is designed to simulate platforms like Netflix, Spotify, or SaaS billing systems but in a decentralized way.

---

## 🚀 Features

### 1. Create Subscription
Users can initialize a subscription by specifying:
- Merchant address
- Payment amount
- Billing interval (in seconds)

### 2. Recurring Payments
- Payments can be triggered after the interval has passed
- Ensures no early payments are made
- Uses blockchain timestamps for accuracy

### 3. Secure Transactions
- Requires user authorization before payment
- Uses Soroban token interface for transfers

### 4. Cancel Subscription
- Users can stop future payments anytime
- Subscription status is stored on-chain

### 5. Fetch Subscription Details
- Retrieve subscription data for any user
- Includes amount, interval, last payment, and status

---

## 🔗 Deployed Smart Contract Link
[stellar expert](https://stellar.expert/explorer/testnet/contract/CAGGMUOEP7J2OLSMD4V77AETTCTBGPWHGCDVYJHQPSDNRGVN6XJCE3GM)


---

## 🛠 Tech Stack
- Stellar Soroban
- Rust
- soroban-sdk

---

## 📈 Future Improvements
- Multiple subscriptions per user
- Support for different tokens
- Auto-trigger using off-chain bots
- Grace periods and retry logic
- Subscription upgrades/downgrades

---

## 📜 License
MIT License
contract address CAGGMUOEP7J2OLSMD4V77AETTCTBGPWHGCDVYJHQPSDNRGVN6XJCE3GM
wallet address GDD46XYLNTSCJGJA3CDEN62DOA222QNTTUCPIPCAI24VF7EVVWR3A44H
[stellar expert](https://stellar.expert/explorer/testnet/contract/CAGGMUOEP7J2OLSMD4V77AETTCTBGPWHGCDVYJHQPSDNRGVN6XJCE3GM)
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/f2ba7454-a206-43eb-af52-6c81c875cd5b" />
