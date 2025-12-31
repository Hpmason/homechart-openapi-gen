# \SecretsVaultApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secrets_vault_create**](SecretsVaultApi.md#secrets_vault_create) | **POST** /secrets/vaults | Create SecretsVault
[**secrets_vault_delete**](SecretsVaultApi.md#secrets_vault_delete) | **DELETE** /secrets/vaults/{id} | Delete SecretsVault
[**secrets_vault_read**](SecretsVaultApi.md#secrets_vault_read) | **GET** /secrets/vaults/{id} | Read SecretsVault
[**secrets_vault_update**](SecretsVaultApi.md#secrets_vault_update) | **PUT** /secrets/vaults/{id} | Update SecretsVault
[**secrets_vaults_read**](SecretsVaultApi.md#secrets_vaults_read) | **GET** /secrets/vaults | Read all SecretsVaults



## secrets_vault_create

> models::HttplibJsonResponseModelsSecretsVaults secrets_vault_create(body)
Create SecretsVault

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsSecretsVault**](ModelsSecretsVault.md) | SecretsVault | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsVaults**](httplib.JSONResponse-models_SecretsVaults.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_vault_delete

> models::HttplibJsonResponseModelsSecretsVaults secrets_vault_delete(id)
Delete SecretsVault

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsVaults**](httplib.JSONResponse-models_SecretsVaults.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_vault_read

> models::HttplibJsonResponseModelsSecretsVaults secrets_vault_read(id)
Read SecretsVault

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsVaults**](httplib.JSONResponse-models_SecretsVaults.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_vault_update

> models::HttplibJsonResponseModelsSecretsVaults secrets_vault_update(id, body)
Update SecretsVault

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsSecretsVault**](ModelsSecretsVault.md) | SecretsVault | [required] |

### Return type

[**models::HttplibJsonResponseModelsSecretsVaults**](httplib.JSONResponse-models_SecretsVaults.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_vaults_read

> models::HttplibJsonResponseModelsSecretsVaults secrets_vaults_read()
Read all SecretsVaults

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsSecretsVaults**](httplib.JSONResponse-models_SecretsVaults.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

