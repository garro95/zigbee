///The framework is 

use std::borrow::Cow;

pub struct ComplexDescriptor<'a> {
    desc: Cow<'a, [ComplexDescriptorField]>
}

pub enum ComplexDescriptorField{
    LanguageCharset{
        iso_language_code: u16,
        charset_id: u8
    },
    ManufacturerName(String),
    ModelName(String),
    SerialNumber(String),
    DeviceUrl(String),
    Icon(Vec<u8>),
    IconUrl(String)
}

pub enum LogicalType{
    Coordinator,
    Router,
    EndDevice
}

pub enum FrequencyBand{
    /// 868 - 868.6 MHz
    Low = 0,
    /// 902 - 928 MHz
    Mid = 2,
    /// 2400 - 2483.5 MHz
    High = 3,
}

pub struct MacCapability{
    pub alternate_pan_coordinator: bool,
    pub device_type: bool,
    pub power_source: bool,
    pub receiver_on_when_idle: bool,
    pub security: bool,
    pub allocate_address: bool
}

pub struct ServerMask{
    pub primary_trust_center: bool,
    pub backup_trust_center: bool,
    pub primary_binding_table_cache: bool,
    pub backup_binding_table_cache: bool,
    pub primary_discovery_cache: bool,
    pub backup_discovery_cache: bool,
    pub network_manager: bool
}

pub struct DescriptorCapability{
    pub extended_active_endpoint_list_available: bool,
    pub extended_simple_descriptor_list_available: bool
}

pub struct NodeDescriptor {
    pub logical_type: LogicalType,
    pub complex_descriptor_available: bool,
    pub user_descriptor_available: bool,
    //aps_flags: unsupported for now
    pub frequency_band: FrequencyBand,
    pub mac_capability_flags: MacCapability,
    pub manufacturer_code: u16,
    pub maximum_buffer_size: u8,
    pub maximum_incoming_transfer_size: u16,
    pub server_mask: ServerMask,
    pub maximum_outgoing_transfer_size: u16,
    pub descriptor_capability_field: DescriptorCapability,
}

///queried in the ZDO management entity device and service discovery
pub struct Descriptor<'a> {
    pub node: NodeDescriptor<u8>,
    pub node_power: u8,
    pub simple: u8,
    pub complex: Option<ComplexDescriptor<'a>>,
    pub user: Option<u8>,
}

pub enum PowerMode{
    /// Receiver synchronized with the receiver on when idle subfield of the node descriptor.
    Synchronized,
    /// Receiver comes on periodically as defined by the node power descriptor.
    OnPeriodically,
    /// Receiver comes on when stimulated, e.g. by a user pressing a button.
    OnDemand
}

pub struct AvailablePowerSources{
    pub constant_mains_power: bool,
    pub rechargeable_battery: bool,
    pub disposable_battery: bool
}

pub enum PowerSource{
    ConstantMainsPower,
    RechargeableBattery,
    DisposableBattery
}

pub enum PowerLevel{
    Critical,
    /// 33%
    Low = 4,
    /// 66%
    High = 8,
    /// 100%
    Full = 12
}

bitfield!{
    pub struct PowerDescriptor([u8]);
    impl Debug;
    pub PowerMode, into PowerMode, current_power_mode, _ : 4;
    pub AvailablePowerSources, into AvailablePowerSources, available_power_sources, _: 4;
    pub PowerSource, into PowerSource, current_power_source, _: 4;
    pub PowerLevel, into PowerLevel, current_power_source_level, _: 4;
}

pub struct SimpleDescriptor<'a, 'b> {
    pub endpoint: u8,
    pub appl_prof_id: u16,
    pub appl_dev_id: u16,
    pub appl_dev_vers: u8,
    pub appl_input_clusters: Cow<'a, [u16]>,
    pub appl_output_clusters: Cow<'b, [u16]>
}

pub struct UserDescriptor{
    pub descriptor: [u8;16]
}
