# \AuthHouseholdApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_household_create**](AuthHouseholdApi.md#auth_household_create) | **POST** /auth/households | Create AuthHousehold
[**auth_household_invite_accept**](AuthHouseholdApi.md#auth_household_invite_accept) | **GET** /auth/households/{id}/invites/{token} | Accept an invite to an AuthHousehold
[**auth_household_invite_create**](AuthHouseholdApi.md#auth_household_invite_create) | **POST** /auth/households/{id}/invites | Invite an AuthAccount to an AuthHousehold
[**auth_household_invite_delete**](AuthHouseholdApi.md#auth_household_invite_delete) | **DELETE** /auth/households/{id}/invites/{token} | Accept an invite to an AuthHousehold
[**auth_household_member_delete**](AuthHouseholdApi.md#auth_household_member_delete) | **DELETE** /auth/households/{auth_household_id}/members/{auth_account_id} | Delete AuthHouseholdMember
[**auth_household_member_update**](AuthHouseholdApi.md#auth_household_member_update) | **PUT** /auth/households/{auth_household_id}/members/{auth_account_id} | Update AuthHouseholdMember EmailAddress and Permissions
[**auth_household_read**](AuthHouseholdApi.md#auth_household_read) | **GET** /auth/households/{id} | Read AuthHousehold
[**auth_household_update**](AuthHouseholdApi.md#auth_household_update) | **PUT** /auth/households/{id} | Update AuthHousehold
[**auth_households_read**](AuthHouseholdApi.md#auth_households_read) | **GET** /auth/households | Read all AuthHouseholds



## auth_household_create

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_create(body)
Create AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsAuthHousehold**](ModelsAuthHousehold.md) | AuthHousehold | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_invite_accept

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_invite_accept(id, token)
Accept an invite to an AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**token** | **String** | Token | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_invite_create

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_invite_create(id, body)
Invite an AuthAccount to an AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccountAuthHousehold | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_invite_delete

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_invite_delete(id, token)
Accept an invite to an AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**token** | **String** | Token | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_member_delete

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_member_delete(auth_account_id, auth_household_id)
Delete AuthHouseholdMember

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_account_id** | **String** | AuthAccountID | [required] |
**auth_household_id** | **String** | AuthHouseholdID | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_member_update

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_member_update(auth_account_id, auth_household_id, body)
Update AuthHouseholdMember EmailAddress and Permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_account_id** | **String** | AuthAccountID | [required] |
**auth_household_id** | **String** | AuthHouseholdID | [required] |
**body** | [**ModelsAuthAccount**](ModelsAuthAccount.md) | AuthAccount | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_household_read

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_read(id)
Read AuthHousehold

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


## auth_household_update

> models::HttplibJsonResponseModelsAuthHouseholds auth_household_update(id, body)
Update AuthHousehold

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsAuthHousehold**](ModelsAuthHousehold.md) | AuthHousehold | [required] |

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_households_read

> models::HttplibJsonResponseModelsAuthHouseholds auth_households_read()
Read all AuthHouseholds

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsAuthHouseholds**](httplib.JSONResponse-models_AuthHouseholds.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

