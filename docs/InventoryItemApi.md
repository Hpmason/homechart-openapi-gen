# \InventoryItemApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inventory_item_create**](InventoryItemApi.md#inventory_item_create) | **POST** /inventory/items | Create InventoryItem
[**inventory_item_delete**](InventoryItemApi.md#inventory_item_delete) | **DELETE** /inventory/items/{id} | Delete InventoryItem
[**inventory_item_read**](InventoryItemApi.md#inventory_item_read) | **GET** /inventory/items/{id} | Read InventoryItem
[**inventory_item_update**](InventoryItemApi.md#inventory_item_update) | **PUT** /inventory/items/{id} | Update InventoryItem
[**inventory_items_read**](InventoryItemApi.md#inventory_items_read) | **GET** /inventory/items | Read all InventoryItems



## inventory_item_create

> models::HttplibJsonResponseModelsInventoryItems inventory_item_create(body)
Create InventoryItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsInventoryItem**](ModelsInventoryItem.md) | InventoryItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryItems**](httplib.JSONResponse-models_InventoryItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_item_delete

> models::HttplibJsonResponseModelsInventoryItems inventory_item_delete(id)
Delete InventoryItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryItems**](httplib.JSONResponse-models_InventoryItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_item_read

> models::HttplibJsonResponseModelsInventoryItems inventory_item_read(id)
Read InventoryItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryItems**](httplib.JSONResponse-models_InventoryItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_item_update

> models::HttplibJsonResponseModelsInventoryItems inventory_item_update(id, body)
Update InventoryItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsInventoryItem**](ModelsInventoryItem.md) | InventoryItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryItems**](httplib.JSONResponse-models_InventoryItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_items_read

> models::HttplibJsonResponseModelsInventoryItems inventory_items_read()
Read all InventoryItems

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsInventoryItems**](httplib.JSONResponse-models_InventoryItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

