# \InventoryCollectionApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inventory_collection_create**](InventoryCollectionApi.md#inventory_collection_create) | **POST** /inventory/collections | Create InventoryCollection
[**inventory_collection_delete**](InventoryCollectionApi.md#inventory_collection_delete) | **DELETE** /inventory/collections/{id} | Delete InventoryCollection
[**inventory_collection_read**](InventoryCollectionApi.md#inventory_collection_read) | **GET** /inventory/collections/{id} | Read InventoryCollection
[**inventory_collection_update**](InventoryCollectionApi.md#inventory_collection_update) | **PUT** /inventory/collections/{id} | Update InventoryCollection
[**inventory_collections_read**](InventoryCollectionApi.md#inventory_collections_read) | **GET** /inventory/collections | Read all InventoryCollections



## inventory_collection_create

> models::HttplibJsonResponseModelsInventoryCollections inventory_collection_create(body)
Create InventoryCollection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsInventoryCollection**](ModelsInventoryCollection.md) | InventoryCollection | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryCollections**](httplib.JSONResponse-models_InventoryCollections.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_collection_delete

> models::HttplibJsonResponseModelsInventoryCollections inventory_collection_delete(id)
Delete InventoryCollection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryCollections**](httplib.JSONResponse-models_InventoryCollections.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_collection_read

> models::HttplibJsonResponseModelsInventoryCollections inventory_collection_read(id)
Read InventoryCollection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryCollections**](httplib.JSONResponse-models_InventoryCollections.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_collection_update

> models::HttplibJsonResponseModelsInventoryCollections inventory_collection_update(id, body)
Update InventoryCollection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsInventoryCollection**](ModelsInventoryCollection.md) | InventoryCollection | [required] |

### Return type

[**models::HttplibJsonResponseModelsInventoryCollections**](httplib.JSONResponse-models_InventoryCollections.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inventory_collections_read

> models::HttplibJsonResponseModelsInventoryCollections inventory_collections_read()
Read all InventoryCollections

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsInventoryCollections**](httplib.JSONResponse-models_InventoryCollections.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

