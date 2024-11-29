import React from 'react';

export default function CampaignCard({ campaign }) {
    return (
        <div>
            <h2>{campaign.name}</h2>
            <p>
                Raised: ${campaign.raised} of ${campaign.target}
            </p>
        </div>
    );
}
