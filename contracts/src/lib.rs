#![cfg_attr(not(feature = "std"), no_std)]

mod campaign;
mod donation;
mod loyalty;

use ink_lang as ink;

#[ink::contract]
mod haqq_charity {
    use crate::campaign::Campaign;

    #[ink(storage)]
    pub struct HaqqCharity {
        campaigns: Vec<Campaign>,
        total_donations: Balance,
    }

    impl HaqqCharity {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                campaigns: Vec::new(),
                total_donations: 0,
            }
        }

        #[ink(message)]
        pub fn create_campaign(&mut self, name: String, target: Balance) {
            let campaign = Campaign::new(name, target);
            self.campaigns.push(campaign);
        }

        #[ink(message)]
        pub fn donate(&mut self, campaign_id: usize, amount: Balance) -> Result<(), String> {
            if let Some(campaign) = self.campaigns.get_mut(campaign_id) {
                campaign.donate(amount)?;
                self.total_donations += amount;
                Ok(())
            } else {
                Err("Campaign not found".to_string())
            }
        }

        #[ink(message)]
        pub fn list_campaigns(&self) -> Vec<Campaign> {
            self.campaigns.clone()
        }
    }
}
