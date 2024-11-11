use crate::traits::*;

use indexmap::IndexMap;

pub struct Nested<T: EsEntity> {
    entities: IndexMap<<<T as EsEntity>::Event as EsEvent>::EntityId, T>,
    new_entities: Vec<<T as EsEntity>::New>,
}

impl<T: EsEntity> Default for Nested<T> {
    fn default() -> Self {
        Self {
            entities: IndexMap::new(),
            new_entities: Vec::new(),
        }
    }
}

impl<T: EsEntity> Nested<T> {
    pub fn add_new(&mut self, new: <T as EsEntity>::New) -> &<T as EsEntity>::New {
        let len = self.new_entities.len();
        self.new_entities.push(new);
        &self.new_entities[len + 1]
    }

    pub fn new_entities_mut(&mut self) -> &mut Vec<<T as EsEntity>::New> {
        &mut self.new_entities
    }

    pub fn entities_mut(
        &mut self,
    ) -> &mut IndexMap<<<T as EsEntity>::Event as EsEvent>::EntityId, T> {
        &mut self.entities
    }

    pub fn extend_entities(&mut self, entities: impl IntoIterator<Item = T>) {
        self.entities.extend(
            entities
                .into_iter()
                .map(|entity| (entity.events().entity_id.clone(), entity)),
        );
    }
}
