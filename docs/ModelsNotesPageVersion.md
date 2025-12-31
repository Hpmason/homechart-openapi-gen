# ModelsNotesPageVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> |  | [optional]
**created_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Updated is technically the created field, but the existing merge and query code uses updated and it's hard to justify duplicating all that code for a semantic change.  Alternatively, having a created and updated field seems very redundant as they will always be equal. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**notes_page_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**updated** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


