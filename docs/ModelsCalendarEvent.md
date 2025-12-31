# ModelsCalendarEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**auth_household_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**calendar_i_calendar_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**color** | Option<[**models::TypesColor**](types.Color.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**date_end** | Option<[**String**](string.md)> | Deprecated: use Recurrence.End  TODO remove 2026-05-06 | [optional]
**date_start** | Option<[**String**](string.md)> | start of event or recurrence | [optional]
**details** | Option<**String**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**i_calendar_uid** | Option<**String**> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**location** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**notify_offset** | Option<**i32**> | Deprecated: Use Reminders  TODO remove 2026-05-06 | [optional]
**participants** | Option<**Vec<String>**> |  | [optional]
**recurrence** | Option<[**models::TypesRecurrence**](types.Recurrence.md)> |  | [optional]
**reminders_end** | Option<**Vec<i32>**> |  | [optional]
**reminders_start** | Option<**Vec<i32>**> |  | [optional]
**skip_days** | Option<[**Vec<models::TypesCivilDate>**](types.CivilDate.md)> |  | [optional]
**time_start** | Option<**String**> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**timestamp_end** | Option<**String**> |  | [optional]
**timestamp_start** | Option<**String**> |  | [optional]
**travel_time** | Option<**i32**> |  | [optional]
**updated** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


