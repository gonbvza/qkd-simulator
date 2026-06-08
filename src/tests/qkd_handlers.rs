use std::sync::mpsc::{channel, Receiver};

use crate::{
    core::{event_loop::EventLoopHandler, state::SimulationState},
    error::{Error, NodeError},
    events::qkd::handle_qkd_init,
    models::{
        event_types::{EventPayload, HandleQkdInitPayload},
        links::Link,
        node::Node,
    },
};

fn make_link(id: i32, src_id: i32, dst_id: i32, length: i64) -> Link {
    Link {
        id,
        length,
        attenuation: 0.0,
        error_rate: 0.0,
        src_id,
        dst_id,
        next_available_time: 0,
        is_secure: true,
    }
}

fn make_node(id: i32, detector_id: i32, locked_by: Option<i32>) -> Node {
    Node {
        id,
        name: format!("node-{id}"),
        locked_by,
        node_type: "0".to_string(),
        detector_id,
    }
}

#[test]
fn handle_qkd_init_creates_process_locks_nodes_and_starts_pair_emission() {
    let mut state = SimulationState::new();
    state.upsert_node(make_node(1, 101, None));
    state.upsert_node(make_node(2, 102, None));
    state.upsert_link(make_link(201, 1, 99, 1000));
    state.upsert_link(make_link(202, 2, 99, 2000));

    let (tx, rx): (_, Receiver<_>) = channel();
    let handler = EventLoopHandler::new(tx);

    handle_qkd_init(
        EventPayload::HandleQkdInit(HandleQkdInitPayload::new(1, 2, 99, 201, 202)),
        123,
        &mut state,
        &handler,
    )
    .unwrap();

    {
        let (nodes, _, pairs, _) = state.split_nodes_links_pairs_detector_mut();
        assert_eq!(nodes.get(&1).unwrap().locked_by, Some(0));
        assert_eq!(nodes.get(&2).unwrap().locked_by, Some(0));

        let pair = pairs.get(&(0, 1)).expect("expected first emitted pair");
        assert_eq!(pair.src_id, 1);
        assert_eq!(pair.dst_id, 2);
        assert_eq!(pair.process_id, 0);
        assert_eq!(pair.qubit_nr, 1);
        assert_eq!(pair.created_at, 123);
    }

    {
        let (_, processes, _) = state.split_pairs_processes_nodes_mut();
        assert_eq!(processes.len(), 1);
        let process = processes.get(&0).expect("expected created process");
        assert_eq!(process.started_at, 123);
        assert_eq!(process.accepted_pairs, 0);
    }

    drop(rx);
}

#[test]
fn handle_qkd_init_aborts_when_one_node_is_busy() {
    let mut state = SimulationState::new();
    state.upsert_node(make_node(1, 101, Some(999)));
    state.upsert_node(make_node(2, 102, None));
    state.upsert_link(make_link(201, 1, 99, 1000));
    state.upsert_link(make_link(202, 2, 99, 2000));

    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    let result = handle_qkd_init(
        EventPayload::HandleQkdInit(HandleQkdInitPayload::new(1, 2, 99, 201, 202)),
        123,
        &mut state,
        &handler,
    );

    assert_eq!(result.unwrap_err(), Error::Node(NodeError::NodeInUse()));

    {
        let (nodes, _, pairs, _) = state.split_nodes_links_pairs_detector_mut();
        assert_eq!(nodes.get(&1).unwrap().locked_by, Some(999));
        assert_eq!(nodes.get(&2).unwrap().locked_by, None);
        assert!(pairs.is_empty());
    }

    assert!(rx.try_recv().is_err());
}
