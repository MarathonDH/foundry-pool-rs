# \SubAccountSettingsApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](SubAccountSettingsApi.md#create_api_key) | **PUT** /api_keys/{subAccountName} | Create API key
[**delete_api_key**](SubAccountSettingsApi.md#delete_api_key) | **DELETE** /api_keys/{subAccountName}/{apiKey} | Delete API key
[**get_api_key**](SubAccountSettingsApi.md#get_api_key) | **GET** /api_keys/{subAccountName} | Get API keys



## create_api_key

> create_api_key(sub_account_name, authorization, key_name, role)
Create API key

Create API key for a given sub-account. Requires sub-account owner authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of sub-account. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**key_name** | Option<**String**> | This can be any text you want under 100 characters. It's intended for describing who you'll be sharing the key with. Default value is \"default\". |  |[default to default]
**role** | Option<**String**> | Valid values are \"technician\" (permission to view hashrate and workers) and \"accountant\" (permission to view hashrate, workers, and financial data). Default value is \"technician\". |  |[default to technician]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(sub_account_name, api_key, authorization)
Delete API key

Delete API key for a given sub-account. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of sub-account. | [required] |
**api_key** | **String** | API key to delete. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> get_api_key(sub_account_name, authorization, role)
Get API keys

Get list of API keys for a given sub-account. Requires sub-account owner authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of sub-account. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**role** | Option<**String**> | Valid values are \"technician\" (permission to view hashrate and workers) and \"accountant\" (permission to view hashrate, workers, and financial data). Default value is \"technician\". |  |[default to technician]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

