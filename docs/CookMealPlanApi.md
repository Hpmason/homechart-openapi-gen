# \CookMealPlanApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cook_meal_plan_create**](CookMealPlanApi.md#cook_meal_plan_create) | **POST** /cook/meal-plans | Create CookMealPlan
[**cook_meal_plan_delete**](CookMealPlanApi.md#cook_meal_plan_delete) | **DELETE** /cook/meal-plans/{id} | Delete CookMealPlan
[**cook_meal_plan_read**](CookMealPlanApi.md#cook_meal_plan_read) | **GET** /cook/meal-plans/{id} | Read CookMealPlan
[**cook_meal_plan_update**](CookMealPlanApi.md#cook_meal_plan_update) | **PUT** /cook/meal-plans/{id} | Update CookMealPlan
[**cook_meal_plans_read**](CookMealPlanApi.md#cook_meal_plans_read) | **GET** /cook/meal-plans | Read all CookMealPlans between from and to



## cook_meal_plan_create

> models::HttplibJsonResponseModelsCookMealPlans cook_meal_plan_create(body)
Create CookMealPlan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsCookMealPlan**](ModelsCookMealPlan.md) | CookMealPlan | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_plan_delete

> models::HttplibJsonResponseModelsCookMealPlans cook_meal_plan_delete(id)
Delete CookMealPlan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_plan_read

> models::HttplibJsonResponseModelsCookMealPlans cook_meal_plan_read(id)
Read CookMealPlan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_plan_update

> models::HttplibJsonResponseModelsCookMealPlans cook_meal_plan_update(id, body)
Update CookMealPlan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsCookMealPlan**](ModelsCookMealPlan.md) | CookMealPlan | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_meal_plans_read

> models::HttplibJsonResponseModelsCookMealPlans cook_meal_plans_read()
Read all CookMealPlans between from and to

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCookMealPlans**](httplib.JSONResponse-models_CookMealPlans.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

