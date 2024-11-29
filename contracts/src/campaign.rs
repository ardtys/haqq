#[derive(Clone, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Campaign {
    pub name: String,
    pub target: Balance,
    pub collected: Balance,
}

impl Campaign {
    pub fn new(name: String, target: Balance) -> Self {
        Self {
            name,
            target,
            collected: 0,
        }
    }

    pub fn donate(&mut self, amount: Balance) -> Result<(), String> {
        if self.collected + amount > self.target {
            Err("Donation exceeds target".to_string())
        } else {
            self.collected += amount;
            Ok(())
        }
    }
}
