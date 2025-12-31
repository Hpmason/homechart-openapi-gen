# \PlanTaskApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plan_task_create**](PlanTaskApi.md#plan_task_create) | **POST** /plan/tasks | Create PlanTask
[**plan_task_delete**](PlanTaskApi.md#plan_task_delete) | **DELETE** /plan/tasks/{id} | Delete PlanTask
[**plan_task_read**](PlanTaskApi.md#plan_task_read) | **GET** /plan/tasks/{id} | Read PlanTask
[**plan_task_update**](PlanTaskApi.md#plan_task_update) | **PUT** /plan/tasks/{id} | Update PlanTask
[**plan_tasks_read**](PlanTaskApi.md#plan_tasks_read) | **GET** /plan/tasks | Read all PlanTasks



## plan_task_create

> models::HttplibJsonResponseModelsPlanTasks plan_task_create(body)
Create PlanTask

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPlanTask**](ModelsPlanTask.md) | PlanTask | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanTasks**](httplib.JSONResponse-models_PlanTasks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_task_delete

> models::HttplibJsonResponseModelsPlanTasks plan_task_delete(id)
Delete PlanTask

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanTasks**](httplib.JSONResponse-models_PlanTasks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_task_read

> models::HttplibJsonResponseModelsPlanTasks plan_task_read(id)
Read PlanTask

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanTasks**](httplib.JSONResponse-models_PlanTasks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_task_update

> models::HttplibJsonResponseModelsPlanTasks plan_task_update(id, body)
Update PlanTask

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsPlanTask**](ModelsPlanTask.md) | PlanTask | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanTasks**](httplib.JSONResponse-models_PlanTasks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_tasks_read

> models::HttplibJsonResponseModelsPlanTasks plan_tasks_read()
Read all PlanTasks

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsPlanTasks**](httplib.JSONResponse-models_PlanTasks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

