# \NotesPageVersionApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notes_page_version_create**](NotesPageVersionApi.md#notes_page_version_create) | **POST** /notes/page-versions | Create NotesPageVersion
[**notes_page_version_read**](NotesPageVersionApi.md#notes_page_version_read) | **GET** /notes/page-versions/{id} | Read NotesPageVersion
[**notes_page_versions_read**](NotesPageVersionApi.md#notes_page_versions_read) | **GET** /notes/page-versions | Read all NotesPageVersions



## notes_page_version_create

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_version_create(body)
Create NotesPageVersion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsNotesPageVersion**](ModelsNotesPageVersion.md) | NotesPageVersion | [required] |

### Return type

[**models::HttplibJsonResponseModelsNotesPageVersions**](httplib.JSONResponse-models_NotesPageVersions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notes_page_version_read

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_version_read(id)
Read NotesPageVersion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsNotesPageVersions**](httplib.JSONResponse-models_NotesPageVersions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notes_page_versions_read

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_versions_read()
Read all NotesPageVersions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsNotesPageVersions**](httplib.JSONResponse-models_NotesPageVersions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

