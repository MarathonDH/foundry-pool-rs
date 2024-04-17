# \SubAccountApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sub_account_and_add_withdrawal_address_for_group**](SubAccountApi.md#create_sub_account_and_add_withdrawal_address_for_group) | **POST** /v2/sub_account/{subAccountName}/group/{groupId}/{addressLabel}/{walletAddress} | Create Sub-Account for given Group and Add Withdrawal Address or Create Approval Request for the same.
[**delete_sub_account**](SubAccountApi.md#delete_sub_account) | **DELETE** /sub_account/{subAccountName} | Delete Sub-Account
[**get_granted_sub_account_stats**](SubAccountApi.md#get_granted_sub_account_stats) | **GET** /granted_subaccount_stats/{userId} | Granted Sub-Account Stats
[**get_sub_account_hashrate**](SubAccountApi.md#get_sub_account_hashrate) | **GET** /subaccount_hashrate_hour/{subAccountName} | Hashrate Per Hour
[**get_sub_account_hashrate1**](SubAccountApi.md#get_sub_account_hashrate1) | **GET** /subaccount_hashrate_day/{subAccountName} | Hashrate Per Day
[**get_sub_account_stats**](SubAccountApi.md#get_sub_account_stats) | **GET** /subaccount_stats/{subAccountName} | Sub-Account Stats



## create_sub_account_and_add_withdrawal_address_for_group

> models::WalletResponse create_sub_account_and_add_withdrawal_address_for_group(authorization, sub_account_name, group_id, address_label, wallet_address, coin, user_id, sub_account_access_request)
Create Sub-Account for given Group and Add Withdrawal Address or Create Approval Request for the same.

Create a new sub-account for given group and add an initial withdrawal address to it, if your group doesn't satisfy the approval request creation threshold. Else, create an approval request to do the same. Requires sub-account owner authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**sub_account_name** | **String** | Name of the new sub-account | [required] |
**group_id** | **i32** | Group ID for the group to which the sub-account belongs | [required] |
**address_label** | **String** | Label for the new withdrawal address | [required] |
**wallet_address** | **String** | Withdrawal address | [required] |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]
**user_id** | Option<**String**> | User ID for the owner of the new-subaccount. Default value is the user ID associated with your access token. Do not include this param if you're not an admin. Only admins with write permission can create sub-accounts on behalf of other users. |  |[default to -1]
**sub_account_access_request** | Option<[**Vec<models::SubAccountAccessRequest>**](SubAccountAccessRequest.md)> | An optional array specifying users and their sub-account role. All group owners must be present in the array. Each array entry must contain the following fields. userId: an Integer that corresponds to a User ID. subAccountRole: a String that represents the subAccountRole you want to be given to the User ID, Allowed values for subAccountRole are \"owner\", \"technician\" or \"accountant\" |  |

### Return type

[**models::WalletResponse**](WalletResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sub_account

> delete_sub_account(sub_account_name, authorization)
Delete Sub-Account

Delete sub-account with no mining history. Requires permission to create sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of the sub-account to be deleted | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_granted_sub_account_stats

> Vec<models::SubAccountStatsWithRoleResponse> get_granted_sub_account_stats(user_id, authorization, coin, sort, group_ids_list)
Granted Sub-Account Stats

Get stats for sub-accounts that the user owns or has granted access to. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID is optional. Defaults to logged in user. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**sort** | Option<**String**> | Valid values are \"highestHashrate\" or \"subAccountName\". |  |[default to subAccountName]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | List of group ids. |  |

### Return type

[**Vec<models::SubAccountStatsWithRoleResponse>**](SubAccountStatsWithRoleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_account_hashrate

> Vec<models::HashrateResponse> get_sub_account_hashrate(sub_account_name, x_api_key, authorization, coin, start, end, start_date_unix_ms, end_date_unix_ms)
Hashrate Per Hour

Get sub-account hashrate per hour for a requested date range & coin. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** |  | [required] |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start** | Option<**String**> | Start date inclusive, in yyyy-MM-ddThh:mm:ss.SSS string format. Default value is one week ago. |  |[default to ]
**end** | Option<**String**> | End date inclusive, in yyyy-MM-ddThh:mm:ss.SSS string format. Default value is current time. |  |[default to ]
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is one week ago. |  |[default to 0]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to 0]

### Return type

[**Vec<models::HashrateResponse>**](HashrateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_account_hashrate1

> Vec<models::HashrateResponse> get_sub_account_hashrate1(sub_account_name, authorization, x_api_key, coin, start, end)
Hashrate Per Day

Get sub-account hashrate and average active workers per day for a requested date range & coin. Requires permission to view hashrate. Note: Average Workers would be -1.0 for the current UTC day and also if the calculation is pending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start** | Option<**String**> | Start date inclusive, in YYYY-MM-DD string format. Default value is 30 days ago. |  |[default to ]
**end** | Option<**String**> | End date inclusive, in YYYY-MM-DD string format. Default value is current day. |  |[default to ]

### Return type

[**Vec<models::HashrateResponse>**](HashrateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_account_stats

> models::SubAccountStatsWithoutOwnersResponse get_sub_account_stats(sub_account_name, authorization, x_api_key, coin, tag_name)
Sub-Account Stats

Get sub-account stats. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Sub-Account Name | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**tag_name** | Option<**String**> | Name of requested tag. Default value is all. |  |[default to all]

### Return type

[**models::SubAccountStatsWithoutOwnersResponse**](SubAccountStatsWithoutOwnersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

