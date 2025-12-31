# \ShopCategoryApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shop_categories_read**](ShopCategoryApi.md#shop_categories_read) | **GET** /shop/categories | Read all ShopCategories
[**shop_category_create**](ShopCategoryApi.md#shop_category_create) | **POST** /shop/categories | Create ShopCategory
[**shop_category_delete**](ShopCategoryApi.md#shop_category_delete) | **DELETE** /shop/categories/{id} | Delete ShopCategory
[**shop_category_read**](ShopCategoryApi.md#shop_category_read) | **GET** /shop/categories/{id} | Read ShopCategory
[**shop_category_update**](ShopCategoryApi.md#shop_category_update) | **PUT** /shop/categories/{id} | Update ShopCategory



## shop_categories_read

> models::HttplibJsonResponseModelsShopCategories shop_categories_read()
Read all ShopCategories

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsShopCategories**](httplib.JSONResponse-models_ShopCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_category_create

> models::HttplibJsonResponseModelsShopCategories shop_category_create(body)
Create ShopCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsShopCategory**](ModelsShopCategory.md) | ShopCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopCategories**](httplib.JSONResponse-models_ShopCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_category_delete

> models::HttplibJsonResponseModelsShopCategories shop_category_delete(id)
Delete ShopCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopCategories**](httplib.JSONResponse-models_ShopCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_category_read

> models::HttplibJsonResponseModelsShopCategories shop_category_read(id)
Read ShopCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopCategories**](httplib.JSONResponse-models_ShopCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shop_category_update

> models::HttplibJsonResponseModelsShopCategories shop_category_update(id, body)
Update ShopCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsShopCategory**](ModelsShopCategory.md) | ShopCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsShopCategories**](httplib.JSONResponse-models_ShopCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

