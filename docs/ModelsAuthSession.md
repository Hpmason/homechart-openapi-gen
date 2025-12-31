# ModelsAuthSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin** | Option<**bool**> |  | [optional]
**auth_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**auth_households_permissions** | Option<[**std::collections::HashMap<String, models::ModelsPermissions>**](models.Permissions.md)> |  | [optional]
**auth_households_permissions_labels** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, models::ModelsPermission>>**](std::collections::HashMap.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**delegations** | Option<[**Vec<models::ModelsAuthAccountDelegation>**](models.AuthAccountDelegation.md)> |  | [optional]
**expires** | Option<**String**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**notify_apn** | Option<**String**> |  | [optional]
**notify_web_push** | Option<[**models::NotifyWebPushClient**](notify.WebPushClient.md)> |  | [optional]
**permissions_account** | Option<[**models::ModelsPermissions**](models.Permissions.md)> |  | [optional]
**permissions_households** | Option<[**std::collections::HashMap<String, models::ModelsPermissions>**](models.Permissions.md)> |  | [optional]
**primary_auth_household_id** | Option<**String**> |  | [optional]
**token** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_agent** | Option<[**models::TypesUserAgent**](types.UserAgent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


