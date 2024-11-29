const { ApiPromise, WsProvider } = require('@polkadot/api');

let api;

async function connect() {
    const provider = new WsProvider('wss://rpc.haqq.network');
    api = await ApiPromise.create({ provider });
}

async function createCampaign(name, target) {
    const tx = api.tx.haqqCharity.createCampaign(name, target);
    await tx.signAndSend('your-wallet-address');
}

module.exports = { connect, createCampaign };
