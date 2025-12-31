# \HealthItemApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_item_create**](HealthItemApi.md#health_item_create) | **POST** /health/items | Create HealthItem
[**health_item_delete**](HealthItemApi.md#health_item_delete) | **DELETE** /health/items/{id} | Delete HealthItem
[**health_item_read**](HealthItemApi.md#health_item_read) | **GET** /health/items/{id} | Read HealthItem
[**health_item_update**](HealthItemApi.md#health_item_update) | **PUT** /health/items/{id} | Update HealthItem
[**health_items_init**](HealthItemApi.md#health_items_init) | **PUT** /health/items | Add default HealthItems
[**health_items_read**](HealthItemApi.md#health_items_read) | **GET** /health/items | Read all HealthItems



## health_item_create

> models::HttplibJsonResponseModelsHealthItems health_item_create(body)
Create HealthItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsHealthItem**](ModelsHealthItem.md) | HealthItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_item_delete

> models::HttplibJsonResponseModelsHealthItems health_item_delete(id)
Delete HealthItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_item_read

> models::HttplibJsonResponseModelsHealthItems health_item_read(id)
Read HealthItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_item_update

> models::HttplibJsonResponseModelsHealthItems health_item_update(id, body)
Update HealthItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsHealthItem**](ModelsHealthItem.md) | HealthItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_items_init

> models::HttplibJsonResponseModelsHealthItems health_items_init()
Add default HealthItems

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_items_read

> models::HttplibJsonResponseModelsHealthItems health_items_read()
Read all HealthItems

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsHealthItems**](httplib.JSONResponse-models_HealthItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

