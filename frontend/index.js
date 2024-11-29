import React from 'react';
import CampaignCard from '../components/CampaignCard';

export default function Home() {
    const campaigns = [
        { name: 'Home for Stray Cats', raised: 3547, target: 25000 },
        { name: 'Build a Mosque', raised: 1206, target: 10000 },
    ];

    return (
        <div>
            <h1>Haqq Wallet</h1>
            {campaigns.map((campaign, index) => (
                <CampaignCard key={index} campaign={campaign} />
            ))}
        </div>
    );
}
