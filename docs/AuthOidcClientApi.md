# \AuthOidcClientApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_oidc_client_create**](AuthOidcClientApi.md#auth_oidc_client_create) | **POST** /auth/oidc/clients | Create AuthOIDCClient
[**auth_oidc_client_delete**](AuthOidcClientApi.md#auth_oidc_client_delete) | **DELETE** /auth/oidc/clients/{id} | Delete AuthOIDCClient
[**auth_oidc_client_read**](AuthOidcClientApi.md#auth_oidc_client_read) | **GET** /auth/oidc/clients/{id} | Read AuthOIDCClient
[**auth_oidc_client_update**](AuthOidcClientApi.md#auth_oidc_client_update) | **PUT** /auth/oidc/clients/{id} | Update AuthOIDCClient
[**auth_oidc_clients_read**](AuthOidcClientApi.md#auth_oidc_clients_read) | **GET** /auth/oidc/clients | Read all AuthOIDCClients



## auth_oidc_client_create

> models::HttplibJsonResponseModelsAuthOidcClients auth_oidc_client_create(body)
Create AuthOIDCClient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthOidcClient**](ModelsAuthOidcClient.md) | AuthOIDCClient | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthOidcClients**](httplib.JSONResponse-models_AuthOIDCClients.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc_client_delete

> models::HttplibJsonResponseModelsAuthOidcClients auth_oidc_client_delete(id)
Delete AuthOIDCClient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthOidcClients**](httplib.JSONResponse-models_AuthOIDCClients.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc_client_read

> models::HttplibJsonResponseModelsAuthOidcClients auth_oidc_client_read(id)
Read AuthOIDCClient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthOidcClients**](httplib.JSONResponse-models_AuthOIDCClients.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc_client_update

> models::HttplibJsonResponseModelsAuthOidcClients auth_oidc_client_update(id, body)
Update AuthOIDCClient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthOidcClient**](ModelsAuthOidcClient.md) | AuthOIDCClient | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthOidcClients**](httplib.JSONResponse-models_AuthOIDCClients.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc_clients_read

> models::HttplibJsonResponseModelsAuthOidcClients auth_oidc_clients_read()
Read all AuthOIDCClients

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsAuthOidcClients**](httplib.JSONResponse-models_AuthOIDCClients.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

