# \WalletsApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_withdrawal_address**](WalletsApi.md#add_withdrawal_address) | **PUT** /wallets/addresses/{subAccountName}/{addressLabel}/{walletAddress} | Add a New Withdrawal Address or Create Approval Request for the same
[**get_auto_withdrawal_address**](WalletsApi.md#get_auto_withdrawal_address) | **GET** /wallets/autoWithdrawalAddress/{subAccountName} | Get Enabled Auto-Withdrawal Addresses
[**get_withdrawal_addresses**](WalletsApi.md#get_withdrawal_addresses) | **GET** /wallets/addresses/{subAccountName} | Get Withdrawal Addresses
[**save_auto_withdrawal_details**](WalletsApi.md#save_auto_withdrawal_details) | **POST** /wallets/saveAutoWithdrawalDetails/{subAccountName}/{coinString} | Save Auto-Withdrawal Details or Create Approval Request for the same
[**set_auto_withdrawal_off**](WalletsApi.md#set_auto_withdrawal_off) | **PUT** /wallets/deactivateAutoWithdrawal/{subAccountName} | Deactivate Auto-Withdrawal



## add_withdrawal_address

> models::WalletResponse add_withdrawal_address(authorization, sub_account_name, address_label, wallet_address, coin)
Add a New Withdrawal Address or Create Approval Request for the same

Add a new withdrawal address for a sub-account, if your group doesn't satisfy the approval request creation threshold. Else, create an approval request to do the same. If there is an existing address where autowithdraw is ON, the new wallets allocationPercent is set to 0 and autoWithdraw is turned off. Else the new wallets allocationPercent is set to 100 and autoWithdraw is turned on. Requires authorized user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**sub_account_name** | **String** | Name of sub-account to which the new address is being added | [required] |
**address_label** | **String** | Label for the address to be added | [required] |
**wallet_address** | **String** | Address to be added | [required] |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC |  |[default to BTC]

### Return type

[**models::WalletResponse**](WalletResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auto_withdrawal_address

> models::WalletsResponse get_auto_withdrawal_address(sub_account_name, authorization, x_api_key, coin)
Get Enabled Auto-Withdrawal Addresses

Get addresses for a sub-account where auto-withdraw is enabled. Requires viewPayments permissions for this sub account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of the sub-account you want to get auto-withdrawal address for | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]

### Return type

[**models::WalletsResponse**](WalletsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_withdrawal_addresses

> models::WalletsResponse get_withdrawal_addresses(sub_account_name, authorization, x_api_key, coin)
Get Withdrawal Addresses

Get addresses for a sub-account. This includes addresses where auto-withdraw is either enabled or disabled. Requires viewPayments permissions for this sub account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of the sub-account you want to get withdrawal addresses for | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]

### Return type

[**models::WalletsResponse**](WalletsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_auto_withdrawal_details

> i32 save_auto_withdrawal_details(authorization, sub_account_name, coin_string, wallet_allocation_request)
Save Auto-Withdrawal Details or Create Approval Request for the same

Save auto-withdrawal details for a sub-account or create approval request to do the same. Requires authorized user. Addresses in the request should already be added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**sub_account_name** | **String** | Name of subAccount. | [required] |
**coin_string** | **String** | Name of requested coin. | [required] |
**wallet_allocation_request** | [**Vec<models::WalletAllocationRequest>**](WalletAllocationRequest.md) | A list of wallet allocation requests. Each allocation request must contain following fields | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_auto_withdrawal_off

> i32 set_auto_withdrawal_off(authorization, sub_account_name, coin)
Deactivate Auto-Withdrawal

Turn auto-withdrawal off for a sub-account. Requires withdraw permissions for the sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**sub_account_name** | **String** | Name of the sub-account you want to deactivate auto-withdrawal for | [required] |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC |  |[default to BTC]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

