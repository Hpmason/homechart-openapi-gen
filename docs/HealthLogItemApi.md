# \HealthLogItemApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_log_item_create**](HealthLogItemApi.md#health_log_item_create) | **POST** /health/log-items | Create HealthLogItem
[**health_log_item_delete**](HealthLogItemApi.md#health_log_item_delete) | **DELETE** /health/log-items/{id} | Delete HealthLogItem
[**health_log_item_read**](HealthLogItemApi.md#health_log_item_read) | **GET** /health/log-items/{id} | Read HealthLogItem
[**health_log_item_update**](HealthLogItemApi.md#health_log_item_update) | **PUT** /health/log-items/{id} | Update HealthLogItem
[**health_log_items_read**](HealthLogItemApi.md#health_log_items_read) | **GET** /health/log-items | Read all HealthLogItems



## health_log_item_create

> models::HttplibJsonResponseModelsHealthLogItems health_log_item_create(body)
Create HealthLogItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsHealthLogItem**](ModelsHealthLogItem.md) | HealthLogItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthLogItems**](httplib.JSONResponse-models_HealthLogItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_log_item_delete

> models::HttplibJsonResponseModelsHealthLogItems health_log_item_delete(id)
Delete HealthLogItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthLogItems**](httplib.JSONResponse-models_HealthLogItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_log_item_read

> models::HttplibJsonResponseModelsHealthLogItems health_log_item_read(id)
Read HealthLogItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthLogItems**](httplib.JSONResponse-models_HealthLogItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_log_item_update

> models::HttplibJsonResponseModelsHealthLogItems health_log_item_update(id, body)
Update HealthLogItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsHealthLogItem**](ModelsHealthLogItem.md) | HealthLogItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthLogItems**](httplib.JSONResponse-models_HealthLogItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_log_items_read

> models::HttplibJsonResponseModelsHealthLogItems health_log_items_read()
Read all HealthLogItems

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsHealthLogItems**](httplib.JSONResponse-models_HealthLogItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

