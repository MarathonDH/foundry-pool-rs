# \ActivityLogsApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity_log_for_groups**](ActivityLogsApi.md#get_activity_log_for_groups) | **GET** /v2/activity_log | Get Activity Logs for a List of Groups
[**get_activity_log_types**](ActivityLogsApi.md#get_activity_log_types) | **GET** /activity_log/activity_types | Get Available Activity Log Types



## get_activity_log_for_groups

> models::ActivityLogResponseV2WithTotal get_activity_log_for_groups(authorization, group_ids_list, start_date_unix_ms, end_date_unix_ms, page_number, page_size, coin, activity_type, user_email, sub_account_name, sort, hide_auth_logs)
Get Activity Logs for a List of Groups

Get activity logs for a list of groups. Mining account owner or accountant authentication required to see own groups activity log. Admin authentication with read permission required to see other groups activity log.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | List of Group IDs is optional. If no list is passed, all groups the logged-in user has view permissions for is returned. |  |
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is 30 days ago. |  |[default to -1]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to -1]
**page_number** | Option<**i32**> | Valid values are 0 and positive integers. |  |[default to 0]
**page_size** | Option<**i32**> | Valid values are -1 (representing max size) and positive integers. |  |[default to 100]
**coin** | Option<**String**> | Filter logs by coin. |  |
**activity_type** | Option<**String**> | Filter logs by activity type. List of available activity types can be GET from /activity_log/activity_types endpoint |  |
**user_email** | Option<**String**> | Filter logs by a user's email address |  |
**sub_account_name** | Option<**String**> | Filter logs by a subAccountName for a Group in which that SubAccount exists |  |
**sort** | Option<**String**> | Sort the logs by \"newest\" or \"oldest\" case-sensitive. |  |[default to newest]
**hide_auth_logs** | Option<**bool**> | Filter logs by hiding all auth0 activity types. |  |

### Return type

[**models::ActivityLogResponseV2WithTotal**](ActivityLogResponseV2WithTotal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_log_types

> Vec<models::ActivityTypesEnumResponse> get_activity_log_types(authorization)
Get Available Activity Log Types

Get list of activities types that are currently being logged by the pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |

### Return type

[**Vec<models::ActivityTypesEnumResponse>**](ActivityTypesEnumResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

