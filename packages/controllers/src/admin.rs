use std::fmt::Debug;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr, Api, CustomQuery, DepsMut, MessageInfo, Response, StdError, StdResult, Storage,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use thiserror::Error;

/// Returned from Admin.query()
#[cw_serde]
pub struct AdminResponse {
    pub admin: Option<String>,
    pub proposed: Option<String>,
}

/// Errors returned from Admin state transitions
#[derive(Error, Debug, PartialEq)]
pub enum AdminError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Caller is not admin")]
    NotAdmin {},

    #[error("Caller is not the proposed admin")]
    NotProposedAdmin {},

    #[error("Admin state transition was not valid")]
    StateTransitionError {},
}

type AdminResult<T> = Result<T, AdminError>;

/// The finite states that are possible
#[cw_serde]
enum AdminState {
    A(AdminUninitialized),
    B(AdminSetNoneProposed),
    C(AdminSetWithProposed),
    D(AdminRoleAbolished),
}

#[cw_serde]
struct AdminUninitialized;

impl AdminUninitialized {
    pub fn initialize(&self, admin: &Addr) -> AdminState {
        AdminState::B(AdminSetNoneProposed {
            admin: admin.clone(),
        })
    }

    pub fn abolish_admin_role(&self) -> AdminState {
        AdminState::D(AdminRoleAbolished)
    }
}

#[cw_serde]
struct AdminSetNoneProposed {
    admin: Addr,
}

impl AdminSetNoneProposed {
    pub fn propose(self, proposed: &Addr) -> AdminState {
        AdminState::C(AdminSetWithProposed {
            admin: self.admin,
            proposed: proposed.clone(),
        })
    }

    pub fn abolish_admin_role(self) -> AdminState {
        AdminState::D(AdminRoleAbolished)
    }
}

#[cw_serde]
struct AdminSetWithProposed {
    admin: Addr,
    proposed: Addr,
}

impl AdminSetWithProposed {
    pub fn clear_proposed(self) -> AdminState {
        AdminState::B(AdminSetNoneProposed { admin: self.admin })
    }

    pub fn accept_proposed(self) -> AdminState {
        AdminState::B(AdminSetNoneProposed {
            admin: self.proposed,
        })
    }

    pub fn abolish_admin_role(self) -> AdminState {
        AdminState::D(AdminRoleAbolished)
    }
}

#[cw_serde]
struct AdminRoleAbolished;

#[cw_serde]
pub enum AdminUpdate {
    /// Sets the initial admin when none. No restrictions permissions to modify.
    InitializeAdmin { admin: Addr },
    /// Proposes a new admin to take role. Only current admin can execute.
    ProposeNewAdmin { sender: Addr, proposed: Addr },
    /// Clears the currently proposed admin. Only current admin can execute.
    ClearProposed { sender: Addr },
    /// Promotes the proposed admin to be the current one. Only the proposed admin can execute.
    AcceptProposed { sender: Addr },
    /// Throws away the keys to the Admin role forever. Once done, no admin can ever be set later.
    /// Requires Admin permission except if event is dispatched from AdminUninitialized state.
    AbolishAdminRole { sender: Option<Addr> },
}

impl<'a> AdminUpdate {
    fn from(api: &'a dyn Api, sender: &Addr, update: AdminExecuteUpdate) -> StdResult<AdminUpdate> {
        Ok(match update {
            AdminExecuteUpdate::InitializeAdmin { admin } => {
                let validated = api.addr_validate(&admin)?;
                AdminUpdate::InitializeAdmin { admin: validated }
            }
            AdminExecuteUpdate::ProposeNewAdmin { proposed } => {
                let validated = api.addr_validate(&proposed)?;
                AdminUpdate::ProposeNewAdmin {
                    sender: sender.clone(),
                    proposed: validated,
                }
            }
            AdminExecuteUpdate::ClearProposed => AdminUpdate::ClearProposed {
                sender: sender.clone(),
            },
            AdminExecuteUpdate::AcceptProposed => AdminUpdate::AcceptProposed {
                sender: sender.clone(),
            },
            AdminExecuteUpdate::AbolishAdminRole => AdminUpdate::AbolishAdminRole {
                sender: Some(sender.clone()),
            },
        })
    }
}

