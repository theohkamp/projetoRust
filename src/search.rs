use crate::graph::Graph;
use crate::index::Index;
use crate::product::Product;

pub struct SearchEngine {
    pub products: Vec<Product>,
    pub index: Index,
    pub graph: Graph,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            products: Vec::new(),
            index: Index::new(),
            graph: Graph::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let product_id = product.id;

        for word in product.name.split_whitespace() {
            self.index.add(word, product_id);
        }

        self.index.add(&product.name, product_id);
        self.index.add(&product.category, product_id);
        self.index.add(&product.brand, product_id);

        self.products.push(product);
    }

    pub fn search(&self, term: &str) -> Vec<&Product> {
        let ids = self.index.search(term);

        self.products
            .iter()
            .filter(|product| ids.contains(&product.id))
            .collect()
    }

    pub fn recommend(&self, product_id: u32) -> Vec<u32> {
        self.graph.bfs(product_id)
    }
}