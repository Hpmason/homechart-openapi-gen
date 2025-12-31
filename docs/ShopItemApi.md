# \ShopItemApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shop_item_create**](ShopItemApi.md#shop_item_create) | **POST** /shop/items | Create ShopItem
[**shop_item_delete**](ShopItemApi.md#shop_item_delete) | **DELETE** /shop/items/{id} | Delete ShopItem
[**shop_item_read**](ShopItemApi.md#shop_item_read) | **GET** /shop/items/{id} | Read ShopItem
[**shop_item_update**](ShopItemApi.md#shop_item_update) | **PUT** /shop/items/{id} | Update ShopItem
[**shop_items_read**](ShopItemApi.md#shop_items_read) | **GET** /shop/items | Read all ShopItems



## shop_item_create

> models::HttplibJsonResponseModelsShopItems shop_item_create(body)
Create ShopItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsShopItem**](ModelsShopItem.md) | ShopItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopItems**](httplib.JSONResponse-models_ShopItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_item_delete

> models::HttplibJsonResponseModelsShopItems shop_item_delete(id)
Delete ShopItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopItems**](httplib.JSONResponse-models_ShopItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_item_read

> models::HttplibJsonResponseModelsShopItems shop_item_read(id)
Read ShopItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopItems**](httplib.JSONResponse-models_ShopItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_item_update

> models::HttplibJsonResponseModelsShopItems shop_item_update(id, body)
Update ShopItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsShopItem**](ModelsShopItem.md) | ShopItem | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopItems**](httplib.JSONResponse-models_ShopItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_items_read

> models::HttplibJsonResponseModelsShopItems shop_items_read()
Read all ShopItems

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsShopItems**](httplib.JSONResponse-models_ShopItems.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

