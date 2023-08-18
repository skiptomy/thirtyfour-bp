use cucumber::{given, when, then};

use crate::world::E2eWorld;

pub mod dashboard;
pub mod ec2_details_page;
pub mod filters;
pub mod inventory;
pub mod login;
pub mod threats_page;
pub mod vul_page;

#[given(regex = "Test step")]
#[when(regex = "Test step")]
#[then(regex = "Test step")]
async fn test_step(world: &mut E2eWorld) {
    let acc = "123456789012".to_string();

    assert_eq!(world.aws_account_id, Some(acc));
}
