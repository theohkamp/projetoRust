use projetorust::product::Product;
use projetorust::search::SearchEngine;

#[test]
fn test_search_by_single_word() {
    let mut engine = SearchEngine::new();

    let product = Product {
        id: 1,
        name: "Notebook Gamer".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Dell".to_string(),
    };

    engine.add_product(product);

    let result = engine.search("notebook");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, 1);
}

#[test]
fn test_search_by_category() {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Notebook Gamer".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Dell".to_string(),
    });

    let result = engine.search("eletronicos");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "Notebook Gamer");
}

#[test]
fn test_search_nonexistent_term() {
    let engine = SearchEngine::new();
    let result = engine.search("cadeira");

    assert!(result.is_empty());
}