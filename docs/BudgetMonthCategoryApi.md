# \BudgetMonthCategoryApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_month_category_create**](BudgetMonthCategoryApi.md#budget_month_category_create) | **POST** /budget/month-categories | Create BudgetMonthCategory
[**budget_month_category_rollup**](BudgetMonthCategoryApi.md#budget_month_category_rollup) | **POST** /budget/months/{authHouseholdID}/{yearMonth}/rollup | Update BudgetMonthCategory
[**budget_month_category_update**](BudgetMonthCategoryApi.md#budget_month_category_update) | **PUT** /budget/month-categories | Update BudgetMonthCategory



## budget_month_category_create

> models::HttplibJsonResponseModelsBudgetMonthCategories budget_month_category_create(body)
Create BudgetMonthCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetMonthCategory**](ModelsBudgetMonthCategory.md) | BudgetMonthCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetMonthCategories**](httplib.JSONResponse-models_BudgetMonthCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_month_category_rollup

> models::HttplibJsonResponseModelsBudgetMonthCategories budget_month_category_rollup(auth_household_id, year_month)
Update BudgetMonthCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_household_id** | **String** | authHouseholdID | [required] |
**year_month** | **String** | yearMonth | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetMonthCategories**](httplib.JSONResponse-models_BudgetMonthCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_month_category_update

> models::HttplibJsonResponseModelsBudgetMonthCategories budget_month_category_update(body)
Update BudgetMonthCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetMonthCategory**](ModelsBudgetMonthCategory.md) | BudgetMonthCategory | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetMonthCategories**](httplib.JSONResponse-models_BudgetMonthCategories.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

