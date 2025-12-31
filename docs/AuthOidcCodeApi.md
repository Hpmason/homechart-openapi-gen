# \AuthOidcCodeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_oidc_code_read**](AuthOidcCodeApi.md#auth_oidc_code_read) | **POST** /auth/oidc/codes | Create AuthOIDCCode



## auth_oidc_code_read

> models::HttplibJsonResponseModelsAuthOidcCodes auth_oidc_code_read(body)
Create AuthOIDCCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthOidcCode**](ModelsAuthOidcCode.md) | AuthOIDCCode | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthOidcCodes**](httplib.JSONResponse-models_AuthOIDCCodes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

