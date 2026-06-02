use std::sync::mpsc::{channel, Receiver};

use crate::{
    core::{
        event_loop::EventLoopHandler, pairs::emit_pair, process::Process, settings::QUBIT_AMOUNT,
        state::SimulationState,
    },
    events::qkd::receive_pair,
    models::{
        detector::Detector,
        entangled_pair::{NewEntangledPair, Side},
        event::{Event, EventTime},
        event_types::{EventName, EventPayload, ReceivePairPayload},
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

fn make_detector(id: i32, last_detection_time: i64) -> Detector {
    Detector {
        id,
        resolution_ps: 0,
        cooldown_ps: 0,
        dark_count_rate: 0,
        last_detection_time,
    }
}

fn recv_event(rx: &Receiver<Event>) -> Event {
    rx.recv().expect("expected event on channel")
}

fn assert_receive_pair_event(
    event: &Event,
    expected_node_id: i32,
    expected_side: Side,
    expected_qubit_nr: i32,
    expected_process_id: i32,
    expected_link_id: i32,
    expected_timestamp: i64,
) {
    assert_eq!(event.name, EventName::ReceivePair);
    match event.timestamp {
        EventTime::At(timestamp) => assert_eq!(timestamp, expected_timestamp),
        other => panic!("unexpected timestamp: {other:?}"),
    }

    match &event.payload {
        EventPayload::ReceivePair(payload) => {
            assert_eq!(payload.node_id, expected_node_id);
            assert_eq!(payload.side, expected_side);
            assert_eq!(payload.qubit_nr, expected_qubit_nr);
            assert_eq!(payload.process_id, expected_process_id);
            assert_eq!(payload.link_id, expected_link_id);
        }
        other => panic!("unexpected event payload: {other:?}"),
    }
}

fn assert_same_basis_event(event: &Event, expected_process_id: i32) {
    assert_eq!(event.name, EventName::PostProcess);
    assert!(matches!(event.timestamp, EventTime::Now));

    match &event.payload {
        EventPayload::PostProcess(payload) => {
            assert_eq!(payload.process_id, expected_process_id);
        }
        other => panic!("unexpected event payload: {other:?}"),
    }
}

#[test]
fn emit_pair_enqueues_receive_events_with_expected_payloads() {
    let (tx, rx) = channel();
    let handler = EventLoopHandler::new(tx);

    let src_link = make_link(10, 1, 99, 1000);
    let dst_link = make_link(11, 2, 99, 2000);

    let pair = emit_pair(
        1,
        2,
        src_link.clone(),
        dst_link.clone(),
        crate::core::state::PairKey {
            qubit_nr: 3,
            process_id: 7,
        },
        50,
        &handler,
    )
    .unwrap();

    assert_eq!(pair.src_id, 1);
    assert_eq!(pair.dst_id, 2);
    assert_eq!(pair.process_id, 7);
    assert_eq!(pair.qubit_nr, 3);
    assert_eq!(pair.created_at, 50);
    assert!(!pair.accepted);

    let expected_src_timestamp = 50 + (src_link.propagation_delay_us() * 3);
    let expected_dst_timestamp = 50 + (dst_link.propagation_delay_us() * 3);

    let src_event = recv_event(&rx);
    assert_receive_pair_event(
        &src_event,
        1,
        Side::Source,
        3,
        7,
        10,
        expected_src_timestamp,
    );

    let dst_event = recv_event(&rx);
    assert_receive_pair_event(
        &dst_event,
        2,
        Side::Destination,
        3,
        7,
        11,
        expected_dst_timestamp,
    );
}
