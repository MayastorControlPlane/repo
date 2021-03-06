# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustActixMayastorCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost/v0*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BlockDevices* | [**get_node_block_devices**](docs/apis/BlockDevices.md#get_node_block_devices) | **Get** /nodes/{node}/block_devices | 
*Children* | [**del_nexus_child**](docs/apis/Children.md#del_nexus_child) | **Delete** /nexuses/{nexus_id}/children/{child_id} | 
*Children* | [**del_node_nexus_child**](docs/apis/Children.md#del_node_nexus_child) | **Delete** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 
*Children* | [**get_nexus_child**](docs/apis/Children.md#get_nexus_child) | **Get** /nexuses/{nexus_id}/children/{child_id} | 
*Children* | [**get_nexus_children**](docs/apis/Children.md#get_nexus_children) | **Get** /nexuses/{nexus_id}/children | 
*Children* | [**get_node_nexus_child**](docs/apis/Children.md#get_node_nexus_child) | **Get** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 
*Children* | [**get_node_nexus_children**](docs/apis/Children.md#get_node_nexus_children) | **Get** /nodes/{node_id}/nexuses/{nexus_id}/children | 
*Children* | [**put_nexus_child**](docs/apis/Children.md#put_nexus_child) | **Put** /nexuses/{nexus_id}/children/{child_id} | 
*Children* | [**put_node_nexus_child**](docs/apis/Children.md#put_node_nexus_child) | **Put** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 
*JsonGrpc* | [**put_node_jsongrpc**](docs/apis/JsonGrpc.md#put_node_jsongrpc) | **Put** /nodes/{node}/jsongrpc/{method} | 
*Nexuses* | [**del_nexus**](docs/apis/Nexuses.md#del_nexus) | **Delete** /nexuses/{nexus_id} | 
*Nexuses* | [**del_node_nexus**](docs/apis/Nexuses.md#del_node_nexus) | **Delete** /nodes/{node_id}/nexuses/{nexus_id} | 
*Nexuses* | [**del_node_nexus_share**](docs/apis/Nexuses.md#del_node_nexus_share) | **Delete** /nodes/{node_id}/nexuses/{nexus_id}/share | 
*Nexuses* | [**get_nexus**](docs/apis/Nexuses.md#get_nexus) | **Get** /nexuses/{nexus_id} | 
*Nexuses* | [**get_nexuses**](docs/apis/Nexuses.md#get_nexuses) | **Get** /nexuses | 
*Nexuses* | [**get_node_nexus**](docs/apis/Nexuses.md#get_node_nexus) | **Get** /nodes/{node_id}/nexuses/{nexus_id} | 
*Nexuses* | [**get_node_nexuses**](docs/apis/Nexuses.md#get_node_nexuses) | **Get** /nodes/{id}/nexuses | 
*Nexuses* | [**put_node_nexus**](docs/apis/Nexuses.md#put_node_nexus) | **Put** /nodes/{node_id}/nexuses/{nexus_id} | 
*Nexuses* | [**put_node_nexus_share**](docs/apis/Nexuses.md#put_node_nexus_share) | **Put** /nodes/{node_id}/nexuses/{nexus_id}/share/{protocol} | 
*Nodes* | [**get_node**](docs/apis/Nodes.md#get_node) | **Get** /nodes/{id} | 
*Nodes* | [**get_nodes**](docs/apis/Nodes.md#get_nodes) | **Get** /nodes | 
*Pools* | [**del_node_pool**](docs/apis/Pools.md#del_node_pool) | **Delete** /nodes/{node_id}/pools/{pool_id} | 
*Pools* | [**del_pool**](docs/apis/Pools.md#del_pool) | **Delete** /pools/{pool_id} | 
*Pools* | [**get_node_pool**](docs/apis/Pools.md#get_node_pool) | **Get** /nodes/{node_id}/pools/{pool_id} | 
*Pools* | [**get_node_pools**](docs/apis/Pools.md#get_node_pools) | **Get** /nodes/{id}/pools | 
*Pools* | [**get_pool**](docs/apis/Pools.md#get_pool) | **Get** /pools/{pool_id} | 
*Pools* | [**get_pools**](docs/apis/Pools.md#get_pools) | **Get** /pools | 
*Pools* | [**put_node_pool**](docs/apis/Pools.md#put_node_pool) | **Put** /nodes/{node_id}/pools/{pool_id} | 
*Replicas* | [**del_node_pool_replica**](docs/apis/Replicas.md#del_node_pool_replica) | **Delete** /nodes/{node_id}/pools/{pool_id}/replicas/{replica_id} | 
*Replicas* | [**del_node_pool_replica_share**](docs/apis/Replicas.md#del_node_pool_replica_share) | **Delete** /nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share | 
*Replicas* | [**del_pool_replica**](docs/apis/Replicas.md#del_pool_replica) | **Delete** /pools/{pool_id}/replicas/{replica_id} | 
*Replicas* | [**del_pool_replica_share**](docs/apis/Replicas.md#del_pool_replica_share) | **Delete** /pools/{pool_id}/replicas/{replica_id}/share | 
*Replicas* | [**get_node_pool_replica**](docs/apis/Replicas.md#get_node_pool_replica) | **Get** /nodes/{node_id}/pools/{pool_id}/replicas/{replica_id} | 
*Replicas* | [**get_node_pool_replicas**](docs/apis/Replicas.md#get_node_pool_replicas) | **Get** /nodes/{node_id}/pools/{pool_id}/replicas | 
*Replicas* | [**get_node_replicas**](docs/apis/Replicas.md#get_node_replicas) | **Get** /nodes/{id}/replicas | 
*Replicas* | [**get_replica**](docs/apis/Replicas.md#get_replica) | **Get** /replicas/{id} | 
*Replicas* | [**get_replicas**](docs/apis/Replicas.md#get_replicas) | **Get** /replicas | 
*Replicas* | [**put_node_pool_replica**](docs/apis/Replicas.md#put_node_pool_replica) | **Put** /nodes/{node_id}/pools/{pool_id}/replicas/{replica_id} | 
*Replicas* | [**put_node_pool_replica_share**](docs/apis/Replicas.md#put_node_pool_replica_share) | **Put** /nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share/{protocol} | 
*Replicas* | [**put_pool_replica**](docs/apis/Replicas.md#put_pool_replica) | **Put** /pools/{pool_id}/replicas/{replica_id} | 
*Replicas* | [**put_pool_replica_share**](docs/apis/Replicas.md#put_pool_replica_share) | **Put** /pools/{pool_id}/replicas/{replica_id}/share/{protocol} | 
*Specs* | [**get_specs**](docs/apis/Specs.md#get_specs) | **Get** /specs | 
*Volumes* | [**del_share**](docs/apis/Volumes.md#del_share) | **Delete** /volumes{volume_id}/share | 
*Volumes* | [**del_volume**](docs/apis/Volumes.md#del_volume) | **Delete** /volumes/{volume_id} | 
*Volumes* | [**get_node_volume**](docs/apis/Volumes.md#get_node_volume) | **Get** /nodes/{node_id}/volumes/{volume_id} | 
*Volumes* | [**get_node_volumes**](docs/apis/Volumes.md#get_node_volumes) | **Get** /nodes/{node_id}/volumes | 
*Volumes* | [**get_volume**](docs/apis/Volumes.md#get_volume) | **Get** /volumes/{volume_id} | 
*Volumes* | [**get_volumes**](docs/apis/Volumes.md#get_volumes) | **Get** /volumes | 
*Volumes* | [**put_volume**](docs/apis/Volumes.md#put_volume) | **Put** /volumes/{volume_id} | 
*Volumes* | [**put_volume_share**](docs/apis/Volumes.md#put_volume_share) | **Put** /volumes/{volume_id}/share/{protocol} | 
*Watches* | [**del_watch_volume**](docs/apis/Watches.md#del_watch_volume) | **Delete** /watches/volumes/{volume_id} | 
*Watches* | [**get_watch_volume**](docs/apis/Watches.md#get_watch_volume) | **Get** /watches/volumes/{volume_id} | 
*Watches* | [**put_watch_volume**](docs/apis/Watches.md#put_watch_volume) | **Put** /watches/volumes/{volume_id} | 


## Documentation For Models

 - [BlockDevice](docs/models/BlockDevice.md)
 - [BlockDeviceFilesystem](docs/models/BlockDeviceFilesystem.md)
 - [BlockDevicePartition](docs/models/BlockDevicePartition.md)
 - [Child](docs/models/Child.md)
 - [ChildState](docs/models/ChildState.md)
 - [CreateNexusBody](docs/models/CreateNexusBody.md)
 - [CreatePoolBody](docs/models/CreatePoolBody.md)
 - [CreateReplicaBody](docs/models/CreateReplicaBody.md)
 - [CreateVolumeBody](docs/models/CreateVolumeBody.md)
 - [ExplicitTopology](docs/models/ExplicitTopology.md)
 - [LabelledTopology](docs/models/LabelledTopology.md)
 - [Nexus](docs/models/Nexus.md)
 - [NexusShareProtocol](docs/models/NexusShareProtocol.md)
 - [NexusSpec](docs/models/NexusSpec.md)
 - [NexusSpecOperation](docs/models/NexusSpecOperation.md)
 - [NexusState](docs/models/NexusState.md)
 - [Node](docs/models/Node.md)
 - [NodeState](docs/models/NodeState.md)
 - [NodeTopology](docs/models/NodeTopology.md)
 - [Pool](docs/models/Pool.md)
 - [PoolSpec](docs/models/PoolSpec.md)
 - [PoolSpecOperation](docs/models/PoolSpecOperation.md)
 - [PoolState](docs/models/PoolState.md)
 - [PoolTopology](docs/models/PoolTopology.md)
 - [Protocol](docs/models/Protocol.md)
 - [Replica](docs/models/Replica.md)
 - [ReplicaShareProtocol](docs/models/ReplicaShareProtocol.md)
 - [ReplicaSpec](docs/models/ReplicaSpec.md)
 - [ReplicaSpecOperation](docs/models/ReplicaSpecOperation.md)
 - [ReplicaSpecOwners](docs/models/ReplicaSpecOwners.md)
 - [ReplicaState](docs/models/ReplicaState.md)
 - [RestJsonError](docs/models/RestJsonError.md)
 - [RestWatch](docs/models/RestWatch.md)
 - [SpecState](docs/models/SpecState.md)
 - [Specs](docs/models/Specs.md)
 - [Topology](docs/models/Topology.md)
 - [Volume](docs/models/Volume.md)
 - [VolumeHealPolicy](docs/models/VolumeHealPolicy.md)
 - [VolumeShareProtocol](docs/models/VolumeShareProtocol.md)
 - [VolumeSpec](docs/models/VolumeSpec.md)
 - [VolumeSpecOperation](docs/models/VolumeSpecOperation.md)
 - [VolumeState](docs/models/VolumeState.md)
 - [WatchCallback](docs/models/WatchCallback.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



