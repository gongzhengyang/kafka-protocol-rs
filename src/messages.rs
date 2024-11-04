//! Messages used by the Kafka protocol.
//!
//! These messages are generated programmatically. See the [Kafka's protocol documentation](https://kafka.apache.org/protocol.html) for more information about a given message type.
// WARNING: the items of this module are generated and should not be edited directly.

#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use crate::protocol::Decodable;
#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use crate::protocol::Encodable;
#[cfg(all(feature = "client", feature = "broker"))]
use crate::protocol::Request;
use crate::protocol::{HeaderVersion, NewType, StrBytes};
#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
use anyhow::Context;
use anyhow::Result;
use std::convert::TryFrom;
#[cfg(feature = "messages_enums")]
use enum_dispatch::enum_dispatch;

pub mod consumer_protocol_assignment;
pub use consumer_protocol_assignment::ConsumerProtocolAssignment;

pub mod consumer_protocol_subscription;
pub use consumer_protocol_subscription::ConsumerProtocolSubscription;

pub mod default_principal_data;
pub use default_principal_data::DefaultPrincipalData;

pub mod k_raft_version_record;
pub use k_raft_version_record::KRaftVersionRecord;

pub mod leader_change_message;
pub use leader_change_message::LeaderChangeMessage;

pub mod request_header;
pub use request_header::RequestHeader;

pub mod response_header;
pub use response_header::ResponseHeader;

pub mod snapshot_footer_record;
pub use snapshot_footer_record::SnapshotFooterRecord;

pub mod snapshot_header_record;
pub use snapshot_header_record::SnapshotHeaderRecord;

pub mod voters_record;
pub use voters_record::VotersRecord;

pub mod produce_request;
pub use produce_request::ProduceRequest;

pub mod fetch_request;
pub use fetch_request::FetchRequest;

pub mod list_offsets_request;
pub use list_offsets_request::ListOffsetsRequest;

pub mod metadata_request;
pub use metadata_request::MetadataRequest;

pub mod leader_and_isr_request;
pub use leader_and_isr_request::LeaderAndIsrRequest;

pub mod stop_replica_request;
pub use stop_replica_request::StopReplicaRequest;

pub mod update_metadata_request;
pub use update_metadata_request::UpdateMetadataRequest;

pub mod controlled_shutdown_request;
pub use controlled_shutdown_request::ControlledShutdownRequest;

pub mod offset_commit_request;
pub use offset_commit_request::OffsetCommitRequest;

pub mod offset_fetch_request;
pub use offset_fetch_request::OffsetFetchRequest;

pub mod find_coordinator_request;
pub use find_coordinator_request::FindCoordinatorRequest;

pub mod join_group_request;
pub use join_group_request::JoinGroupRequest;

pub mod heartbeat_request;
pub use heartbeat_request::HeartbeatRequest;

pub mod leave_group_request;
pub use leave_group_request::LeaveGroupRequest;

pub mod sync_group_request;
pub use sync_group_request::SyncGroupRequest;

pub mod describe_groups_request;
pub use describe_groups_request::DescribeGroupsRequest;

pub mod list_groups_request;
pub use list_groups_request::ListGroupsRequest;

pub mod sasl_handshake_request;
pub use sasl_handshake_request::SaslHandshakeRequest;

pub mod api_versions_request;
pub use api_versions_request::ApiVersionsRequest;

pub mod create_topics_request;
pub use create_topics_request::CreateTopicsRequest;

pub mod delete_topics_request;
pub use delete_topics_request::DeleteTopicsRequest;

pub mod delete_records_request;
pub use delete_records_request::DeleteRecordsRequest;

pub mod init_producer_id_request;
pub use init_producer_id_request::InitProducerIdRequest;

pub mod offset_for_leader_epoch_request;
pub use offset_for_leader_epoch_request::OffsetForLeaderEpochRequest;

pub mod add_partitions_to_txn_request;
pub use add_partitions_to_txn_request::AddPartitionsToTxnRequest;

pub mod add_offsets_to_txn_request;
pub use add_offsets_to_txn_request::AddOffsetsToTxnRequest;

pub mod end_txn_request;
pub use end_txn_request::EndTxnRequest;

pub mod write_txn_markers_request;
pub use write_txn_markers_request::WriteTxnMarkersRequest;

pub mod txn_offset_commit_request;
pub use txn_offset_commit_request::TxnOffsetCommitRequest;

pub mod describe_acls_request;
pub use describe_acls_request::DescribeAclsRequest;

pub mod create_acls_request;
pub use create_acls_request::CreateAclsRequest;

pub mod delete_acls_request;
pub use delete_acls_request::DeleteAclsRequest;

pub mod describe_configs_request;
pub use describe_configs_request::DescribeConfigsRequest;

pub mod alter_configs_request;
pub use alter_configs_request::AlterConfigsRequest;

pub mod alter_replica_log_dirs_request;
pub use alter_replica_log_dirs_request::AlterReplicaLogDirsRequest;

pub mod describe_log_dirs_request;
pub use describe_log_dirs_request::DescribeLogDirsRequest;

pub mod sasl_authenticate_request;
pub use sasl_authenticate_request::SaslAuthenticateRequest;

pub mod create_partitions_request;
pub use create_partitions_request::CreatePartitionsRequest;

pub mod create_delegation_token_request;
pub use create_delegation_token_request::CreateDelegationTokenRequest;

pub mod renew_delegation_token_request;
pub use renew_delegation_token_request::RenewDelegationTokenRequest;

pub mod expire_delegation_token_request;
pub use expire_delegation_token_request::ExpireDelegationTokenRequest;

pub mod describe_delegation_token_request;
pub use describe_delegation_token_request::DescribeDelegationTokenRequest;

pub mod delete_groups_request;
pub use delete_groups_request::DeleteGroupsRequest;

pub mod elect_leaders_request;
pub use elect_leaders_request::ElectLeadersRequest;

pub mod incremental_alter_configs_request;
pub use incremental_alter_configs_request::IncrementalAlterConfigsRequest;

pub mod alter_partition_reassignments_request;
pub use alter_partition_reassignments_request::AlterPartitionReassignmentsRequest;

pub mod list_partition_reassignments_request;
pub use list_partition_reassignments_request::ListPartitionReassignmentsRequest;

pub mod offset_delete_request;
pub use offset_delete_request::OffsetDeleteRequest;

pub mod describe_client_quotas_request;
pub use describe_client_quotas_request::DescribeClientQuotasRequest;

pub mod alter_client_quotas_request;
pub use alter_client_quotas_request::AlterClientQuotasRequest;

pub mod describe_user_scram_credentials_request;
pub use describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest;

pub mod alter_user_scram_credentials_request;
pub use alter_user_scram_credentials_request::AlterUserScramCredentialsRequest;

pub mod vote_request;
pub use vote_request::VoteRequest;

pub mod begin_quorum_epoch_request;
pub use begin_quorum_epoch_request::BeginQuorumEpochRequest;

pub mod end_quorum_epoch_request;
pub use end_quorum_epoch_request::EndQuorumEpochRequest;

pub mod describe_quorum_request;
pub use describe_quorum_request::DescribeQuorumRequest;

pub mod alter_partition_request;
pub use alter_partition_request::AlterPartitionRequest;

pub mod update_features_request;
pub use update_features_request::UpdateFeaturesRequest;

pub mod envelope_request;
pub use envelope_request::EnvelopeRequest;

pub mod fetch_snapshot_request;
pub use fetch_snapshot_request::FetchSnapshotRequest;

pub mod describe_cluster_request;
pub use describe_cluster_request::DescribeClusterRequest;

pub mod describe_producers_request;
pub use describe_producers_request::DescribeProducersRequest;

pub mod broker_registration_request;
pub use broker_registration_request::BrokerRegistrationRequest;

pub mod broker_heartbeat_request;
pub use broker_heartbeat_request::BrokerHeartbeatRequest;

pub mod unregister_broker_request;
pub use unregister_broker_request::UnregisterBrokerRequest;

pub mod describe_transactions_request;
pub use describe_transactions_request::DescribeTransactionsRequest;

pub mod list_transactions_request;
pub use list_transactions_request::ListTransactionsRequest;

pub mod allocate_producer_ids_request;
pub use allocate_producer_ids_request::AllocateProducerIdsRequest;

pub mod consumer_group_heartbeat_request;
pub use consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest;

pub mod consumer_group_describe_request;
pub use consumer_group_describe_request::ConsumerGroupDescribeRequest;

pub mod controller_registration_request;
pub use controller_registration_request::ControllerRegistrationRequest;

pub mod get_telemetry_subscriptions_request;
pub use get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest;

pub mod push_telemetry_request;
pub use push_telemetry_request::PushTelemetryRequest;

pub mod assign_replicas_to_dirs_request;
pub use assign_replicas_to_dirs_request::AssignReplicasToDirsRequest;

pub mod list_client_metrics_resources_request;
pub use list_client_metrics_resources_request::ListClientMetricsResourcesRequest;

pub mod describe_topic_partitions_request;
pub use describe_topic_partitions_request::DescribeTopicPartitionsRequest;

pub mod produce_response;
pub use produce_response::ProduceResponse;

pub mod fetch_response;
pub use fetch_response::FetchResponse;

pub mod list_offsets_response;
pub use list_offsets_response::ListOffsetsResponse;

pub mod metadata_response;
pub use metadata_response::MetadataResponse;

pub mod leader_and_isr_response;
pub use leader_and_isr_response::LeaderAndIsrResponse;

pub mod stop_replica_response;
pub use stop_replica_response::StopReplicaResponse;

pub mod update_metadata_response;
pub use update_metadata_response::UpdateMetadataResponse;

pub mod controlled_shutdown_response;
pub use controlled_shutdown_response::ControlledShutdownResponse;

pub mod offset_commit_response;
pub use offset_commit_response::OffsetCommitResponse;

pub mod offset_fetch_response;
pub use offset_fetch_response::OffsetFetchResponse;

pub mod find_coordinator_response;
pub use find_coordinator_response::FindCoordinatorResponse;

pub mod join_group_response;
pub use join_group_response::JoinGroupResponse;

pub mod heartbeat_response;
pub use heartbeat_response::HeartbeatResponse;

pub mod leave_group_response;
pub use leave_group_response::LeaveGroupResponse;

pub mod sync_group_response;
pub use sync_group_response::SyncGroupResponse;

pub mod describe_groups_response;
pub use describe_groups_response::DescribeGroupsResponse;

pub mod list_groups_response;
pub use list_groups_response::ListGroupsResponse;

pub mod sasl_handshake_response;
pub use sasl_handshake_response::SaslHandshakeResponse;

pub mod api_versions_response;
pub use api_versions_response::ApiVersionsResponse;

pub mod create_topics_response;
pub use create_topics_response::CreateTopicsResponse;

pub mod delete_topics_response;
pub use delete_topics_response::DeleteTopicsResponse;

pub mod delete_records_response;
pub use delete_records_response::DeleteRecordsResponse;

pub mod init_producer_id_response;
pub use init_producer_id_response::InitProducerIdResponse;

pub mod offset_for_leader_epoch_response;
pub use offset_for_leader_epoch_response::OffsetForLeaderEpochResponse;

pub mod add_partitions_to_txn_response;
pub use add_partitions_to_txn_response::AddPartitionsToTxnResponse;

pub mod add_offsets_to_txn_response;
pub use add_offsets_to_txn_response::AddOffsetsToTxnResponse;

pub mod end_txn_response;
pub use end_txn_response::EndTxnResponse;

pub mod write_txn_markers_response;
pub use write_txn_markers_response::WriteTxnMarkersResponse;

pub mod txn_offset_commit_response;
pub use txn_offset_commit_response::TxnOffsetCommitResponse;

