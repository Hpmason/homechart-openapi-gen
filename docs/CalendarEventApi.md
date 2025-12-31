# \CalendarEventApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calendar_event_create**](CalendarEventApi.md#calendar_event_create) | **POST** /calendar/events | Create CalendarEvent
[**calendar_event_delete**](CalendarEventApi.md#calendar_event_delete) | **DELETE** /calendar/events/{id} | Delete CalendarEvent
[**calendar_event_read**](CalendarEventApi.md#calendar_event_read) | **GET** /calendar/events/{id} | Read CalendarEvent
[**calendar_event_update**](CalendarEventApi.md#calendar_event_update) | **PUT** /calendar/events/{id} | Update CalendarEvent
[**calendar_events_read**](CalendarEventApi.md#calendar_events_read) | **GET** /calendar/events | Read all CalendarEvents



## calendar_event_create

> models::HttplibJsonResponseModelsCalendarEvents calendar_event_create(body)
Create CalendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsCalendarEvent**](ModelsCalendarEvent.md) | CalendarEvent | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarEvents**](httplib.JSONResponse-models_CalendarEvents.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_delete

> models::HttplibJsonResponseModelsCalendarEvents calendar_event_delete(id)
Delete CalendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarEvents**](httplib.JSONResponse-models_CalendarEvents.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_read

> models::HttplibJsonResponseModelsCalendarEvents calendar_event_read(id)
Read CalendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarEvents**](httplib.JSONResponse-models_CalendarEvents.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_update

> models::HttplibJsonResponseModelsCalendarEvents calendar_event_update(id, body)
Update CalendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsCalendarEvent**](ModelsCalendarEvent.md) | CalendarEvent | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarEvents**](httplib.JSONResponse-models_CalendarEvents.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_read

> models::HttplibJsonResponseModelsCalendarEvents calendar_events_read()
Read all CalendarEvents

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCalendarEvents**](httplib.JSONResponse-models_CalendarEvents.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

