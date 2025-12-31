# \SecretsValueApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secrets_value_create**](SecretsValueApi.md#secrets_value_create) | **POST** /secrets/values | Create SecretsValue
[**secrets_value_delete**](SecretsValueApi.md#secrets_value_delete) | **DELETE** /secrets/values/{id} | Delete SecretsValue
[**secrets_value_read**](SecretsValueApi.md#secrets_value_read) | **GET** /secrets/values/{id} | Read SecretsValue
[**secrets_value_update**](SecretsValueApi.md#secrets_value_update) | **PUT** /secrets/values/{id} | Update SecretsValue
[**secrets_values_read**](SecretsValueApi.md#secrets_values_read) | **GET** /secrets/values | Read all SecretsValues



## secrets_value_create

> models::HttplibJsonResponseModelsSecretsValues secrets_value_create(body)
Create SecretsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsSecretsValue**](ModelsSecretsValue.md) | SecretsValue | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsValues**](httplib.JSONResponse-models_SecretsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_value_delete

> models::HttplibJsonResponseModelsSecretsValues secrets_value_delete(id)
Delete SecretsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsValues**](httplib.JSONResponse-models_SecretsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_value_read

> models::HttplibJsonResponseModelsSecretsValues secrets_value_read(id)
Read SecretsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsValues**](httplib.JSONResponse-models_SecretsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_value_update

> models::HttplibJsonResponseModelsSecretsValues secrets_value_update(id, body)
Update SecretsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsSecretsValue**](ModelsSecretsValue.md) | SecretsValue | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsValues**](httplib.JSONResponse-models_SecretsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_values_read

> models::HttplibJsonResponseModelsSecretsValues secrets_values_read()
Read all SecretsValues

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsSecretsValues**](httplib.JSONResponse-models_SecretsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

