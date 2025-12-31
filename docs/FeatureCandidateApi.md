# \FeatureCandidateApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**feature_candidate_create**](FeatureCandidateApi.md#feature_candidate_create) | **POST** /feature/candidates | Create FeatureCandidate
[**feature_candidate_delete**](FeatureCandidateApi.md#feature_candidate_delete) | **DELETE** /features/candidates/{id} | Delete FeatureCandidate
[**feature_candidate_read**](FeatureCandidateApi.md#feature_candidate_read) | **GET** /feature/candidates/{id} | Read FeatureCandidate
[**feature_candidate_update**](FeatureCandidateApi.md#feature_candidate_update) | **PUT** /features/candidates/{id} | Update FeatureCandidate
[**feature_candidates_read**](FeatureCandidateApi.md#feature_candidates_read) | **GET** /feature/candidates | Read all FeatureCandidates



## feature_candidate_create

> models::HttplibJsonResponseModelsFeatureCandidates feature_candidate_create(body)
Create FeatureCandidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsFeatureCandidate**](ModelsFeatureCandidate.md) | FeatureCandidate | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureCandidates**](httplib.JSONResponse-models_FeatureCandidates.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_candidate_delete

> models::HttplibJsonResponseModelsFeatureCandidates feature_candidate_delete(id)
Delete FeatureCandidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureCandidates**](httplib.JSONResponse-models_FeatureCandidates.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_candidate_read

> models::HttplibJsonResponseModelsFeatureCandidates feature_candidate_read(id)
Read FeatureCandidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureCandidates**](httplib.JSONResponse-models_FeatureCandidates.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_candidate_update

> models::HttplibJsonResponseModelsFeatureCandidates feature_candidate_update(id, body)
Update FeatureCandidate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsFeatureCandidate**](ModelsFeatureCandidate.md) | FeatureCandidate | [required] |

### Return type

[**models::HttplibJsonResponseModelsFeatureCandidates**](httplib.JSONResponse-models_FeatureCandidates.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_candidates_read

> models::HttplibJsonResponseModelsFeatureCandidates feature_candidates_read()
Read all FeatureCandidates

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsFeatureCandidates**](httplib.JSONResponse-models_FeatureCandidates.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

