use es_entity_derive::EsEntityRepository;

#[derive(EsEntityRepository)]
#[es_repo(indexes(id))]
pub struct TestEntityRepo {}

#[test]
fn test() {
    // assert_eq!(TestEntityRepo::create_in_tx(), "TestEntity");
    // assert_eq!(TestEntityRepo::indexes(), ["id"]);
}

// NewEntity
// EntityEvent
// EntityId
// Entity
// Repo
//
// Load
