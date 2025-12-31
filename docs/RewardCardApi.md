# \RewardCardApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reward_card_create**](RewardCardApi.md#reward_card_create) | **POST** /reward/cards | Create RewardCard
[**reward_card_delete**](RewardCardApi.md#reward_card_delete) | **DELETE** /reward/cards/{id} | Delete RewardCard
[**reward_card_read**](RewardCardApi.md#reward_card_read) | **GET** /reward/cards/{id} | Read RewardCard
[**reward_card_update**](RewardCardApi.md#reward_card_update) | **PUT** /reward/cards/{id} | Update RewardCard
[**reward_cards_read**](RewardCardApi.md#reward_cards_read) | **GET** /reward/cards | Read all RewardCards



## reward_card_create

> models::HttplibJsonResponseModelsRewardCards reward_card_create(body)
Create RewardCard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsRewardCard**](ModelsRewardCard.md) | RewardCard | [required] |

### Return type

[**models::HttplibJsonResponseModelsRewardCards**](httplib.JSONResponse-models_RewardCards.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reward_card_delete

> models::HttplibJsonResponseModelsRewardCards reward_card_delete(id)
Delete RewardCard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsRewardCards**](httplib.JSONResponse-models_RewardCards.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reward_card_read

> models::HttplibJsonResponseModelsRewardCards reward_card_read(id)
Read RewardCard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsRewardCards**](httplib.JSONResponse-models_RewardCards.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reward_card_update

> models::HttplibJsonResponseModelsRewardCards reward_card_update(id, body)
Update RewardCard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsRewardCard**](ModelsRewardCard.md) | RewardCard | [required] |

### Return type

[**models::HttplibJsonResponseModelsRewardCards**](httplib.JSONResponse-models_RewardCards.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reward_cards_read

> models::HttplibJsonResponseModelsRewardCards reward_cards_read()
Read all RewardCards

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsRewardCards**](httplib.JSONResponse-models_RewardCards.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

