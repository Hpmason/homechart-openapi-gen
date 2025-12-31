# \SseApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**s_se_read**](SseApi.md#s_se_read) | **GET** /sse/{id} | Listen for events via SSE



## s_se_read

> models::ModelsEvent s_se_read(id)
Listen for events via SSE

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | AuthSessionID | [required] |

### Return type

[**models::ModelsEvent**](models.Event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

