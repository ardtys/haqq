use ink_prelude::collections::BTreeMap;

/// Struct to track loyalty points for a specific donor
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LoyaltyAccount {
    pub donor: AccountId,
    pub points: u64,
    pub donations_count: u64,
}

impl LoyaltyAccount {
    /// Creates a new loyalty account for a donor
    pub fn new(donor: AccountId) -> Self {
        Self {
            donor,
            points: 0,
            donations_count: 0,
        }
    }

    /// Updates loyalty points based on donation amount
    pub fn update_points(&mut self, amount: Balance) {
        let points_earned = (amount / 10) as u64; // Earn 1 point for every 10 units donated
        self.points += points_earned;
        self.donations_count += 1;
    }

    /// Retrieves the donor's loyalty points
    pub fn get_points(&self) -> u64 {
        self.points
    }
}

/// Manages loyalty accounts for multiple donors
#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct LoyaltyManager {
    accounts: BTreeMap<AccountId, LoyaltyAccount>,
}

impl LoyaltyManager {
    /// Initializes a new loyalty manager
    pub fn new() -> Self {
        Self {
            accounts: BTreeMap::new(),
        }
    }

    /// Adds or updates a donor's loyalty account
    pub fn add_or_update_account(&mut self, donor: AccountId, amount: Balance) {
        let account = self
            .accounts
            .entry(donor)
            .or_insert_with(|| LoyaltyAccount::new(donor));
        account.update_points(amount);
    }

    /// Retrieves a donor's loyalty account, if it exists
    pub fn get_account(&self, donor: &AccountId) -> Option<&LoyaltyAccount> {
        self.accounts.get(donor)
    }

    /// Lists all loyalty accounts
    pub fn list_accounts(&self) -> Vec<LoyaltyAccount> {
        self.accounts.values().cloned().collect()
    }
}
