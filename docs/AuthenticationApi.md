# \AuthenticationApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authentication_test**](AuthenticationApi.md#authentication_test) | **GET** /authentication_test | Authentication Test



## authentication_test

> String authentication_test(authorization, x_api_key)
Authentication Test

Please follow these instructions for all endpoints that require authentication. You may generate an API key by creating a \"Viewer Link\" in the pool Sub-Accounts page. Use either an API key or access token (not both) to authenticate. API Keys are intended for pool customers while access tokens are intended for internal use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

