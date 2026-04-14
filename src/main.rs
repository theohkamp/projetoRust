use projetorust::product::Product;
use projetorust::search::SearchEngine;

fn main() {
    let mut engine = SearchEngine::new();

    let p1 = Product {
        id: 1,
        name: "Notebook Gamer".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Dell".to_string(),
    };

    let p2 = Product {
        id: 2,
        name: "Mouse Gamer".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Logitech".to_string(),
    };

    let p3 = Product {
        id: 3,
        name: "Teclado Mecanico".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Redragon".to_string(),
    };

    engine.add_product(p1);
    engine.add_product(p2);
    engine.add_product(p3);

    engine.graph.add_edge(1, 2);
    engine.graph.add_edge(1, 3);

    let results = engine.search("notebook");

    println!("Resultados:");
    for product in results {
        println!("{:?}", product);
    }

    let recommendations = engine.recommend(1);
    println!("Recomendações: {:?}", recommendations);
}