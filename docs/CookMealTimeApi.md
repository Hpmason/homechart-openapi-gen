# \CookMealTimeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cook_meal_time_create**](CookMealTimeApi.md#cook_meal_time_create) | **POST** /cook/meal-times | Create CookMealTime
[**cook_meal_time_delete**](CookMealTimeApi.md#cook_meal_time_delete) | **DELETE** /cook/meal-times/{id} | Delete CookMealTime
[**cook_meal_time_read**](CookMealTimeApi.md#cook_meal_time_read) | **GET** /cook/meal-times/{id} | Read CookMealTime
[**cook_meal_time_update**](CookMealTimeApi.md#cook_meal_time_update) | **PUT** /cook/meal-times/{id} | Update CookMealTime
[**cook_meal_times_read**](CookMealTimeApi.md#cook_meal_times_read) | **GET** /cook/meal-times | Read all CookMealTimes



## cook_meal_time_create

> models::HttplibJsonResponseModelsCookMealTimes cook_meal_time_create(body)
Create CookMealTime

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsCookMealTime**](ModelsCookMealTime.md) | CookMealTime | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealTimes**](httplib.JSONResponse-models_CookMealTimes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_time_delete

> models::HttplibJsonResponseModelsCookMealTimes cook_meal_time_delete(id)
Delete CookMealTime

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealTimes**](httplib.JSONResponse-models_CookMealTimes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_time_read

> models::HttplibJsonResponseModelsCookMealTimes cook_meal_time_read(id)
Read CookMealTime

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealTimes**](httplib.JSONResponse-models_CookMealTimes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_time_update

> models::HttplibJsonResponseModelsCookMealTimes cook_meal_time_update(id, body)
Update CookMealTime

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsCookMealTime**](ModelsCookMealTime.md) | CookMealTime | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealTimes**](httplib.JSONResponse-models_CookMealTimes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_times_read

> models::HttplibJsonResponseModelsCookMealTimes cook_meal_times_read()
Read all CookMealTimes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCookMealTimes**](httplib.JSONResponse-models_CookMealTimes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

