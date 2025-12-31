# \FeatureVoteApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**feature_vote_create**](FeatureVoteApi.md#feature_vote_create) | **POST** /feature/votes | Create FeatureVote
[**feature_vote_delete**](FeatureVoteApi.md#feature_vote_delete) | **DELETE** /feature/votes/{id} | Delete FeatureVote
[**feature_vote_read**](FeatureVoteApi.md#feature_vote_read) | **GET** /feature/votes/{id} | Read FeatureVote
[**feature_vote_update**](FeatureVoteApi.md#feature_vote_update) | **PUT** /feature/votes/{id} | Update FeatureVote
[**feature_votes_read**](FeatureVoteApi.md#feature_votes_read) | **GET** /feature/votes | Read all FeatureVotes



## feature_vote_create

> models::HttplibJsonResponseModelsFeatureVotes feature_vote_create(body)
Create FeatureVote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsFeatureVote**](ModelsFeatureVote.md) | FeatureVote | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureVotes**](httplib.JSONResponse-models_FeatureVotes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_vote_delete

> models::HttplibJsonResponseModelsFeatureVotes feature_vote_delete(id)
Delete FeatureVote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureVotes**](httplib.JSONResponse-models_FeatureVotes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_vote_read

> models::HttplibJsonResponseModelsFeatureVotes feature_vote_read(id)
Read FeatureVote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureVotes**](httplib.JSONResponse-models_FeatureVotes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_vote_update

> models::HttplibJsonResponseModelsFeatureVotes feature_vote_update(id, body)
Update FeatureVote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsFeatureVote**](ModelsFeatureVote.md) | FeatureVote | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureVotes**](httplib.JSONResponse-models_FeatureVotes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_votes_read

> models::HttplibJsonResponseModelsFeatureVotes feature_votes_read()
Read all FeatureVotes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsFeatureVotes**](httplib.JSONResponse-models_FeatureVotes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

