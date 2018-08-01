use futures::future::Future;
use apl::AddrAndEp;
use std::borrow::Cow;

pub struct DiscoveryStoreReq<'a> {
    pub nwk_addr: u16,
    pub ieee_addr: u64,
    pub node_desc_size: u8,
    pub power_desc_size: u8,
    pub active_ep_size: u8,
    pub simple_desc_count: u8,
    pub simple_desc_size_list: Cow<'a, [u8]>
}

pub struct MatchDescReq<'a> {
    pub nwk_addr_of_interest: u16,
    pub profile_id: u16,
    pub in_cluster_list: Cow<'a, [u16]>,
    pub out_cluster_list: Cow<'a, [u16]>
}

pub struct EndDeviceBindReq<'a>{
    pub binding_target: u16,
    pub src_ieee_address: u64,
    pub src_endpoint: u8,
    pub profile_id: u16,
    pub in_cluster_list: Cow<'a, [u16]>,
    pub out_cluster_list: Cow<'a, [u16]>,
}

pub struct BindReq{
    pub src_add: u64,
    pub src_endp: u8,
    pub cluster_id: u16,
    pub dst_addr: AddrAndEp
}

pub struct MgmtNwkUpdateReq{
    scan_channels: u32,
    scan_duration: u8,
    scan_count: Option<u8>,
    nwk_update_id: Option<u8>,
    nwk_manager_addr: Option<u16>
}

pub enum Status{
    Success,
    InvRequestType,
    DeviceNotFound,
    NoDescriptor,
    InvalidEp,
    NotActive,
    NotSupported,
    InsufficientSpace,
    NotPermitted,
    Timeout,
    NoMatch,
    DeviceBindingTableFull,
    TableFull,
    NoEntry,
    NotAuthorized,
    
}

pub struct NwkAddrRsp<'a> {
    status: Status,
    ieee_addr_remote_dev: u64,
    nwk_addr_remote_dev: u16,
    nwk_addr_assoc_dev_list: Cow<'a, [u16]>,
}

pub struct IeeeAddrRsp<'a> {
    status: Status,
    ieee_addr_remote_dev: u64,
    nwk_addr_remote_dev: u16,
    nwk_addr_assoc_dev_list: Cow<'a, [u64]>
}

pub struct NodeDescRsp{
    status: Status,
    nwk_addr_of_interest: u16,
    desc: NodeDescriptor
}

pub struct PowerDescRsp{
    status: Status,
    nwk_addr_of_interest: u16,
    desc: PowerDescriptor
}

pub struct SimpleDescRsp<'a, 'b>{
    status: Status,
    nwk_addr_of_interest: u16,
    desc: SimpleDescriptor<'a, 'b>
}

pub struct ActiveEpRsp<'a> {
    status: Status,
    nwk_addr_of_interest: u16,
    active_ep_list: Cow<'a, [u16]>
}

pub struct MatchDescRsp<'a> {
    status: Status,
    nwk_addr_of_interest: u16,
    match_list: Cow<'a, [u8]>
}

pub struct ComplexDescRsp<'a> {
    status: Status,
    nwk_addr_of_interest: u16,
    complex_descriptor: ComplexDescriptor<'a>
}

pub struct UserDescRsp {
    status: Status,
    nwk_addr_of_interest: u16,
    user_descriptor: UserDescriptor
}

pub struct SystemServerDiscoveryRsp {
    status: Status,
    server_mask: ServerMask
}

pub struct UserDescConf {
    status: Status,
    nwk_addr_of_interest: u16,
}

pub struct FindNodeCacheRsp {
    cache_nwk_addr: u16,
    nwk_addr: u16,
    ieee_addr: u64
}

pub struct ExtendedSimpleDescRsp<'a> {
    status: Status,
    nwk_addr_of_interest: u16,
    endpoint: u8,
    app_input_cluster_count: u8,
    app_output_cluster_count: u8,
    start_index: u8,
    app_cluster_list: Cow<'a, [u16]>
}

pub struct ExtendedActiveEpRsp<'a> {
    status: Status,
    nwk_addr_of_interest: u16,
    active_ep_count: u8,
    start_index: u8,
    active_ep_list: Cow<'a, [u8]>
}

pub struct BindRegisterRsp<'a> {
    status: Status,
    binding_table_entries: u16,
    binding_table_list_count: u16,
    binding_table_list: Cow<'a, [BindReq]>
}

pub struct BackupBindTableRsp {
    status: Status,
    entry_count: u16
}

