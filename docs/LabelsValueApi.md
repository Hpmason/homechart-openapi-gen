# \LabelsValueApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**labels_value_create**](LabelsValueApi.md#labels_value_create) | **POST** /label/values | Create LabelsValue
[**labels_value_delete**](LabelsValueApi.md#labels_value_delete) | **DELETE** /label/values/{id} | Delete LabelsValue
[**labels_value_read**](LabelsValueApi.md#labels_value_read) | **GET** /label/values/{id} | Read LabelsValue
[**labels_value_update**](LabelsValueApi.md#labels_value_update) | **PUT** /label/values/{id} | Update LabelsValue
[**labels_values_read**](LabelsValueApi.md#labels_values_read) | **GET** /label/values | Read all LabelsValues



## labels_value_create

> models::HttplibJsonResponseModelsLabelsValues labels_value_create(body)
Create LabelsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsLabelsValue**](ModelsLabelsValue.md) | LabelsValue | [required] |

### Return type

[**models::HttplibJsonResponseModelsLabelsValues**](httplib.JSONResponse-models_LabelsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## labels_value_delete

> models::HttplibJsonResponseModelsLabelsValues labels_value_delete(id)
Delete LabelsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsLabelsValues**](httplib.JSONResponse-models_LabelsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## labels_value_read

> models::HttplibJsonResponseModelsLabelsValues labels_value_read(id)
Read LabelsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsLabelsValues**](httplib.JSONResponse-models_LabelsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## labels_value_update

> models::HttplibJsonResponseModelsLabelsValues labels_value_update(id, body)
Update LabelsValue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsLabelsValue**](ModelsLabelsValue.md) | LabelsValue | [required] |

### Return type

[**models::HttplibJsonResponseModelsLabelsValues**](httplib.JSONResponse-models_LabelsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## labels_values_read

> models::HttplibJsonResponseModelsLabelsValues labels_values_read()
Read all LabelsValues

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsLabelsValues**](httplib.JSONResponse-models_LabelsValues.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

