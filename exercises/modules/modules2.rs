mod delicious_snacks {
    // Fix the use statements
    use self::fruits::{PEAR};
    use self::veggies::{CUCUMBER};

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    // Bring the constants into scope with new names
    pub(crate) use self::fruits::{PEAR as fruit};
    pub(crate) use self::veggies::{CUCUMBER as veggie};
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
