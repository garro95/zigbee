use apl::aps::*;
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

pub struct NwkAddrResp{
    status: Status,
    ieee_addr_remote_dev: u64,
    nwk_address_remote_dev: u16,
    nwk_addr_assoc_dev_list: Vec<u16>,
}

use apl::framework::{NodeDescriptor, PowerDescriptor, SimpleDescriptor};

pub trait DeviceProfileClient {
    //2.4.3.1 Device and Service Discovery Client Services
    fn nwk_addr_req(ieee_address: u64, request_type: u8, start_index: u8) -> NwkAddrResp;
    fn ieee_addr_req(nwk_addr_of_interest: u16, request_type: u8, start_index:u8) -> IeeeAddrResp;
    fn node_desc_req(nwk_addr_of_interest: u16);
    fn power_desc_req(nwk_addr_of_interest: u16);
    fn simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8);
    fn active_ep_req(nwk_addr_of_interest: u16);
    fn match_desc_req(request: MatchDescReq);
    fn complex_desc_req(nwk_addr_of_interest: u16);
    fn user_desc_req(nwk_addr_of_interest: u16);
    fn discovery_cache_req(nwk_addr: u16, ieee_addr: u64);
    fn devce_annce(nwk_addr: u16, ieee_addr:u64, capability:u8);
    fn user_desc_set(nwk_addr_of_interest:u16, &[u8]);
    fn system_server_discover_req(server_mask:u16);
    fn discovery_store_req(request: DiscoveryStoreReq);
    fn node_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: NodeDescriptor);
    fn power_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: PowerDescriptor);
    fn active_ep_store_req(nwk_addr: u16, ieee_addr: u64, active_ep_list: &[u8]);
    fn simple_desc_store_req(nwk_addr: u16, ieee_addr: u64, descriptor: SimpleDescriptor);
    fn remove_node_cache_req(nwk_addr: u16, ieee_addr: u64);
    fn find_node_cache_req(nwk_addr: u16, ieee_addr: u64);
    fn extended_simple_desc_req(nwk_addr_of_interest: u16, endpoint: u8, start_index: u8);
    fn extended_active_ep_req(nwk_addr_of_interest: u16, start_index: u8);
    // 2.4.3.2 End Device Bind, Bind, Unbind, and Bind Management Client Services Primitives
    fn end_device_bind_req(request: EndDeviceBindReq);
    fn bind_req(request: BindReq);
    fn unbind_req(request: BindReq);
    fn bind_register_req(node_address: u64);
    fn replace_device_req(old_address: u64, old_endpoint: u8, new_address: u64, new_endpoint: u8);
    fn store_bkup_bind_entry_req(request: BindReq);
    fn remove_bkup_bind_entry_req(request: BindReq);
    fn backup_bind_table_req(binding_table_entries: u16, binding_table_list: Vec<>);
    fn recover_bind_table_req(start_index: u16);
    fn backup_source_bind_req(start_table_entries: u16, start_indes: u16, source_table_list: List);
    fn recover_source_bind_req(start_index: u16);
    // 2.4.3.3 Network Management Client Services
    fn mgmt_nwk_disc_req(scan_channels: u32, scan_duration: u8, start_index: u8);
    fn mgmt_lqi_req(start_index: u8);
    fn mgmt_rtg_req(start_index: u8);
    fn mgmt_bind_req(start_index: u8);
    fn mgmt_leave_req(device_address: u64, remove_children: bool, rejoin: bool);
    fn mgmt_direct_join_req(device_address: u64, capability_information: u8);
    fn mgmt_permit_joining_req(permit_duration: u8, tc_significance: bool);
    fn mgmt_cache_req(start_index: u8);
    fn mgmt_nwk_update_req(request: MgmtNwkUpdateReq);   
}
