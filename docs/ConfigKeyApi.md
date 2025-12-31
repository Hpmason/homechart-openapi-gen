# \ConfigKeyApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**config_key_set**](ConfigKeyApi.md#config_key_set) | **GET** /config/keys/{key} | Set a ConfigKey
[**config_keys_read**](ConfigKeyApi.md#config_keys_read) | **GET** /config/keys | Read all ConfigKeys



## config_key_set

> models::HttplibJsonResponseModelsConfigKeys config_key_set(key, body)
Set a ConfigKey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Key | [required] |
**body** | [**ModelsConfigKey**](ModelsConfigKey.md) | ConfigKey | [required] |

### Return type

[**models::HttplibJsonResponseModelsConfigKeys**](httplib.JSONResponse-models_ConfigKeys.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_keys_read

> models::HttplibJsonResponseModelsCookMealPlans config_keys_read()
Read all ConfigKeys

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

