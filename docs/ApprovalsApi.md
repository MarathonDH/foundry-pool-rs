# \ApprovalsApi

All URIs are relative to *http://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approval_action**](ApprovalsApi.md#approval_action) | **POST** /approvals/{approvalRequestId} | Approve Or Reject An Approval Request



## approval_action

> approval_action(authorization, approval_request_id, body)
Approve Or Reject An Approval Request

Approve or reject an approval request. Requires approver permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**approval_request_id** | **String** | Approval request ID | [required] |
**body** | **serde_json::Value** | Accepted values are \"approve\" or \"reject\" | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