/// Same as above, but used for execute helpers. Sender and inputs are validated.
#[cw_serde]
pub enum AdminExecuteUpdate {
    InitializeAdmin { admin: String },
    ProposeNewAdmin { proposed: String },
    ClearProposed,
    AcceptProposed,
    AbolishAdminRole,
}

/// A struct designed to help facilitate a two-step transition between contract admins safely.
/// It implements a finite state machine with dispatched events to manage state transitions.
/// State A: AdminUninitialized
///     - No restrictions on who can initialize the admin role
/// State B: AdminSetNoneProposed
///     - Once admin is set. Only they can execute the following updates:
///       - ProposeNewAdmin
///       - ClearProposed
/// State C: AdminSetWithProposed
///     - Only the proposed new admin can accept the new role via AcceptProposed {}
///     - The current admin can also clear the proposed new admin via ClearProposed {}
///
///```text
///                                                                  Clear Proposed
///                                                    +-------------------------------------^
///                                                    |                                     |
///                                                    v                                     |
/// +----------------+                      +----------------+                       +-------+--------+
/// | Admin: None    |   Initialize Admin   | Admin: Gabe    |   Propose New Admin   | Admin: Gabe    |
/// | Proposed: None +--------------------->| Proposed: None +---------------------->| Proposed: Joy  |
/// +-----+----------+                      ++---------------+                       +-------+----+---+
///       |                                  | Admin: Joy                                    |    |
///       |                                  | Proposed: None                                |    |
///   Abolish Role                           |      ^                                        |    |
///       |                *immutable        |      |              Accept Proposed           |    |
///       |            +----------------+    |      <----------------------------------------+    |
///       +----------->| Admin: None    |    |                                                    |
///                    | Proposed: None +----+------------------ Abolish Role --------------------+
///                    +----------------+
/// ```
pub struct Admin<'a>(Item<'a, AdminState>);

impl<'a> Admin<'a> {
    pub const fn new(namespace: &'a str) -> Self {
        Self(Item::new(namespace))
    }

    fn state(&self, storage: &'a dyn Storage) -> StdResult<AdminState> {
        Ok(self
            .0
            .may_load(storage)?
            .unwrap_or(AdminState::A(AdminUninitialized)))
    }

    //--------------------------------------------------------------------------------------------------
    // Queries
    //--------------------------------------------------------------------------------------------------
    pub fn current(&self, storage: &'a dyn Storage) -> StdResult<Option<Addr>> {
        Ok(match self.state(storage)? {
            AdminState::B(b) => Some(b.admin),
            AdminState::C(c) => Some(c.admin),
            _ => None,
        })
    }

