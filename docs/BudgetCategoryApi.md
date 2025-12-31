# \BudgetCategoryApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_categories_read**](BudgetCategoryApi.md#budget_categories_read) | **GET** /budget/categories | Read all BudgetCategories
[**budget_category_create**](BudgetCategoryApi.md#budget_category_create) | **POST** /budget/categories | Create BudgetCategory
[**budget_category_delete**](BudgetCategoryApi.md#budget_category_delete) | **DELETE** /budget/categories/{id} | Delete BudgetCategory
[**budget_category_read**](BudgetCategoryApi.md#budget_category_read) | **GET** /budget/categories/{id} | Read BudgetCategory
[**budget_category_update**](BudgetCategoryApi.md#budget_category_update) | **PUT** /budget/categories/{id} | Read BudgetCategory
[**budget_transactions_read_category**](BudgetCategoryApi.md#budget_transactions_read_category) | **GET** /budget/categories/{id}/transactions | Read BudgetTransactions for a Category



## budget_categories_read

> models::HttplibJsonResponseModelsBudgetCategories budget_categories_read()
Read all BudgetCategories

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsBudgetCategories**](httplib.JSONResponse-models_BudgetCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_category_create

> models::HttplibJsonResponseModelsBudgetCategories budget_category_create(body)
Create BudgetCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetCategory**](ModelsBudgetCategory.md) | BudgetCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetCategories**](httplib.JSONResponse-models_BudgetCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_category_delete

> models::HttplibJsonResponseModelsBudgetCategories budget_category_delete(id)
Delete BudgetCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetCategories**](httplib.JSONResponse-models_BudgetCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_category_read

> models::HttplibJsonResponseModelsBudgetCategories budget_category_read(id)
Read BudgetCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetCategories**](httplib.JSONResponse-models_BudgetCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_category_update

> models::HttplibJsonResponseModelsBudgetCategories budget_category_update(id, body)
Read BudgetCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBudgetCategory**](ModelsBudgetCategory.md) | BudgetCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetCategories**](httplib.JSONResponse-models_BudgetCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_transactions_read_category

> models::HttplibJsonResponseModelsBudgetTransactions budget_transactions_read_category(id)
Read BudgetTransactions for a Category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetTransactions**](httplib.JSONResponse-models_BudgetTransactions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

