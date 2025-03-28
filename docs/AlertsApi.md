# \AlertsApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_sub_account_alert**](AlertsApi.md#create_new_sub_account_alert) | **POST** /alerts/addSubAccount/{coinString}/{subAccountName} | Create Alert For A Sub-Account
[**delete_alert**](AlertsApi.md#delete_alert) | **DELETE** /alerts/{coinString}/{subAccountName}/{alertId} | Delete Alert
[**get_sub_account_alerts**](AlertsApi.md#get_sub_account_alerts) | **GET** /alerts/subaccount_alerts/{userId} | Get All Alerts
[**update_sub_account_alert**](AlertsApi.md#update_sub_account_alert) | **PUT** /alerts/updateSubAccountAlert/{coinString}/{subAccountName}/{alertId} | Updating Existing Sub-Account Alert



## create_new_sub_account_alert

> i32 create_new_sub_account_alert(coin_string, sub_account_name, request_body, authorization, hashrate_threshold, active_workers_threshold)
Create Alert For A Sub-Account

Creates an alert for a specified sub-account. ID of created alert is returned. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**request_body** | [**Vec<String>**](String.md) | List of email addresses to be notified when the alert is triggered. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**hashrate_threshold** | Option<**i64**> | Hashrate threshold for the alert in GH/s. |  |[default to 0]
**active_workers_threshold** | Option<**i32**> | Active Workers threshold for the alert. |  |[default to 0]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert

> delete_alert(coin_string, sub_account_name, alert_id, authorization)
Delete Alert

Delete an alert. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**alert_id** | **i32** | Alert ID | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_account_alerts

> models::AlertsListResponse get_sub_account_alerts(user_id, authorization, x_api_key, coin_string, group_ids_list, triggered_only, page_number, page_size, sort)
Get All Alerts

Get alerts for all sub-accounts in selected group id(s) or in all the user's groups, if no group ids are submitted in request. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID is optional. Defaults to logged in user | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin_string** | Option<**String**> | Coin |  |[default to BTC]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | List of group ids. |  |
**triggered_only** | Option<**bool**> | triggeredOnly is optional. It's a boolean that determines if all alerts are returned or only triggered alerts. Defaults to 'false' so all alerts are returned. |  |[default to false]
**page_number** | Option<**i32**> | Valid values are 0 and positive integers. Default value is 0. |  |[default to 0]
**page_size** | Option<**i32**> | Valid values are -1 (representing max size) and positive integers. Default value is -1. |  |[default to -1]
**sort** | Option<**String**> | Valid values are \"ascSubAccount\" and \"descSubAccount\". Default value is ascSubAccount. |  |[default to ascSubAccount]

### Return type

[**models::AlertsListResponse**](AlertsListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sub_account_alert

> i32 update_sub_account_alert(coin_string, sub_account_name, alert_id, request_body, authorization, hashrate_threshold, active_workers_threshold)
Updating Existing Sub-Account Alert

Updates an existing sub-account alert. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**alert_id** | **i32** | Alert ID | [required] |
**request_body** | [**Vec<String>**](String.md) | List of email addresses to be notified when the alert is triggered. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**hashrate_threshold** | Option<**i64**> | Hashrate threshold for the alert in GH/s. |  |[default to 0]
**active_workers_threshold** | Option<**i32**> | Active Workers threshold for the alert. |  |[default to 0]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

