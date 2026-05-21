#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph::{Graph, GraphNode};
    use crate::error::{GraphError, LinkError};
    use crate::models::links::Link;
    use crate::models::node::{Node, NodeKind};

    fn make_client(id: i32) -> Node {
        Node {
            id,
            name: format!("client_{}", id),
            locked_by: None,
            node_type: NodeKind::ClientNode.to_string(),
            detector_id: 0,
        }
    }

    fn make_epr(id: i32) -> Node {
        Node {
            id,
            name: format!("epr_{}", id),
            locked_by: None,
            node_type: NodeKind::EprNode.to_string(),
            detector_id: 0,
        }
    }

    fn make_link(id: i32, src_id: i32, dst_id: i32) -> Link {
        Link {
            id,
            length: 100,
            error_rate: 0.2,
            attenuation: 0.2,
            src_id,
            dst_id,
            next_available_time: 2,
        }
    }

    fn base_graph() -> Graph {
        let nodes = vec![make_client(1), make_client(2), make_epr(3)];
        let links = vec![make_link(1, 1, 3), make_link(2, 2, 3)];
        Graph::from_data(nodes, links).unwrap()
    }

    #[test]
    fn test_from_data_empty() {
        let graph = Graph::from_data(vec![], vec![]).unwrap();
        assert!(graph.nodes.is_empty());
        assert!(graph.connections.is_empty());
    }

    #[test]
    fn test_from_data_nodes_are_inserted() {
        let nodes = vec![make_client(1), make_epr(2)];
        let graph = Graph::from_data(nodes, vec![]).unwrap();

        assert_eq!(graph.nodes.len(), 2);
        assert!(matches!(
            graph.nodes.get(&1),
            Some(GraphNode::ClientNode(1))
        ));
        assert!(matches!(graph.nodes.get(&2), Some(GraphNode::EprNode(2))));
    }

    #[test]
    fn test_from_data_connections_are_bidirectional() {
        let nodes = vec![make_client(1), make_epr(2), make_client(3)];
        let links = vec![make_link(1, 1, 2), make_link(2, 2, 3)];
        let graph = Graph::from_data(nodes, links).unwrap();

        assert!(graph.connections[&1].contains(&GraphNode::EprNode(2)));
        assert!(graph.connections[&2].contains(&GraphNode::ClientNode(1)));
        assert!(graph.connections[&2].contains(&GraphNode::ClientNode(3)));
        assert!(graph.connections[&3].contains(&GraphNode::EprNode(2)));
    }

    #[test]
    fn test_from_data_link_with_missing_node_returns_error() {
        let nodes = vec![make_client(1)];
        let links = vec![make_link(1, 1, 99)];
        let result = Graph::from_data(nodes, links);

        assert!(matches!(
            result,
            Err(GraphError::Link(LinkError::MissingNode(99)))
        ));
    }

    #[test]
    fn test_get_node_epr_finds_common_epr() {
        let graph = base_graph();
        assert_eq!(graph.get_node_epr(1, 2).unwrap(), 3);
    }

    #[test]
    fn test_get_node_epr_no_common_epr_returns_error() {
        let nodes = vec![make_client(1), make_client(2), make_epr(3), make_epr(4)];
        let links = vec![make_link(1, 1, 3), make_link(2, 2, 4)];
        let graph = Graph::from_data(nodes, links).unwrap();

        assert!(matches!(
            graph.get_node_epr(1, 2),
            Err(GraphError::NoCommonEpr(1, 2))
        ));
    }

    #[test]
    fn test_get_node_epr_returns_lowest_id_when_multiple_shared() {
        let nodes = vec![make_client(1), make_client(2), make_epr(3), make_epr(4)];
        let links = vec![
            make_link(1, 1, 3),
            make_link(2, 2, 3),
            make_link(3, 1, 4),
            make_link(4, 2, 4),
        ];
        let graph = Graph::from_data(nodes, links).unwrap();

        assert_eq!(graph.get_node_epr(1, 2).unwrap(), 3);
    }

    #[test]
    fn test_get_node_epr_missing_node_returns_error() {
        let graph = base_graph();

        assert!(matches!(
            graph.get_node_epr(1, 99),
            Err(GraphError::Link(LinkError::MissingNode(99)))
        ));
    }

    #[test]
    fn test_get_node_epr_does_not_return_client_node() {
        let nodes = vec![make_client(1), make_client(2), make_client(3)];
        let links = vec![make_link(1, 1, 3), make_link(2, 2, 3)];
        let graph = Graph::from_data(nodes, links).unwrap();

        assert!(matches!(
            graph.get_node_epr(1, 2),
            Err(GraphError::NoCommonEpr(1, 2))
        ));
    }
}
