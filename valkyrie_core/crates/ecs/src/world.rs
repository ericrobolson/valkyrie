use crate::{
    component_store::StoreableComponent, entity::MAX_ENTITIES, ComponentStore, Entity,
    EntityManager,
};

pub enum WorldType {
    Client,
    Server,
}

#[macro_export]
macro_rules! define_world {
    {
        id: $world:ident,
        server_authoritative: [$($server_component:ident : $server_component_type:ty = 300),*],
        client_authoritative: [$($client_component:ident : $client_component_type:ty),*],
        not_networked: [$($unnetworked_component:ident : $unnetworked_component_type:ty),*],
        systems: [$($system_execution:ident),*]
    } => {
        pub struct $world {
            world_type: WorldType,
            $($server_component : ComponentStore<$server_component_type>,)*
            $($client_component : ComponentStore<$client_component_type>,)*
            $($unnetworked_component : ComponentStore<$unnetworked_component_type>,)*
        }

        impl $world {
            /// Instantiates a new world
            pub fn new(world_type: WorldType) -> Self {
                todo!()
            }

            /// Executes all relevant network code
            fn network(&mut self) {
                // TODO: the various networking fun

                match self.world_type{
                    WorldType::Client => {},
                    WorldType::Server => {},
                }
            }

            /// Execute all systems for the world.
            pub fn dispatch(&mut self) {
                // TODO: system calls
                $($system_execution(&mut self));*

                // Finally execute the networking.
                self.network();
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::component_store::StoreableComponent;

    use super::*;

    define_world! {
        id: World,
        server_authoritative: [alive:bool = 300],
        client_authoritative: [],
        not_networked: [],
        systems: [test]
    }
}

// TODO: this probably should be in a module...

impl StoreableComponent for bool {
    fn serialize(&self) -> &[u8] {
        todo!()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, crate::component_store::NetworkableComponentErr> {
        todo!()
    }
}
