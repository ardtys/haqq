#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Donation {
    pub donor: AccountId,
    pub amount: Balance,
    pub timestamp: u64,
}

impl Donation {
    /// Creates a new donation instance
    pub fn new(donor: AccountId, amount: Balance, timestamp: u64) -> Self {
        Self {
            donor,
            amount,
            timestamp,
        }
    }
}

/// A struct to hold all donations for a campaign
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct DonationManager {
    pub donations: Vec<Donation>,
    pub total_donated: Balance,
}

impl DonationManager {
    /// Initializes a new DonationManager
    pub fn new() -> Self {
        Self {
            donations: Vec::new(),
            total_donated: 0,
        }
    }

    /// Adds a new donation to the manager
    pub fn add_donation(
        &mut self,
        donor: AccountId,
        amount: Balance,
        timestamp: u64,
    ) -> Result<(), String> {
        if amount <= 0 {
            return Err("Donation amount must be greater than zero".to_string());
        }

        self.donations.push(Donation::new(donor, amount, timestamp));
        self.total_donated += amount;
        Ok(())
    }

    /// Retrieves all donations for the campaign
    pub fn get_donations(&self) -> Vec<Donation> {
        self.donations.clone()
    }

    /// Returns the total donated amount for the campaign
    pub fn get_total_donated(&self) -> Balance {
        self.total_donated
    }
}
