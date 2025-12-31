use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod auth_account_api;
pub mod auth_account_delegation_api;
pub mod auth_household_api;
pub mod auth_oidc_client_api;
pub mod auth_oidc_code_api;
pub mod auth_session_api;
pub mod bookmark_api;
pub mod budget_account_api;
pub mod budget_category_api;
pub mod budget_month_api;
pub mod budget_month_category_api;
pub mod budget_payee_api;
pub mod budget_recurrence_api;
pub mod budget_transaction_api;
pub mod calendar_event_api;
pub mod calendar_i_calendar_api;
pub mod change_api;
pub mod config_api;
pub mod config_key_api;
pub mod cook_meal_plan_api;
pub mod cook_meal_time_api;
pub mod cook_recipe_api;
pub mod feature_candidate_api;
pub mod feature_vote_api;
pub mod health_item_api;
pub mod health_log_item_api;
pub mod inventory_collection_api;
pub mod inventory_item_api;
pub mod labels_value_api;
pub mod notes_page_api;
pub mod notes_page_version_api;
pub mod plan_project_api;
pub mod plan_task_api;
pub mod reward_card_api;
pub mod sse_api;
pub mod secrets_value_api;
pub mod secrets_vault_api;
pub mod shop_category_api;
pub mod shop_item_api;
pub mod shop_list_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn auth_account_api(&self) -> &dyn auth_account_api::AuthAccountApi;
    fn auth_account_delegation_api(&self) -> &dyn auth_account_delegation_api::AuthAccountDelegationApi;
    fn auth_household_api(&self) -> &dyn auth_household_api::AuthHouseholdApi;
    fn auth_oidc_client_api(&self) -> &dyn auth_oidc_client_api::AuthOidcClientApi;
    fn auth_oidc_code_api(&self) -> &dyn auth_oidc_code_api::AuthOidcCodeApi;
    fn auth_session_api(&self) -> &dyn auth_session_api::AuthSessionApi;
    fn bookmark_api(&self) -> &dyn bookmark_api::BookmarkApi;
    fn budget_account_api(&self) -> &dyn budget_account_api::BudgetAccountApi;
    fn budget_category_api(&self) -> &dyn budget_category_api::BudgetCategoryApi;
    fn budget_month_api(&self) -> &dyn budget_month_api::BudgetMonthApi;
    fn budget_month_category_api(&self) -> &dyn budget_month_category_api::BudgetMonthCategoryApi;
    fn budget_payee_api(&self) -> &dyn budget_payee_api::BudgetPayeeApi;
    fn budget_recurrence_api(&self) -> &dyn budget_recurrence_api::BudgetRecurrenceApi;
    fn budget_transaction_api(&self) -> &dyn budget_transaction_api::BudgetTransactionApi;
    fn calendar_event_api(&self) -> &dyn calendar_event_api::CalendarEventApi;
    fn calendar_i_calendar_api(&self) -> &dyn calendar_i_calendar_api::CalendarICalendarApi;
    fn change_api(&self) -> &dyn change_api::ChangeApi;
    fn config_api(&self) -> &dyn config_api::ConfigApi;
    fn config_key_api(&self) -> &dyn config_key_api::ConfigKeyApi;
    fn cook_meal_plan_api(&self) -> &dyn cook_meal_plan_api::CookMealPlanApi;
    fn cook_meal_time_api(&self) -> &dyn cook_meal_time_api::CookMealTimeApi;
    fn cook_recipe_api(&self) -> &dyn cook_recipe_api::CookRecipeApi;
    fn feature_candidate_api(&self) -> &dyn feature_candidate_api::FeatureCandidateApi;
    fn feature_vote_api(&self) -> &dyn feature_vote_api::FeatureVoteApi;
    fn health_item_api(&self) -> &dyn health_item_api::HealthItemApi;
    fn health_log_item_api(&self) -> &dyn health_log_item_api::HealthLogItemApi;
    fn inventory_collection_api(&self) -> &dyn inventory_collection_api::InventoryCollectionApi;
    fn inventory_item_api(&self) -> &dyn inventory_item_api::InventoryItemApi;
    fn labels_value_api(&self) -> &dyn labels_value_api::LabelsValueApi;
    fn notes_page_api(&self) -> &dyn notes_page_api::NotesPageApi;
    fn notes_page_version_api(&self) -> &dyn notes_page_version_api::NotesPageVersionApi;
    fn plan_project_api(&self) -> &dyn plan_project_api::PlanProjectApi;
    fn plan_task_api(&self) -> &dyn plan_task_api::PlanTaskApi;
    fn reward_card_api(&self) -> &dyn reward_card_api::RewardCardApi;
    fn sse_api(&self) -> &dyn sse_api::SseApi;
    fn secrets_value_api(&self) -> &dyn secrets_value_api::SecretsValueApi;
    fn secrets_vault_api(&self) -> &dyn secrets_vault_api::SecretsVaultApi;
    fn shop_category_api(&self) -> &dyn shop_category_api::ShopCategoryApi;
    fn shop_item_api(&self) -> &dyn shop_item_api::ShopItemApi;
    fn shop_list_api(&self) -> &dyn shop_list_api::ShopListApi;
}

