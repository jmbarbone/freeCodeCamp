
// This implements inventory structs because I wanted to play around with them a bit
// This totally over engineers the problem

fn main() {
    let mut x = ProductList::from(vec![(0, "_")]);
    x.sort();
    println!("{:?}", x.len())
}

fn update_inventory(old: ProductList, new: ProductList) -> ProductList {
    let inventory = check_inventory(&old, &new);
    ProductList::from(inventory)
}

fn check_inventory(old: &ProductList, new: &ProductList) -> ProductList {
    let old_no_len = old.len() == 0;
    let new_no_len = new.len() == 0;
    let mut inventory = vec![Product{ amount: 0, name: "_".to_string() }; 0];

    if old_no_len {
        if new_no_len {
            panic!("old or new should have length");
        }
        let mut inventory = &new.products;
    } else {
        let mut inventory = &old.products;
    }

    if !old_no_len && !new_no_len {
        // take a copy of the old product list
        let new_names = new.get_names();
        let old_names = old.get_names();
        let new_amounts = new.get_amounts();
        // let old_amounts = old.get_amounts();
        // grab the products vector and use that to append
        
        for i in 0..old.len() {
            // old_prod_names.iter().any(|n| n.eq(&new_prod_names[i])) 
            for j in 0..new.len() {
                // get the Product as a tuple
                let prod = old.products.get(i).unwrap();
                if old_names[i].eq(&new_names[j]) {
                    // name is found, so we have to add the values together
                    inventory[i] = Product::from((prod.amount + new_amounts[j], old_names[i].to_string()));
                } else {
                    // value not found, so we can just append
                    inventory.push(Product::from((prod.amount, prod.name.to_string())));
                }
            }
        }
    }

    ProductList::from(inventory)
}


// Create structs

// ProductList ========================

#[derive(Clone, Debug)]  
struct ProductList {
    products: Vec<Product>,
}

impl ProductList {
    fn new(products: Vec<Product>) -> ProductList {
        ProductList { products: products }
    }

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

    fn sort(&mut self) -> &Self {
        self.products.sort_by_key(|p| p.name.to_lowercase());
        self
    }

    fn len(&self) -> usize {
        self.products.len()
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

#[derive(Default, Clone, Debug)]  
struct Product {
    amount: i32,
    name: String,
}

impl Product {
    fn new(amount: i32, name: String) -> Product {
        if amount < 0 {
            panic!("Amount cannot be < 0")
        };
        Product {
            amount: amount,
            name: name.to_string(),
        }
    }

    // fn update(self, amount: &i32) -> Self {
    //     if amount < 0 {
    //         panic!("amount cannot be < 0");
    //     }

    //     self.amount = self.amount + amount;
    //     self
    // }
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

// R      92.63
// Python  4.97
// Rust    1.58
