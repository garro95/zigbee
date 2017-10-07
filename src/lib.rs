//! This library aims to provide a standard API for the interoperation of
//! other crates that implements (parts of) the Zigbee network, that
//! provides a protocol stack to be used in internet of things applications.
//! Its architecture follow the stack as proposed in the Zigbee
//! Specification document attached in the doc directory.
//! MAC and PHY layers are provided by the IEEE 802.15.4-2003 standard.
//! This library covers the upper layes as the Zigbee Standard does.
extern crate futures;
#[macro_use] extern crate bitfield;

use std::time::Duration;
use std::fmt;
use std::error::Error;
/// This module focus on the application layer.
pub mod apl {
    /// This module provides the Frames and the primitives of the
    /// Application Support sub-layer Data Entity and Mangement Entity
    pub mod aps {
        use futures::Future;
        mod frame_format{
            pub enum FrameType {
                Data,
                Command,
                Acknowledgement
            }
            pub enum DeliveryMode {
                NormalUnicastDelivery = 0,
                Broadcast = 2,
                GroupAddressing = 3
            }
            pub struct FrameControlField {
                pub frame_type:  FrameType, //this should be a bit bield of 2 bits
                pub delivery_mode: DeliveryMode,
                pub ack_format: bool, //false for data frame acknowledgement and true for APS command frame acknowledgement.
                pub security: bool,
                pub ack_request: bool,
                pub extended_header_present: bool
            }
            pub struct AddressingFields {
                pub destination_endpoint: Option<u8>, // only for broadcast or normal unicast
                pub group_address: Option<u16>, // only for group addressing
                pub cluster_identifier: Option<u16>, // only for data or ack frames
                pub profile_identifier: Option<u16>, // only for data or ack frames
                pub source_endpoint: u8 
            }

            pub enum FragmentationField {
                NotFragmented,
                FirstPart,
                NotFirstPart
            }
            pub struct AckBitfield {

            }
            pub struct ExtHeader {
                pub fragmentation: FragmentationField,
                pub block_number: Option<u8>,
                pub ack_bitfield: Option<AckBitfield>
            }
            
            pub struct Apdu<'a> {
                pub frame_control: FrameControlField,
                pub address: AddressingFields,
                pub aps_counter: u8,
                pub extended_header: Option<ExtHeader>,
                pub frame_payload: &'a[u8]
            }
        }

        #[derive(Clone, Copy)]
        pub enum AddrAndEp{
            None,
            ShortAddressNoEp(u16),
            ShortAddressWithEp(u16, u8),
            LongAddress(u64, u8)
        }

        pub enum SecurityStatus {
            Unsecured,
            SecuredNwkKey,
            SecuredLinkKey
        }

        pub enum IndicationStatus {
            Success,
            DefragUnupported,
            DefragDeferred,
            SecurityProcessingError/*(Error)*/// error 
        }

        pub struct TxOptions {
            pub security_enabled: bool,
            pub use_nwk_key: bool,
            pub acknowledged_transmission: bool,
            pub fragmentation_permitted: bool,
            pub include_extended_nonce_in_aps_security_frame: bool,
        }

