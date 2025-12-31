# \BudgetRecurrenceApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budget_recurrence_create**](BudgetRecurrenceApi.md#budget_recurrence_create) | **POST** /budget/recurrences | Create BudgetRecurrence
[**budget_recurrence_delete**](BudgetRecurrenceApi.md#budget_recurrence_delete) | **DELETE** /budget/recurrences/{id} | Delete BudgetRecurrences
[**budget_recurrence_read**](BudgetRecurrenceApi.md#budget_recurrence_read) | **GET** /budget/recurrences/{id} | Read BudgetRecurrences
[**budget_recurrence_update**](BudgetRecurrenceApi.md#budget_recurrence_update) | **PUT** /budget/recurrences/{id} | Update BudgetRecurrence
[**budget_recurrences_read**](BudgetRecurrenceApi.md#budget_recurrences_read) | **GET** /budget/recurrences | Read all BudgetRecurrences



## budget_recurrence_create

> models::HttplibJsonResponseModelsBudgetRecurrences budget_recurrence_create(body)
Create BudgetRecurrence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBudgetRecurrence**](ModelsBudgetRecurrence.md) | BudgetRecurrence | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetRecurrences**](httplib.JSONResponse-models_BudgetRecurrences.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_recurrence_delete

> models::HttplibJsonResponseModelsBudgetRecurrences budget_recurrence_delete(id)
Delete BudgetRecurrences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetRecurrences**](httplib.JSONResponse-models_BudgetRecurrences.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_recurrence_read

> models::HttplibJsonResponseModelsBudgetRecurrences budget_recurrence_read(id)
Read BudgetRecurrences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetRecurrences**](httplib.JSONResponse-models_BudgetRecurrences.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_recurrence_update

> models::HttplibJsonResponseModelsBudgetRecurrences budget_recurrence_update(id, body)
Update BudgetRecurrence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBudgetRecurrence**](ModelsBudgetRecurrence.md) | BudgetRecurrence | [required] |

### Return type

[**models::HttplibJsonResponseModelsBudgetRecurrences**](httplib.JSONResponse-models_BudgetRecurrences.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## budget_recurrences_read

> models::HttplibJsonResponseModelsBudgetRecurrences budget_recurrences_read()
Read all BudgetRecurrences

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsBudgetRecurrences**](httplib.JSONResponse-models_BudgetRecurrences.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

