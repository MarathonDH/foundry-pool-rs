# \FinancialApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_balance_info**](FinancialApi.md#get_balance_info) | **GET** /balance/{subAccountName} | Balance Info
[**get_earnings**](FinancialApi.md#get_earnings) | **GET** /earnings/{subAccountName} | Daily Earnings
[**get_financial_stats_for_sub_accounts**](FinancialApi.md#get_financial_stats_for_sub_accounts) | **GET** /financial_overview/{userId} | Financial Overview
[**get_transactions**](FinancialApi.md#get_transactions) | **GET** /transactions/{subAccountName} | Transactions



## get_balance_info

> models::BalanceResponse get_balance_info(sub_account_name, authorization, x_api_key, coin)
Balance Info

Get balance, total earned, total fees and total withdrawn amount for a requested sub-account. Requires authorized user or admin authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Sub-Account name | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]

### Return type

[**models::BalanceResponse**](BalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_earnings

> Vec<models::EarningResponse> get_earnings(sub_account_name, x_api_key, authorization, coin, start_date_unix_ms, end_date_unix_ms)
Daily Earnings

Get daily aggregated earnings for a requested sub-account. Requires authorized user or admin authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Sub-Account name | [required] |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is 0. |  |[default to 0]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to -1]

### Return type

[**Vec<models::EarningResponse>**](EarningResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_financial_stats_for_sub_accounts

> models::SubAccountsFinancialStatsResponse get_financial_stats_for_sub_accounts(authorization, user_id, coin, start_date_unix_ms, end_date_unix_ms, group_ids_list)
Financial Overview

Get financial stats and their total for sub-accounts associated with a user. User authentication required to see their own granted sub-accounts. Admin authentication with read permission required to see other users granted sub-accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **String** | User ID is optional. Defaults to logged in user | [required] |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is 0. Specify UTC start of the day epoch time. |  |[default to 0]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to -1]
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | List of group ids. |  |

### Return type

[**models::SubAccountsFinancialStatsResponse**](SubAccountsFinancialStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> Vec<models::TransactionResponse> get_transactions(sub_account_name, x_api_key, authorization, coin, start_date_unix_ms, end_date_unix_ms)
Transactions

Get transactions for a requested sub-account. Requires authorized user or admin authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Sub-Account name | [required] |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start_date_unix_ms** | Option<**i64**> | Start date inclusive, in unix epoch time (milliseconds). Default value is 0. |  |[default to 0]
**end_date_unix_ms** | Option<**i64**> | End date inclusive, in unix epoch time (milliseconds). Default value is current time. |  |[default to -1]

### Return type

[**Vec<models::TransactionResponse>**](TransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

