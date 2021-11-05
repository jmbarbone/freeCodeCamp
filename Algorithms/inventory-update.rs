
// This implements inventory structs because I wanted to play around with them a bit
// This totally over engineers the problem

fn main() {
    let mut x = ProductList::from(vec![(0, "_")]);
    x.sort();
    println!("{:?}", x.len())
}

fn update_inventory(old: ProductList, new: ProductList) -> ProductList {
    // if old.len() == 0:
    //     return new
    // ProductList { products: vec![(1, "Nothing")] }
    let inventory = check_inventory(old, new);
    ProductList::from(vec![(0, "_")])
}

fn check_inventory(old: ProductList, new: ProductList) -> ProductList {
    if old.len() == 0 {
        return new;
    } else if new.len() == 0 {
        return old;
    } else {
        // take a copy of the old product list
        let mut inventory = old.clone();
        let new_prod_names = new.get_names();
        let old_prod_names = old.get_names();
        
        for i in 0..new.len() {
            if old_prod_names.iter().any(|n| n.eq(&new_prod_names[i])) {

            }
        }
    }

    old
}


// Create structs

// ProductList ========================

#[derive(Clone, Debug)]  
struct ProductList {
    products: Vec<Product>
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

// Product =====================

#[derive(Default, Clone, Debug)]  
struct Product {
    amount: i32,
    name: String
}

impl Product {
    fn new(amount: i32, name: String) -> Product {
        if amount < 0 {
            panic!("Amount cannot be < 0")
        };
        Product {
            amount: amount,
            name: name.to_string()
        }
    }
}

impl From<(i32, &str)> for Product {
    fn from(tuple: (i32, &str)) -> Product {
        Product {
            amount: tuple.0,
            name: tuple.1.to_string()
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