pub struct ApiClient {
    auth_account_api: Box<dyn auth_account_api::AuthAccountApi>,
    auth_account_delegation_api: Box<dyn auth_account_delegation_api::AuthAccountDelegationApi>,
    auth_household_api: Box<dyn auth_household_api::AuthHouseholdApi>,
    auth_oidc_client_api: Box<dyn auth_oidc_client_api::AuthOidcClientApi>,
    auth_oidc_code_api: Box<dyn auth_oidc_code_api::AuthOidcCodeApi>,
    auth_session_api: Box<dyn auth_session_api::AuthSessionApi>,
    bookmark_api: Box<dyn bookmark_api::BookmarkApi>,
    budget_account_api: Box<dyn budget_account_api::BudgetAccountApi>,
    budget_category_api: Box<dyn budget_category_api::BudgetCategoryApi>,
    budget_month_api: Box<dyn budget_month_api::BudgetMonthApi>,
    budget_month_category_api: Box<dyn budget_month_category_api::BudgetMonthCategoryApi>,
    budget_payee_api: Box<dyn budget_payee_api::BudgetPayeeApi>,
    budget_recurrence_api: Box<dyn budget_recurrence_api::BudgetRecurrenceApi>,
    budget_transaction_api: Box<dyn budget_transaction_api::BudgetTransactionApi>,
    calendar_event_api: Box<dyn calendar_event_api::CalendarEventApi>,
    calendar_i_calendar_api: Box<dyn calendar_i_calendar_api::CalendarICalendarApi>,
    change_api: Box<dyn change_api::ChangeApi>,
    config_api: Box<dyn config_api::ConfigApi>,
    config_key_api: Box<dyn config_key_api::ConfigKeyApi>,
    cook_meal_plan_api: Box<dyn cook_meal_plan_api::CookMealPlanApi>,
    cook_meal_time_api: Box<dyn cook_meal_time_api::CookMealTimeApi>,
    cook_recipe_api: Box<dyn cook_recipe_api::CookRecipeApi>,
    feature_candidate_api: Box<dyn feature_candidate_api::FeatureCandidateApi>,
    feature_vote_api: Box<dyn feature_vote_api::FeatureVoteApi>,
    health_item_api: Box<dyn health_item_api::HealthItemApi>,
    health_log_item_api: Box<dyn health_log_item_api::HealthLogItemApi>,
    inventory_collection_api: Box<dyn inventory_collection_api::InventoryCollectionApi>,
    inventory_item_api: Box<dyn inventory_item_api::InventoryItemApi>,
    labels_value_api: Box<dyn labels_value_api::LabelsValueApi>,
    notes_page_api: Box<dyn notes_page_api::NotesPageApi>,
    notes_page_version_api: Box<dyn notes_page_version_api::NotesPageVersionApi>,
    plan_project_api: Box<dyn plan_project_api::PlanProjectApi>,
    plan_task_api: Box<dyn plan_task_api::PlanTaskApi>,
    reward_card_api: Box<dyn reward_card_api::RewardCardApi>,
    sse_api: Box<dyn sse_api::SseApi>,
    secrets_value_api: Box<dyn secrets_value_api::SecretsValueApi>,
    secrets_vault_api: Box<dyn secrets_vault_api::SecretsVaultApi>,
    shop_category_api: Box<dyn shop_category_api::ShopCategoryApi>,
    shop_item_api: Box<dyn shop_item_api::ShopItemApi>,
    shop_list_api: Box<dyn shop_list_api::ShopListApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            auth_account_api: Box::new(auth_account_api::AuthAccountApiClient::new(configuration.clone())),
            auth_account_delegation_api: Box::new(auth_account_delegation_api::AuthAccountDelegationApiClient::new(configuration.clone())),
            auth_household_api: Box::new(auth_household_api::AuthHouseholdApiClient::new(configuration.clone())),
            auth_oidc_client_api: Box::new(auth_oidc_client_api::AuthOidcClientApiClient::new(configuration.clone())),
            auth_oidc_code_api: Box::new(auth_oidc_code_api::AuthOidcCodeApiClient::new(configuration.clone())),
            auth_session_api: Box::new(auth_session_api::AuthSessionApiClient::new(configuration.clone())),
            bookmark_api: Box::new(bookmark_api::BookmarkApiClient::new(configuration.clone())),
            budget_account_api: Box::new(budget_account_api::BudgetAccountApiClient::new(configuration.clone())),
            budget_category_api: Box::new(budget_category_api::BudgetCategoryApiClient::new(configuration.clone())),
            budget_month_api: Box::new(budget_month_api::BudgetMonthApiClient::new(configuration.clone())),
            budget_month_category_api: Box::new(budget_month_category_api::BudgetMonthCategoryApiClient::new(configuration.clone())),
            budget_payee_api: Box::new(budget_payee_api::BudgetPayeeApiClient::new(configuration.clone())),
            budget_recurrence_api: Box::new(budget_recurrence_api::BudgetRecurrenceApiClient::new(configuration.clone())),
            budget_transaction_api: Box::new(budget_transaction_api::BudgetTransactionApiClient::new(configuration.clone())),
            calendar_event_api: Box::new(calendar_event_api::CalendarEventApiClient::new(configuration.clone())),
            calendar_i_calendar_api: Box::new(calendar_i_calendar_api::CalendarICalendarApiClient::new(configuration.clone())),
            change_api: Box::new(change_api::ChangeApiClient::new(configuration.clone())),
            config_api: Box::new(config_api::ConfigApiClient::new(configuration.clone())),
            config_key_api: Box::new(config_key_api::ConfigKeyApiClient::new(configuration.clone())),
            cook_meal_plan_api: Box::new(cook_meal_plan_api::CookMealPlanApiClient::new(configuration.clone())),
            cook_meal_time_api: Box::new(cook_meal_time_api::CookMealTimeApiClient::new(configuration.clone())),
            cook_recipe_api: Box::new(cook_recipe_api::CookRecipeApiClient::new(configuration.clone())),
            feature_candidate_api: Box::new(feature_candidate_api::FeatureCandidateApiClient::new(configuration.clone())),
            feature_vote_api: Box::new(feature_vote_api::FeatureVoteApiClient::new(configuration.clone())),
            health_item_api: Box::new(health_item_api::HealthItemApiClient::new(configuration.clone())),
            health_log_item_api: Box::new(health_log_item_api::HealthLogItemApiClient::new(configuration.clone())),
            inventory_collection_api: Box::new(inventory_collection_api::InventoryCollectionApiClient::new(configuration.clone())),
            inventory_item_api: Box::new(inventory_item_api::InventoryItemApiClient::new(configuration.clone())),
            labels_value_api: Box::new(labels_value_api::LabelsValueApiClient::new(configuration.clone())),
            notes_page_api: Box::new(notes_page_api::NotesPageApiClient::new(configuration.clone())),
            notes_page_version_api: Box::new(notes_page_version_api::NotesPageVersionApiClient::new(configuration.clone())),
            plan_project_api: Box::new(plan_project_api::PlanProjectApiClient::new(configuration.clone())),
            plan_task_api: Box::new(plan_task_api::PlanTaskApiClient::new(configuration.clone())),
            reward_card_api: Box::new(reward_card_api::RewardCardApiClient::new(configuration.clone())),
            sse_api: Box::new(sse_api::SseApiClient::new(configuration.clone())),
            secrets_value_api: Box::new(secrets_value_api::SecretsValueApiClient::new(configuration.clone())),
            secrets_vault_api: Box::new(secrets_vault_api::SecretsVaultApiClient::new(configuration.clone())),
            shop_category_api: Box::new(shop_category_api::ShopCategoryApiClient::new(configuration.clone())),
            shop_item_api: Box::new(shop_item_api::ShopItemApiClient::new(configuration.clone())),
            shop_list_api: Box::new(shop_list_api::ShopListApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn auth_account_api(&self) -> &dyn auth_account_api::AuthAccountApi {
        self.auth_account_api.as_ref()
    }
    fn auth_account_delegation_api(&self) -> &dyn auth_account_delegation_api::AuthAccountDelegationApi {
        self.auth_account_delegation_api.as_ref()
    }
    fn auth_household_api(&self) -> &dyn auth_household_api::AuthHouseholdApi {
        self.auth_household_api.as_ref()
    }
    fn auth_oidc_client_api(&self) -> &dyn auth_oidc_client_api::AuthOidcClientApi {
        self.auth_oidc_client_api.as_ref()
    }
    fn auth_oidc_code_api(&self) -> &dyn auth_oidc_code_api::AuthOidcCodeApi {
        self.auth_oidc_code_api.as_ref()
    }
    fn auth_session_api(&self) -> &dyn auth_session_api::AuthSessionApi {
        self.auth_session_api.as_ref()
    }
    fn bookmark_api(&self) -> &dyn bookmark_api::BookmarkApi {
        self.bookmark_api.as_ref()
    }
    fn budget_account_api(&self) -> &dyn budget_account_api::BudgetAccountApi {
        self.budget_account_api.as_ref()
    }
    fn budget_category_api(&self) -> &dyn budget_category_api::BudgetCategoryApi {
        self.budget_category_api.as_ref()
    }
    fn budget_month_api(&self) -> &dyn budget_month_api::BudgetMonthApi {
        self.budget_month_api.as_ref()
    }
    fn budget_month_category_api(&self) -> &dyn budget_month_category_api::BudgetMonthCategoryApi {
        self.budget_month_category_api.as_ref()
    }
    fn budget_payee_api(&self) -> &dyn budget_payee_api::BudgetPayeeApi {
        self.budget_payee_api.as_ref()
    }
    fn budget_recurrence_api(&self) -> &dyn budget_recurrence_api::BudgetRecurrenceApi {
        self.budget_recurrence_api.as_ref()
    }
    fn budget_transaction_api(&self) -> &dyn budget_transaction_api::BudgetTransactionApi {
        self.budget_transaction_api.as_ref()
    }
    fn calendar_event_api(&self) -> &dyn calendar_event_api::CalendarEventApi {
        self.calendar_event_api.as_ref()
    }
    fn calendar_i_calendar_api(&self) -> &dyn calendar_i_calendar_api::CalendarICalendarApi {
        self.calendar_i_calendar_api.as_ref()
    }
    fn change_api(&self) -> &dyn change_api::ChangeApi {
        self.change_api.as_ref()
    }
    fn config_api(&self) -> &dyn config_api::ConfigApi {
        self.config_api.as_ref()
    }
    fn config_key_api(&self) -> &dyn config_key_api::ConfigKeyApi {
        self.config_key_api.as_ref()
    }
    fn cook_meal_plan_api(&self) -> &dyn cook_meal_plan_api::CookMealPlanApi {
        self.cook_meal_plan_api.as_ref()
    }
    fn cook_meal_time_api(&self) -> &dyn cook_meal_time_api::CookMealTimeApi {
        self.cook_meal_time_api.as_ref()
    }
    fn cook_recipe_api(&self) -> &dyn cook_recipe_api::CookRecipeApi {
        self.cook_recipe_api.as_ref()
    }
    fn feature_candidate_api(&self) -> &dyn feature_candidate_api::FeatureCandidateApi {
        self.feature_candidate_api.as_ref()
    }
    fn feature_vote_api(&self) -> &dyn feature_vote_api::FeatureVoteApi {
        self.feature_vote_api.as_ref()
    }
    fn health_item_api(&self) -> &dyn health_item_api::HealthItemApi {
        self.health_item_api.as_ref()
    }
    fn health_log_item_api(&self) -> &dyn health_log_item_api::HealthLogItemApi {
        self.health_log_item_api.as_ref()
    }
    fn inventory_collection_api(&self) -> &dyn inventory_collection_api::InventoryCollectionApi {
        self.inventory_collection_api.as_ref()
    }
    fn inventory_item_api(&self) -> &dyn inventory_item_api::InventoryItemApi {
        self.inventory_item_api.as_ref()
    }
    fn labels_value_api(&self) -> &dyn labels_value_api::LabelsValueApi {
        self.labels_value_api.as_ref()
    }
    fn notes_page_api(&self) -> &dyn notes_page_api::NotesPageApi {
        self.notes_page_api.as_ref()
    }
    fn notes_page_version_api(&self) -> &dyn notes_page_version_api::NotesPageVersionApi {
        self.notes_page_version_api.as_ref()
    }
    fn plan_project_api(&self) -> &dyn plan_project_api::PlanProjectApi {
        self.plan_project_api.as_ref()
    }
    fn plan_task_api(&self) -> &dyn plan_task_api::PlanTaskApi {
        self.plan_task_api.as_ref()
    }
    fn reward_card_api(&self) -> &dyn reward_card_api::RewardCardApi {
        self.reward_card_api.as_ref()
    }
    fn sse_api(&self) -> &dyn sse_api::SseApi {
        self.sse_api.as_ref()
    }
    fn secrets_value_api(&self) -> &dyn secrets_value_api::SecretsValueApi {
        self.secrets_value_api.as_ref()
    }
    fn secrets_vault_api(&self) -> &dyn secrets_vault_api::SecretsVaultApi {
        self.secrets_vault_api.as_ref()
    }
    fn shop_category_api(&self) -> &dyn shop_category_api::ShopCategoryApi {
        self.shop_category_api.as_ref()
    }
    fn shop_item_api(&self) -> &dyn shop_item_api::ShopItemApi {
        self.shop_item_api.as_ref()
    }
    fn shop_list_api(&self) -> &dyn shop_list_api::ShopListApi {
        self.shop_list_api.as_ref()
    }
}


