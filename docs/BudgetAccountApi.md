# \BudgetAccountApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_account_create**](BudgetAccountApi.md#budget_account_create) | **POST** /budget/accounts | Create BudgetAccount
[**budget_account_delete**](BudgetAccountApi.md#budget_account_delete) | **DELETE** /budget/accounts/{id} | Delete BudgetAccount
[**budget_account_read**](BudgetAccountApi.md#budget_account_read) | **GET** /budget/accounts/{id} | Read BudgetAccount
[**budget_account_reconcile**](BudgetAccountApi.md#budget_account_reconcile) | **POST** /budget/accounts/{id}/reconcile | Reconcile a BudgetAccount
[**budget_account_rollup_balance**](BudgetAccountApi.md#budget_account_rollup_balance) | **POST** /budget/accounts/{id}/rollup/{yearmonth}/balance | Rollup a BudgetAccount starting balance
[**budget_account_rollup_summary**](BudgetAccountApi.md#budget_account_rollup_summary) | **POST** /budget/accounts/{id}/rollup/{yearmonth}/summary | Rollup BudgetAccount transactions into summary transactions
[**budget_account_update**](BudgetAccountApi.md#budget_account_update) | **PUT** /budget/accounts/{id} | Update BudgetAccount
[**budget_accounts_read**](BudgetAccountApi.md#budget_accounts_read) | **GET** /budget/accounts | Read all BudgetAccounts
[**budget_transactions_read**](BudgetAccountApi.md#budget_transactions_read) | **GET** /budget/accounts/{id}/transactions | Read BudgetTransactions for an Account



## budget_account_create

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_create(body)
Create BudgetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetAccount**](ModelsBudgetAccount.md) | BudgetAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_delete

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_delete(id)
Delete BudgetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_read

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_read(id)
Read BudgetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_reconcile

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_reconcile(id)
Reconcile a BudgetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_rollup_balance

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_rollup_balance(id, yearmonth)
Rollup a BudgetAccount starting balance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**yearmonth** | **i32** | YearMonth | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_rollup_summary

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_rollup_summary(id, yearmonth)
Rollup BudgetAccount transactions into summary transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**yearmonth** | **i32** | YearMonth | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_account_update

> models::HttplibJsonResponseModelsBudgetAccounts budget_account_update(id, body)
Update BudgetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBudgetAccount**](ModelsBudgetAccount.md) | BudgetAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_accounts_read

> models::HttplibJsonResponseModelsBudgetAccounts budget_accounts_read()
Read all BudgetAccounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsBudgetAccounts**](httplib.JSONResponse-models_BudgetAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_transactions_read

> models::HttplibJsonResponseModelsBudgetTransactions budget_transactions_read(id)
Read BudgetTransactions for an Account

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

