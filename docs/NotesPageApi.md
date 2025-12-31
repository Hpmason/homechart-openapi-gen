# \NotesPageApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notes_page_create**](NotesPageApi.md#notes_page_create) | **POST** /notes/pages | Create NotesPage
[**notes_page_delete**](NotesPageApi.md#notes_page_delete) | **DELETE** /notes/pages/{id} | Delete NotesPage
[**notes_page_read**](NotesPageApi.md#notes_page_read) | **GET** /notes/pages/{id} | Read NotesPage
[**notes_page_update**](NotesPageApi.md#notes_page_update) | **PUT** /notes/pages/{id} | Update NotesPage
[**notes_pages_read**](NotesPageApi.md#notes_pages_read) | **GET** /notes/pages | Read all NotesPages



## notes_page_create

> models::HttplibJsonResponseModelsNotesPages notes_page_create(body)
Create NotesPage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsNotesPage**](ModelsNotesPage.md) | NotesPage | [required] |

### Return type

[**models::HttplibJsonResponseModelsNotesPages**](httplib.JSONResponse-models_NotesPages.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notes_page_delete

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_delete(id)
Delete NotesPage

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


## notes_page_read

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_read(id)
Read NotesPage

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


## notes_page_update

> models::HttplibJsonResponseModelsNotesPageVersions notes_page_update(id, body)
Update NotesPage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsNotesPage**](ModelsNotesPage.md) | NotesPage | [required] |

### Return type

[**models::HttplibJsonResponseModelsNotesPageVersions**](httplib.JSONResponse-models_NotesPageVersions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notes_pages_read

> models::HttplibJsonResponseModelsNotesPageVersions notes_pages_read()
Read all NotesPages

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

