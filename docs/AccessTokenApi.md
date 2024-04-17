# \AccessTokenApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_access_token**](AccessTokenApi.md#get_access_token) | **GET** /access_token | Get Access Token



## get_access_token

> models::AccessToken get_access_token(email_address, password)
Get Access Token

Returns the access token for a specific user. Requires API User permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_address** | **String** | Email address of the user | [required] |
**password** | **String** | Password of the user | [required] |

### Return type

[**models::AccessToken**](AccessToken.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

