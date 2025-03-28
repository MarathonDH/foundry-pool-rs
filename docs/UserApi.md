# \UserApi

All URIs are relative to *https://api.foundryusapool.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_sub_account_user**](UserApi.md#add_sub_account_user) | **PUT** /users/{userId}/sub-account_role/{subAccountName} | Add User to an Existing Sub-Account
[**deactivate_user**](UserApi.md#deactivate_user) | **DELETE** /v2/users/{userId} | Deactivate User Associated With a List of Groups or Create Approval Request for the same
[**delete_user_sub_account_role**](UserApi.md#delete_user_sub_account_role) | **DELETE** /users/{userId}/sub-account_role/{subAccountName} | Delete User’s Role for a Sub-Account
[**get_role_info_by_sub_account_name_and_user_id**](UserApi.md#get_role_info_by_sub_account_name_and_user_id) | **GET** /get_role/{subAccountName}/{userId} | Get a Users Role for a Sub-Account
[**get_user_by_logged_in_user**](UserApi.md#get_user_by_logged_in_user) | **GET** /users/logged-in-user | Get User Info (Self)
[**get_users_by_group**](UserApi.md#get_users_by_group) | **GET** /v2/users/users-by-group | Get All Users Associated with a Group
[**get_users_by_group_list**](UserApi.md#get_users_by_group_list) | **GET** /v2/users/users-by-groups | Get All Users Associated with a List of Groups
[**register**](UserApi.md#register) | **POST** /v2/users | Create User or Add User to group(s) or Create Approval Request for the same
[**update_all_user_sub_account_roles**](UserApi.md#update_all_user_sub_account_roles) | **PUT** /v2/users/{userId}/group/{groupId}/role/{newSubAccountRoleName} | Update User’s Role for All Sub-Accounts or Create Approval Request for the same
[**update_user_sub_account_role**](UserApi.md#update_user_sub_account_role) | **PUT** /users/{userId}/sub-account_role/{subAccountName}/{subAccountRoleName} | Update User’s Role for a Sub-Account



## add_sub_account_user

> add_sub_account_user(authorization, user_id, sub_account_name, add_user_to_sub_account_request)
Add User to an Existing Sub-Account

Add user to an existing sub-account. Requires admin write or owner permissions for the sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **i32** | User ID to be added to sub-account | [required] |
**sub_account_name** | **String** | Sub-Account name the user is to be added | [required] |
**add_user_to_sub_account_request** | Option<[**AddUserToSubAccountRequest**](AddUserToSubAccountRequest.md)> | Sub-Account role to be provided to this user. Valid values are \"owner\", \"technician\", \"accountant\" or \"approver\".  Optional - Will default to user's defaultSubAccountRole if not included. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_user

> models::UserResponseV2 deactivate_user(authorization, user_id, group_ids_list)
Deactivate User Associated With a List of Groups or Create Approval Request for the same

Deactivate user associated with a list of groups or Create Approval Request to do so. Requires mining account owner authentication or admin write authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **i32** | Id of the user to be deactivated | [required] |
**group_ids_list** | [**Vec<i32>**](i32.md) | List of group IDs to deactivate the user from | [required] |

### Return type

[**models::UserResponseV2**](UserResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_sub_account_role

> delete_user_sub_account_role(authorization, user_id, sub_account_name)
Delete User’s Role for a Sub-Account

Delete user’s role for a sub-account. Requires set role permissions for the sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **i32** | User ID associated with the role to be deleted | [required] |
**sub_account_name** | **String** | Sub-Account name the user's role is to be deleted from | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_info_by_sub_account_name_and_user_id

> models::UserSubAccountRoleResponse get_role_info_by_sub_account_name_and_user_id(authorization, sub_account_name, user_id)
Get a Users Role for a Sub-Account

Get the role of a user for a sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**sub_account_name** | **String** | The name of the sub-account | [required] |
**user_id** | **String** | The ID of the user is optional. Defaults to logged in user. | [required] |

### Return type

[**models::UserSubAccountRoleResponse**](UserSubAccountRoleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_logged_in_user

> models::User get_user_by_logged_in_user(authorization)
Get User Info (Self)

Get info for user associated with the provided authentication token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_group

> Vec<models::GroupWithUsersResponseV2> get_users_by_group(authorization, group_id, page_number, page_size, sort, user_full_name)
Get All Users Associated with a Group

Get all users associated with a group. Requires logged-in user to have access to and view permissions for group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**group_id** | **i32** | Group ID. | [required] |
**page_number** | Option<**i32**> | Optional page number.  Defaults to 0. |  |[default to 0]
**page_size** | Option<**i32**> | Optional page size.  If empty, returns all results. |  |[default to -1]
**sort** | Option<**String**> | Optional sort value.  Options are 'fullNameAsc', 'fullNameDesc', 'emailAddressAsc', 'emailAddressDesc', 'defaultRoleAsc', or 'defaultRoleDesc'.  If empty, defaults to fullNameAsc. |  |[default to fullNameAsc]
**user_full_name** | Option<**String**> | Optional user name filter value.  If empty, returns all results. |  |[default to ]

### Return type

[**Vec<models::GroupWithUsersResponseV2>**](GroupWithUsersResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_group_list

> Vec<models::GroupWithUsersResponseV2> get_users_by_group_list(authorization, group_ids_list)
Get All Users Associated with a List of Groups

Get all users associated with a list of groups. Requires logged-in user to have access to, and view permissions for, all groups in list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**group_ids_list** | Option<[**Vec<i32>**](i32.md)> | Optional List of Group IDs.  If empty, returns all users in all groups the logged-in user has view permissions for. |  |

### Return type

[**Vec<models::GroupWithUsersResponseV2>**](GroupWithUsersResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::UserResponseV2 register(authorization, new_user_request_v2)
Create User or Add User to group(s) or Create Approval Request for the same

Creates a user and adds it to the groups that don't satisfy the approval request creation threshold to it. Else, create an approval request to do the same. Requires mining account owner or admin write authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**new_user_request_v2** | [**NewUserRequestV2**](NewUserRequestV2.md) | User info that needs to be added to the group | [required] |

### Return type

[**models::UserResponseV2**](UserResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_all_user_sub_account_roles

> models::UserResponseV2 update_all_user_sub_account_roles(authorization, user_id, group_id, new_sub_account_role_name)
Update User’s Role for All Sub-Accounts or Create Approval Request for the same

Update user’s role for all sub-accounts, if your group doesn't satisfy the approval request creation threshold. Else, create an approval request to do the same. Requires mining account owner authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **i32** | The ID of the user | [required] |
**group_id** | **i32** | The ID of the group | [required] |
**new_sub_account_role_name** | **String** | Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\". | [required] |

### Return type

[**models::UserResponseV2**](UserResponseV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_sub_account_role

> i32 update_user_sub_account_role(authorization, user_id, sub_account_name, sub_account_role_name)
Update User’s Role for a Sub-Account

Update user’s role for a sub-account. Requires set role permissions for the sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | OAuth2.0 access token. | [required] |
**user_id** | **i32** | The ID of the user | [required] |
**sub_account_name** | **String** | The name of the sub-account | [required] |
**sub_account_role_name** | **String** | Sub-Account Role Name, valid values are \"owner\", \"technician\", \"accountant\" or \"approver\". | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