pub mod describe_acls_response;
pub use describe_acls_response::DescribeAclsResponse;

pub mod create_acls_response;
pub use create_acls_response::CreateAclsResponse;

pub mod delete_acls_response;
pub use delete_acls_response::DeleteAclsResponse;

pub mod describe_configs_response;
pub use describe_configs_response::DescribeConfigsResponse;

pub mod alter_configs_response;
pub use alter_configs_response::AlterConfigsResponse;

pub mod alter_replica_log_dirs_response;
pub use alter_replica_log_dirs_response::AlterReplicaLogDirsResponse;

pub mod describe_log_dirs_response;
pub use describe_log_dirs_response::DescribeLogDirsResponse;

pub mod sasl_authenticate_response;
pub use sasl_authenticate_response::SaslAuthenticateResponse;

pub mod create_partitions_response;
pub use create_partitions_response::CreatePartitionsResponse;

pub mod create_delegation_token_response;
pub use create_delegation_token_response::CreateDelegationTokenResponse;

pub mod renew_delegation_token_response;
pub use renew_delegation_token_response::RenewDelegationTokenResponse;

pub mod expire_delegation_token_response;
pub use expire_delegation_token_response::ExpireDelegationTokenResponse;

pub mod describe_delegation_token_response;
pub use describe_delegation_token_response::DescribeDelegationTokenResponse;

pub mod delete_groups_response;
pub use delete_groups_response::DeleteGroupsResponse;

pub mod elect_leaders_response;
pub use elect_leaders_response::ElectLeadersResponse;

pub mod incremental_alter_configs_response;
pub use incremental_alter_configs_response::IncrementalAlterConfigsResponse;

pub mod alter_partition_reassignments_response;
pub use alter_partition_reassignments_response::AlterPartitionReassignmentsResponse;

pub mod list_partition_reassignments_response;
pub use list_partition_reassignments_response::ListPartitionReassignmentsResponse;

pub mod offset_delete_response;
pub use offset_delete_response::OffsetDeleteResponse;

pub mod describe_client_quotas_response;
pub use describe_client_quotas_response::DescribeClientQuotasResponse;

pub mod alter_client_quotas_response;
pub use alter_client_quotas_response::AlterClientQuotasResponse;

pub mod describe_user_scram_credentials_response;
pub use describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse;

pub mod alter_user_scram_credentials_response;
pub use alter_user_scram_credentials_response::AlterUserScramCredentialsResponse;

pub mod vote_response;
pub use vote_response::VoteResponse;

pub mod begin_quorum_epoch_response;
pub use begin_quorum_epoch_response::BeginQuorumEpochResponse;

pub mod end_quorum_epoch_response;
pub use end_quorum_epoch_response::EndQuorumEpochResponse;

pub mod describe_quorum_response;
pub use describe_quorum_response::DescribeQuorumResponse;

pub mod alter_partition_response;
pub use alter_partition_response::AlterPartitionResponse;

pub mod update_features_response;
pub use update_features_response::UpdateFeaturesResponse;

pub mod envelope_response;
pub use envelope_response::EnvelopeResponse;

pub mod fetch_snapshot_response;
pub use fetch_snapshot_response::FetchSnapshotResponse;

pub mod describe_cluster_response;
pub use describe_cluster_response::DescribeClusterResponse;

pub mod describe_producers_response;
pub use describe_producers_response::DescribeProducersResponse;

pub mod broker_registration_response;
pub use broker_registration_response::BrokerRegistrationResponse;

pub mod broker_heartbeat_response;
pub use broker_heartbeat_response::BrokerHeartbeatResponse;

pub mod unregister_broker_response;
pub use unregister_broker_response::UnregisterBrokerResponse;

pub mod describe_transactions_response;
pub use describe_transactions_response::DescribeTransactionsResponse;

pub mod list_transactions_response;
pub use list_transactions_response::ListTransactionsResponse;

pub mod allocate_producer_ids_response;
pub use allocate_producer_ids_response::AllocateProducerIdsResponse;

pub mod consumer_group_heartbeat_response;
pub use consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse;

pub mod consumer_group_describe_response;
pub use consumer_group_describe_response::ConsumerGroupDescribeResponse;

pub mod controller_registration_response;
pub use controller_registration_response::ControllerRegistrationResponse;

pub mod get_telemetry_subscriptions_response;
pub use get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse;

pub mod push_telemetry_response;
pub use push_telemetry_response::PushTelemetryResponse;

pub mod assign_replicas_to_dirs_response;
pub use assign_replicas_to_dirs_response::AssignReplicasToDirsResponse;

pub mod list_client_metrics_resources_response;
pub use list_client_metrics_resources_response::ListClientMetricsResourcesResponse;

pub mod describe_topic_partitions_response;
pub use describe_topic_partitions_response::DescribeTopicPartitionsResponse;

macro_rules! impl_request {
    ($(($key:expr, $req_type:ty, $res_type:ty)),* $(,)?) => {
        $(
            #[cfg(all(feature = "client", feature = "broker"))]
            impl Request for $req_type {
                const KEY: i16 = $key;
                type Response = $res_type;
            }
        )*
    };
}

impl_request!(
    (0, ProduceRequest, ProduceResponse),
    (1, FetchRequest, FetchResponse),
    (2, ListOffsetsRequest, ListOffsetsResponse),
    (3, MetadataRequest, MetadataResponse),
    (4, LeaderAndIsrRequest, LeaderAndIsrResponse),
    (5, StopReplicaRequest, StopReplicaResponse),
    (6, UpdateMetadataRequest, UpdateMetadataResponse),
    (7, ControlledShutdownRequest, ControlledShutdownResponse),
    (8, OffsetCommitRequest, OffsetCommitResponse),
    (9, OffsetFetchRequest, OffsetFetchResponse),
    (10, FindCoordinatorRequest, FindCoordinatorResponse),
    (11, JoinGroupRequest, JoinGroupResponse),
    (12, HeartbeatRequest, HeartbeatResponse),
    (13, LeaveGroupRequest, LeaveGroupResponse),
    (14, SyncGroupRequest, SyncGroupResponse),
    (15, DescribeGroupsRequest, DescribeGroupsResponse),
    (16, ListGroupsRequest, ListGroupsResponse),
    (17, SaslHandshakeRequest, SaslHandshakeResponse),
    (18, ApiVersionsRequest, ApiVersionsResponse),
    (19, CreateTopicsRequest, CreateTopicsResponse),
    (20, DeleteTopicsRequest, DeleteTopicsResponse),
    (21, DeleteRecordsRequest, DeleteRecordsResponse),
    (22, InitProducerIdRequest, InitProducerIdResponse),
    (23, OffsetForLeaderEpochRequest, OffsetForLeaderEpochResponse),
    (24, AddPartitionsToTxnRequest, AddPartitionsToTxnResponse),
    (25, AddOffsetsToTxnRequest, AddOffsetsToTxnResponse),
    (26, EndTxnRequest, EndTxnResponse),
    (27, WriteTxnMarkersRequest, WriteTxnMarkersResponse),
    (28, TxnOffsetCommitRequest, TxnOffsetCommitResponse),
    (29, DescribeAclsRequest, DescribeAclsResponse),
    (30, CreateAclsRequest, CreateAclsResponse),
    (31, DeleteAclsRequest, DeleteAclsResponse),
    (32, DescribeConfigsRequest, DescribeConfigsResponse),
    (33, AlterConfigsRequest, AlterConfigsResponse),
    (34, AlterReplicaLogDirsRequest, AlterReplicaLogDirsResponse),
    (35, DescribeLogDirsRequest, DescribeLogDirsResponse),
    (36, SaslAuthenticateRequest, SaslAuthenticateResponse),
    (37, CreatePartitionsRequest, CreatePartitionsResponse),
    (38, CreateDelegationTokenRequest, CreateDelegationTokenResponse),
    (39, RenewDelegationTokenRequest, RenewDelegationTokenResponse),
    (40, ExpireDelegationTokenRequest, ExpireDelegationTokenResponse),
    (41, DescribeDelegationTokenRequest, DescribeDelegationTokenResponse),
    (42, DeleteGroupsRequest, DeleteGroupsResponse),
    (43, ElectLeadersRequest, ElectLeadersResponse),
    (44, IncrementalAlterConfigsRequest, IncrementalAlterConfigsResponse),
    (45, AlterPartitionReassignmentsRequest, AlterPartitionReassignmentsResponse),
    (46, ListPartitionReassignmentsRequest, ListPartitionReassignmentsResponse),
    (47, OffsetDeleteRequest, OffsetDeleteResponse),
    (48, DescribeClientQuotasRequest, DescribeClientQuotasResponse),
    (49, AlterClientQuotasRequest, AlterClientQuotasResponse),
    (50, DescribeUserScramCredentialsRequest, DescribeUserScramCredentialsResponse),
    (51, AlterUserScramCredentialsRequest, AlterUserScramCredentialsResponse),
    (52, VoteRequest, VoteResponse),
    (53, BeginQuorumEpochRequest, BeginQuorumEpochResponse),
    (54, EndQuorumEpochRequest, EndQuorumEpochResponse),
    (55, DescribeQuorumRequest, DescribeQuorumResponse),
    (56, AlterPartitionRequest, AlterPartitionResponse),
    (57, UpdateFeaturesRequest, UpdateFeaturesResponse),
    (58, EnvelopeRequest, EnvelopeResponse),
    (59, FetchSnapshotRequest, FetchSnapshotResponse),
    (60, DescribeClusterRequest, DescribeClusterResponse),
    (61, DescribeProducersRequest, DescribeProducersResponse),
    (62, BrokerRegistrationRequest, BrokerRegistrationResponse),
    (63, BrokerHeartbeatRequest, BrokerHeartbeatResponse),
    (64, UnregisterBrokerRequest, UnregisterBrokerResponse),
    (65, DescribeTransactionsRequest, DescribeTransactionsResponse),
    (66, ListTransactionsRequest, ListTransactionsResponse),
    (67, AllocateProducerIdsRequest, AllocateProducerIdsResponse),
    (68, ConsumerGroupHeartbeatRequest, ConsumerGroupHeartbeatResponse),
    (69, ConsumerGroupDescribeRequest, ConsumerGroupDescribeResponse),
    (70, ControllerRegistrationRequest, ControllerRegistrationResponse),
    (71, GetTelemetrySubscriptionsRequest, GetTelemetrySubscriptionsResponse),
    (72, PushTelemetryRequest, PushTelemetryResponse),
    (73, AssignReplicasToDirsRequest, AssignReplicasToDirsResponse),
    (74, ListClientMetricsResourcesRequest, ListClientMetricsResourcesResponse),
    (75, DescribeTopicPartitionsRequest, DescribeTopicPartitionsResponse),
);

