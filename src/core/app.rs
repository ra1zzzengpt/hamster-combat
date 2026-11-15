pub struct HamsterApp {
    coins : u64,
    coins_per_click : u64,
}

impl HamsterApp {
    pub fn new(coins : u64, coins_per_click : u64) -> HamsterApp {
        HamsterApp{coins, coins_per_click}
    }
    
    pub fn tap(&mut self) {
        self.coins += self.coins_per_click;
    }
    
    pub fn coins(&self) -> u64 { self.coins }
    pub fn per_click(&self) -> u64 { self.coins_per_click }
    
    pub fn buy(&mut self, amount:u64) {
        self.coins -= amount;
    }
    
    pub fn upgrade_per_click(&mut self) {
        self.coins_per_click += 1;
    }
}