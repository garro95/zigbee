use apl::AddrAndEp;

pub struct DiscoveryStoreReq {
    pub nwk_addr: u16,
    pub ieee_addr: u64,
    pub node_desc_size: u8,
    pub power_desc_size: u8,
    pub active_ep_size: u8,
    pub simple_desc_count: u8,
    pub simple_desc_size_list: &[u8]
}

pub struct MatchDescReq{
    pub nwk_addr_of_interest: u16,
    pub profile_id: u16,
    pub in_cluster_list: &[u16],
    pub out_cluster_list: &[u16]
}

pub struct EndDeviceBindReq{
    pub binding_target: u16,
    pub src_ieee_address: u64,
    pub src_endpoint: u8,
    pub profile_id: u16,
    pub in_cluster_list: &[u16],
    pub out_cluster_list: &[u16],
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

pub struct NwkAddrRsp{
    status: Status,
    ieee_addr_remote_dev: u64,
    nwk_addr_remote_dev: u16,
    nwk_addr_assoc_dev_list: Vec<u16>,
}

pub struct IeeeAddrRsp{
    status: Status,
    ieee_addr_remote_dev: u64,
    nwk_addr_remote_dev: u16,
    nwk_addr_assoc_dev_list: Vec<u64>
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

pub struct SimpleDescRsp{
    status: Status,
    nwk_addr_of_interest: u16,
    desc: SimpleDescriptor
}

pub struct ActiveEpRsp{
    status: Status,
    nwk_addr_of_interest: u16,
    active_ep_list: Vec<u8>
}

pub struct MatchDescRsp {
    status: Status,
    nwk_addr_of_interest: u16,
    match_list: Vec<u8>
}

pub struct ComplexDescRsp {
    status: Status,
    nwk_addr_of_interest: u16,
    complex_descriptor: ComplexDescriptor
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

pub struct ExtendedSimpleDescRsp {
    status: Status,
    nwk_addr_of_interest: u16,
    endpoint: u8,
    app_input_cluster_count: u8,
    app_output_cluster_count: u8,
    start_index: u8,
    app_cluster_list: Vec<u16>
}

pub struct ExtendedActiveEpRsp {
    status: Status,
    nwk_addr_of_interest: u16,
    active_ep_count: u8,
    start_index: u8,
    active_ep_list: Vec<u8>
}

pub struct BindRegisterRsp {
    status: Status,
    binding_table_entries: u16,
    binding_table_list_count: u16,
    binding_table_list: Vec<BindReq>
}

pub struct BackupBindTableRsp {
    status: Status,
    entry_count: u16
}

pub struct RecoverBindTableRsp {
    status: Status,
    binding_table_entries: u16,
    start_index: u16,
    binding_table_list_count: u16,
    binding_table_list: Vec<BindReq>
}

pub struct MgmtLqiRsp {
    status: Status,
    neighbor_table_entries: u8,
    start_index: u8,
    neighbor_table_list_count: u8,
    neighbor_table_list: Vec<NeighborTableListRecord>
}

pub struct NeighborTableListRecord {
    extended_pan_id: u64,
    extended_address: u64,
    network_address: u64,
    device_type: DeviceType,
    rx_on_when_idle: bool,
    relationship: Relationship,
    permit_joining: bool,
    depth: u8,
    lqi: u8
}

pub struct MgmtRtgRsp {
    status: Status,
    routing_table_entries: u8,
    start_index: u8,
    routing_table_list_count: u8,
    routing_table_list: Vec<RoutingTableListRecord>
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

pub struct MgmtBindRsp {
    status: Status,
    binding_table_entries: u8,
    start_index: u8,
    binding_table_list_count: u8,
    binding_table_list: Vec<BindingTableListRecord>
}

pub struct BindingTableListRecord {
    src_addr: u64,
    src_endpoint: u8,
    cluster_id: u16,
    dst_addr: AddrAndEp
}

pub struct MgmtCacheRsp {
    status: Status,
    discovery_cache_entries: u8,
    start_index: u8,
    dicovery_cache_list_count: u8,
    discovery_cache_list: Vec<DiscoveryCacheListRecord>
}

pub struct DiscoveryCacheListRecord {
    extended_address: u64,
    network_address: u16
}

pub struct MgmtNwkUpdateNotify {
    status: Status,
    scanned_channels: u32,
    total_transmissions: u16,
    transmission_failures: u16,
    scanned_channels_list_count: u8,
    energy_values: Vec<u8>
}

pub struct RecoverSourceBindRsp {
    status: Status,
    source_table_entries: u16,
    start_index: u16,
    source_table_list_count: u16,
    source_table_list: Vec<u64>
}

pub struct MgmtNwkDiscRsp {
    status: Status,
    network_count: u8,
    start_index: u8,
    network_list_count: u8,
    network_list: Vec<NetworkListRecord>
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

use apl::framework::{NodeDescriptor, PowerDescriptor, SimpleDescriptor,
                     ComplexDescriptor, UserDescriptor, ServerMask};

pub trait DeviceProfileClient {
    //2.4.3.1 Device and Service Discovery Client Services
    //directed to remote devices => should return futures.
    fn nwk_addr_req(ieee_address: u64, request_type: u8, start_index: u8) -> NwkAddrRsp;
    fn ieee_addr_req(nwk_addr_of_interest: u16, request_type: u8, start_index:u8) -> IeeeAddrRsp;
    fn node_desc_req(nwk_addr_of_interest: u16) -> NodeDescRsp;
    fn power_desc_req(nwk_addr_of_interest: u16)-> PowerDescRsp;
    fn simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8) -> SimpleDescRsp;
    fn active_ep_req(nwk_addr_of_interest: u16) -> ActiveEpRsp;
    fn match_desc_req(request: MatchDescReq) -> MatchDescRsp;
    fn complex_desc_req(nwk_addr_of_interest: u16) -> ComplexDescRsp;
    fn user_desc_req(nwk_addr_of_interest: u16) -> UserDescRsp;
    fn discovery_cache_req(nwk_addr: u16, ieee_addr: u64) -> Status;
    fn devce_annce(nwk_addr: u16, ieee_addr:u64, capability:u8);
    fn user_desc_set(nwk_addr_of_interest:u16, &[u8]) -> UserDescConf;
    fn system_server_discovery_req(server_mask:u16) -> SystemServerDiscoveryRsp;
    fn discovery_store_req(request: DiscoveryStoreReq) -> Status;
    fn node_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: NodeDescriptor) -> Status;
    fn power_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: PowerDescriptor);
    fn active_ep_store_req(nwk_addr: u16, ieee_addr: u64, active_ep_list: &[u8]);
    fn simple_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: SimpleDescriptor);
    fn remove_node_cache_req(nwk_addr: u16, ieee_addr: u64);
    fn find_node_cache_req(nwk_addr: u16, ieee_addr: u64) -> FindNodeCacheRsp;
    fn extended_simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8, start_index: u8) -> ExtendedSimpleDescRsp;
    fn extended_active_ep_req(nwk_addr_of_interest: u16, start_index: u8) -> ExtendedActiveEpRsp;
    // 2.4.3.2 End Device Bind, Bind, Unbind, and Bind Management Client Services Primitives
    fn end_device_bind_req(request: EndDeviceBindReq) -> Status;
    fn bind_req(request: BindReq) -> Status;
    fn unbind_req(request: BindReq) -> Status;
    fn bind_register_req(node_address: u64) -> BindRegisterRsp;
    fn replace_device_req(old_address: u64, old_endpoint: u8, new_address: u64, new_endpoint: u8) -> Status;
    fn store_bkup_bind_entry_req(request: BindReq) -> Status;
    fn remove_bkup_bind_entry_req(request: BindReq) -> Status;
    fn backup_bind_table_req(binding_table_entries: u16, binding_table_list: Vec<BindReq>) -> BackupBindTableRsp;
    fn recover_bind_table_req(start_index: u16) -> RecoverBindTableRsp;
    fn backup_source_bind_req(start_table_entries: u16, start_indes: u16, source_table_list: Vec<u64>) -> Status;
    fn recover_source_bind_req(start_index: u16) -> RecoverSourceBindRsp;
    // 2.4.3.3 Network Management Client Services
    fn mgmt_nwk_disc_req(scan_channels: u32, scan_duration: u8, start_index: u8) -> MgmtNwkDiscRsp;
    fn mgmt_lqi_req(start_index: u8) -> MgmtLqiRsp;
    fn mgmt_rtg_req(start_index: u8) -> MgmtRtgRsp;
    fn mgmt_bind_req(start_index: u8) -> MgmtBindRsp;
    fn mgmt_leave_req(device_address: u64, remove_children: bool, rejoin: bool) -> Status;
    fn mgmt_direct_join_req(device_address: u64, capability_information: u8) -> Status;
    fn mgmt_permit_joining_req(permit_duration: u8, tc_significance: bool) -> Status;
    fn mgmt_cache_req(start_index: u8) -> MgmtCacheRsp;
    fn mgmt_nwk_update_req(request: MgmtNwkUpdateReq) -> MgmtNwkUpdateNotify;   
}