    pub fn is_admin(&self, storage: &'a dyn Storage, addr: &Addr) -> StdResult<bool> {
        match self.current(storage)? {
            Some(admin) if &admin == addr => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn proposed(&self, storage: &'a dyn Storage) -> StdResult<Option<Addr>> {
        Ok(match self.state(storage)? {
            AdminState::C(c) => Some(c.proposed),
            _ => None,
        })
    }

    pub fn is_proposed(&self, storage: &'a dyn Storage, addr: &Addr) -> StdResult<bool> {
        match self.proposed(storage)? {
            Some(proposed) if &proposed == addr => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn query(&self, storage: &'a dyn Storage) -> StdResult<AdminResponse> {
        Ok(AdminResponse {
            admin: self.current(storage)?.map(Into::into),
            proposed: self.proposed(storage)?.map(Into::into),
        })
    }

    //--------------------------------------------------------------------------------------------------
    // Mutations
    //--------------------------------------------------------------------------------------------------
    /// Executes admin state transitions
    pub fn update(&self, storage: &'a mut dyn Storage, event: AdminUpdate) -> AdminResult<()> {
        let state = self.state(storage)?;

        let new_state = match (state, event) {
            (AdminState::A(a), AdminUpdate::InitializeAdmin { admin }) => a.initialize(&admin),
            (AdminState::A(a), AdminUpdate::AbolishAdminRole { .. }) => a.abolish_admin_role(),
            (AdminState::B(b), AdminUpdate::ProposeNewAdmin { sender, proposed }) => {
                self.assert_admin(storage, &sender)?;
                b.propose(&proposed)
            }
            (AdminState::B(b), AdminUpdate::AbolishAdminRole { sender }) => {
                let addr = sender.ok_or(AdminError::NotAdmin {})?;
                self.assert_admin(storage, &addr)?;
                b.abolish_admin_role()
            }
            (AdminState::C(c), AdminUpdate::AcceptProposed { sender }) => {
                self.assert_proposed(storage, &sender)?;
                c.accept_proposed()
            }
            (AdminState::C(c), AdminUpdate::ClearProposed { sender }) => {
                self.assert_admin(storage, &sender)?;
                c.clear_proposed()
            }
            (AdminState::C(c), AdminUpdate::AbolishAdminRole { sender }) => {
                let addr = sender.ok_or(AdminError::NotAdmin {})?;
                self.assert_admin(storage, &addr)?;
                c.abolish_admin_role()
            }
            (_, _) => return Err(AdminError::StateTransitionError {}),
        };
        self.0.save(storage, &new_state)?;
        Ok(())
    }

    /// Helper for composing execute responses
    pub fn execute_update<C, Q: CustomQuery>(
        &self,
        deps: DepsMut<Q>,
        info: MessageInfo,
        update: AdminExecuteUpdate,
    ) -> AdminResult<Response<C>>
    where
        C: Clone + Debug + PartialEq + JsonSchema,
    {
        let validated_update = AdminUpdate::from(deps.api, &info.sender, update)?;
        self.update(deps.storage, validated_update)?;
        let res = self.query(deps.storage)?;
        Ok(Response::new()
            .add_attribute("action", "update_admin")
            .add_attribute("admin", res.admin.unwrap_or_else(|| "None".to_string()))
            .add_attribute(
                "proposed",
                res.proposed.unwrap_or_else(|| "None".to_string()),
            )
            .add_attribute("sender", info.sender))
    }

    //--------------------------------------------------------------------------------------------------
    // Assertions
    //--------------------------------------------------------------------------------------------------
    /// Similar to is_admin() except it raises an exception if caller is not current admin
    pub fn assert_admin(&self, storage: &'a dyn Storage, caller: &Addr) -> AdminResult<()> {
        if !self.is_admin(storage, caller)? {
            Err(AdminError::NotAdmin {})
        } else {
            Ok(())
        }
    }

    /// Similar to is_proposed() except it raises an exception if caller is not currently proposed new admin
    pub fn assert_proposed(&self, storage: &'a dyn Storage, caller: &Addr) -> AdminResult<()> {
        if !self.is_proposed(storage, caller)? {
            Err(AdminError::NotProposedAdmin {})
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_info};
    use cosmwasm_std::Empty;

    use crate::AdminUpdate::{
        AbolishAdminRole, AcceptProposed, ClearProposed, InitializeAdmin, ProposeNewAdmin,
    };

    use super::*;

    //--------------------------------------------------------------------------------------------------
    // Test invalid state transitions
    //--------------------------------------------------------------------------------------------------

    #[test]
    fn invalid_uninitialized_state_transitions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let new_admin = Addr::unchecked("peter_parker");

        let err = admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: new_admin.clone(),
                    proposed: new_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                ClearProposed {
                    sender: new_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(storage, AcceptProposed { sender: new_admin })
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});
    }

    #[test]
    fn invalid_admin_set_no_proposed_state_transitions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();

        let err = admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                ClearProposed {
                    sender: original_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                AcceptProposed {
                    sender: original_admin,
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});
    }

    #[test]
    fn invalid_admin_set_with_proposed_state_transitions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin.clone(),
                    proposed: proposed_admin.clone(),
                },
            )
            .unwrap();

