# \ShopListApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shop_list_create**](ShopListApi.md#shop_list_create) | **POST** /shop/lists | Create ShopList
[**shop_list_delete**](ShopListApi.md#shop_list_delete) | **DELETE** /shop/lists/{id} | Delete ShopList
[**shop_list_read**](ShopListApi.md#shop_list_read) | **GET** /shop/lists/{id} | Read ShopList
[**shop_list_update**](ShopListApi.md#shop_list_update) | **PUT** /shop/lists/{id} | Update ShopList
[**shop_lists_read**](ShopListApi.md#shop_lists_read) | **GET** /shop/lists | Read all ShopLists



## shop_list_create

> models::HttplibJsonResponseModelsShopLists shop_list_create(body)
Create ShopList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsShopList**](ModelsShopList.md) | ShopList | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopLists**](httplib.JSONResponse-models_ShopLists.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_list_delete

> models::HttplibJsonResponseModelsShopLists shop_list_delete(id)
Delete ShopList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopLists**](httplib.JSONResponse-models_ShopLists.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_list_read

> models::HttplibJsonResponseModelsShopLists shop_list_read(id)
Read ShopList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopLists**](httplib.JSONResponse-models_ShopLists.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_list_update

> models::HttplibJsonResponseModelsShopLists shop_list_update(id, body)
Update ShopList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsShopList**](ModelsShopList.md) | ShopList | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopLists**](httplib.JSONResponse-models_ShopLists.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_lists_read

> models::HttplibJsonResponseModelsShopLists shop_lists_read()
Read all ShopLists

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsShopLists**](httplib.JSONResponse-models_ShopLists.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

