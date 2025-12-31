# \CalendarICalendarApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calendar_i_calendar_create**](CalendarICalendarApi.md#calendar_i_calendar_create) | **POST** /calendar/icalendars | Create CalendarICalendar
[**calendar_i_calendar_delete**](CalendarICalendarApi.md#calendar_i_calendar_delete) | **DELETE** /calendar/icalendars/{id} | Delete CalendarICalendar
[**calendar_i_calendar_read**](CalendarICalendarApi.md#calendar_i_calendar_read) | **GET** /calendar/icalendars/{id} | Read CalendarICalendar
[**calendar_i_calendar_update**](CalendarICalendarApi.md#calendar_i_calendar_update) | **PUT** /calendar/icalendars/{id} | Update CalendarICalendar
[**calendar_i_calendars_read**](CalendarICalendarApi.md#calendar_i_calendars_read) | **GET** /calendar/icalendars | Read all CalendarICalendars



## calendar_i_calendar_create

> models::HttplibJsonResponseModelsCalendarICalendars calendar_i_calendar_create(body)
Create CalendarICalendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsCalendarICalendar**](ModelsCalendarICalendar.md) | CalendarICalendar | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarICalendars**](httplib.JSONResponse-models_CalendarICalendars.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_i_calendar_delete

> models::HttplibJsonResponseModelsCalendarICalendars calendar_i_calendar_delete(id)
Delete CalendarICalendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarICalendars**](httplib.JSONResponse-models_CalendarICalendars.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_i_calendar_read

> models::HttplibJsonResponseModelsCalendarICalendars calendar_i_calendar_read(id)
Read CalendarICalendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarICalendars**](httplib.JSONResponse-models_CalendarICalendars.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_i_calendar_update

> models::HttplibJsonResponseModelsCalendarICalendars calendar_i_calendar_update(id, body)
Update CalendarICalendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsCalendarICalendar**](ModelsCalendarICalendar.md) | CalendarICalendar | [required] |

### Return type

[**models::HttplibJsonResponseModelsCalendarICalendars**](httplib.JSONResponse-models_CalendarICalendars.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_i_calendars_read

> models::HttplibJsonResponseModelsCalendarICalendars calendar_i_calendars_read()
Read all CalendarICalendars

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCalendarICalendars**](httplib.JSONResponse-models_CalendarICalendars.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

