//! Device connection system.
//!
//! Tracks logical and physical connections between devices (socketed,
/// cabled, bus-attached). The world owns the authoritative topology.

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConnectionType {
    Socket,   // e.g. CPU in motherboard socket
    Cable,
    Bus,
    PowerRail,
    Logical,
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub a: Entity,
    pub b: Entity,
    pub kind: ConnectionType,
}

#[derive(Resource, Default)]
pub struct ConnectionSystem {
    pub connections: Vec<Connection>,
    /// Quick lookup: entity → connected peers.
    pub adjacency: HashMap<Entity, Vec<(Entity, ConnectionType)>>,
}

impl ConnectionSystem {
    pub fn connect(&mut self, a: Entity, b: Entity, kind: ConnectionType) {
        self.connections.push(Connection { a, b, kind });
        self.adjacency.entry(a).or_default().push((b, kind));
        self.adjacency.entry(b).or_default().push((a, kind));
    }

    pub fn disconnect(&mut self, a: Entity, b: Entity) {
        self.connections.retain(|c| {
            !((c.a == a && c.b == b) || (c.a == b && c.b == a))
        });
        if let Some(list) = self.adjacency.get_mut(&a) {
            list.retain(|(e, _)| *e != b);
        }
        if let Some(list) = self.adjacency.get_mut(&b) {
            list.retain(|(e, _)| *e != a);
        }
    }

    pub fn peers(&self, entity: Entity) -> &[(Entity, ConnectionType)] {
        self.adjacency
            .get(&entity)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

#[derive(Event, Clone, Debug)]
pub struct ConnectionEvent {
    pub a: Entity,
    pub b: Entity,
    pub kind: ConnectionType,
    pub connected: bool,
}

/// Component: declares intended connections (resolved by the world).
#[derive(Component, Default)]
pub struct DeviceConnections {
    pub targets: Vec<(Entity, ConnectionType)>,
}

pub fn update_connections(
    mut system: ResMut<ConnectionSystem>,
    mut events: EventWriter<ConnectionEvent>,
    query: Query<(Entity, &DeviceConnections)>,
) {
    // Simple model: ensure declared connections are present.
    for (entity, conns) in query.iter() {
        for &(target, kind) in &conns.targets {
            let already = system
                .peers(entity)
                .iter()
                .any(|(e, k)| *e == target && *k == kind);
            if !already {
                system.connect(entity, target, kind);
                events.send(ConnectionEvent {
                    a: entity,
                    b: target,
                    kind,
                    connected: true,
                });
            }
        }
    }
}
