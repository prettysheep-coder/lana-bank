mod user_entity;

use user_entity::*;

#[test]
pub fn check_compile() {
    let new_user = NewUser {
        id: UserId::new(),
        email: "email".to_string(),
    };

    let user = User::try_from_events(new_user.into_events()).unwrap();

    assert!(accepts_mutable_entity(&user));
    assert!(accepts_mutable_entity(user));
}

fn accepts_mutable_entity(user: impl IntoMutableEntity<Entity = User>) -> bool {
    user.as_ref().email().len() > 0
}
