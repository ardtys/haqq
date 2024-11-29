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

// Route: Create a new campaign
router.post('/create', async (req, res) => {
    const { name, target } = req.body;

    if (!name || !target) {
        return res.status(400).send({ error: 'Missing campaign name or target.' });
    }

    try {
        const tx = api.tx.haqqCharity.createCampaign(name, BigInt(target));
        await tx.signAndSend('your-wallet-address'); // Replace with the wallet address
        res.status(200).send({ success: true, message: 'Campaign created successfully.' });
    } catch (err) {
        console.error(err);
        res.status(500).send({ error: 'Failed to create campaign.' });
    }
});

// Route: Get all campaigns
router.get('/list', async (req, res) => {
    try {
        const campaigns = await api.query.haqqCharity.campaigns.entries();
        const result = campaigns.map(([key, value]) => {
            const { name, target, collected } = value.toHuman();
            return { name, target, collected };
        });
        res.status(200).send(result);
    } catch (err) {
        console.error(err);
        res.status(500).send({ error: 'Failed to fetch campaigns.' });
    }
});

module.exports = router;
