# \CookRecipeApi

All URIs are relative to */api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cook_recipe_create**](CookRecipeApi.md#cook_recipe_create) | **POST** /cook/recipes | Create CookRecipe
[**cook_recipe_delete**](CookRecipeApi.md#cook_recipe_delete) | **DELETE** /cook/recipes/{id} | Delete CookRecipe
[**cook_recipe_read**](CookRecipeApi.md#cook_recipe_read) | **GET** /cook/recipes/{id} | Read CookRecipe
[**cook_recipe_update**](CookRecipeApi.md#cook_recipe_update) | **PUT** /cook/recipes/{id} | Update CookRecipe
[**cook_recipes_read**](CookRecipeApi.md#cook_recipes_read) | **GET** /cook/recipes | Read all CookRecipes



## cook_recipe_create

> models::HttplibJsonResponseModelsCookRecipes cook_recipe_create(body)
Create CookRecipe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsCookRecipe**](ModelsCookRecipe.md) | CookRecipe | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookRecipes**](httplib.JSONResponse-models_CookRecipes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_recipe_delete

> models::HttplibJsonResponseModelsCookRecipes cook_recipe_delete(id)
Delete CookRecipe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookRecipes**](httplib.JSONResponse-models_CookRecipes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_recipe_read

> models::HttplibJsonResponseModelsCookRecipes cook_recipe_read(id)
Read CookRecipe

Can read recipes marked public unauthenticated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookRecipes**](httplib.JSONResponse-models_CookRecipes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_recipe_update

> models::HttplibJsonResponseModelsCookRecipes cook_recipe_update(id, body)
Update CookRecipe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID | [required] |
**body** | [**ModelsCookRecipe**](ModelsCookRecipe.md) | CookRecipe | [required] |

### Return type

[**models::HttplibJsonResponseModelsCookRecipes**](httplib.JSONResponse-models_CookRecipes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cook_recipes_read

> models::HttplibJsonResponseModelsCookRecipes cook_recipes_read()
Read all CookRecipes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HttplibJsonResponseModelsCookRecipes**](httplib.JSONResponse-models_CookRecipes.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