        let err = admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin,
                    proposed: proposed_admin,
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});
    }

    #[test]
    fn invalid_admin_role_abolished_state_transitions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                AbolishAdminRole {
                    sender: Some(original_admin.clone()),
                },
            )
            .unwrap();

        let err = admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin.clone(),
                    proposed: proposed_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                ClearProposed {
                    sender: original_admin.clone(),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                AcceptProposed {
                    sender: proposed_admin,
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});

        let err = admin
            .update(
                storage,
                AbolishAdminRole {
                    sender: Some(original_admin),
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::StateTransitionError {});
    }

    //--------------------------------------------------------------------------------------------------
    // Test permissions
    //--------------------------------------------------------------------------------------------------

    #[test]
    fn initialize_admin_permissions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");

        // Anyone can initialize the first admin
        let user_a = Addr::unchecked("peter_parker");
        admin
            .update(deps.as_mut().storage, InitializeAdmin { admin: user_a })
            .unwrap();

        let mut deps = mock_dependencies();
        let user_b = Addr::unchecked("miles_morales");
        admin
            .update(deps.as_mut().storage, InitializeAdmin { admin: user_b })
            .unwrap();
    }

    #[test]
    fn propose_new_admin_permissions() {
        let mut deps = mock_dependencies();
        let storage = deps.as_mut().storage;
        let admin = Admin::new("xyz");
        let original_admin = Addr::unchecked("peter_parker");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin,
                },
            )
            .unwrap();

        let bad_guy = Addr::unchecked("doc_oc");
        let err = admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: bad_guy.clone(),
                    proposed: bad_guy,
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::NotAdmin {})
    }

    #[test]
    fn clear_proposed_permissions() {
        let mut deps = mock_dependencies();
        let storage = deps.as_mut().storage;
        let admin = Admin::new("xyz");
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin,
                    proposed: proposed_admin,
                },
            )
            .unwrap();

        let bad_guy = Addr::unchecked("doc_oc");
        let err = admin
            .update(storage, ClearProposed { sender: bad_guy })
            .unwrap_err();
        assert_eq!(err, AdminError::NotAdmin {})
    }

    #[test]
    fn accept_proposed_permissions() {
        let mut deps = mock_dependencies();
        let storage = deps.as_mut().storage;
        let admin = Admin::new("xyz");
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin.clone(),
                    proposed: proposed_admin,
                },
            )
            .unwrap();

        let err = admin
            .update(
                storage,
                AcceptProposed {
                    sender: original_admin,
                },
            )
            .unwrap_err();
        assert_eq!(err, AdminError::NotProposedAdmin {})
    }

    #[test]
    fn abolish_admin_role_permissions() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let user = Addr::unchecked("peter_parker");

        // As no admin is set, no restrictions on abolishing from uninitialized state
        admin
            .update(deps.as_mut().storage, AbolishAdminRole { sender: None })
            .unwrap();

        let mut deps = mock_dependencies();
        admin
            .update(
                deps.as_mut().storage,
                AbolishAdminRole { sender: Some(user) },
            )
            .unwrap();
    }

    //--------------------------------------------------------------------------------------------------
    // Test success cases
    //--------------------------------------------------------------------------------------------------

    fn assert_uninitialized(storage: &dyn Storage, admin: &Admin) {
        let state = admin.state(storage).unwrap();
        match state {
            AdminState::A(_) => {}
            _ => panic!("Should be in the AdminUninitialized state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, None);

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: None,
                proposed: None
            }
        );
    }

    #[test]
    fn uninitialized_state() {
        let deps = mock_dependencies();
        let admin = Admin::new("xyz");
        assert_uninitialized(deps.as_ref().storage, &admin);
    }

    #[test]
    fn initialize_admin() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();

        let state = admin.state(storage).unwrap();
        match state {
            AdminState::B(_) => {}
            _ => panic!("Should be in the AdminSetNoneProposed state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, Some(original_admin.clone()));
        assert!(admin.is_admin(storage, &original_admin).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: Some(original_admin.to_string()),
                proposed: None
            }
        );
    }

    #[test]
    fn propose_new_admin() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin.clone(),
                    proposed: proposed_admin.clone(),
                },
            )
            .unwrap();

        let state = admin.state(storage).unwrap();
        match state {
            AdminState::C(_) => {}
            _ => panic!("Should be in the AdminSetWithProposed state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, Some(original_admin.clone()));
        assert!(admin.is_admin(storage, &original_admin).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, Some(proposed_admin.clone()));
        assert!(admin.is_proposed(storage, &proposed_admin).unwrap());

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: Some(original_admin.to_string()),
                proposed: Some(proposed_admin.to_string())
            }
        );
    }

    #[test]
    fn clear_proposed() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin.clone(),
                    proposed: proposed_admin.clone(),
                },
            )
            .unwrap();

        admin
            .update(
                storage,
                ClearProposed {
                    sender: original_admin.clone(),
                },
            )
            .unwrap();

        let state = admin.state(storage).unwrap();
        match state {
            AdminState::B(_) => {}
            _ => panic!("Should be in the AdminSetNoneProposed state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, Some(original_admin.clone()));
        assert!(admin.is_admin(storage, &original_admin).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);
        assert!(!admin.is_proposed(storage, &proposed_admin).unwrap());

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: Some(original_admin.to_string()),
                proposed: None
            }
        );
    }

    #[test]
    fn accept_proposed() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");
        let proposed_admin = Addr::unchecked("miles_morales");
        admin
            .update(
                storage,
                InitializeAdmin {
                    admin: original_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                ProposeNewAdmin {
                    sender: original_admin,
                    proposed: proposed_admin.clone(),
                },
            )
            .unwrap();
        admin
            .update(
                storage,
                AcceptProposed {
                    sender: proposed_admin.clone(),
                },
            )
            .unwrap();

        let state = admin.state(storage).unwrap();
        match state {
            AdminState::B(_) => {}
            _ => panic!("Should be in the AdminSetNoneProposed state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, Some(proposed_admin.clone()));
        assert!(admin.is_admin(storage, &proposed_admin).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);
        assert!(!admin.is_proposed(storage, &proposed_admin).unwrap());

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: Some(proposed_admin.to_string()),
                proposed: None
            }
        );
    }

    #[test]
    fn abolish_admin_role() {
        let mut deps = mock_dependencies();
        let admin = Admin::new("xyz");
        let storage = deps.as_mut().storage;
        let original_admin = Addr::unchecked("peter_parker");

        admin
            .update(
                storage,
                AbolishAdminRole {
                    sender: Some(original_admin.clone()),
                },
            )
            .unwrap();

        let state = admin.state(storage).unwrap();
        match state {
            AdminState::D(_) => {}
            _ => panic!("Should be in the AdminRoleAbolished state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, None);
        assert!(!admin.is_admin(storage, &original_admin).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);
        assert!(!admin.is_proposed(storage, &original_admin).unwrap());

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: None,
                proposed: None
            }
        );
    }

    #[test]
    fn execute_helper() {
        let mut deps = mock_dependencies();
        let sender = Addr::unchecked("peter_parker");
        let info = mock_info(sender.as_ref(), &[]);
        let admin = Admin::new("xyz");
        admin
            .execute_update::<Empty, Empty>(
                deps.as_mut(),
                info,
                AdminExecuteUpdate::InitializeAdmin {
                    admin: sender.clone().into(),
                },
            )
            .unwrap();

        let storage = deps.as_ref().storage;
        let state = admin.state(storage).unwrap();
        match state {
            AdminState::B(_) => {}
            _ => panic!("Should be in the AdminSetNoneProposed state"),
        }

        let current = admin.current(storage).unwrap();
        assert_eq!(current, Some(sender.clone()));
        assert!(admin.is_admin(storage, &sender).unwrap());

        let proposed = admin.proposed(storage).unwrap();
        assert_eq!(proposed, None);

        let res = admin.query(storage).unwrap();
        assert_eq!(
            res,
            AdminResponse {
                admin: Some(sender.to_string()),
                proposed: None
            }
        );
    }
}
