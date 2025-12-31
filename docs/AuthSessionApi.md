# \AuthSessionApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_session_create**](AuthSessionApi.md#auth_session_create) | **POST** /auth/sessions | Create AuthSession
[**auth_session_delete**](AuthSessionApi.md#auth_session_delete) | **DELETE** /auth/sessions/{id} | Delete AuthSession
[**auth_session_delete_all**](AuthSessionApi.md#auth_session_delete_all) | **DELETE** /auth/sessions | Delete all AuthSessions
[**auth_session_read**](AuthSessionApi.md#auth_session_read) | **GET** /auth/sessions | Read all AuthSessions
[**auth_session_update**](AuthSessionApi.md#auth_session_update) | **PUT** /auth/sessions/{id} | Update AuthSession
[**auth_sign_in_create**](AuthSessionApi.md#auth_sign_in_create) | **POST** /auth/signin | Create AuthSession using an AuthAccount
[**auth_sign_in_read**](AuthSessionApi.md#auth_sign_in_read) | **GET** /auth/signin | Check if AuthSession is valid



## auth_session_create

> models::HttplibJsonResponseModelsAuthSessions auth_session_create(body)
Create AuthSession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthSession**](ModelsAuthSession.md) | AuthSession | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthSessions**](httplib.JSONResponse-models_AuthSessions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_session_delete

> auth_session_delete(id)
Delete AuthSession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_session_delete_all

> auth_session_delete_all()
Delete all AuthSessions

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_session_read

> models::HttplibJsonResponseModelsAuthSessions auth_session_read()
Read all AuthSessions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsAuthSessions**](httplib.JSONResponse-models_AuthSessions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_session_update

> models::HttplibJsonResponseModelsAuthSessions auth_session_update(id, body)
Update AuthSession

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthSession**](ModelsAuthSession.md) | AuthSession | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthSessions**](httplib.JSONResponse-models_AuthSessions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_sign_in_create

> models::HttplibJsonResponseModelsAuthSessions auth_sign_in_create(body)
Create AuthSession using an AuthAccount

Requires emailAddress and password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthSessions**](httplib.JSONResponse-models_AuthSessions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_sign_in_read

> models::HttplibJsonResponseModelsAuthSessions auth_sign_in_read()
Check if AuthSession is valid

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsAuthSessions**](httplib.JSONResponse-models_AuthSessions.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

