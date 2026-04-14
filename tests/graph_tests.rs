use projetorust::graph::Graph;

#[test]
fn test_graph_bfs_finds_connected_nodes() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2);
    graph.add_edge(1, 3);

    let result = graph.bfs(1);

    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

#[test]
fn test_graph_avoids_duplicate_edges() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2);
    graph.add_edge(1, 2);

    let result = graph.bfs(1);

    let count_2 = result.iter().filter(|&&x| x == 2).count();
    assert_eq!(count_2, 1);
}