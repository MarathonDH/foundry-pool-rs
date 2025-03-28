# \ApprovalsApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approval_action**](ApprovalsApi.md#approval_action) | **POST** /approvals/{approvalRequestId} | Approve Or Reject An Approval Request
[**update_group_approval_threshold**](ApprovalsApi.md#update_group_approval_threshold) | **PUT** /group/{groupId}/update_approval_threshold/{newApprovalThreshold} | Update Group Approval Threshold



## approval_action

> String approval_action(authorization, approval_request_id, body)
Approve Or Reject An Approval Request

Approve or reject an approval request. Requires approver permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**approval_request_id** | **String** | Approval request ID | [required] |
**body** | **serde_json::Value** | Accepted values are \"approve\" or \"reject\" | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_approval_threshold

> models::GroupThresholdUpdateResponse update_group_approval_threshold(authorization, group_id, new_approval_threshold)
Update Group Approval Threshold

Update a Group's Approval Threshold, or Create Approval Request for the same.  Requires group or sub-account owner/approver permission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**group_id** | **i32** | Id of the Group to update | [required] |
**new_approval_threshold** | **i32** | New Approval Threshold for Group | [required] |

### Return type

[**models::GroupThresholdUpdateResponse**](GroupThresholdUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

