# \AlertsApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_email_to_alert**](AlertsApi.md#add_email_to_alert) | **PUT** /alerts/addEmail/{coinString}/{subAccountName}/{alertId}/{emailAddress} | Add Email To Alert
[**create_tag_alert**](AlertsApi.md#create_tag_alert) | **PUT** /alerts/tagAlerts/{coinString}/{subAccountName}/{tagName} | Create Alert For A Tag
[**delete_alert**](AlertsApi.md#delete_alert) | **DELETE** /alerts/{coinString}/{subAccountName}/{alertId} | Delete Alert
[**get_tag_alerts**](AlertsApi.md#get_tag_alerts) | **GET** /alerts/tagAlerts/{coinString}/{subAccountName} | Get Tag Alerts
[**remove_email_from_alert**](AlertsApi.md#remove_email_from_alert) | **DELETE** /alerts/removeEmail/{coinString}/{subAccountName}/{alertId}/{emailAddress} | Remove Email From The Alert



## add_email_to_alert

> i32 add_email_to_alert(coin_string, sub_account_name, alert_id, email_address, authorization, x_api_key)
Add Email To Alert

Add an email address to existing alert. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**alert_id** | **i32** | Alert ID | [required] |
**email_address** | **String** | Email Address to be added to the alert. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tag_alert

> i32 create_tag_alert(coin_string, sub_account_name, tag_name, request_body, authorization, x_api_key, hashrate_threshold, active_workers_threshold)
Create Alert For A Tag

Creates an alert for a specified tag. ID of created alert is returned. Note: You can also add an alert for All workers by supplying tagName value as 'all'. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**tag_name** | **String** | Tag Name | [required] |
**request_body** | [**Vec<String>**](String.md) | List of email addresses to be notified when the alert is triggered. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
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

> delete_alert(coin_string, sub_account_name, alert_id, authorization, x_api_key)
Delete Alert

Delete an alert. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**alert_id** | **i32** | Alert ID | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_alerts

> models::TagAlertsResponse get_tag_alerts(coin_string, sub_account_name, authorization, x_api_key)
Get Tag Alerts

Get alerts configured for worker tags. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

[**models::TagAlertsResponse**](TagAlertsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_email_from_alert

> remove_email_from_alert(coin_string, sub_account_name, alert_id, email_address, authorization, x_api_key)
Remove Email From The Alert

Remove email from the alert. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin | [required] |
**sub_account_name** | **String** | Sub Account Name | [required] |
**alert_id** | **i32** | Alert ID | [required] |
**email_address** | **String** | Email Address to be added to the alert. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