pub struct RecoverBindTableRsp<'a> {
    status: Status,
    binding_table_entries: u16,
    start_index: u16,
    binding_table_list_count: u16,
    binding_table_list: Cow<'a, [BindReq]>
}

pub struct MgmtLqiRsp<'a> {
    status: Status,
    neighbor_table_entries: u8,
    start_index: u8,
    neighbor_table_list_count: u8,
    neighbor_table_list: Cow<'a, [NeighborTableListRecord]>
}

use apl::framework::LogicalType;
use Unknownable;

pub enum Relationship {
    Parent,
    Child,
    Sibling,
    None,
    PreviousChild
}

pub struct NeighborTableListRecord {
    extended_pan_id: u64,
    extended_address: u64,
    network_address: u64,
    device_type: Unknownable<LogicalType>,
    rx_on_when_idle: Unknownable<bool>,
    relationship: Relationship,
    permit_joining: Unknownable<bool>,
    depth: u8,
    lqi: u8
}

pub struct MgmtRtgRsp<'a> {
    status: Status,
    routing_table_entries: u8,
    start_index: u8,
    routing_table_list_count: u8,
    routing_table_list: Cow<'a, [RoutingTableListRecord]>
}

pub enum RouteStatus {
    Active,
    DiscoveryUnderway,
    DiscoveryFailed,
    Inactive,
    ValidationUnderway
}

pub struct RoutingTableListRecord {
    destination_address: u16,
    status: RouteStatus,
    memory_constrained: bool,
    many_to_one: bool,
    route_record_required: bool,
    next_hop_address: u16
}

pub struct MgmtBindRsp<'a> {
    status: Status,
    binding_table_entries: u8,
    start_index: u8,
    binding_table_list_count: u8,
    binding_table_list: Cow<'a, [BindingTableListRecord]>
}

pub struct BindingTableListRecord {
    src_addr: u64,
    src_endpoint: u8,
    cluster_id: u16,
    dst_addr: AddrAndEp
}

pub struct MgmtCacheRsp<'a> {
    status: Status,
    discovery_cache_entries: u8,
    start_index: u8,
    dicovery_cache_list_count: u8,
    discovery_cache_list: Cow<'a, [DiscoveryCacheListRecord]>
}

pub struct DiscoveryCacheListRecord {
    extended_address: u64,
    network_address: u16
}

pub struct MgmtNwkUpdateNotify<'a> {
    status: Status,
    scanned_channels: u32,
    total_transmissions: u16,
    transmission_failures: u16,
    scanned_channels_list_count: u8,
    energy_values: Cow<'a, [u8]>
}

pub struct RecoverSourceBindRsp<'a> {
    status: Status,
    source_table_entries: u16,
    start_index: u16,
    source_table_list_count: u16,
    source_table_list: Cow<'a, [u64]>
}

pub struct MgmtNwkDiscRsp<'a> {
    status: Status,
    network_count: u8,
    start_index: u8,
    network_list_count: u8,
    network_list: Cow<'a, NetworkListRecord>
}

pub struct NetworkListRecord {
    extended_pan_id: u64,
    logical_channel: u8,
    stack_profile: u8,
    zigbee_version: u8,
    beacon_order: u8,
    superframe_order: u8,
    permit_joining: bool
}

pub struct PowerDescStoreRsp {
    status: Status,
    ieee_addr: u64,
    power_descriptor: PowerDescriptor
}

use apl::framework::{NodeDescriptor, PowerDescriptor, SimpleDescriptor,
                     ComplexDescriptor, UserDescriptor, ServerMask};

