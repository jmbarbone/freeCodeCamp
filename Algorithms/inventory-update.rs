
// This implements inventory structs because I wanted to play around with them a bit
// This totally over engineers the problem

fn main() {
    let inventory_a = ProductList::from(vec![(21, "Bowling Ball"), (2, "Dirty Sock"), (1, "Hair Pin"), (5, "Microphone")]);
    let inventory_b = ProductList::from(vec![(2, "Hair Pin"), (3, "Half-Eaten Apple"), (67, "Bowling Ball"), (7, "Toothpaste")]);
    let inventory_n = ProductList::from(vec![Product::from((0, "_")); 0]);
    
    let res = update_inventory(&inventory_a, &inventory_b);
    let exp = ProductList::from(vec![(88, "Bowling Ball"), (2, "Dirty Sock"), (3, "Hair Pin"), (3, "Half-Eaten Apple"), (5, "Microphone"), (7, "Toothpaste")]);
    assert_eq!(res, exp);

    let res = update_inventory(&inventory_a, &inventory_n);
    assert_eq!(res, inventory_a.sort());

    let res = update_inventory(&inventory_n, &inventory_b);
    assert_eq!(res, inventory_b.sort());

    let old = ProductList::from(vec![(0, "Bowling Ball"), (0, "Dirty Sock"), (0, "Hair Pin"), (0, "Microphone")]);
    let new = ProductList::from(vec![(1, "Hair Pin"), (1, "Half-Eaten Apple"), (1, "Bowling Ball"), (1, "Toothpaste")]);
    let res = update_inventory(&old, &new);
    let exp = ProductList::from(vec![(1, "Bowling Ball"), (0, "Dirty Sock"), (1, "Hair Pin"), (1, "Half-Eaten Apple"), (0, "Microphone"), (1, "Toothpaste")]);
    assert_eq!(res, exp);
}
    
fn update_inventory(old: &ProductList, new: &ProductList) -> ProductList {
    // let mut inventory = ProductList::from(check_inventory(&old, &new));
    let mut inventory = check_inventory(&old, &new);
    // ensure that the inventory is sorted
    inventory = inventory.sort();
    inventory
}

#[allow(unused_variables, unused_mut)]
fn check_inventory(old: &ProductList, new: &ProductList) -> ProductList {
    let old_has_len = old.len() != 0;
    let new_has_len = new.len() != 0;
    let mut inventory_amounts: Vec<i32>;
    let mut inventory_names: Vec<String>;

    if old_has_len {
        if !new_has_len {
            println!("using old for inventory");
            return ProductList::from((old.get_amounts(), old.get_names()));
        }

        inventory_amounts = old.get_amounts();
        inventory_names = old.get_names();
    } else if new_has_len {
        println!("using new for inventory");
        return ProductList::from((new.get_amounts(), new.get_names()));
    } else {
        panic!("bad");
    };

    if old_has_len && new_has_len {
        // take a copy of the old product list
        let new_names = new.get_names();
        let new_amounts = new.get_amounts();
        
        for i in 0..old.len() {
            for j in 0..new.len() {
                if inventory_names[i].eq(&new_names[j]) {
                    // name is found, so we have to add the values together
                    // get the Product as a tuple
                    let prod = old.products.get(i).unwrap();
                    // add the new product amount to the old and replace the inventory element
                    println!("UPDATING {:?}", prod);
                    inventory_amounts[i] = prod.amount + new_amounts[j]
                }
            }
        }
        // okay, now I need to check for the ones that were missed
        let mut add_amounts = vec![0; 0];
        let mut add_names = vec![String::from(""); 0];

        for i in 0..new.len() {
            // if the new inventory name is not in the current inventory name, push a new object
            if !inventory_names.iter().any(|x| x.eq(&new_names[i])) {
                // println!("ADDING {:?}", (new_amounts[i], new_names[i].to_string()));
                add_amounts.push(new_amounts[i]);
                add_names.push(new_names[i].to_string());
            }
        }

        inventory_amounts.append(&mut add_amounts);
        inventory_names.append(&mut add_names);
    };

    let mut inventory = vec![Product::from((0, "_")); 0];
    for i in 0..inventory_amounts.len() {
        inventory.push(Product::from((inventory_amounts[i], inventory_names[i].to_string())));
    }
    ProductList::from(inventory)
}

// Create structs

// ProductList ========================

#[derive(Default, Clone, Debug, PartialEq)]  
struct ProductList {
    products: Vec<Product>,
}

impl ProductList {

    fn get_names(&self) -> Vec<String> {
        let n = self.len();
        let mut names = vec![String::from(""); n];
        for i in 0..n {
            names[i] = self.products.get(i).unwrap().name.to_string();
        }
        names
    }

    fn get_amounts(&self) -> Vec<i32> {
        let n = self.len();
        let mut amounts = vec![0; n];
        for i in 0..n {
            amounts[i] = self.products.get(i).unwrap().amount;
        }
        amounts
    }

    fn sort(mut self) -> Self {
        self.products.sort_by_key(|p| p.name.to_lowercase());
        ProductList::from(self)
    }

    fn len(&self) -> usize {
        self.products.len()
    }
}

impl From<(Vec<i32>, Vec<String>)> for ProductList {
    fn from(tuple: (Vec<i32>, Vec<String>)) -> Self {
        let n = tuple.0.len();
        let amounts = tuple.0;
        let names = tuple.1;
        let mut out = vec![Product::from((0, "_")); n];
        for i in 0..n {
            out[i] = Product::from((amounts[i], names[i].to_string()));
        }

        ProductList::from(out)
    }
}

impl From<Vec<(i32, &str)>> for ProductList {
    fn from(vector: Vec<(i32, &str)>) -> Self {
        let n = vector.len();
        // create empty vector for Product
        let mut res = vec![ Product::from((0, "_nothing_")); n];

        // For each element in the vector, extract the tuple
        for i in 0..n {
            res[i] = Product::from(vector.get(i));
        }

        ProductList { 
            products: res
        }
    }
}

impl From<Vec<Product>> for ProductList {
    fn from(vector: Vec<Product>) -> ProductList {
        ProductList {
            products: vector,
        }
    }
}

// Product =====================

#[derive(Default, Clone, Debug, PartialEq)]  
struct Product {
    amount: i32,
    name: String,
}

impl From<(i32, &str)> for Product {
    fn from(tuple: (i32, &str)) -> Product {
        Product {
            amount: tuple.0,
            name: tuple.1.to_string(),
        }
    }
}

impl From<(i32, String)> for Product {
    fn from(tuple: (i32, String)) -> Product {
        Product {
            amount: tuple.0,
            name: tuple.1,
        }
    }
}

impl From<Option<&(i32, &str)>> for Product {
    fn from(tuple: Option<&(i32, &str)>) -> Product {
        let good = match tuple {
            None => panic!("tuple cannot be None"),
            Some(tuple) => tuple,
        };

        Product {
            amount: good.0,
            name: good.1.to_string()
        }
    }
}

impl From<Option<&Product>> for Product {
    fn from(prod: Option<&Product>) -> Product {
        let good = match prod {
            None => panic!("prod cannot be None"),
            Some(prod) => prod,
        };

        Product {
            amount: good.amount,
            name: good.name.to_string()
        }
    }
}

