# \AuthAccountApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_account_create**](AuthAccountApi.md#auth_account_create) | **POST** /auth/accounts | Create AuthAccount
[**auth_account_delete**](AuthAccountApi.md#auth_account_delete) | **DELETE** /auth/accounts/{id} | Delete AuthAccount
[**auth_account_keys_update**](AuthAccountApi.md#auth_account_keys_update) | **PUT** /auth/accounts/{id}/keys | Update AuthAccount
[**auth_account_oidc_update**](AuthAccountApi.md#auth_account_oidc_update) | **PUT** /auth/accounts/{id}/oidc | Update AuthAccount OIDC
[**auth_account_password_update**](AuthAccountApi.md#auth_account_password_update) | **PUT** /auth/accounts/{id}/password | Update AuthAccount Password
[**auth_account_read**](AuthAccountApi.md#auth_account_read) | **GET** /auth/accounts/{id} | Read AuthAccount
[**auth_account_totp_create**](AuthAccountApi.md#auth_account_totp_create) | **POST** /auth/accounts/{id}/totp | Generate an initial TOTP token
[**auth_account_totp_read**](AuthAccountApi.md#auth_account_totp_read) | **GET** /auth/accounts/{id}/totp | Read TOTP backup token
[**auth_account_totp_update**](AuthAccountApi.md#auth_account_totp_update) | **PUT** /auth/accounts/{id}/totp | Confirm TOTP token works by sending a code
[**auth_account_update**](AuthAccountApi.md#auth_account_update) | **PUT** /auth/accounts/{id} | Update AuthAccount
[**auth_account_verify_read**](AuthAccountApi.md#auth_account_verify_read) | **GET** /auth/verify | Resend AuthAccount verification email
[**auth_account_verify_update**](AuthAccountApi.md#auth_account_verify_update) | **PUT** /auth/verify/{id}/{token} | Verifies an AuthAccount using a verification token
[**auth_account_web_authn_read**](AuthAccountApi.md#auth_account_web_authn_read) | **GET** /auth/accounts/{id}/webauthn | Get WebAuthn creation options
[**auth_account_web_authn_update**](AuthAccountApi.md#auth_account_web_authn_update) | **PUT** /auth/accounts/{id}/webauthn | Update WebAuthn credentials
[**auth_accounts_read**](AuthAccountApi.md#auth_accounts_read) | **GET** /auth/accounts | Read all AuthAccounts
[**auth_household_delete**](AuthAccountApi.md#auth_household_delete) | **DELETE** /auth/households/{id} | Delete AuthHousehold
[**auth_methods_create**](AuthAccountApi.md#auth_methods_create) | **POST** /auth/methods | List AuthMethods for an account.



## auth_account_create

> models::HttplibJsonResponseModelsAuthSessions auth_account_create(body)
Create AuthAccount

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


## auth_account_delete

> auth_account_delete(id)
Delete AuthAccount

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


## auth_account_keys_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_keys_update(id, body)
Update AuthAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_oidc_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_oidc_update(id)
Update AuthAccount OIDC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_password_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_password_update(id)
Update AuthAccount Password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_read

> models::HttplibJsonResponseModelsAuthAccounts auth_account_read(id)
Read AuthAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_totp_create

> models::HttplibJsonResponseModelsAuthAccounts auth_account_totp_create(id)
Generate an initial TOTP token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_totp_read

> models::HttplibJsonResponseModelsAuthAccounts auth_account_totp_read(id)
Read TOTP backup token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_totp_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_totp_update(id, body)
Confirm TOTP token works by sending a code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_update(id, body)
Update AuthAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_verify_read

> models::HttplibJsonResponseModelsAuthAccounts auth_account_verify_read(body)
Resend AuthAccount verification email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_verify_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_verify_update(id, token)
Verifies an AuthAccount using a verification token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**token** | **String** | Verification token | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_web_authn_read

> models::HttplibJsonResponseWebauthnCredentialCreationOpts auth_account_web_authn_read(id)
Get WebAuthn creation options

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseWebauthnCredentialCreationOpts**](httplib.JSONResponse-webauthn_CredentialCreationOpts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_account_web_authn_update

> models::HttplibJsonResponseModelsAuthAccounts auth_account_web_authn_update(id, body)
Update WebAuthn credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_accounts_read

> models::HttplibJsonResponseModelsAuthAccounts auth_accounts_read()
Read all AuthAccounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsAuthAccounts**](httplib.JSONResponse-models_AuthAccounts.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_delete

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_delete(id)
Delete AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_methods_create

> models::HttplibJsonResponseModelsAuthSessions auth_methods_create(body)
List AuthMethods for an account.

Requires emailAddress

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

