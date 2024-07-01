// use derive_builder::Builder;
// use serde::{Deserialize, Serialize};

// use crate::{entity::*, primitives::*};

// use super::Terms;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "snake_case")]
// pub enum LoanEvent {
//     Initialized { id: LoanId, terms: Terms },
// }

// impl EntityEvent for LoanEvent {
//     type EntityId = UserId;
//     fn event_table_name() -> &'static str {
//         "loan_events"
//     }
// }

// #[derive(Builder)]
// #[builder(pattern = "owned", build_fn(error = "EntityError"))]
// pub struct Loan {
//     pub id: LoanId,
//     pub terms: Terms,
//     pub(super) events: EntityEvents<LoanEvent>,
// }

// impl Entity for Loan {
//     type Event = LoanEvent;
// }

// impl TryFrom<EntityEvents<UserEvent>> for Loan {
//     type Error = EntityError;

//     fn try_from(events: EntityEvents<UserEvent>) -> Result<Self, Self::Error> {
//         let mut builder = UserBuilder::default();
//         for event in events.iter() {
//             match event {
//                 UserEvent::Initialized {
//                     id,
//                     email,
//                     account_ids,
//                     account_addresses,
//                 } => {
//                     builder = builder
//                         .id(*id)
//                         .account_ids(*account_ids)
//                         .account_addresses(account_addresses.clone())
//                         .email(email.clone())
//                         .account_ids(*account_ids);
//                 }
//             }
//         }
//         builder.events(events).build()
//     }
// }

// #[derive(Debug, Builder)]
// pub struct NewUser {
//     #[builder(setter(into))]
//     pub(super) id: UserId,
//     #[builder(setter(into))]
//     pub(super) email: String,
//     pub(super) account_ids: UserLedgerAccountIds,
//     pub(super) account_addresses: UserLedgerAccountAddresses,
// }

// impl NewUser {
//     pub fn builder() -> NewUserBuilder {
//         NewUserBuilder::default()
//     }
// }
