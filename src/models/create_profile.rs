use std::fmt;
use std::time::SystemTime;

use chrono::NaiveDate;
use uuid::Uuid;

use stq_static_resources::{Device, Gender, Project, Provider};
use stq_types::{Alpha3, EmarsysId, MerchantId, RoleId, SagaId, UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub is_active: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Option<Gender>,
    pub birthdate: Option<NaiveDate>,
    pub last_login_at: SystemTime,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub saga_id: String,
    pub avatar: Option<String>,
    pub is_blocked: bool,
    pub emarsys_id: Option<EmarsysId>,
    pub referal: Option<UserId>,
    pub utm_marks: Option<serde_json::Value>,
    pub country: Option<Alpha3>,
    pub referer: Option<String>,
    pub revoke_before: SystemTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub gender: Option<Gender>,
    pub birthdate: Option<NaiveDate>,
    pub last_login_at: SystemTime,
    pub saga_id: SagaId,
    pub referal: Option<UserId>,
    pub utm_marks: Option<serde_json::Value>,
    pub country: Option<Alpha3>,
    pub referer: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub emarsys_id: Option<EmarsysId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewIdentity {
    pub email: String,
    pub password: Option<String>,
    pub provider: Provider,
    pub saga_id: SagaId,
}

impl fmt::Display for NewIdentity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NewIdentity: 
        email: {},
        password: '****',
        provider: {:?},
        saga_id: {}",
            self.email, self.provider, self.saga_id,
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SagaCreateProfile {
    pub user: Option<NewUser>,
    pub identity: NewIdentity,
    pub device: Option<Device>,
    pub project: Option<Project>,
}

impl fmt::Display for SagaCreateProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SagaCreateProfile - user: {:#?}, identity: {})", self.user, self.identity)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserMerchantPayload {
    pub id: UserId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Merchant {
    pub merchant_id: MerchantId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetRequest {
    pub email: String,
    pub device: Option<Device>,
    pub project: Option<Project>,
    pub uuid: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyRequest {
    pub email: String,
    pub device: Option<Device>,
    pub project: Option<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVerifyApply {
    pub token: String,
    pub project: Option<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVerifyApplyToken {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetApplyToken {
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordResetApply {
    pub token: String,
    pub password: String,
    pub project: Option<Project>,
}

pub type CreateProfileOperationLog = Vec<CreateProfileOperationStage>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum CreateProfileOperationStage {
    AccountCreationStart(SagaId),
    AccountCreationComplete(SagaId),
    UsersRoleSetStart(RoleId),
    UsersRoleSetComplete(RoleId),
    StoreRoleSetStart(RoleId),
    StoreRoleSetComplete(RoleId),
    BillingRoleSetStart(RoleId),
    BillingRoleSetComplete(RoleId),
    DeliveryRoleSetStart(RoleId),
    DeliveryRoleSetComplete(RoleId),
    BillingCreateMerchantStart(UserId),
    BillingCreateMerchantComplete(UserId),
}
