/// common/serial/typed_message.proto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedMessage {
    /// The name of the message type, retrieved from protobuf API.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Serialized proto message.
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// common/net/port.proto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortRange {
    /// The port that this range starts from.
    #[prost(uint32, tag="1")]
    pub from: u32,
    /// The port that this range ends with (inclusive).
    #[prost(uint32, tag="2")]
    pub to: u32,
}
/// PortList is a list of ports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortList {
    #[prost(message, repeated, tag="1")]
    pub range: ::prost::alloc::vec::Vec<PortRange>,
}
/// NetworkList is a list of Networks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkList {
    #[prost(enumeration="Network", repeated, tag="1")]
    pub network: ::prost::alloc::vec::Vec<i32>,
}
/// app/router/config.proto
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    /// Domain matching type.
    #[prost(enumeration="domain::Type", tag="1")]
    pub r#type: i32,
    /// Domain value.
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    /// Attributes of this domain. May be used for filtering.
    #[prost(message, repeated, tag="3")]
    pub attribute: ::prost::alloc::vec::Vec<domain::Attribute>,
}
/// Nested message and enum types in `Domain`.
pub mod domain {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribute {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(oneof="attribute::TypedValue", tags="2, 3")]
        pub typed_value: ::core::option::Option<attribute::TypedValue>,
    }
    /// Nested message and enum types in `Attribute`.
    pub mod attribute {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TypedValue {
            #[prost(bool, tag="2")]
            BoolValue(bool),
            #[prost(int64, tag="3")]
            IntValue(i64),
        }
    }
    /// Type of domain value.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The value is used as is.
        Plain = 0,
        /// The value is used as a regular expression.
        Regex = 1,
        /// The value is a root domain.
        Domain = 2,
        /// The value is a domain.
        Full = 3,
    }
}
/// IP for routing decision, in CIDR form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cidr {
    /// IP address, should be either 4 or 16 bytes.
    #[prost(bytes="vec", tag="1")]
    pub ip: ::prost::alloc::vec::Vec<u8>,
    /// Number of leading ones in the network mask.
    #[prost(uint32, tag="2")]
    pub prefix: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoIp {
    #[prost(string, tag="1")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub cidr: ::prost::alloc::vec::Vec<Cidr>,
    #[prost(bool, tag="3")]
    pub reverse_match: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoIpList {
    #[prost(message, repeated, tag="1")]
    pub entry: ::prost::alloc::vec::Vec<GeoIp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoSite {
    #[prost(string, tag="1")]
    pub country_code: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub domain: ::prost::alloc::vec::Vec<Domain>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoSiteList {
    #[prost(message, repeated, tag="1")]
    pub entry: ::prost::alloc::vec::Vec<GeoSite>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingRule {
    #[prost(string, tag="18")]
    pub rule_tag: ::prost::alloc::string::String,
    /// List of domains for target domain matching.
    #[prost(message, repeated, tag="2")]
    pub domain: ::prost::alloc::vec::Vec<Domain>,
    /// List of CIDRs for target IP address matching.
    /// Deprecated. Use geoip below.
    #[deprecated]
    #[prost(message, repeated, tag="3")]
    pub cidr: ::prost::alloc::vec::Vec<Cidr>,
    /// List of GeoIPs for target IP address matching. If this entry exists, the
    /// cidr above will have no effect. GeoIP fields with the same country code are
    /// supposed to contain exactly same content. They will be merged during
    /// runtime. For customized GeoIPs, please leave country code empty.
    #[prost(message, repeated, tag="10")]
    pub geoip: ::prost::alloc::vec::Vec<GeoIp>,
    /// A range of port [from, to]. If the destination port is in this range, this
    /// rule takes effect. Deprecated. Use port_list.
    #[deprecated]
    #[prost(message, optional, tag="4")]
    pub port_range: ::core::option::Option<PortRange>,
    /// List of ports.
    #[prost(message, optional, tag="14")]
    pub port_list: ::core::option::Option<PortList>,
    /// List of networks. Deprecated. Use networks.
    #[deprecated]
    #[prost(message, optional, tag="5")]
    pub network_list: ::core::option::Option<NetworkList>,
    /// List of networks for matching.
    #[prost(enumeration="Network", repeated, tag="13")]
    pub networks: ::prost::alloc::vec::Vec<i32>,
    /// List of CIDRs for source IP address matching.
    #[deprecated]
    #[prost(message, repeated, tag="6")]
    pub source_cidr: ::prost::alloc::vec::Vec<Cidr>,
    /// List of GeoIPs for source IP address matching. If this entry exists, the
    /// source_cidr above will have no effect.
    #[prost(message, repeated, tag="11")]
    pub source_geoip: ::prost::alloc::vec::Vec<GeoIp>,
    /// List of ports for source port matching.
    #[prost(message, optional, tag="16")]
    pub source_port_list: ::core::option::Option<PortList>,
    #[prost(string, repeated, tag="7")]
    pub user_email: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub inbound_tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub protocol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map="string, string", tag="15")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="17")]
    pub domain_matcher: ::prost::alloc::string::String,
    #[prost(oneof="routing_rule::TargetTag", tags="1, 12")]
    pub target_tag: ::core::option::Option<routing_rule::TargetTag>,
}
/// Nested message and enum types in `RoutingRule`.
pub mod routing_rule {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetTag {
        /// Tag of outbound that this rule is pointing to.
        #[prost(string, tag="1")]
        Tag(::prost::alloc::string::String),
        /// Tag of routing balancer.
        #[prost(string, tag="12")]
        BalancingTag(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancingRule {
    #[prost(string, tag="1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub outbound_selector: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub strategy: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub strategy_settings: ::core::option::Option<TypedMessage>,
    #[prost(string, tag="5")]
    pub fallback_tag: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyWeight {
    #[prost(bool, tag="1")]
    pub regexp: bool,
    #[prost(string, tag="2")]
    pub r#match: ::prost::alloc::string::String,
    #[prost(float, tag="3")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyLeastLoadConfig {
    /// weight settings
    #[prost(message, repeated, tag="2")]
    pub costs: ::prost::alloc::vec::Vec<StrategyWeight>,
    /// RTT baselines for selecting, int64 values of time.Duration
    #[prost(int64, repeated, tag="3")]
    pub baselines: ::prost::alloc::vec::Vec<i64>,
    /// expected nodes count to select
    #[prost(int32, tag="4")]
    pub expected: i32,
    /// max acceptable rtt, filter away high delay nodes. default 0
    #[prost(int64, tag="5")]
    pub max_rtt: i64,
    /// acceptable failure rate
    #[prost(float, tag="6")]
    pub tolerance: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(enumeration="config::DomainStrategy", tag="1")]
    pub domain_strategy: i32,
    #[prost(message, repeated, tag="2")]
    pub rule: ::prost::alloc::vec::Vec<RoutingRule>,
    #[prost(message, repeated, tag="3")]
    pub balancing_rule: ::prost::alloc::vec::Vec<BalancingRule>,
}
/// Nested message and enum types in `Config`.
pub mod config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DomainStrategy {
        /// Use domain as is.
        AsIs = 0,
        /// Always resolve IP for domains.
        UseIp = 1,
        /// Resolve to IP if the domain doesn't match any rules.
        IpIfNonMatch = 2,
        /// Resolve to IP if any rule requires IP matching.
        IpOnDemand = 3,
    }
}
/// common/net/network.proto
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Network {
    Unknown = 0,
    RawTcp = 1,
    Tcp = 2,
    Udp = 3,
    Unix = 4,
}
