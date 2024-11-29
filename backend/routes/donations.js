const express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');

const router = express.Router();

let api;

// Initialize the Polkadot API connection
async function connectBlockchain() {
    const provider = new WsProvider('wss://rpc.haqq.network');
    api = await ApiPromise.create({ provider });
}
connectBlockchain();

// Route: Make a donation
router.post('/donate', async (req, res) => {
    const { campaignId, amount } = req.body;

    if (campaignId === undefined || amount === undefined) {
        return res.status(400).send({ error: 'Missing campaign ID or donation amount.' });
    }

    try {
        const tx = api.tx.haqqCharity.donate(campaignId, BigInt(amount));
        await tx.signAndSend('your-wallet-address'); // Replace with the wallet address
        res.status(200).send({ success: true, message: 'Donation successful.' });
    } catch (err) {
        console.error(err);
        res.status(500).send({ error: 'Failed to process donation.' });
    }
});

// Route: Get total donations
router.get('/total', async (req, res) => {
    try {
        const total = await api.query.haqqCharity.totalDonations();
        res.status(200).send({ total: total.toHuman() });
    } catch (err) {
        console.error(err);
        res.status(500).send({ error: 'Failed to fetch total donations.' });
    }
});

module.exports = router;