/// Valid API keys in the Kafka protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKey {
    /// API key for request ProduceRequest
    ProduceKey = 0,
    /// API key for request FetchRequest
    FetchKey = 1,
    /// API key for request ListOffsetsRequest
    ListOffsetsKey = 2,
    /// API key for request MetadataRequest
    MetadataKey = 3,
    /// API key for request LeaderAndIsrRequest
    LeaderAndIsrKey = 4,
    /// API key for request StopReplicaRequest
    StopReplicaKey = 5,
    /// API key for request UpdateMetadataRequest
    UpdateMetadataKey = 6,
    /// API key for request ControlledShutdownRequest
    ControlledShutdownKey = 7,
    /// API key for request OffsetCommitRequest
    OffsetCommitKey = 8,
    /// API key for request OffsetFetchRequest
    OffsetFetchKey = 9,
    /// API key for request FindCoordinatorRequest
    FindCoordinatorKey = 10,
    /// API key for request JoinGroupRequest
    JoinGroupKey = 11,
    /// API key for request HeartbeatRequest
    HeartbeatKey = 12,
    /// API key for request LeaveGroupRequest
    LeaveGroupKey = 13,
    /// API key for request SyncGroupRequest
    SyncGroupKey = 14,
    /// API key for request DescribeGroupsRequest
    DescribeGroupsKey = 15,
    /// API key for request ListGroupsRequest
    ListGroupsKey = 16,
    /// API key for request SaslHandshakeRequest
    SaslHandshakeKey = 17,
    /// API key for request ApiVersionsRequest
    ApiVersionsKey = 18,
    /// API key for request CreateTopicsRequest
    CreateTopicsKey = 19,
    /// API key for request DeleteTopicsRequest
    DeleteTopicsKey = 20,
    /// API key for request DeleteRecordsRequest
    DeleteRecordsKey = 21,
    /// API key for request InitProducerIdRequest
    InitProducerIdKey = 22,
    /// API key for request OffsetForLeaderEpochRequest
    OffsetForLeaderEpochKey = 23,
    /// API key for request AddPartitionsToTxnRequest
    AddPartitionsToTxnKey = 24,
    /// API key for request AddOffsetsToTxnRequest
    AddOffsetsToTxnKey = 25,
    /// API key for request EndTxnRequest
    EndTxnKey = 26,
    /// API key for request WriteTxnMarkersRequest
    WriteTxnMarkersKey = 27,
    /// API key for request TxnOffsetCommitRequest
    TxnOffsetCommitKey = 28,
    /// API key for request DescribeAclsRequest
    DescribeAclsKey = 29,
    /// API key for request CreateAclsRequest
    CreateAclsKey = 30,
    /// API key for request DeleteAclsRequest
    DeleteAclsKey = 31,
    /// API key for request DescribeConfigsRequest
    DescribeConfigsKey = 32,
    /// API key for request AlterConfigsRequest
    AlterConfigsKey = 33,
    /// API key for request AlterReplicaLogDirsRequest
    AlterReplicaLogDirsKey = 34,
    /// API key for request DescribeLogDirsRequest
    DescribeLogDirsKey = 35,
    /// API key for request SaslAuthenticateRequest
    SaslAuthenticateKey = 36,
    /// API key for request CreatePartitionsRequest
    CreatePartitionsKey = 37,
    /// API key for request CreateDelegationTokenRequest
    CreateDelegationTokenKey = 38,
    /// API key for request RenewDelegationTokenRequest
    RenewDelegationTokenKey = 39,
    /// API key for request ExpireDelegationTokenRequest
    ExpireDelegationTokenKey = 40,
    /// API key for request DescribeDelegationTokenRequest
    DescribeDelegationTokenKey = 41,
    /// API key for request DeleteGroupsRequest
    DeleteGroupsKey = 42,
    /// API key for request ElectLeadersRequest
    ElectLeadersKey = 43,
    /// API key for request IncrementalAlterConfigsRequest
    IncrementalAlterConfigsKey = 44,
    /// API key for request AlterPartitionReassignmentsRequest
    AlterPartitionReassignmentsKey = 45,
    /// API key for request ListPartitionReassignmentsRequest
    ListPartitionReassignmentsKey = 46,
    /// API key for request OffsetDeleteRequest
    OffsetDeleteKey = 47,
    /// API key for request DescribeClientQuotasRequest
    DescribeClientQuotasKey = 48,
    /// API key for request AlterClientQuotasRequest
    AlterClientQuotasKey = 49,
    /// API key for request DescribeUserScramCredentialsRequest
    DescribeUserScramCredentialsKey = 50,
    /// API key for request AlterUserScramCredentialsRequest
    AlterUserScramCredentialsKey = 51,
    /// API key for request VoteRequest
    VoteKey = 52,
    /// API key for request BeginQuorumEpochRequest
    BeginQuorumEpochKey = 53,
    /// API key for request EndQuorumEpochRequest
    EndQuorumEpochKey = 54,
    /// API key for request DescribeQuorumRequest
    DescribeQuorumKey = 55,
    /// API key for request AlterPartitionRequest
    AlterPartitionKey = 56,
    /// API key for request UpdateFeaturesRequest
    UpdateFeaturesKey = 57,
    /// API key for request EnvelopeRequest
    EnvelopeKey = 58,
    /// API key for request FetchSnapshotRequest
    FetchSnapshotKey = 59,
    /// API key for request DescribeClusterRequest
    DescribeClusterKey = 60,
    /// API key for request DescribeProducersRequest
    DescribeProducersKey = 61,
    /// API key for request BrokerRegistrationRequest
    BrokerRegistrationKey = 62,
    /// API key for request BrokerHeartbeatRequest
    BrokerHeartbeatKey = 63,
    /// API key for request UnregisterBrokerRequest
    UnregisterBrokerKey = 64,
    /// API key for request DescribeTransactionsRequest
    DescribeTransactionsKey = 65,
    /// API key for request ListTransactionsRequest
    ListTransactionsKey = 66,
    /// API key for request AllocateProducerIdsRequest
    AllocateProducerIdsKey = 67,
    /// API key for request ConsumerGroupHeartbeatRequest
    ConsumerGroupHeartbeatKey = 68,
    /// API key for request ConsumerGroupDescribeRequest
    ConsumerGroupDescribeKey = 69,
    /// API key for request ControllerRegistrationRequest
    ControllerRegistrationKey = 70,
    /// API key for request GetTelemetrySubscriptionsRequest
    GetTelemetrySubscriptionsKey = 71,
    /// API key for request PushTelemetryRequest
    PushTelemetryKey = 72,
    /// API key for request AssignReplicasToDirsRequest
    AssignReplicasToDirsKey = 73,
    /// API key for request ListClientMetricsResourcesRequest
    ListClientMetricsResourcesKey = 74,
    /// API key for request DescribeTopicPartitionsRequest
    DescribeTopicPartitionsKey = 75,
}

