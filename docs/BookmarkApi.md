# \BookmarkApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bookmark_create**](BookmarkApi.md#bookmark_create) | **POST** /bookmarks | Create Bookmark
[**bookmark_delete**](BookmarkApi.md#bookmark_delete) | **DELETE** /bookmarks/{id} | Delete Bookmark
[**bookmark_read**](BookmarkApi.md#bookmark_read) | **GET** /bookmarks/{id} | Read Bookmark
[**bookmark_update**](BookmarkApi.md#bookmark_update) | **PUT** /bookmarks/{id} | Update Bookmark
[**bookmarks_read**](BookmarkApi.md#bookmarks_read) | **GET** /bookmarks | Read all Bookmarks



## bookmark_create

> models::HttplibJsonResponseModelsBookmarks bookmark_create(body)
Create Bookmark

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsBookmark**](ModelsBookmark.md) | Bookmark | [required] |

### Return type

[**models::HttplibJsonResponseModelsBookmarks**](httplib.JSONResponse-models_Bookmarks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookmark_delete

> models::HttplibJsonResponseModelsBookmarks bookmark_delete(id)
Delete Bookmark

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBookmarks**](httplib.JSONResponse-models_Bookmarks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookmark_read

> models::HttplibJsonResponseModelsBookmarks bookmark_read(id)
Read Bookmark

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsBookmarks**](httplib.JSONResponse-models_Bookmarks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookmark_update

> models::HttplibJsonResponseModelsBookmarks bookmark_update(id, body)
Update Bookmark

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsBookmark**](ModelsBookmark.md) | Bookmark | [required] |

### Return type

[**models::HttplibJsonResponseModelsBookmarks**](httplib.JSONResponse-models_Bookmarks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookmarks_read

> models::HttplibJsonResponseModelsBookmarks bookmarks_read()
Read all Bookmarks

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsBookmarks**](httplib.JSONResponse-models_Bookmarks.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

