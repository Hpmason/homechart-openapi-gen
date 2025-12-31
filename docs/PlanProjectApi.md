# \PlanProjectApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plan_project_create**](PlanProjectApi.md#plan_project_create) | **POST** /plan/projects | Create PlanProject
[**plan_project_read**](PlanProjectApi.md#plan_project_read) | **GET** /plan/projects/{id} | Read PlanProject
[**plan_project_update**](PlanProjectApi.md#plan_project_update) | **PUT** /plan/projects/{id} | Update PlanProject
[**plan_projects_read**](PlanProjectApi.md#plan_projects_read) | **GET** /plan/projects | Read all PlanProjects



## plan_project_create

> models::HttplibJsonResponseModelsPlanProjects plan_project_create(body)
Create PlanProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPlanProject**](ModelsPlanProject.md) | PlanProject | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanProjects**](httplib.JSONResponse-models_PlanProjects.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_project_read

> models::HttplibJsonResponseModelsPlanProjects plan_project_read(id)
Read PlanProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanProjects**](httplib.JSONResponse-models_PlanProjects.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_project_update

> models::HttplibJsonResponseModelsPlanProjects plan_project_update(id, body)
Update PlanProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsPlanProject**](ModelsPlanProject.md) | PlanProject | [required] |

### Return type

[**models::HttplibJsonResponseModelsPlanProjects**](httplib.JSONResponse-models_PlanProjects.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plan_projects_read

> models::HttplibJsonResponseModelsPlanProjects plan_projects_read()
Read all PlanProjects

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsPlanProjects**](httplib.JSONResponse-models_PlanProjects.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

