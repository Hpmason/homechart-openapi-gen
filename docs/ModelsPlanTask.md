# ModelsPlanTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignees** | Option<**Vec<String>**> |  | [optional]
**auth_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**auth_household_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**color** | Option<[**models::TypesColor**](types.Color.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**date_end** | Option<[**String**](string.md)> | Deprecated: use Recurrence.End  TODO remove 2026-05-06 | [optional]
**details** | Option<**String**> |  | [optional]
**done** | Option<**bool**> |  | [optional]
**due_date** | Option<**String**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**inventory_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**last_done_by** | Option<**String**> |  | [optional]
**last_done_date** | Option<[**models::TypesCivilDate**](types.CivilDate.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**notify** | Option<**bool**> |  | [optional]
**parent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**plan_project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**position** | Option<**String**> |  | [optional]
**recur_on_done** | Option<**bool**> |  | [optional]
**recurrence** | Option<[**models::TypesRecurrence**](types.Recurrence.md)> |  | [optional]
**short_id** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**template** | Option<**bool**> |  | [optional]
**updated** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


