use crate::{
    core::{event_loop::EventLoopHandler, state::PairKey},
    error::Error,
    models::{
        entangled_pair::{NewEntangledPair, Side},
        event::Event,
        event_types::{EventName, EventPayload, ReceivePairPayload},
        links::Link,
    },
};

/// Creates one entangled pair and schedules its transmission to both client nodes.
///
/// Computes per-qubit timestamps from the link propagation delay and schedules
/// two [`EventName::ReceivePair`] events, one toward the source node and one
/// toward the destination node.
pub fn emit_pair(
    src_node_id: i32,
    dst_node_id: i32,
    src_epr_link: Link,
    dst_epr_link: Link,
    pair_key: PairKey,
    current_time: i64,
    handle: &EventLoopHandler,
) -> Result<NewEntangledPair, Error> {
    // Create entangled pair
    let entangled_pair = NewEntangledPair::new(
        src_node_id,
        dst_node_id,
        pair_key.process_id,
        pair_key.qubit_nr,
        false,
        current_time,
    )?;
    let src_detector_payload: ReceivePairPayload = ReceivePairPayload::new(
        src_node_id,
        Side::Source,
        pair_key.qubit_nr,
        pair_key.process_id,
        src_epr_link.id,
    );

    let dst_detector_payload: ReceivePairPayload = ReceivePairPayload::new(
        dst_node_id,
        Side::Destination,
        pair_key.qubit_nr,
        pair_key.process_id,
        dst_epr_link.id,
    );

    handle.push_event(Event::new_at(
        EventName::ReceivePair,
        EventPayload::ReceivePair(src_detector_payload),
        current_time + (src_epr_link.propagation_delay_us() * pair_key.qubit_nr as i64),
    ))?;
    handle.push_event(Event::new_at(
        EventName::ReceivePair,
        EventPayload::ReceivePair(dst_detector_payload),
        current_time + (dst_epr_link.propagation_delay_us() * pair_key.qubit_nr as i64),
    ))?;
    Ok(entangled_pair)
}
