# ClusterState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exited** | Option<**i32**> | Present when the cluster process has exited on the local node with this exit code. An exit code of 0 indicates success (e.g. the node has cleanly left the cluster).  | [optional]
**signaled** | Option<**i32**> | Present when the cluster process has been killed by this signal.  | [optional]
**stopped** | Option<**i32**> | Present when the cluster process has been stopped by this signal.  | [optional]
**running** | **bool** | Whether the cluster process is currently running.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


