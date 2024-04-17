# \WorkersApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](WorkersApi.md#create_tag) | **PUT** /tags/{coinString}/{subAccountName}/{tagName} | Create Tag
[**delete_tag**](WorkersApi.md#delete_tag) | **DELETE** /tags/{coinString}/{subAccountName}/{tagName} | Delete Tag
[**delete_workers_by_sub_account_name_and_worker_ids**](WorkersApi.md#delete_workers_by_sub_account_name_and_worker_ids) | **DELETE** /workers/{subAccountName}/{coinString} | Delete Workers
[**get_tagged_worker_count**](WorkersApi.md#get_tagged_worker_count) | **GET** /tags/taggedWorkerCount/{coinString}/{subAccountName} | Get Tagged Worker Count
[**get_tags**](WorkersApi.md#get_tags) | **GET** /tags/{coinString}/{subAccountName} | Worker Tags
[**get_worker_by_sub_account_and_worker_name**](WorkersApi.md#get_worker_by_sub_account_and_worker_name) | **GET** /workers/{subAccountName}/{workerName} | Worker Stats
[**get_worker_counts**](WorkersApi.md#get_worker_counts) | **GET** /workers/workerCounts/{subAccountName} | Worker Counts by Tag Name
[**get_worker_hashrate**](WorkersApi.md#get_worker_hashrate) | **GET** /worker_hashrate_hour/{subAccountName}/{workerId} | Worker Hashrate Per Hour
[**get_worker_hashrate1**](WorkersApi.md#get_worker_hashrate1) | **GET** /worker_hashrate_day/{subAccountName}/{workerId} | Worker Hashrate Per Day
[**get_workers_by_sub_account**](WorkersApi.md#get_workers_by_sub_account) | **GET** /workers/{subAccountName} | All Worker Stats
[**tag_workers**](WorkersApi.md#tag_workers) | **PUT** /tags/tag/{coinString}/{subAccountName}/{tagName} | Tag Workers
[**untag_workers**](WorkersApi.md#untag_workers) | **DELETE** /tags/untag/{coinString}/{subAccountName} | Untag Workers
[**update_tag**](WorkersApi.md#update_tag) | **PUT** /tags/update/{coinString}/{subAccountName}/{tagName}/{newTagName} | Update Tag Name



## create_tag

> i32 create_tag(coin_string, sub_account_name, tag_name, authorization, x_api_key)
Create Tag

Create a tag. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name | [required] |
**tag_name** | **String** | Tag name | [required] |
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


## delete_tag

> delete_tag(coin_string, sub_account_name, tag_name, authorization, x_api_key)
Delete Tag

Delete a tag. Deleting will untag all the workers under the specified tag. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name associated with the tag | [required] |
**tag_name** | **String** | Tag name to be deleted | [required] |
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


## delete_workers_by_sub_account_name_and_worker_ids

> i32 delete_workers_by_sub_account_name_and_worker_ids(sub_account_name, coin_string, authorization, last_share_time, request_body)
Delete Workers

Delete workers for a sub-account by providing a list of worker ids OR by providing a last share time timestamp in milliseconds. Requires sub-account owner authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**coin_string** | **String** | Name of requested coin. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. |  |
**last_share_time** | Option<**i64**> | Timestamp in Unix milliseconds. Must be at least 15 minutes before current time. Workers whose last share time is equal to or older than this timestamp will be deleted. |  |
**request_body** | Option<[**Vec<i64>**](i64.md)> | Optional list of worker ids to be deleted. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tagged_worker_count

> i32 get_tagged_worker_count(sub_account_name, coin_string, x_api_key)
Get Tagged Worker Count

Get the number of workers tagged by sub-account id and coinstring. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Sub-account name | [required] |
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**x_api_key** | Option<**String**> | API key. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> models::TagsResponse get_tags(coin_string, sub_account_name, authorization, x_api_key)
Worker Tags

Get worker tags. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

[**models::TagsResponse**](TagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worker_by_sub_account_and_worker_name

> models::WorkerResponse get_worker_by_sub_account_and_worker_name(sub_account_name, worker_name, authorization, x_api_key, coin)
Worker Stats

Get worker stats. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**worker_name** | **String** | Name of requested worker. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]

### Return type

[**models::WorkerResponse**](WorkerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worker_counts

> models::WorkerCounts get_worker_counts(sub_account_name, authorization, x_api_key, coin, tag_name)
Worker Counts by Tag Name

Get worker counts by tag name. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**tag_name** | Option<**String**> | Name of requested tag. Default value is \"all\". |  |[default to all]

### Return type

[**models::WorkerCounts**](WorkerCounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_worker_hashrate

> Vec<models::HashrateResponse> get_worker_hashrate(sub_account_name, worker_id, authorization, x_api_key, coin, start, end, start_date_unix_ms, end_date_unix_ms)
Worker Hashrate Per Hour

Get worker hashrate per hour for a requested date range & coin. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** |  | [required] |
**worker_id** | **i64** |  | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**start** | Option<**String**> | Start date inclusive, in YYYY-MM-DD string format. Default value is one week ago. |  |[default to ]
**end** | Option<**String**> | End date inclusive, in YYYY-MM-DD string format. Default value is current time. |  |[default to ]
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


## get_worker_hashrate1

> Vec<models::HashrateResponse> get_worker_hashrate1(sub_account_name, worker_id, authorization, x_api_key, coin, start, end)
Worker Hashrate Per Day

Get worker hashrate per day for a requested date range & coin. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**worker_id** | **i64** | ID of requested worker. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. |  |[default to BTC]
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


## get_workers_by_sub_account

> models::WorkersResponse get_workers_by_sub_account(sub_account_name, authorization, x_api_key, coin, sort, status, tag, page_number, page_size, worker_name_search_str)
All Worker Stats

Get all worker stats for a sub-account. Requires permission to view hashrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sub_account_name** | **String** | Name of requested sub-account. | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |
**coin** | Option<**String**> | Name of requested coin. Default value is BTC. |  |[default to BTC]
**sort** | Option<**String**> | Valid values are \"highestHashrate\", \"lowestHashrate\", \"highestReject\", \"lowestReject\", \"newestShareTime\", \"oldestShareTime\", \"workerName\", \"reverseWorkerName\", \"tagName\", \"reverseTagName\". Default value is highestHashrate. |  |[default to highestHashrate]
**status** | Option<**String**> | Valid values are \"all\", \"online<15min\", \"offline<24hr\", or \"offline>24hr\". Default value is all. |  |[default to all]
**tag** | Option<**String**> | Valid values are \"all\", \"untagged\" or user created tag name. Default value is all. |  |[default to all]
**page_number** | Option<**i32**> | Valid values are 0 and positive integers. Default value is 0. |  |[default to 0]
**page_size** | Option<**i32**> | Valid values are -1 (representing max size) and positive integers. Default value is -1. |  |[default to -1]
**worker_name_search_str** | Option<**String**> | Default value is \"\" and includes all worker names |  |[default to ]

### Return type

[**models::WorkersResponse**](WorkersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_workers

> i32 tag_workers(coin_string, sub_account_name, tag_name, request_body, authorization, x_api_key)
Tag Workers

Tag a list of workers. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name associated with the tag | [required] |
**tag_name** | **String** | Tag name | [required] |
**request_body** | [**Vec<i32>**](i32.md) | List of worker ids to be tagged | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## untag_workers

> untag_workers(coin_string, sub_account_name, request_body, authorization, x_api_key)
Untag Workers

Untag a list of workers. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name associated with the tag | [required] |
**request_body** | [**Vec<i64>**](i64.md) | List of worker ids to be untagged | [required] |
**authorization** | Option<**String**> | OAuth2.0 access token. Not required if using API key. |  |
**x_api_key** | Option<**String**> | API key. Not required if using access token. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> i32 update_tag(coin_string, sub_account_name, tag_name, new_tag_name, authorization, x_api_key)
Update Tag Name

Update a tag name. Requires permission to edit workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_string** | **String** | Coin string. valid values are \"BTC\" or \"BCH\" | [required] |
**sub_account_name** | **String** | Sub-account name associated with the tag | [required] |
**tag_name** | **String** | Tag name to be updated | [required] |
**new_tag_name** | **String** | New tag name | [required] |
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

