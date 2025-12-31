# \ChangeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_read**](ChangeApi.md#change_read) | **GET** /changes/{id} | Read Change
[**changes_read**](ChangeApi.md#changes_read) | **GET** /changes | Read all Changes



## change_read

> models::HttplibJsonResponseModelsChanges change_read(id)
Read Change

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsChanges**](httplib.JSONResponse-models_Changes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## changes_read

> models::HttplibJsonResponseModelsChanges changes_read()
Read all Changes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsChanges**](httplib.JSONResponse-models_Changes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

