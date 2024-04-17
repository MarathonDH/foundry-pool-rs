# \SubAccountsApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cumulative_granted_sub_account_stats**](SubAccountsApi.md#get_cumulative_granted_sub_account_stats) | **GET** /cumulated_granted_subaccount_stats/{userId} | Cumulated Granted Sub-Account Stats
[**get_cumulative_granted_sub_accounts_hashrate**](SubAccountsApi.md#get_cumulative_granted_sub_accounts_hashrate) | **GET** /cumulated_subaccount_hashrate_hour/{userId} | Cumulated Hashrate Per Hour for Granted Sub-Accounts
[**get_cumulative_granted_sub_accounts_hashrate1**](SubAccountsApi.md#get_cumulative_granted_sub_accounts_hashrate1) | **GET** /cumulated_subaccount_hashrate_day/{userId} | Cumulated Hashrate Per Day for Granted Sub-Accounts



## get_cumulative_granted_sub_account_stats

> models::CumulativeSubAccountStats get_cumulative_granted_sub_account_stats(user_id, authorization, coin, sort, group_ids_list)
Cumulated Granted Sub-Account Stats

Get cumulated stats for sub-accounts that the user owns or has granted access to. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID argument is optional. Defaults to logged in user | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**sort** | Option<**String**> | Valid values are \"highestHashrate\" or \"subAccountName\". |  |[default to subAccountName]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | Optional list of group IDs to filter by. |  |

### Return type

[**models::CumulativeSubAccountStats**](CumulativeSubAccountStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cumulative_granted_sub_accounts_hashrate

> Vec<models::HashrateResponse> get_cumulative_granted_sub_accounts_hashrate(user_id, x_api_key, authorization, coin, start, end, start_date_unix_ms, end_date_unix_ms, group_ids_list)
Cumulated Hashrate Per Hour for Granted Sub-Accounts

Get cumulative hashrate per hour for all granted sub-accounts that the user owns or has been granted access to, for a requested date range & coin. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID argument is optional. Defaults to logged in user | [required] |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start** | Option<**String**> | Start date inclusive, in YYYY-MM-DD string format. Default value is one week ago. |  |[default to ]
**end** | Option<**String**> | End date inclusive, in YYYY-MM-DD string format. Default value is current time. |  |[default to ]
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is one week ago. |  |[default to 0]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to 0]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | Optional list of group IDs to filter by. |  |

### Return type

[**Vec<models::HashrateResponse>**](HashrateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cumulative_granted_sub_accounts_hashrate1

> Vec<models::HashrateResponse> get_cumulative_granted_sub_accounts_hashrate1(user_id, authorization, x_api_key, coin, start, end, group_ids_list)
Cumulated Hashrate Per Day for Granted Sub-Accounts

Get cumulative hashrate per day for all granted sub-accounts that the user owns or has been granted access to, for a requested date range & coin. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID argument is optional. Defaults to logged in user | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start** | Option<**String**> | Start date inclusive, in YYYY-MM-DD string format. Default value is 30 days ago. |  |[default to ]
**end** | Option<**String**> | End date inclusive, in YYYY-MM-DD string format. Default value is current day. |  |[default to ]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | Optional list of group IDs to filter by. |  |

### Return type

[**Vec<models::HashrateResponse>**](HashrateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

