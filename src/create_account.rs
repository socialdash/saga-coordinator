use config;

use stq_router;

use failure;
use futures;
use hyper;
use serde_json;

use futures::prelude::*;
use hyper::{Method, Uri};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use stq_http::Client as HttpClient;
use stq_router::model::Model as StqModel;
use stq_router::role::Role as StqRole;
use stq_router::role::UserRole as StqUserRole;
use stq_router::role::NewUserRole as StqNewUserRole;
use stq_router::service::Service as StqService;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Gender {
    Male,
    Female,
    Undefined,
}

impl FromStr for Gender {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            _ => Ok(Gender::Undefined),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_active: bool,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub birthdate: Option<String>,
}

pub type OperationLog = Vec<OperationStage>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum OperationStage {
    AccountCreationStart,
    AccountCreationComplete,
    UsersRoleSetStart,
    UsersRoleSetComplete,
    StoreRoleSetStart,
    StoreRoleSetComplete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputView {
    email: String,
}

// Contains happy path for account creation
#[async]
fn op_happy(
    http_client: Arc<HttpClient>,
    log: Arc<Mutex<OperationLog>>,
    config: config::Config,
    input: InputView,
    body: String,
) -> Result<String, failure::Error> {
    let mut log = log.lock().unwrap();

    // Create account
    log.push(OperationStage::AccountCreationStart);
    let res = await!(http_client.handle().request::<User>(
        Method::Post,
        format!(
            "{}/{}",
            config.service_url(StqService::Users),
            StqModel::User.to_url()
        ),
        Some(body),
        None
    )).map_err(|e| format_err!("{}", e))?;
    log.push(OperationStage::AccountCreationComplete);

    // Set roles in users
    log.push(OperationStage::UsersRoleSetStart);
    let user_role = StqNewUserRole {
        user_id: res.id,
        role: StqRole::User,
    };

    let body = serde_json::to_string(&user_role)
        .map_err(|e| format_err!("{}", e))?
        .to_string();

    await!(http_client.handle().request::<StqUserRole>(
        Method::Post,
        format!(
            "{}/{}",
            config.service_url(StqService::Users),
            StqModel::UserRoles.to_url()
        ),
        Some(body),
        None
    )).map_err(|e| format_err!("{}", e))?;
    log.push(OperationStage::UsersRoleSetComplete);

    /*
    // Set roles in stores
    log.push(OperationStage::StoreRoleSetStart);
    let res_set_store_role = await!(
        http_client.handle()
            .get(Uri::new(format!("{}/set_role", config.stores_addr)).unwrap())
            .map_err(|e| (log, e))
    )?;
    log.push(OperationStage::StoreRoleSetComplete);
    */

    Ok(serde_json::to_string(&res)?)
}

// Contains reversal of account creation
#[async]
fn op_revert(
    http_client: Arc<HttpClient>,
    operation_log: OperationLog,
    config: config::Config,
    input: InputView,
) -> Result<(), failure::Error> {
    if operation_log.contains(&OperationStage::UsersRoleSetStart) {}

    if operation_log.contains(&OperationStage::StoreRoleSetStart) {
        let fut = http_client.handle().request::<String>(
            Method::Post,
            format!("{}/remove_role", config.stores_addr),
            Some(format!("user_id=xxx")),
            None,
        );

        await!(fut);
    }

    if operation_log.contains(&OperationStage::AccountCreationStart) {
        let fut = http_client.handle().request::<String>(
            Method::Post,
            format!("{}/remove_user", config.users_addr),
            Some(format!("user_id=xxx")),
            None,
        );

        await!(fut);
    }

    Ok(())
}

#[async]
pub fn op(http_client: Arc<HttpClient>, config: config::Config, body: String) -> Result<String, failure::Error> {
    let input = serde_json::from_str(&body)?;

    let log = Arc::new(Mutex::new(OperationLog::new()));
    let happy_path = op_happy(http_client, log.clone(), config, input, body);

    match await!(happy_path) {
        Err(e) => {
            eprintln!(
                "Failed to create user {} (error {}). Reverting.",
                &input.email, &e
            );
            let revert_path = op_revert(http_client, *log.lock().unwrap(), config, input);

            await!(revert_path)?;

            Ok("Complete".into())
        }
        Ok(s) => Ok(s),
    }
}