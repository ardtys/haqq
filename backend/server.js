const express = require('express');
const bodyParser = require('body-parser');
const campaignRoutes = require('./routes/campaign');
const donationRoutes = require('./routes/donation');

const app = express();
app.use(bodyParser.json());

// Campaign routes
app.use('/campaigns', campaignRoutes);

// Donation routes
app.use('/donations', donationRoutes);

// Start the server
const PORT = 3001;
app.listen(PORT, () => {
    console.log(`Backend running on port ${PORT}`);
});