        /// The information that the APS provides to the upper layer when a
        /// new frame is received. This is transmitted to the NHLE issuing the
        /// APSDE-DATA.indication primitive, calling the function provided on
        /// the registration of the interested endpoint.
        pub struct DataIndication<'a> {
            pub dst_addr_ep: AddrAndEp,
            pub src_addr_ep: AddrAndEp,
            pub profile_id: u16,
            pub cluster_id: u16,
            pub asdu: &'a[u8],
            pub status: IndicationStatus,
            pub security_status: SecurityStatus,
            pub link_quality: u8,
            pub rx_time: ::Duration
        }

        #[derive(Debug)]
        pub enum DataConfirmStatus {
            Success,
            NoShortAddress,
            NoBoundDevice,
            SecurityFail,
            NoAck,
            AsduTooLong, //Application service data unit, specification contains a typo
            NldeError/*(Error)*/
        }

        #[derive(Debug)]
        pub struct DataError {
            status: DataConfirmStatus
        }

        impl ::Error for DataError {
            fn description(&self) -> &str {
                "APSE-DATA request error"
            }
        }

        impl ::fmt::Display for DataError {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                write!(f, "APSE-DATA request error: {:?}", self.status)
            }
        }

        /// The data contained in the APSDE-DATA.confirm primitive that is issued by the
        /// APS to the Next Higher Layer Entity in response to a APSDE-DATA.request
        pub struct DataConfirm {
            pub dst: AddrAndEp,
            pub src_endpoint: u8,
            pub status: DataConfirmStatus,
            pub tx_time: ::Duration
        }

        /// The arguments needed to issue the APSDE-DATA.request primitive
        pub struct DataRequest<'a> {
            pub dst: AddrAndEp,
            pub profile_id: u16,
            pub cluster_id: u16,
            pub src_endpoint: u8,
            pub asdu: &'a[u8],
            pub options: TxOptions,
            pub radius: u8
        }

        pub enum RegistrationError {
            
        }

        /// This trait is implemented by all these structs that implements an
        /// Application Support sub-layer Data Entity in order to provide the
        /// functionalities required by the specification.
        ///
        /// Crates that need the functionalities of a zigbee APSDE can rely on this trait.
        pub trait ApsdeSap{
            fn data_request(&self, request: DataRequest) -> Box<Future<Item=DataConfirm, Error=DataError>>;
            fn register_application_object(&self, endpoint: u8, indication_callback: Fn(DataIndication)) -> Result<(), RegistrationError>;
            fn deregister_application_object(&self, endpoint: u8);
        }

        pub struct BindRequest {
            pub src_addr: u64,
            pub src_endpoint: u8,
            pub cluster_id: u16,
            pub dst_addr: AddrAndEp
        }

        pub enum BindStatus {
            Success,
            IllegalRequest,
            TableFull,
            NotSupported
        }
        
        pub struct BindConfirm {
            pub status: BindStatus,
            pub src_addr: u64,
            pub src_endpoint: u8,
            pub cluster_id: u16,
            pub dst_addr: AddrAndEp
        }

        type UnbindRequest = BindRequest;

        pub enum UnbindStatus {
            Success,
            IllegalRequest,
            InvalidBinding
        }

        pub struct UnbindConfirm {
            pub status: UnbindStatus,
            pub src_addr: u64,
            pub src_endpoint: u8,
            pub cluster_id: u16,
            pub dst_addr: AddrAndEp
        }

        pub struct BindingTable {
        }
        pub struct ChannelMask {
        }
        pub struct GroupTable<'a> {
            entries: &'a Iterator<Item=(u16, Iterator<Item = u8>)>
        }
        pub struct PermissionsConfiguration {
        }

        pub enum AddGroupStatus {
            Success,
            InvalidParameter,
            TableFull
        }
        pub struct AddGroupConfirm{
            pub status: AddGroupStatus,
            pub group_address: u16,
            pub endpoint: u8
        }

        pub enum RemoveGroupStatus {
            Success,
            InvalidGroup,
            InvalidParameter,
        }
        pub struct RemoveGroupConfirm{
            pub status: RemoveGroupStatus,
            pub group_address: u16,
            pub endpoint: u8
        }

        pub enum RemoveAllGroupsStatus {
            Success,
            InvalidParameter
        }
        pub struct RemoveAllGroupsConfirm{
            pub status: RemoveAllGroupsStatus,
            pub endpoint: u8
        }

        pub trait ApsmeSap{
            fn bind_request(&self, request: BindRequest) -> BindConfirm;
            fn unbind_request(&self, request: UnbindRequest) -> UnbindConfirm;
            fn binding_table(&self) -> Option<BindingTable> { None }
            fn designated_coordinator(&self) -> bool { false }
            fn channel_mask(&self) -> Option<ChannelMask>;
            fn use_extended_pan_id(&self) -> u64 { 0 }
            fn group_table(&self) -> Option<GroupTable> { None }
            fn nonmember_radius(&self) -> u8 { 2 }
            fn permissions_configuration(&self) -> Option<PermissionsConfiguration>{
                None
            }
            fn use_insecure_join(&self) -> bool { true }
            fn interframe_delay(&self) -> u8;
            fn last_channel_energy(&self) -> Option<u8>;
            fn last_channel_failure_rate(&self) -> Option<f32>;
            fn channel_timer(&self) -> Option<f32>;
            fn max_window_size(&self) -> Option<u8>;
            fn set_binding_table(&self, BindingTable) -> Result<(), ()>;
            fn set_designated_coordinator(&self, bool) -> Result<(), ()>;
            fn set_channel_mask(&self, ChannelMask) -> Result<(), ()>;
            fn set_extended_pan_id(&self, u64) -> Result<(), ()>;
            fn set_group_table(&self, GroupTable) -> Result<(), ()>;
            fn set_nonmember_radius(&self, u8) -> Result<(), ()>;
            fn set_permissions_configuration(&self, PermissionsConfiguration) -> Result<(), ()>;
            fn set_insecure_join(&self, bool) -> Result<(), ()>;
            fn set_interframe_delay(&self, u8) -> Result<(), ()>;
            fn set_last_channel_energy(&self, u8) -> Result<(), ()>;
            fn set_last_channel_failure_rate(&self, f32) -> Result<(), ()>;
            fn set_channel_timer(&self, f32) -> Result<(), ()>;
            fn set_max_window_size(&self, u8) -> Result<(), ()>;
            fn add_group_request(&self, group_adderss: u16, endpoint: u8) -> AddGroupConfirm;
            fn remove_group_request(&self, group_adderss: u16, endpoint: u8) -> RemoveGroupConfirm;
            fn remove_all_groups_request(&self, endpoint: u8) -> RemoveAllGroupsConfirm;
        }
        pub const MAX_DESCRIPTOR_SIZE: usize = 64;
        pub const MAX_FRAME_RETRIES: u32 = 3;
        pub const MIN_DUPLICATE_REJECTION_TABLE_SIZE: usize = 1;
        pub const MIN_HEADER_OVERHEAD: usize = 0x0C;
    }

    ///The framework is 
    pub mod framework{
        pub struct ComplexDescriptor{
            desc: Vec<u8>
        }

        pub enum LogicalType{
            Coordinator,
            Router,
            EndDevice
        }

        bitfield!{
            pub struct NodeDescriptor([u8]);
            pub LogicalType, into LogicalType, logical_type, _: 3;
            pub bool, into bool,  complex_descriptor_available, _: 1;
            pub bool, into bool, user_descriptor_available, _: 1;
            pub aps_flags, _: 3;
            pub frequency_band, _:5;
            pub u8, into u8, max_capability_flags,_: 8;
            pub u16, into u16, manufacturer_code, _: 16;
            pub u8, into u8, maximum_buffer_size, _: 8;
            pub u16, into u16, maximum_incoming_transfer_size,_: 16;
            pub u16, into u16, server_mask, _: 16;
            pub u16, into u16, maximum_outgoing_transfer_size, _: 16;
            pub u8, into u8, descriptor_capability_field, _: 8;
        }
        
        ///queried in the ZDO management entity device and service discovery
        pub struct Descriptor {
            pub node: NodeDescriptor<u8>,
            pub node_power: u8,
            pub simple: u8,
            pub complex: Option<ComplexDescriptor>,
            pub user: Option<u8>,
        }
    }
}
/// This module contains traits and data structures for the network layer.
pub mod nwk{
    pub trait NlmeSap{
        fn nlme_get(&self);
        fn nlme_set(&self);
    }
}
