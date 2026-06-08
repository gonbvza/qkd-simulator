use crate::{
    core::{event_loop::EventLoopHandler, state::PairKey},
    error::Error,
    models::{
        entangled_pair::{NewEntangledPair, Side},
        event::Event,
        event_types::{EventName, EventPayload, PairTimeoutPayload, ReceivePairPayload},
        links::Link,
    },
};
use std::cmp::max;

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
    schedule_pair_event(
        src_node_id,
        Side::Source,
        &src_epr_link,
        &pair_key,
        current_time,
        handle,
    )?;
    schedule_pair_event(
        dst_node_id,
        Side::Destination,
        &dst_epr_link,
        &pair_key,
        current_time,
        handle,
    )?;

    // Schedule timeout
    schedule_pair_timeout(&src_epr_link, &dst_epr_link, pair_key, current_time, handle)?;

    Ok(entangled_pair)
}

/// Schedules [`EventName::ReceivePair`] events for both the source and destination nodes.
///
/// Computes per-qubit timestamps from the link propagation delay.
pub fn schedule_pair_event(
    node_id: i32,
    side: Side,
    epr_link: &Link,
    pair_key: &PairKey,
    current_time: i64,
    handle: &EventLoopHandler,
) -> Result<i64, Error> {
    let timestamp = current_time + (epr_link.propagation_delay_us() * pair_key.qubit_nr as i64);
    handle.push_event(Event::new_at(
        EventName::ReceivePair,
        EventPayload::ReceivePair(ReceivePairPayload::new(
            node_id,
            side,
            pair_key.qubit_nr,
            pair_key.process_id,
            epr_link.id,
        )),
        timestamp,
    ))?;
    Ok(timestamp)
}

/// Schedules a [`EventName::PairTimeout`] event based on the longer of the two link propagation delays.
pub fn schedule_pair_timeout(
    src_epr_link: &Link,
    dst_epr_link: &Link,
    pair_key: PairKey,
    // Timestamp of the later pair to arrive
    higher_timestamp: i64,
    handle: &EventLoopHandler,
) -> Result<(), Error> {
    let timeout_timestamp = higher_timestamp
        + max(
            src_epr_link.propagation_delay_us(),
            dst_epr_link.propagation_delay_us(),
        );
    handle.push_event(Event::new_at(
        EventName::PairTimeout,
        EventPayload::PairTimeout(PairTimeoutPayload::new(
            pair_key,
            src_epr_link.id,
            dst_epr_link.id,
        )),
        timeout_timestamp,
    ))?;
    Ok(())
}