impl ApiKey {
    /// Get the version of request header that needs to be prepended to this message
    pub fn request_header_version(&self, version: i16) -> i16 {
        match self {
            ApiKey::ProduceKey => ProduceRequest::header_version(version),
            ApiKey::FetchKey => FetchRequest::header_version(version),
            ApiKey::ListOffsetsKey => ListOffsetsRequest::header_version(version),
            ApiKey::MetadataKey => MetadataRequest::header_version(version),
            ApiKey::LeaderAndIsrKey => LeaderAndIsrRequest::header_version(version),
            ApiKey::StopReplicaKey => StopReplicaRequest::header_version(version),
            ApiKey::UpdateMetadataKey => UpdateMetadataRequest::header_version(version),
            ApiKey::ControlledShutdownKey => ControlledShutdownRequest::header_version(version),
            ApiKey::OffsetCommitKey => OffsetCommitRequest::header_version(version),
            ApiKey::OffsetFetchKey => OffsetFetchRequest::header_version(version),
            ApiKey::FindCoordinatorKey => FindCoordinatorRequest::header_version(version),
            ApiKey::JoinGroupKey => JoinGroupRequest::header_version(version),
            ApiKey::HeartbeatKey => HeartbeatRequest::header_version(version),
            ApiKey::LeaveGroupKey => LeaveGroupRequest::header_version(version),
            ApiKey::SyncGroupKey => SyncGroupRequest::header_version(version),
            ApiKey::DescribeGroupsKey => DescribeGroupsRequest::header_version(version),
            ApiKey::ListGroupsKey => ListGroupsRequest::header_version(version),
            ApiKey::SaslHandshakeKey => SaslHandshakeRequest::header_version(version),
            ApiKey::ApiVersionsKey => ApiVersionsRequest::header_version(version),
            ApiKey::CreateTopicsKey => CreateTopicsRequest::header_version(version),
            ApiKey::DeleteTopicsKey => DeleteTopicsRequest::header_version(version),
            ApiKey::DeleteRecordsKey => DeleteRecordsRequest::header_version(version),
            ApiKey::InitProducerIdKey => InitProducerIdRequest::header_version(version),
            ApiKey::OffsetForLeaderEpochKey => OffsetForLeaderEpochRequest::header_version(version),
            ApiKey::AddPartitionsToTxnKey => AddPartitionsToTxnRequest::header_version(version),
            ApiKey::AddOffsetsToTxnKey => AddOffsetsToTxnRequest::header_version(version),
            ApiKey::EndTxnKey => EndTxnRequest::header_version(version),
            ApiKey::WriteTxnMarkersKey => WriteTxnMarkersRequest::header_version(version),
            ApiKey::TxnOffsetCommitKey => TxnOffsetCommitRequest::header_version(version),
            ApiKey::DescribeAclsKey => DescribeAclsRequest::header_version(version),
            ApiKey::CreateAclsKey => CreateAclsRequest::header_version(version),
            ApiKey::DeleteAclsKey => DeleteAclsRequest::header_version(version),
            ApiKey::DescribeConfigsKey => DescribeConfigsRequest::header_version(version),
            ApiKey::AlterConfigsKey => AlterConfigsRequest::header_version(version),
            ApiKey::AlterReplicaLogDirsKey => AlterReplicaLogDirsRequest::header_version(version),
            ApiKey::DescribeLogDirsKey => DescribeLogDirsRequest::header_version(version),
            ApiKey::SaslAuthenticateKey => SaslAuthenticateRequest::header_version(version),
            ApiKey::CreatePartitionsKey => CreatePartitionsRequest::header_version(version),
            ApiKey::CreateDelegationTokenKey => {
                CreateDelegationTokenRequest::header_version(version)
            }
            ApiKey::RenewDelegationTokenKey => RenewDelegationTokenRequest::header_version(version),
            ApiKey::ExpireDelegationTokenKey => {
                ExpireDelegationTokenRequest::header_version(version)
            }
            ApiKey::DescribeDelegationTokenKey => {
                DescribeDelegationTokenRequest::header_version(version)
            }
            ApiKey::DeleteGroupsKey => DeleteGroupsRequest::header_version(version),
            ApiKey::ElectLeadersKey => ElectLeadersRequest::header_version(version),
            ApiKey::IncrementalAlterConfigsKey => {
                IncrementalAlterConfigsRequest::header_version(version)
            }
            ApiKey::AlterPartitionReassignmentsKey => {
                AlterPartitionReassignmentsRequest::header_version(version)
            }
            ApiKey::ListPartitionReassignmentsKey => {
                ListPartitionReassignmentsRequest::header_version(version)
            }
            ApiKey::OffsetDeleteKey => OffsetDeleteRequest::header_version(version),
            ApiKey::DescribeClientQuotasKey => DescribeClientQuotasRequest::header_version(version),
            ApiKey::AlterClientQuotasKey => AlterClientQuotasRequest::header_version(version),
            ApiKey::DescribeUserScramCredentialsKey => {
                DescribeUserScramCredentialsRequest::header_version(version)
            }
            ApiKey::AlterUserScramCredentialsKey => {
                AlterUserScramCredentialsRequest::header_version(version)
            }
            ApiKey::VoteKey => VoteRequest::header_version(version),
            ApiKey::BeginQuorumEpochKey => BeginQuorumEpochRequest::header_version(version),
            ApiKey::EndQuorumEpochKey => EndQuorumEpochRequest::header_version(version),
            ApiKey::DescribeQuorumKey => DescribeQuorumRequest::header_version(version),
            ApiKey::AlterPartitionKey => AlterPartitionRequest::header_version(version),
            ApiKey::UpdateFeaturesKey => UpdateFeaturesRequest::header_version(version),
            ApiKey::EnvelopeKey => EnvelopeRequest::header_version(version),
            ApiKey::FetchSnapshotKey => FetchSnapshotRequest::header_version(version),
            ApiKey::DescribeClusterKey => DescribeClusterRequest::header_version(version),
            ApiKey::DescribeProducersKey => DescribeProducersRequest::header_version(version),
            ApiKey::BrokerRegistrationKey => BrokerRegistrationRequest::header_version(version),
            ApiKey::BrokerHeartbeatKey => BrokerHeartbeatRequest::header_version(version),
            ApiKey::UnregisterBrokerKey => UnregisterBrokerRequest::header_version(version),
            ApiKey::DescribeTransactionsKey => DescribeTransactionsRequest::header_version(version),
            ApiKey::ListTransactionsKey => ListTransactionsRequest::header_version(version),
            ApiKey::AllocateProducerIdsKey => AllocateProducerIdsRequest::header_version(version),
            ApiKey::ConsumerGroupHeartbeatKey => {
                ConsumerGroupHeartbeatRequest::header_version(version)
            }
            ApiKey::ConsumerGroupDescribeKey => {
                ConsumerGroupDescribeRequest::header_version(version)
            }
            ApiKey::ControllerRegistrationKey => {
                ControllerRegistrationRequest::header_version(version)
            }
            ApiKey::GetTelemetrySubscriptionsKey => {
                GetTelemetrySubscriptionsRequest::header_version(version)
            }
            ApiKey::PushTelemetryKey => PushTelemetryRequest::header_version(version),
            ApiKey::AssignReplicasToDirsKey => AssignReplicasToDirsRequest::header_version(version),
            ApiKey::ListClientMetricsResourcesKey => {
                ListClientMetricsResourcesRequest::header_version(version)
            }
            ApiKey::DescribeTopicPartitionsKey => {
                DescribeTopicPartitionsRequest::header_version(version)
            }
        }
    }
    /// Get the version of response header that needs to be prepended to this message
    pub fn response_header_version(&self, version: i16) -> i16 {
        match self {
            ApiKey::ProduceKey => ProduceResponse::header_version(version),
            ApiKey::FetchKey => FetchResponse::header_version(version),
            ApiKey::ListOffsetsKey => ListOffsetsResponse::header_version(version),
            ApiKey::MetadataKey => MetadataResponse::header_version(version),
            ApiKey::LeaderAndIsrKey => LeaderAndIsrResponse::header_version(version),
            ApiKey::StopReplicaKey => StopReplicaResponse::header_version(version),
            ApiKey::UpdateMetadataKey => UpdateMetadataResponse::header_version(version),
            ApiKey::ControlledShutdownKey => ControlledShutdownResponse::header_version(version),
            ApiKey::OffsetCommitKey => OffsetCommitResponse::header_version(version),
            ApiKey::OffsetFetchKey => OffsetFetchResponse::header_version(version),
            ApiKey::FindCoordinatorKey => FindCoordinatorResponse::header_version(version),
            ApiKey::JoinGroupKey => JoinGroupResponse::header_version(version),
            ApiKey::HeartbeatKey => HeartbeatResponse::header_version(version),
            ApiKey::LeaveGroupKey => LeaveGroupResponse::header_version(version),
            ApiKey::SyncGroupKey => SyncGroupResponse::header_version(version),
            ApiKey::DescribeGroupsKey => DescribeGroupsResponse::header_version(version),
            ApiKey::ListGroupsKey => ListGroupsResponse::header_version(version),
            ApiKey::SaslHandshakeKey => SaslHandshakeResponse::header_version(version),
            ApiKey::ApiVersionsKey => ApiVersionsResponse::header_version(version),
            ApiKey::CreateTopicsKey => CreateTopicsResponse::header_version(version),
            ApiKey::DeleteTopicsKey => DeleteTopicsResponse::header_version(version),
            ApiKey::DeleteRecordsKey => DeleteRecordsResponse::header_version(version),
            ApiKey::InitProducerIdKey => InitProducerIdResponse::header_version(version),
            ApiKey::OffsetForLeaderEpochKey => {
                OffsetForLeaderEpochResponse::header_version(version)
            }
            ApiKey::AddPartitionsToTxnKey => AddPartitionsToTxnResponse::header_version(version),
            ApiKey::AddOffsetsToTxnKey => AddOffsetsToTxnResponse::header_version(version),
            ApiKey::EndTxnKey => EndTxnResponse::header_version(version),
            ApiKey::WriteTxnMarkersKey => WriteTxnMarkersResponse::header_version(version),
            ApiKey::TxnOffsetCommitKey => TxnOffsetCommitResponse::header_version(version),
            ApiKey::DescribeAclsKey => DescribeAclsResponse::header_version(version),
            ApiKey::CreateAclsKey => CreateAclsResponse::header_version(version),
            ApiKey::DeleteAclsKey => DeleteAclsResponse::header_version(version),
            ApiKey::DescribeConfigsKey => DescribeConfigsResponse::header_version(version),
            ApiKey::AlterConfigsKey => AlterConfigsResponse::header_version(version),
            ApiKey::AlterReplicaLogDirsKey => AlterReplicaLogDirsResponse::header_version(version),
            ApiKey::DescribeLogDirsKey => DescribeLogDirsResponse::header_version(version),
            ApiKey::SaslAuthenticateKey => SaslAuthenticateResponse::header_version(version),
            ApiKey::CreatePartitionsKey => CreatePartitionsResponse::header_version(version),
            ApiKey::CreateDelegationTokenKey => {
                CreateDelegationTokenResponse::header_version(version)
            }
            ApiKey::RenewDelegationTokenKey => {
                RenewDelegationTokenResponse::header_version(version)
            }
            ApiKey::ExpireDelegationTokenKey => {
                ExpireDelegationTokenResponse::header_version(version)
            }
            ApiKey::DescribeDelegationTokenKey => {
                DescribeDelegationTokenResponse::header_version(version)
            }
            ApiKey::DeleteGroupsKey => DeleteGroupsResponse::header_version(version),
            ApiKey::ElectLeadersKey => ElectLeadersResponse::header_version(version),
            ApiKey::IncrementalAlterConfigsKey => {
                IncrementalAlterConfigsResponse::header_version(version)
            }
            ApiKey::AlterPartitionReassignmentsKey => {
                AlterPartitionReassignmentsResponse::header_version(version)
            }
            ApiKey::ListPartitionReassignmentsKey => {
                ListPartitionReassignmentsResponse::header_version(version)
            }
            ApiKey::OffsetDeleteKey => OffsetDeleteResponse::header_version(version),
            ApiKey::DescribeClientQuotasKey => {
                DescribeClientQuotasResponse::header_version(version)
            }
            ApiKey::AlterClientQuotasKey => AlterClientQuotasResponse::header_version(version),
            ApiKey::DescribeUserScramCredentialsKey => {
                DescribeUserScramCredentialsResponse::header_version(version)
            }
            ApiKey::AlterUserScramCredentialsKey => {
                AlterUserScramCredentialsResponse::header_version(version)
            }
            ApiKey::VoteKey => VoteResponse::header_version(version),
            ApiKey::BeginQuorumEpochKey => BeginQuorumEpochResponse::header_version(version),
            ApiKey::EndQuorumEpochKey => EndQuorumEpochResponse::header_version(version),
            ApiKey::DescribeQuorumKey => DescribeQuorumResponse::header_version(version),
            ApiKey::AlterPartitionKey => AlterPartitionResponse::header_version(version),
            ApiKey::UpdateFeaturesKey => UpdateFeaturesResponse::header_version(version),
            ApiKey::EnvelopeKey => EnvelopeResponse::header_version(version),
            ApiKey::FetchSnapshotKey => FetchSnapshotResponse::header_version(version),
            ApiKey::DescribeClusterKey => DescribeClusterResponse::header_version(version),
            ApiKey::DescribeProducersKey => DescribeProducersResponse::header_version(version),
            ApiKey::BrokerRegistrationKey => BrokerRegistrationResponse::header_version(version),
            ApiKey::BrokerHeartbeatKey => BrokerHeartbeatResponse::header_version(version),
            ApiKey::UnregisterBrokerKey => UnregisterBrokerResponse::header_version(version),
            ApiKey::DescribeTransactionsKey => {
                DescribeTransactionsResponse::header_version(version)
            }
            ApiKey::ListTransactionsKey => ListTransactionsResponse::header_version(version),
            ApiKey::AllocateProducerIdsKey => AllocateProducerIdsResponse::header_version(version),
            ApiKey::ConsumerGroupHeartbeatKey => {
                ConsumerGroupHeartbeatResponse::header_version(version)
            }
            ApiKey::ConsumerGroupDescribeKey => {
                ConsumerGroupDescribeResponse::header_version(version)
            }
            ApiKey::ControllerRegistrationKey => {
                ControllerRegistrationResponse::header_version(version)
            }
            ApiKey::GetTelemetrySubscriptionsKey => {
                GetTelemetrySubscriptionsResponse::header_version(version)
            }
            ApiKey::PushTelemetryKey => PushTelemetryResponse::header_version(version),
            ApiKey::AssignReplicasToDirsKey => {
                AssignReplicasToDirsResponse::header_version(version)
            }
            ApiKey::ListClientMetricsResourcesKey => {
                ListClientMetricsResourcesResponse::header_version(version)
            }
            ApiKey::DescribeTopicPartitionsKey => {
                DescribeTopicPartitionsResponse::header_version(version)
            }
        }
    }
}
impl TryFrom<i16> for ApiKey {
    type Error = ();

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == ApiKey::ProduceKey as i16 => Ok(ApiKey::ProduceKey),
            x if x == ApiKey::FetchKey as i16 => Ok(ApiKey::FetchKey),
            x if x == ApiKey::ListOffsetsKey as i16 => Ok(ApiKey::ListOffsetsKey),
            x if x == ApiKey::MetadataKey as i16 => Ok(ApiKey::MetadataKey),
            x if x == ApiKey::LeaderAndIsrKey as i16 => Ok(ApiKey::LeaderAndIsrKey),
            x if x == ApiKey::StopReplicaKey as i16 => Ok(ApiKey::StopReplicaKey),
            x if x == ApiKey::UpdateMetadataKey as i16 => Ok(ApiKey::UpdateMetadataKey),
            x if x == ApiKey::ControlledShutdownKey as i16 => Ok(ApiKey::ControlledShutdownKey),
            x if x == ApiKey::OffsetCommitKey as i16 => Ok(ApiKey::OffsetCommitKey),
            x if x == ApiKey::OffsetFetchKey as i16 => Ok(ApiKey::OffsetFetchKey),
            x if x == ApiKey::FindCoordinatorKey as i16 => Ok(ApiKey::FindCoordinatorKey),
            x if x == ApiKey::JoinGroupKey as i16 => Ok(ApiKey::JoinGroupKey),
            x if x == ApiKey::HeartbeatKey as i16 => Ok(ApiKey::HeartbeatKey),
            x if x == ApiKey::LeaveGroupKey as i16 => Ok(ApiKey::LeaveGroupKey),
            x if x == ApiKey::SyncGroupKey as i16 => Ok(ApiKey::SyncGroupKey),
            x if x == ApiKey::DescribeGroupsKey as i16 => Ok(ApiKey::DescribeGroupsKey),
            x if x == ApiKey::ListGroupsKey as i16 => Ok(ApiKey::ListGroupsKey),
            x if x == ApiKey::SaslHandshakeKey as i16 => Ok(ApiKey::SaslHandshakeKey),
            x if x == ApiKey::ApiVersionsKey as i16 => Ok(ApiKey::ApiVersionsKey),
            x if x == ApiKey::CreateTopicsKey as i16 => Ok(ApiKey::CreateTopicsKey),
            x if x == ApiKey::DeleteTopicsKey as i16 => Ok(ApiKey::DeleteTopicsKey),
            x if x == ApiKey::DeleteRecordsKey as i16 => Ok(ApiKey::DeleteRecordsKey),
            x if x == ApiKey::InitProducerIdKey as i16 => Ok(ApiKey::InitProducerIdKey),
            x if x == ApiKey::OffsetForLeaderEpochKey as i16 => Ok(ApiKey::OffsetForLeaderEpochKey),
            x if x == ApiKey::AddPartitionsToTxnKey as i16 => Ok(ApiKey::AddPartitionsToTxnKey),
            x if x == ApiKey::AddOffsetsToTxnKey as i16 => Ok(ApiKey::AddOffsetsToTxnKey),
            x if x == ApiKey::EndTxnKey as i16 => Ok(ApiKey::EndTxnKey),
            x if x == ApiKey::WriteTxnMarkersKey as i16 => Ok(ApiKey::WriteTxnMarkersKey),
            x if x == ApiKey::TxnOffsetCommitKey as i16 => Ok(ApiKey::TxnOffsetCommitKey),
            x if x == ApiKey::DescribeAclsKey as i16 => Ok(ApiKey::DescribeAclsKey),
            x if x == ApiKey::CreateAclsKey as i16 => Ok(ApiKey::CreateAclsKey),
            x if x == ApiKey::DeleteAclsKey as i16 => Ok(ApiKey::DeleteAclsKey),
            x if x == ApiKey::DescribeConfigsKey as i16 => Ok(ApiKey::DescribeConfigsKey),
            x if x == ApiKey::AlterConfigsKey as i16 => Ok(ApiKey::AlterConfigsKey),
            x if x == ApiKey::AlterReplicaLogDirsKey as i16 => Ok(ApiKey::AlterReplicaLogDirsKey),
            x if x == ApiKey::DescribeLogDirsKey as i16 => Ok(ApiKey::DescribeLogDirsKey),
            x if x == ApiKey::SaslAuthenticateKey as i16 => Ok(ApiKey::SaslAuthenticateKey),
            x if x == ApiKey::CreatePartitionsKey as i16 => Ok(ApiKey::CreatePartitionsKey),
            x if x == ApiKey::CreateDelegationTokenKey as i16 => {
                Ok(ApiKey::CreateDelegationTokenKey)
            }
            x if x == ApiKey::RenewDelegationTokenKey as i16 => Ok(ApiKey::RenewDelegationTokenKey),
            x if x == ApiKey::ExpireDelegationTokenKey as i16 => {
                Ok(ApiKey::ExpireDelegationTokenKey)
            }
            x if x == ApiKey::DescribeDelegationTokenKey as i16 => {
                Ok(ApiKey::DescribeDelegationTokenKey)
            }
            x if x == ApiKey::DeleteGroupsKey as i16 => Ok(ApiKey::DeleteGroupsKey),
            x if x == ApiKey::ElectLeadersKey as i16 => Ok(ApiKey::ElectLeadersKey),
            x if x == ApiKey::IncrementalAlterConfigsKey as i16 => {
                Ok(ApiKey::IncrementalAlterConfigsKey)
            }
            x if x == ApiKey::AlterPartitionReassignmentsKey as i16 => {
                Ok(ApiKey::AlterPartitionReassignmentsKey)
            }
            x if x == ApiKey::ListPartitionReassignmentsKey as i16 => {
                Ok(ApiKey::ListPartitionReassignmentsKey)
            }
            x if x == ApiKey::OffsetDeleteKey as i16 => Ok(ApiKey::OffsetDeleteKey),
            x if x == ApiKey::DescribeClientQuotasKey as i16 => Ok(ApiKey::DescribeClientQuotasKey),
            x if x == ApiKey::AlterClientQuotasKey as i16 => Ok(ApiKey::AlterClientQuotasKey),
            x if x == ApiKey::DescribeUserScramCredentialsKey as i16 => {
                Ok(ApiKey::DescribeUserScramCredentialsKey)
            }
            x if x == ApiKey::AlterUserScramCredentialsKey as i16 => {
                Ok(ApiKey::AlterUserScramCredentialsKey)
            }
            x if x == ApiKey::VoteKey as i16 => Ok(ApiKey::VoteKey),
            x if x == ApiKey::BeginQuorumEpochKey as i16 => Ok(ApiKey::BeginQuorumEpochKey),
            x if x == ApiKey::EndQuorumEpochKey as i16 => Ok(ApiKey::EndQuorumEpochKey),
            x if x == ApiKey::DescribeQuorumKey as i16 => Ok(ApiKey::DescribeQuorumKey),
            x if x == ApiKey::AlterPartitionKey as i16 => Ok(ApiKey::AlterPartitionKey),
            x if x == ApiKey::UpdateFeaturesKey as i16 => Ok(ApiKey::UpdateFeaturesKey),
            x if x == ApiKey::EnvelopeKey as i16 => Ok(ApiKey::EnvelopeKey),
            x if x == ApiKey::FetchSnapshotKey as i16 => Ok(ApiKey::FetchSnapshotKey),
            x if x == ApiKey::DescribeClusterKey as i16 => Ok(ApiKey::DescribeClusterKey),
            x if x == ApiKey::DescribeProducersKey as i16 => Ok(ApiKey::DescribeProducersKey),
            x if x == ApiKey::BrokerRegistrationKey as i16 => Ok(ApiKey::BrokerRegistrationKey),
            x if x == ApiKey::BrokerHeartbeatKey as i16 => Ok(ApiKey::BrokerHeartbeatKey),
            x if x == ApiKey::UnregisterBrokerKey as i16 => Ok(ApiKey::UnregisterBrokerKey),
            x if x == ApiKey::DescribeTransactionsKey as i16 => Ok(ApiKey::DescribeTransactionsKey),
            x if x == ApiKey::ListTransactionsKey as i16 => Ok(ApiKey::ListTransactionsKey),
            x if x == ApiKey::AllocateProducerIdsKey as i16 => Ok(ApiKey::AllocateProducerIdsKey),
            x if x == ApiKey::ConsumerGroupHeartbeatKey as i16 => {
                Ok(ApiKey::ConsumerGroupHeartbeatKey)
            }
            x if x == ApiKey::ConsumerGroupDescribeKey as i16 => {
                Ok(ApiKey::ConsumerGroupDescribeKey)
            }
            x if x == ApiKey::ControllerRegistrationKey as i16 => {
                Ok(ApiKey::ControllerRegistrationKey)
            }
            x if x == ApiKey::GetTelemetrySubscriptionsKey as i16 => {
                Ok(ApiKey::GetTelemetrySubscriptionsKey)
            }
            x if x == ApiKey::PushTelemetryKey as i16 => Ok(ApiKey::PushTelemetryKey),
            x if x == ApiKey::AssignReplicasToDirsKey as i16 => Ok(ApiKey::AssignReplicasToDirsKey),
            x if x == ApiKey::ListClientMetricsResourcesKey as i16 => {
                Ok(ApiKey::ListClientMetricsResourcesKey)
            }
            x if x == ApiKey::DescribeTopicPartitionsKey as i16 => {
                Ok(ApiKey::DescribeTopicPartitionsKey)
            }
            _ => Err(()),
        }
    }
}