pub trait DeviceProfileClient {
    //2.4.3.1 Device and Service Discovery Client Services
    //directed to remote devices => should return futures.
    fn nwk_addr_req(ieee_address: u64, request_type: u8, start_index: u8) -> Box<Future<Output=NwkAddrRsp<'a>>>;
    fn ieee_addr_req(nwk_addr_of_interest: u16, request_type: u8, start_index:u8) -> Box<Future<Output=IeeeAddrRsp<'a>>>;
    fn node_desc_req(nwk_addr_of_interest: u16) -> Box<Future<Output=NodeDescRsp>>;
    fn power_desc_req(nwk_addr_of_interest: u16)-> Box<Future<Output=PowerDescRsp>>;
    fn simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8) -> Box<Future<Output=SimpleDescRsp<'a, 'b>>>;
    fn active_ep_req(nwk_addr_of_interest: u16) -> Box<Future<Output=ActiveEpRsp<'a>>>;
    fn match_desc_req<'a, 'b>(request: MatchDescReq<'a>) -> Box<Future<Output=MatchDescRsp<'b>>>;
    fn complex_desc_req(nwk_addr_of_interest: u16) -> Box<Future<Output=ComplexDescRsp<'a>>>;
    fn user_desc_req(nwk_addr_of_interest: u16) -> Box<Future<Output=UserDescRsp>>;
    fn discovery_cache_req(nwk_addr: u16, ieee_addr: u64) -> Box<Future<Output=Status>>;
    fn devce_annce(nwk_addr: u16, ieee_addr:u64, capability:u8);
    fn user_desc_set(nwk_addr_of_interest:u16, descriptor: Cow<[u8]>) -> Box<Future<Output=UserDescConf>>;
    fn system_server_discovery_req(server_mask:u16) -> Box<Future<Output=SystemServerDiscoveryRsp>>;
    fn discovery_store_req(request: DiscoveryStoreReq<'a>) -> Box<Future<Output=Status>>;
    fn node_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: NodeDescriptor<'a>) -> Box<Future<Output=Status>>;
    fn power_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: PowerDescriptor<'a>) -> Box<Future<Output=PowerDescStoreRsp>>;
    fn active_ep_store_req(nwk_addr: u16, ieee_addr: u64, active_ep_list: Cow<'a, [u8]>) -> Box<Future<Output=Status>>;
    fn simple_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: SimpleDescriptor) -> Box<Future<Output=Status>>;
    fn remove_node_cache_req(nwk_addr: u16, ieee_addr: u64) -> Box<Future<Output=Status>>;
    fn find_node_cache_req(nwk_addr: u16, ieee_addr: u64) -> Box<Future<Output=FindNodeCacheRsp>>;
    fn extended_simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8, start_index: u8) -> Box<Future<Output=ExtendedSimpleDescRsp<'a>>>;
    fn extended_active_ep_req(nwk_addr_of_interest: u16, start_index: u8) -> Box<Future<Output=ExtendedActiveEpRsp<'a>>>;
    // 2.4.3.2 End Device Bind, Bind, Unbind, and Bind Management Client Services Primitives
    fn end_device_bind_req(request: EndDeviceBindReq<'a>) -> Box<Future<Output=Status>>;
    fn bind_req(request: BindReq) -> Box<Future<Output=Status>>;
    fn unbind_req(request: BindReq) -> Box<Future<Output=Status>>;
    fn bind_register_req(node_address: u64) -> Box<Future<Output=BindRegisterRsp<'a>>>;
    fn replace_device_req(old_address: u64, old_endpoint: u8, new_address: u64, new_endpoint: u8) -> Box<Future<Output=Status>>;
    fn store_bkup_bind_entry_req(request: BindReq) -> Box<Future<Output=Status>>;
    fn remove_bkup_bind_entry_req(request: BindReq) -> Box<Future<Output=Status>>;
    fn backup_bind_table_req(binding_table_entries: u16, binding_table_list: Cow<'a, [BindReq]>) -> Box<Future<Output=BackupBindTableRsp>>;
    fn recover_bind_table_req(start_index: u16) -> Box<Future<Output=RecoverBindTableRsp<'a>>>;
    fn backup_source_bind_req(start_table_entries: u16, start_indes: u16, source_table_list: Cow<'a, [u64]>) -> Box<Future<Output=Status>>;
    fn recover_source_bind_req(start_index: u16) -> Box<Future<Output=RecoverSourceBindRsp<'a>>>;
    // 2.4.3.3 Network Management Client Services
    fn mgmt_nwk_disc_req(scan_channels: u32, scan_duration: u8, start_index: u8) -> Box<Future<Output=MgmtNwkDiscRsp<'a>>>;
    fn mgmt_lqi_req(start_index: u8) -> Box<Future<Output=MgmtLqiRsp<'a>>>;
    fn mgmt_rtg_req(start_index: u8) -> Box<Future<Output=MgmtRtgRsp<'a>>>;
    fn mgmt_bind_req(start_index: u8) -> Box<Future<Output=MgmtBindRsp<'a>>>;
    fn mgmt_leave_req(device_address: u64, remove_children: bool, rejoin: bool) -> Box<Future<Output=Status>>;
    fn mgmt_direct_join_req(device_address: u64, capability_information: u8) -> Box<Future<Output=Status>>;
    fn mgmt_permit_joining_req(permit_duration: u8, tc_significance: bool) -> Box<Future<Output=Status>>;
    fn mgmt_cache_req(start_index: u8) -> Box<Future<Output=MgmtCacheRsp<'a>>>;
    fn mgmt_nwk_update_req(request: MgmtNwkUpdateReq) -> Box<Future<Output=MgmtNwkUpdateNotify<'a>>>;
}
