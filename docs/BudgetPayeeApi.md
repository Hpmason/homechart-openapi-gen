# \BudgetPayeeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_payee_create**](BudgetPayeeApi.md#budget_payee_create) | **POST** /budget/payees | Create BudgetPayee
[**budget_payee_delete**](BudgetPayeeApi.md#budget_payee_delete) | **DELETE** /budget/payees/{id} | Delete BudgetPayee
[**budget_payee_read**](BudgetPayeeApi.md#budget_payee_read) | **GET** /budget/payees/{id} | Read BudgetPayee
[**budget_payee_update**](BudgetPayeeApi.md#budget_payee_update) | **PUT** /budget/payees/{id} | Update BudgetPayee
[**budget_payees_read**](BudgetPayeeApi.md#budget_payees_read) | **GET** /budget/payees | Read all BudgetPayees
[**budget_transactions_read_payee**](BudgetPayeeApi.md#budget_transactions_read_payee) | **GET** /budget/payees/{id}/transactions | Read BudgetTransactions for a Payee



## budget_payee_create

> models::HttplibJsonResponseModelsBudgetPayees budget_payee_create(body)
Create BudgetPayee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetPayee**](ModelsBudgetPayee.md) | BudgetPayee | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetPayees**](httplib.JSONResponse-models_BudgetPayees.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_payee_delete

> models::HttplibJsonResponseModelsBudgetPayees budget_payee_delete(id)
Delete BudgetPayee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetPayees**](httplib.JSONResponse-models_BudgetPayees.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_payee_read

> models::HttplibJsonResponseModelsBudgetPayees budget_payee_read(id)
Read BudgetPayee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetPayees**](httplib.JSONResponse-models_BudgetPayees.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_payee_update

> models::HttplibJsonResponseModelsBudgetPayees budget_payee_update(id, body)
Update BudgetPayee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBudgetPayee**](ModelsBudgetPayee.md) | BudgetPayee | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetPayees**](httplib.JSONResponse-models_BudgetPayees.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_payees_read

> models::HttplibJsonResponseModelsBudgetPayees budget_payees_read()
Read all BudgetPayees

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsBudgetPayees**](httplib.JSONResponse-models_BudgetPayees.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_transactions_read_payee

> models::HttplibJsonResponseModelsBudgetTransactions budget_transactions_read_payee(id)
Read BudgetTransactions for a Payee

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