/// Wrapping enum for all requests in the Kafka protocol.
#[cfg(feature = "messages_enums")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[enum_dispatch(Encodable, Decodable)]
pub enum RequestKind {
    /// ProduceRequest,
    Produce(ProduceRequest),
    /// FetchRequest,
    Fetch(FetchRequest),
    /// ListOffsetsRequest,
    ListOffsets(ListOffsetsRequest),
    /// MetadataRequest,
    Metadata(MetadataRequest),
    /// LeaderAndIsrRequest,
    LeaderAndIsr(LeaderAndIsrRequest),
    /// StopReplicaRequest,
    StopReplica(StopReplicaRequest),
    /// UpdateMetadataRequest,
    UpdateMetadata(UpdateMetadataRequest),
    /// ControlledShutdownRequest,
    ControlledShutdown(ControlledShutdownRequest),
    /// OffsetCommitRequest,
    OffsetCommit(OffsetCommitRequest),
    /// OffsetFetchRequest,
    OffsetFetch(OffsetFetchRequest),
    /// FindCoordinatorRequest,
    FindCoordinator(FindCoordinatorRequest),
    /// JoinGroupRequest,
    JoinGroup(JoinGroupRequest),
    /// HeartbeatRequest,
    Heartbeat(HeartbeatRequest),
    /// LeaveGroupRequest,
    LeaveGroup(LeaveGroupRequest),
    /// SyncGroupRequest,
    SyncGroup(SyncGroupRequest),
    /// DescribeGroupsRequest,
    DescribeGroups(DescribeGroupsRequest),
    /// ListGroupsRequest,
    ListGroups(ListGroupsRequest),
    /// SaslHandshakeRequest,
    SaslHandshake(SaslHandshakeRequest),
    /// ApiVersionsRequest,
    ApiVersions(ApiVersionsRequest),
    /// CreateTopicsRequest,
    CreateTopics(CreateTopicsRequest),
    /// DeleteTopicsRequest,
    DeleteTopics(DeleteTopicsRequest),
    /// DeleteRecordsRequest,
    DeleteRecords(DeleteRecordsRequest),
    /// InitProducerIdRequest,
    InitProducerId(InitProducerIdRequest),
    /// OffsetForLeaderEpochRequest,
    OffsetForLeaderEpoch(OffsetForLeaderEpochRequest),
    /// AddPartitionsToTxnRequest,
    AddPartitionsToTxn(AddPartitionsToTxnRequest),
    /// AddOffsetsToTxnRequest,
    AddOffsetsToTxn(AddOffsetsToTxnRequest),
    /// EndTxnRequest,
    EndTxn(EndTxnRequest),
    /// WriteTxnMarkersRequest,
    WriteTxnMarkers(WriteTxnMarkersRequest),
    /// TxnOffsetCommitRequest,
    TxnOffsetCommit(TxnOffsetCommitRequest),
    /// DescribeAclsRequest,
    DescribeAcls(DescribeAclsRequest),
    /// CreateAclsRequest,
    CreateAcls(CreateAclsRequest),
    /// DeleteAclsRequest,
    DeleteAcls(DeleteAclsRequest),
    /// DescribeConfigsRequest,
    DescribeConfigs(DescribeConfigsRequest),
    /// AlterConfigsRequest,
    AlterConfigs(AlterConfigsRequest),
    /// AlterReplicaLogDirsRequest,
    AlterReplicaLogDirs(AlterReplicaLogDirsRequest),
    /// DescribeLogDirsRequest,
    DescribeLogDirs(DescribeLogDirsRequest),
    /// SaslAuthenticateRequest,
    SaslAuthenticate(SaslAuthenticateRequest),
    /// CreatePartitionsRequest,
    CreatePartitions(CreatePartitionsRequest),
    /// CreateDelegationTokenRequest,
    CreateDelegationToken(CreateDelegationTokenRequest),
    /// RenewDelegationTokenRequest,
    RenewDelegationToken(RenewDelegationTokenRequest),
    /// ExpireDelegationTokenRequest,
    ExpireDelegationToken(ExpireDelegationTokenRequest),
    /// DescribeDelegationTokenRequest,
    DescribeDelegationToken(DescribeDelegationTokenRequest),
    /// DeleteGroupsRequest,
    DeleteGroups(DeleteGroupsRequest),
    /// ElectLeadersRequest,
    ElectLeaders(ElectLeadersRequest),
    /// IncrementalAlterConfigsRequest,
    IncrementalAlterConfigs(IncrementalAlterConfigsRequest),
    /// AlterPartitionReassignmentsRequest,
    AlterPartitionReassignments(AlterPartitionReassignmentsRequest),
    /// ListPartitionReassignmentsRequest,
    ListPartitionReassignments(ListPartitionReassignmentsRequest),
    /// OffsetDeleteRequest,
    OffsetDelete(OffsetDeleteRequest),
    /// DescribeClientQuotasRequest,
    DescribeClientQuotas(DescribeClientQuotasRequest),
    /// AlterClientQuotasRequest,
    AlterClientQuotas(AlterClientQuotasRequest),
    /// DescribeUserScramCredentialsRequest,
    DescribeUserScramCredentials(DescribeUserScramCredentialsRequest),
    /// AlterUserScramCredentialsRequest,
    AlterUserScramCredentials(AlterUserScramCredentialsRequest),
    /// VoteRequest,
    Vote(VoteRequest),
    /// BeginQuorumEpochRequest,
    BeginQuorumEpoch(BeginQuorumEpochRequest),
    /// EndQuorumEpochRequest,
    EndQuorumEpoch(EndQuorumEpochRequest),
    /// DescribeQuorumRequest,
    DescribeQuorum(DescribeQuorumRequest),
    /// AlterPartitionRequest,
    AlterPartition(AlterPartitionRequest),
    /// UpdateFeaturesRequest,
    UpdateFeatures(UpdateFeaturesRequest),
    /// EnvelopeRequest,
    Envelope(EnvelopeRequest),
    /// FetchSnapshotRequest,
    FetchSnapshot(FetchSnapshotRequest),
    /// DescribeClusterRequest,
    DescribeCluster(DescribeClusterRequest),
    /// DescribeProducersRequest,
    DescribeProducers(DescribeProducersRequest),
    /// BrokerRegistrationRequest,
    BrokerRegistration(BrokerRegistrationRequest),
    /// BrokerHeartbeatRequest,
    BrokerHeartbeat(BrokerHeartbeatRequest),
    /// UnregisterBrokerRequest,
    UnregisterBroker(UnregisterBrokerRequest),
    /// DescribeTransactionsRequest,
    DescribeTransactions(DescribeTransactionsRequest),
    /// ListTransactionsRequest,
    ListTransactions(ListTransactionsRequest),
    /// AllocateProducerIdsRequest,
    AllocateProducerIds(AllocateProducerIdsRequest),
    /// ConsumerGroupHeartbeatRequest,
    ConsumerGroupHeartbeat(ConsumerGroupHeartbeatRequest),
    /// ConsumerGroupDescribeRequest,
    ConsumerGroupDescribe(ConsumerGroupDescribeRequest),
    /// ControllerRegistrationRequest,
    ControllerRegistration(ControllerRegistrationRequest),
    /// GetTelemetrySubscriptionsRequest,
    GetTelemetrySubscriptions(GetTelemetrySubscriptionsRequest),
    /// PushTelemetryRequest,
    PushTelemetry(PushTelemetryRequest),
    /// AssignReplicasToDirsRequest,
    AssignReplicasToDirs(AssignReplicasToDirsRequest),
    /// ListClientMetricsResourcesRequest,
    ListClientMetricsResources(ListClientMetricsResourcesRequest),
    /// DescribeTopicPartitionsRequest,
    DescribeTopicPartitions(DescribeTopicPartitionsRequest),
}

