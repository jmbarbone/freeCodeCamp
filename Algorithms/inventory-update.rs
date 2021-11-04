
// This implements inventory structs because I wanted to play around with them a bit

fn main() {

}

fn updateInventory(old: InventoryList, new: InventoryList) {
    if old.len() == 0:
        return new


}

fn checkInventory(old: InventoryList, new: InventoryList) -> InventoryList {
    if old.len() == 0 {
        new;
    } else if new.len() == 0 {
        old
    } else {
        for i in 0..old.len() {

        }
    }


}

struct InventoryList {

}

struct Product {
    amount: u32,
    name: String
}

impl Product {
    fn new((amount, name): (u32, String)) -> Product {
        Product {
            amount: n,
            name: nm.to_string()
        }
    }
}



fn updateInventory(new_inventory: )

enum Animal {
    // without syntax sugar
    Cat(i32),
    // desugars to `Dog(())` (empty tuple/unit)
    Dog,
    // desugars to `Horse((i32, bool))` (tuple)
    Horse(i32, bool),
    // desugars to `Eagle(GeneratedEagleType)` and a struct definition outside
    // of this enum `struct GeneratedEagleType { weight: i32, male: bool }`
    Eagle { weight: i32, male: bool }
}
