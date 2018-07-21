///The framework is 

pub struct ComplexDescriptor{
    desc: Vec<ComplexDescriptorField>
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

bitfield!{
    pub struct NodeDescriptor([u8]);
    impl Debug;
    pub LogicalType, into LogicalType, logical_type, _: 3;
    pub bool, into bool,  complex_descriptor_available, _: 1;
    pub bool, into bool, user_descriptor_available, _: 1;
    pub aps_flags, _: 3;
    pub FrequencyBand, into FrequencyBand, frequency_band, _:5;
    pub MacCapability, into MacCapability, mac_capability_flags,_: 8;
    pub u16, into u16, manufacturer_code, _: 16;
    pub u8, into u8, maximum_buffer_size, _: 8;
    pub u16, into u16, maximum_incoming_transfer_size,_: 16;
    pub ServerMask, into ServerMask, server_mask, _: 16;
    pub u16, into u16, maximum_outgoing_transfer_size, _: 16;
    pub DescriptorCapability, into DescriptorCapability, descriptor_capability_field, _: 8;
}

///queried in the ZDO management entity device and service discovery
pub struct Descriptor {
    pub node: NodeDescriptor<u8>,
    pub node_power: u8,
    pub simple: u8,
    pub complex: Option<ComplexDescriptor>,
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

pub struct SimpleDescriptor{
    pub endpoint: u8,
    pub appl_prof_id: u16,
    pub appl_dev_id: u16,
    pub appl_dev_vers: u8,
    pub appl_input_clusters: Vec<u16>,
    pub appl_output_clusters: Vec<u16>
}

pub struct UserDescriptor{
    pub descriptor: [u8;16]
}
