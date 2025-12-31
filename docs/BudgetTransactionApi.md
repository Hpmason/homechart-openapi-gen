# \BudgetTransactionApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_transaction_create**](BudgetTransactionApi.md#budget_transaction_create) | **POST** /budget/transactions | Create BudgetTransaction
[**budget_transaction_delete**](BudgetTransactionApi.md#budget_transaction_delete) | **DELETE** /budget/transactions/{id} | Delete BudgetTransaction
[**budget_transaction_update**](BudgetTransactionApi.md#budget_transaction_update) | **PUT** /budget/transactions/{id} | Update BudgetTransaction



## budget_transaction_create

> models::HttplibJsonResponseModelsBudgetTransactions budget_transaction_create(body)
Create BudgetTransaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetTransaction**](ModelsBudgetTransaction.md) | BudgetTransaction | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetTransactions**](httplib.JSONResponse-models_BudgetTransactions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_transaction_delete

> models::HttplibJsonResponseModelsBudgetTransactions budget_transaction_delete(id)
Delete BudgetTransaction

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


## budget_transaction_update

> models::HttplibJsonResponseModelsBudgetTransactions budget_transaction_update(id, body)
Update BudgetTransaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBudgetTransaction**](ModelsBudgetTransaction.md) | BudgetTransaction | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetTransactions**](httplib.JSONResponse-models_BudgetTransactions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

