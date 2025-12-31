# \BudgetMonthApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_month_read**](BudgetMonthApi.md#budget_month_read) | **GET** /budget/months/{authHouseholdID}/{yearMonth} | Read BudgetMonth



## budget_month_read

> models::HttplibJsonResponseModelsBudgetMonths budget_month_read(auth_household_id, year_month)
Read BudgetMonth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_household_id** | **String** | authHouseholdID | [required] |
**year_month** | **String** | yearMonth | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetMonths**](httplib.JSONResponse-models_BudgetMonths.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