#[cfg(feature = "messages_enums")]
impl RequestKind {
    /// Encode the message into the target buffer
    #[cfg(feature = "client")]
    pub fn encode_request(&self, bytes: &mut bytes::BytesMut, version: i16) -> anyhow::Result<()> {
        self.encode(bytes, version).with_context(|| {
            format!(
                "Failed to encode {} v{} body",
                std::any::type_name_of_val(&self),
                version
            )
        })
    }
    /// Decode the message from the provided buffer and version
    #[cfg(feature = "broker")]
    pub fn decode(
        api_key: ApiKey,
        bytes: &mut bytes::Bytes,
        version: i16,
    ) -> anyhow::Result<RequestKind> {
        match api_key {
            ApiKey::ProduceKey => Ok(RequestKind::Produce(decode(bytes, version)?)),
            ApiKey::FetchKey => Ok(RequestKind::Fetch(decode(bytes, version)?)),
            ApiKey::ListOffsetsKey => Ok(RequestKind::ListOffsets(decode(bytes, version)?)),
            ApiKey::MetadataKey => Ok(RequestKind::Metadata(decode(bytes, version)?)),
            ApiKey::LeaderAndIsrKey => Ok(RequestKind::LeaderAndIsr(decode(bytes, version)?)),
            ApiKey::StopReplicaKey => Ok(RequestKind::StopReplica(decode(bytes, version)?)),
            ApiKey::UpdateMetadataKey => Ok(RequestKind::UpdateMetadata(decode(bytes, version)?)),
            ApiKey::ControlledShutdownKey => {
                Ok(RequestKind::ControlledShutdown(decode(bytes, version)?))
            }
            ApiKey::OffsetCommitKey => Ok(RequestKind::OffsetCommit(decode(bytes, version)?)),
            ApiKey::OffsetFetchKey => Ok(RequestKind::OffsetFetch(decode(bytes, version)?)),
            ApiKey::FindCoordinatorKey => Ok(RequestKind::FindCoordinator(decode(bytes, version)?)),
            ApiKey::JoinGroupKey => Ok(RequestKind::JoinGroup(decode(bytes, version)?)),
            ApiKey::HeartbeatKey => Ok(RequestKind::Heartbeat(decode(bytes, version)?)),
            ApiKey::LeaveGroupKey => Ok(RequestKind::LeaveGroup(decode(bytes, version)?)),
            ApiKey::SyncGroupKey => Ok(RequestKind::SyncGroup(decode(bytes, version)?)),
            ApiKey::DescribeGroupsKey => Ok(RequestKind::DescribeGroups(decode(bytes, version)?)),
            ApiKey::ListGroupsKey => Ok(RequestKind::ListGroups(decode(bytes, version)?)),
            ApiKey::SaslHandshakeKey => Ok(RequestKind::SaslHandshake(decode(bytes, version)?)),
            ApiKey::ApiVersionsKey => Ok(RequestKind::ApiVersions(decode(bytes, version)?)),
            ApiKey::CreateTopicsKey => Ok(RequestKind::CreateTopics(decode(bytes, version)?)),
            ApiKey::DeleteTopicsKey => Ok(RequestKind::DeleteTopics(decode(bytes, version)?)),
            ApiKey::DeleteRecordsKey => Ok(RequestKind::DeleteRecords(decode(bytes, version)?)),
            ApiKey::InitProducerIdKey => Ok(RequestKind::InitProducerId(decode(bytes, version)?)),
            ApiKey::OffsetForLeaderEpochKey => {
                Ok(RequestKind::OffsetForLeaderEpoch(decode(bytes, version)?))
            }
            ApiKey::AddPartitionsToTxnKey => {
                Ok(RequestKind::AddPartitionsToTxn(decode(bytes, version)?))
            }
            ApiKey::AddOffsetsToTxnKey => Ok(RequestKind::AddOffsetsToTxn(decode(bytes, version)?)),
            ApiKey::EndTxnKey => Ok(RequestKind::EndTxn(decode(bytes, version)?)),
            ApiKey::WriteTxnMarkersKey => Ok(RequestKind::WriteTxnMarkers(decode(bytes, version)?)),
            ApiKey::TxnOffsetCommitKey => Ok(RequestKind::TxnOffsetCommit(decode(bytes, version)?)),
            ApiKey::DescribeAclsKey => Ok(RequestKind::DescribeAcls(decode(bytes, version)?)),
            ApiKey::CreateAclsKey => Ok(RequestKind::CreateAcls(decode(bytes, version)?)),
            ApiKey::DeleteAclsKey => Ok(RequestKind::DeleteAcls(decode(bytes, version)?)),
            ApiKey::DescribeConfigsKey => Ok(RequestKind::DescribeConfigs(decode(bytes, version)?)),
            ApiKey::AlterConfigsKey => Ok(RequestKind::AlterConfigs(decode(bytes, version)?)),
            ApiKey::AlterReplicaLogDirsKey => {
                Ok(RequestKind::AlterReplicaLogDirs(decode(bytes, version)?))
            }
            ApiKey::DescribeLogDirsKey => Ok(RequestKind::DescribeLogDirs(decode(bytes, version)?)),
            ApiKey::SaslAuthenticateKey => {
                Ok(RequestKind::SaslAuthenticate(decode(bytes, version)?))
            }
            ApiKey::CreatePartitionsKey => {
                Ok(RequestKind::CreatePartitions(decode(bytes, version)?))
            }
            ApiKey::CreateDelegationTokenKey => {
                Ok(RequestKind::CreateDelegationToken(decode(bytes, version)?))
            }
            ApiKey::RenewDelegationTokenKey => {
                Ok(RequestKind::RenewDelegationToken(decode(bytes, version)?))
            }
            ApiKey::ExpireDelegationTokenKey => {
                Ok(RequestKind::ExpireDelegationToken(decode(bytes, version)?))
            }
            ApiKey::DescribeDelegationTokenKey => Ok(RequestKind::DescribeDelegationToken(decode(
                bytes, version,
            )?)),
            ApiKey::DeleteGroupsKey => Ok(RequestKind::DeleteGroups(decode(bytes, version)?)),
            ApiKey::ElectLeadersKey => Ok(RequestKind::ElectLeaders(decode(bytes, version)?)),
            ApiKey::IncrementalAlterConfigsKey => Ok(RequestKind::IncrementalAlterConfigs(decode(
                bytes, version,
            )?)),
            ApiKey::AlterPartitionReassignmentsKey => Ok(RequestKind::AlterPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::ListPartitionReassignmentsKey => Ok(RequestKind::ListPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::OffsetDeleteKey => Ok(RequestKind::OffsetDelete(decode(bytes, version)?)),
            ApiKey::DescribeClientQuotasKey => {
                Ok(RequestKind::DescribeClientQuotas(decode(bytes, version)?))
            }
            ApiKey::AlterClientQuotasKey => {
                Ok(RequestKind::AlterClientQuotas(decode(bytes, version)?))
            }
            ApiKey::DescribeUserScramCredentialsKey => Ok(
                RequestKind::DescribeUserScramCredentials(decode(bytes, version)?),
            ),
            ApiKey::AlterUserScramCredentialsKey => Ok(RequestKind::AlterUserScramCredentials(
                decode(bytes, version)?,
            )),
            ApiKey::VoteKey => Ok(RequestKind::Vote(decode(bytes, version)?)),
            ApiKey::BeginQuorumEpochKey => {
                Ok(RequestKind::BeginQuorumEpoch(decode(bytes, version)?))
            }
            ApiKey::EndQuorumEpochKey => Ok(RequestKind::EndQuorumEpoch(decode(bytes, version)?)),
            ApiKey::DescribeQuorumKey => Ok(RequestKind::DescribeQuorum(decode(bytes, version)?)),
            ApiKey::AlterPartitionKey => Ok(RequestKind::AlterPartition(decode(bytes, version)?)),
            ApiKey::UpdateFeaturesKey => Ok(RequestKind::UpdateFeatures(decode(bytes, version)?)),
            ApiKey::EnvelopeKey => Ok(RequestKind::Envelope(decode(bytes, version)?)),
            ApiKey::FetchSnapshotKey => Ok(RequestKind::FetchSnapshot(decode(bytes, version)?)),
            ApiKey::DescribeClusterKey => Ok(RequestKind::DescribeCluster(decode(bytes, version)?)),
            ApiKey::DescribeProducersKey => {
                Ok(RequestKind::DescribeProducers(decode(bytes, version)?))
            }
            ApiKey::BrokerRegistrationKey => {
                Ok(RequestKind::BrokerRegistration(decode(bytes, version)?))
            }
            ApiKey::BrokerHeartbeatKey => Ok(RequestKind::BrokerHeartbeat(decode(bytes, version)?)),
            ApiKey::UnregisterBrokerKey => {
                Ok(RequestKind::UnregisterBroker(decode(bytes, version)?))
            }
            ApiKey::DescribeTransactionsKey => {
                Ok(RequestKind::DescribeTransactions(decode(bytes, version)?))
            }
            ApiKey::ListTransactionsKey => {
                Ok(RequestKind::ListTransactions(decode(bytes, version)?))
            }
            ApiKey::AllocateProducerIdsKey => {
                Ok(RequestKind::AllocateProducerIds(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupHeartbeatKey => {
                Ok(RequestKind::ConsumerGroupHeartbeat(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupDescribeKey => {
                Ok(RequestKind::ConsumerGroupDescribe(decode(bytes, version)?))
            }
            ApiKey::ControllerRegistrationKey => {
                Ok(RequestKind::ControllerRegistration(decode(bytes, version)?))
            }
            ApiKey::GetTelemetrySubscriptionsKey => Ok(RequestKind::GetTelemetrySubscriptions(
                decode(bytes, version)?,
            )),
            ApiKey::PushTelemetryKey => Ok(RequestKind::PushTelemetry(decode(bytes, version)?)),
            ApiKey::AssignReplicasToDirsKey => {
                Ok(RequestKind::AssignReplicasToDirs(decode(bytes, version)?))
            }
            ApiKey::ListClientMetricsResourcesKey => Ok(RequestKind::ListClientMetricsResources(
                decode(bytes, version)?,
            )),
            ApiKey::DescribeTopicPartitionsKey => Ok(RequestKind::DescribeTopicPartitions(decode(
                bytes, version,
            )?)),
        }
    }
}

#[cfg(feature = "messages_enums")]
#[cfg(any(feature = "client", feature = "broker"))]
fn decode<T: Decodable>(bytes: &mut bytes::Bytes, version: i16) -> Result<T> {
    T::decode(bytes, version).with_context(|| {
        format!(
            "Failed to decode {} v{} body",
            std::any::type_name::<T>(),
            version
        )
    })
}

/// Wrapping enum for all responses in the Kafka protocol.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "messages_enums")]
#[enum_dispatch(Encodable)]
pub enum ResponseKind {
    /// ProduceResponse,
    Produce(ProduceResponse),
    /// FetchResponse,
    Fetch(FetchResponse),
    /// ListOffsetsResponse,
    ListOffsets(ListOffsetsResponse),
    /// MetadataResponse,
    Metadata(MetadataResponse),
    /// LeaderAndIsrResponse,
    LeaderAndIsr(LeaderAndIsrResponse),
    /// StopReplicaResponse,
    StopReplica(StopReplicaResponse),
    /// UpdateMetadataResponse,
    UpdateMetadata(UpdateMetadataResponse),
    /// ControlledShutdownResponse,
    ControlledShutdown(ControlledShutdownResponse),
    /// OffsetCommitResponse,
    OffsetCommit(OffsetCommitResponse),
    /// OffsetFetchResponse,
    OffsetFetch(OffsetFetchResponse),
    /// FindCoordinatorResponse,
    FindCoordinator(FindCoordinatorResponse),
    /// JoinGroupResponse,
    JoinGroup(JoinGroupResponse),
    /// HeartbeatResponse,
    Heartbeat(HeartbeatResponse),
    /// LeaveGroupResponse,
    LeaveGroup(LeaveGroupResponse),
    /// SyncGroupResponse,
    SyncGroup(SyncGroupResponse),
    /// DescribeGroupsResponse,
    DescribeGroups(DescribeGroupsResponse),
    /// ListGroupsResponse,
    ListGroups(ListGroupsResponse),
    /// SaslHandshakeResponse,
    SaslHandshake(SaslHandshakeResponse),
    /// ApiVersionsResponse,
    ApiVersions(ApiVersionsResponse),
    /// CreateTopicsResponse,
    CreateTopics(CreateTopicsResponse),
    /// DeleteTopicsResponse,
    DeleteTopics(DeleteTopicsResponse),
    /// DeleteRecordsResponse,
    DeleteRecords(DeleteRecordsResponse),
    /// InitProducerIdResponse,
    InitProducerId(InitProducerIdResponse),
    /// OffsetForLeaderEpochResponse,
    OffsetForLeaderEpoch(OffsetForLeaderEpochResponse),
    /// AddPartitionsToTxnResponse,
    AddPartitionsToTxn(AddPartitionsToTxnResponse),
    /// AddOffsetsToTxnResponse,
    AddOffsetsToTxn(AddOffsetsToTxnResponse),
    /// EndTxnResponse,
    EndTxn(EndTxnResponse),
    /// WriteTxnMarkersResponse,
    WriteTxnMarkers(WriteTxnMarkersResponse),
    /// TxnOffsetCommitResponse,
    TxnOffsetCommit(TxnOffsetCommitResponse),
    /// DescribeAclsResponse,
    DescribeAcls(DescribeAclsResponse),
    /// CreateAclsResponse,
    CreateAcls(CreateAclsResponse),
    /// DeleteAclsResponse,
    DeleteAcls(DeleteAclsResponse),
    /// DescribeConfigsResponse,
    DescribeConfigs(DescribeConfigsResponse),
    /// AlterConfigsResponse,
    AlterConfigs(AlterConfigsResponse),
    /// AlterReplicaLogDirsResponse,
    AlterReplicaLogDirs(AlterReplicaLogDirsResponse),
    /// DescribeLogDirsResponse,
    DescribeLogDirs(DescribeLogDirsResponse),
    /// SaslAuthenticateResponse,
    SaslAuthenticate(SaslAuthenticateResponse),
    /// CreatePartitionsResponse,
    CreatePartitions(CreatePartitionsResponse),
    /// CreateDelegationTokenResponse,
    CreateDelegationToken(CreateDelegationTokenResponse),
    /// RenewDelegationTokenResponse,
    RenewDelegationToken(RenewDelegationTokenResponse),
    /// ExpireDelegationTokenResponse,
    ExpireDelegationToken(ExpireDelegationTokenResponse),
    /// DescribeDelegationTokenResponse,
    DescribeDelegationToken(DescribeDelegationTokenResponse),
    /// DeleteGroupsResponse,
    DeleteGroups(DeleteGroupsResponse),
    /// ElectLeadersResponse,
    ElectLeaders(ElectLeadersResponse),
    /// IncrementalAlterConfigsResponse,
    IncrementalAlterConfigs(IncrementalAlterConfigsResponse),
    /// AlterPartitionReassignmentsResponse,
    AlterPartitionReassignments(AlterPartitionReassignmentsResponse),
    /// ListPartitionReassignmentsResponse,
    ListPartitionReassignments(ListPartitionReassignmentsResponse),
    /// OffsetDeleteResponse,
    OffsetDelete(OffsetDeleteResponse),
    /// DescribeClientQuotasResponse,
    DescribeClientQuotas(DescribeClientQuotasResponse),
    /// AlterClientQuotasResponse,
    AlterClientQuotas(AlterClientQuotasResponse),
    /// DescribeUserScramCredentialsResponse,
    DescribeUserScramCredentials(DescribeUserScramCredentialsResponse),
    /// AlterUserScramCredentialsResponse,
    AlterUserScramCredentials(AlterUserScramCredentialsResponse),
    /// VoteResponse,
    Vote(VoteResponse),
    /// BeginQuorumEpochResponse,
    BeginQuorumEpoch(BeginQuorumEpochResponse),
    /// EndQuorumEpochResponse,
    EndQuorumEpoch(EndQuorumEpochResponse),
    /// DescribeQuorumResponse,
    DescribeQuorum(DescribeQuorumResponse),
    /// AlterPartitionResponse,
    AlterPartition(AlterPartitionResponse),
    /// UpdateFeaturesResponse,
    UpdateFeatures(UpdateFeaturesResponse),
    /// EnvelopeResponse,
    Envelope(EnvelopeResponse),
    /// FetchSnapshotResponse,
    FetchSnapshot(FetchSnapshotResponse),
    /// DescribeClusterResponse,
    DescribeCluster(DescribeClusterResponse),
    /// DescribeProducersResponse,
    DescribeProducers(DescribeProducersResponse),
    /// BrokerRegistrationResponse,
    BrokerRegistration(BrokerRegistrationResponse),
    /// BrokerHeartbeatResponse,
    BrokerHeartbeat(BrokerHeartbeatResponse),
    /// UnregisterBrokerResponse,
    UnregisterBroker(UnregisterBrokerResponse),
    /// DescribeTransactionsResponse,
    DescribeTransactions(DescribeTransactionsResponse),
    /// ListTransactionsResponse,
    ListTransactions(ListTransactionsResponse),
    /// AllocateProducerIdsResponse,
    AllocateProducerIds(AllocateProducerIdsResponse),
    /// ConsumerGroupHeartbeatResponse,
    ConsumerGroupHeartbeat(ConsumerGroupHeartbeatResponse),
    /// ConsumerGroupDescribeResponse,
    ConsumerGroupDescribe(ConsumerGroupDescribeResponse),
    /// ControllerRegistrationResponse,
    ControllerRegistration(ControllerRegistrationResponse),
    /// GetTelemetrySubscriptionsResponse,
    GetTelemetrySubscriptions(GetTelemetrySubscriptionsResponse),
    /// PushTelemetryResponse,
    PushTelemetry(PushTelemetryResponse),
    /// AssignReplicasToDirsResponse,
    AssignReplicasToDirs(AssignReplicasToDirsResponse),
    /// ListClientMetricsResourcesResponse,
    ListClientMetricsResources(ListClientMetricsResourcesResponse),
    /// DescribeTopicPartitionsResponse,
    DescribeTopicPartitions(DescribeTopicPartitionsResponse),
}

#[cfg(feature = "messages_enums")]
impl ResponseKind {
    /// Encode the message into the target buffer
    #[cfg(feature = "broker")]
    pub fn encode_response(&self, bytes: &mut bytes::BytesMut, version: i16) -> anyhow::Result<()> {
        self.encode(bytes, version).with_context(|| {
            format!(
                "Failed to encode {} v{} body",
                std::any::type_name_of_val(&self),
                version
            )
        })
    }
    /// Decode the message from the provided buffer and version
    #[cfg(feature = "client")]
    pub fn decode(
        api_key: ApiKey,
        bytes: &mut bytes::Bytes,
        version: i16,
    ) -> anyhow::Result<ResponseKind> {
        match api_key {
            ApiKey::ProduceKey => Ok(ResponseKind::Produce(decode(bytes, version)?)),
            ApiKey::FetchKey => Ok(ResponseKind::Fetch(decode(bytes, version)?)),
            ApiKey::ListOffsetsKey => Ok(ResponseKind::ListOffsets(decode(bytes, version)?)),
            ApiKey::MetadataKey => Ok(ResponseKind::Metadata(decode(bytes, version)?)),
            ApiKey::LeaderAndIsrKey => Ok(ResponseKind::LeaderAndIsr(decode(bytes, version)?)),
            ApiKey::StopReplicaKey => Ok(ResponseKind::StopReplica(decode(bytes, version)?)),
            ApiKey::UpdateMetadataKey => Ok(ResponseKind::UpdateMetadata(decode(bytes, version)?)),
            ApiKey::ControlledShutdownKey => {
                Ok(ResponseKind::ControlledShutdown(decode(bytes, version)?))
            }
            ApiKey::OffsetCommitKey => Ok(ResponseKind::OffsetCommit(decode(bytes, version)?)),
            ApiKey::OffsetFetchKey => Ok(ResponseKind::OffsetFetch(decode(bytes, version)?)),
            ApiKey::FindCoordinatorKey => {
                Ok(ResponseKind::FindCoordinator(decode(bytes, version)?))
            }
            ApiKey::JoinGroupKey => Ok(ResponseKind::JoinGroup(decode(bytes, version)?)),
            ApiKey::HeartbeatKey => Ok(ResponseKind::Heartbeat(decode(bytes, version)?)),
            ApiKey::LeaveGroupKey => Ok(ResponseKind::LeaveGroup(decode(bytes, version)?)),
            ApiKey::SyncGroupKey => Ok(ResponseKind::SyncGroup(decode(bytes, version)?)),
            ApiKey::DescribeGroupsKey => Ok(ResponseKind::DescribeGroups(decode(bytes, version)?)),
            ApiKey::ListGroupsKey => Ok(ResponseKind::ListGroups(decode(bytes, version)?)),
            ApiKey::SaslHandshakeKey => Ok(ResponseKind::SaslHandshake(decode(bytes, version)?)),
            ApiKey::ApiVersionsKey => Ok(ResponseKind::ApiVersions(decode(bytes, version)?)),
            ApiKey::CreateTopicsKey => Ok(ResponseKind::CreateTopics(decode(bytes, version)?)),
            ApiKey::DeleteTopicsKey => Ok(ResponseKind::DeleteTopics(decode(bytes, version)?)),
            ApiKey::DeleteRecordsKey => Ok(ResponseKind::DeleteRecords(decode(bytes, version)?)),
            ApiKey::InitProducerIdKey => Ok(ResponseKind::InitProducerId(decode(bytes, version)?)),
            ApiKey::OffsetForLeaderEpochKey => {
                Ok(ResponseKind::OffsetForLeaderEpoch(decode(bytes, version)?))
            }
            ApiKey::AddPartitionsToTxnKey => {
                Ok(ResponseKind::AddPartitionsToTxn(decode(bytes, version)?))
            }
            ApiKey::AddOffsetsToTxnKey => {
                Ok(ResponseKind::AddOffsetsToTxn(decode(bytes, version)?))
            }
            ApiKey::EndTxnKey => Ok(ResponseKind::EndTxn(decode(bytes, version)?)),
            ApiKey::WriteTxnMarkersKey => {
                Ok(ResponseKind::WriteTxnMarkers(decode(bytes, version)?))
            }
            ApiKey::TxnOffsetCommitKey => {
                Ok(ResponseKind::TxnOffsetCommit(decode(bytes, version)?))
            }
            ApiKey::DescribeAclsKey => Ok(ResponseKind::DescribeAcls(decode(bytes, version)?)),
            ApiKey::CreateAclsKey => Ok(ResponseKind::CreateAcls(decode(bytes, version)?)),
            ApiKey::DeleteAclsKey => Ok(ResponseKind::DeleteAcls(decode(bytes, version)?)),
            ApiKey::DescribeConfigsKey => {
                Ok(ResponseKind::DescribeConfigs(decode(bytes, version)?))
            }
            ApiKey::AlterConfigsKey => Ok(ResponseKind::AlterConfigs(decode(bytes, version)?)),
            ApiKey::AlterReplicaLogDirsKey => {
                Ok(ResponseKind::AlterReplicaLogDirs(decode(bytes, version)?))
            }
            ApiKey::DescribeLogDirsKey => {
                Ok(ResponseKind::DescribeLogDirs(decode(bytes, version)?))
            }
            ApiKey::SaslAuthenticateKey => {
                Ok(ResponseKind::SaslAuthenticate(decode(bytes, version)?))
            }
            ApiKey::CreatePartitionsKey => {
                Ok(ResponseKind::CreatePartitions(decode(bytes, version)?))
            }
            ApiKey::CreateDelegationTokenKey => {
                Ok(ResponseKind::CreateDelegationToken(decode(bytes, version)?))
            }
            ApiKey::RenewDelegationTokenKey => {
                Ok(ResponseKind::RenewDelegationToken(decode(bytes, version)?))
            }
            ApiKey::ExpireDelegationTokenKey => {
                Ok(ResponseKind::ExpireDelegationToken(decode(bytes, version)?))
            }
            ApiKey::DescribeDelegationTokenKey => Ok(ResponseKind::DescribeDelegationToken(
                decode(bytes, version)?,
            )),
            ApiKey::DeleteGroupsKey => Ok(ResponseKind::DeleteGroups(decode(bytes, version)?)),
            ApiKey::ElectLeadersKey => Ok(ResponseKind::ElectLeaders(decode(bytes, version)?)),
            ApiKey::IncrementalAlterConfigsKey => Ok(ResponseKind::IncrementalAlterConfigs(
                decode(bytes, version)?,
            )),
            ApiKey::AlterPartitionReassignmentsKey => Ok(
                ResponseKind::AlterPartitionReassignments(decode(bytes, version)?),
            ),
            ApiKey::ListPartitionReassignmentsKey => Ok(ResponseKind::ListPartitionReassignments(
                decode(bytes, version)?,
            )),
            ApiKey::OffsetDeleteKey => Ok(ResponseKind::OffsetDelete(decode(bytes, version)?)),
            ApiKey::DescribeClientQuotasKey => {
                Ok(ResponseKind::DescribeClientQuotas(decode(bytes, version)?))
            }
            ApiKey::AlterClientQuotasKey => {
                Ok(ResponseKind::AlterClientQuotas(decode(bytes, version)?))
            }
            ApiKey::DescribeUserScramCredentialsKey => Ok(
                ResponseKind::DescribeUserScramCredentials(decode(bytes, version)?),
            ),
            ApiKey::AlterUserScramCredentialsKey => Ok(ResponseKind::AlterUserScramCredentials(
                decode(bytes, version)?,
            )),
            ApiKey::VoteKey => Ok(ResponseKind::Vote(decode(bytes, version)?)),
            ApiKey::BeginQuorumEpochKey => {
                Ok(ResponseKind::BeginQuorumEpoch(decode(bytes, version)?))
            }
            ApiKey::EndQuorumEpochKey => Ok(ResponseKind::EndQuorumEpoch(decode(bytes, version)?)),
            ApiKey::DescribeQuorumKey => Ok(ResponseKind::DescribeQuorum(decode(bytes, version)?)),
            ApiKey::AlterPartitionKey => Ok(ResponseKind::AlterPartition(decode(bytes, version)?)),
            ApiKey::UpdateFeaturesKey => Ok(ResponseKind::UpdateFeatures(decode(bytes, version)?)),
            ApiKey::EnvelopeKey => Ok(ResponseKind::Envelope(decode(bytes, version)?)),
            ApiKey::FetchSnapshotKey => Ok(ResponseKind::FetchSnapshot(decode(bytes, version)?)),
            ApiKey::DescribeClusterKey => {
                Ok(ResponseKind::DescribeCluster(decode(bytes, version)?))
            }
            ApiKey::DescribeProducersKey => {
                Ok(ResponseKind::DescribeProducers(decode(bytes, version)?))
            }
            ApiKey::BrokerRegistrationKey => {
                Ok(ResponseKind::BrokerRegistration(decode(bytes, version)?))
            }
            ApiKey::BrokerHeartbeatKey => {
                Ok(ResponseKind::BrokerHeartbeat(decode(bytes, version)?))
            }
            ApiKey::UnregisterBrokerKey => {
                Ok(ResponseKind::UnregisterBroker(decode(bytes, version)?))
            }
            ApiKey::DescribeTransactionsKey => {
                Ok(ResponseKind::DescribeTransactions(decode(bytes, version)?))
            }
            ApiKey::ListTransactionsKey => {
                Ok(ResponseKind::ListTransactions(decode(bytes, version)?))
            }
            ApiKey::AllocateProducerIdsKey => {
                Ok(ResponseKind::AllocateProducerIds(decode(bytes, version)?))
            }
            ApiKey::ConsumerGroupHeartbeatKey => Ok(ResponseKind::ConsumerGroupHeartbeat(decode(
                bytes, version,
            )?)),
            ApiKey::ConsumerGroupDescribeKey => {
                Ok(ResponseKind::ConsumerGroupDescribe(decode(bytes, version)?))
            }
            ApiKey::ControllerRegistrationKey => Ok(ResponseKind::ControllerRegistration(decode(
                bytes, version,
            )?)),
            ApiKey::GetTelemetrySubscriptionsKey => Ok(ResponseKind::GetTelemetrySubscriptions(
                decode(bytes, version)?,
            )),
            ApiKey::PushTelemetryKey => Ok(ResponseKind::PushTelemetry(decode(bytes, version)?)),
            ApiKey::AssignReplicasToDirsKey => {
                Ok(ResponseKind::AssignReplicasToDirs(decode(bytes, version)?))
            }
            ApiKey::ListClientMetricsResourcesKey => Ok(ResponseKind::ListClientMetricsResources(
                decode(bytes, version)?,
            )),
            ApiKey::DescribeTopicPartitionsKey => Ok(ResponseKind::DescribeTopicPartitions(
                decode(bytes, version)?,
            )),
        }
    }
    /// Get the version of request header that needs to be prepended to this message
    pub fn response_header_version(&self, version: i16) -> i16 {
        <self as HeaderVersion>:: header_version(version)
    }
}

/// The ID of the leader broker.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct BrokerId(pub i32);

impl From<i32> for BrokerId {
    fn from(other: i32) -> Self {
        Self(other)
    }
}
impl From<BrokerId> for i32 {
    fn from(other: BrokerId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<i32> for BrokerId {
    fn borrow(&self) -> &i32 {
        &self.0
    }
}
impl std::ops::Deref for BrokerId {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<i32> for BrokerId {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<BrokerId> for i32 {
    fn eq(&self, other: &BrokerId) -> bool {
        self == &other.0
    }
}
impl std::fmt::Debug for BrokerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl NewType<i32> for BrokerId {}

/// The group ID string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct GroupId(pub StrBytes);

impl From<StrBytes> for GroupId {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<GroupId> for StrBytes {
    fn from(other: GroupId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for GroupId {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for GroupId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for GroupId {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<GroupId> for StrBytes {
    fn eq(&self, other: &GroupId) -> bool {
        self == &other.0
    }
}
impl std::fmt::Debug for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl NewType<StrBytes> for GroupId {}

/// The first producer ID in this range, inclusive
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Copy)]
pub struct ProducerId(pub i64);

impl From<i64> for ProducerId {
    fn from(other: i64) -> Self {
        Self(other)
    }
}
impl From<ProducerId> for i64 {
    fn from(other: ProducerId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<i64> for ProducerId {
    fn borrow(&self) -> &i64 {
        &self.0
    }
}
impl std::ops::Deref for ProducerId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<i64> for ProducerId {
    fn eq(&self, other: &i64) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<ProducerId> for i64 {
    fn eq(&self, other: &ProducerId) -> bool {
        self == &other.0
    }
}
impl std::fmt::Debug for ProducerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl NewType<i64> for ProducerId {}

/// The topic name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TopicName(pub StrBytes);

impl From<StrBytes> for TopicName {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<TopicName> for StrBytes {
    fn from(other: TopicName) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for TopicName {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for TopicName {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for TopicName {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<TopicName> for StrBytes {
    fn eq(&self, other: &TopicName) -> bool {
        self == &other.0
    }
}
impl std::fmt::Debug for TopicName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl NewType<StrBytes> for TopicName {}

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TransactionalId(pub StrBytes);

impl From<StrBytes> for TransactionalId {
    fn from(other: StrBytes) -> Self {
        Self(other)
    }
}
impl From<TransactionalId> for StrBytes {
    fn from(other: TransactionalId) -> Self {
        other.0
    }
}
impl std::borrow::Borrow<StrBytes> for TransactionalId {
    fn borrow(&self) -> &StrBytes {
        &self.0
    }
}
impl std::ops::Deref for TransactionalId {
    type Target = StrBytes;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq<StrBytes> for TransactionalId {
    fn eq(&self, other: &StrBytes) -> bool {
        &self.0 == other
    }
}
impl std::cmp::PartialEq<TransactionalId> for StrBytes {
    fn eq(&self, other: &TransactionalId) -> bool {
        self == &other.0
    }
}
impl std::fmt::Debug for TransactionalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl NewType<StrBytes> for TransactionalId {}
