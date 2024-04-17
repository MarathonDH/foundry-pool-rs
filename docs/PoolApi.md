# \PoolApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pool_hashrate**](PoolApi.md#get_pool_hashrate) | **GET** /pool_hashrate | Pool Hashrate
[**get_pool_stats**](PoolApi.md#get_pool_stats) | **GET** /pool_stats | Pool Stats



## get_pool_hashrate

> String get_pool_hashrate(coin)
Pool Hashrate

Get pool hashrate (24 hour avg in GH/s). No authentication required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pool_stats

> models::PoolStats get_pool_stats(coin)
Pool Stats

Get pool stats.  No authentication required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]

### Return type

[**models::PoolStats**](PoolStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

