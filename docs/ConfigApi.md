# \ConfigApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**config_apply**](ConfigApi.md#config_apply) | **POST** /config/apply | Restarts Homechart
[**config_read**](ConfigApi.md#config_read) | **GET** /config | Get current Homechart config



## config_apply

> models::ControllersResponse config_apply()
Restarts Homechart

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ControllersResponse**](controllers.Response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_read

> models::HttplibJsonResponseConfigConfig config_read()
Get current Homechart config

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseConfigConfig**](httplib.JSONResponse-config_Config.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

