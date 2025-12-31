# \AuthAccountDelegationApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_account_delegation_create**](AuthAccountDelegationApi.md#auth_account_delegation_create) | **POST** /auth/accounts/{authAccountID}/delegation | Create AuthAccountDelegation
[**auth_account_delegation_delete**](AuthAccountDelegationApi.md#auth_account_delegation_delete) | **DELETE** /auth/accounts/{authAccountID}/delegation/{id} | Delete AuthAccountDelegation
[**auth_account_delegation_update**](AuthAccountDelegationApi.md#auth_account_delegation_update) | **PUT** /auth/accounts/{authAccountID}/delegation/{id} | Update AuthAccountDelegation



## auth_account_delegation_create

> models::HttplibJsonResponseModelsAuthAccountDelegations auth_account_delegation_create(auth_account_id, body)
Create AuthAccountDelegation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_account_id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccountDelegation**](ModelsAuthAccountDelegation.md) | AuthAccountDelegation | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccountDelegations**](httplib.JSONResponse-models_AuthAccountDelegations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_delegation_delete

> models::HttplibJsonResponseModelsAuthAccountDelegations auth_account_delegation_delete(auth_account_id, id)
Delete AuthAccountDelegation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_account_id** | **String** | ID | [required] |
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccountDelegations**](httplib.JSONResponse-models_AuthAccountDelegations.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_delegation_update

> models::HttplibJsonResponseModelsAuthAccountDelegations auth_account_delegation_update(auth_account_id, id, body)
Update AuthAccountDelegation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_account_id** | **String** | ID | [required] |
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccountDelegation**](ModelsAuthAccountDelegation.md) | AuthAccountDelegation | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccountDelegations**](httplib.JSONResponse-models_AuthAccountDelegations.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

