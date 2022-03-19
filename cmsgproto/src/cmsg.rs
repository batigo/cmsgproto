/// client消息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMsg {
    /// 消息id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// 消息类型
    #[prost(enumeration="ClientMsgType", tag="2")]
    pub r#type: i32,
    /// 本条消息是否需要回ack
    #[prost(bool, tag="3")]
    pub ack: bool,
    /// 业务消息对应的service id
    #[prost(string, optional, tag="4")]
    pub service_id: ::core::option::Option<::prost::alloc::string::String>,
    /// biz_data压缩类型
    #[prost(enumeration="CompressorType", optional, tag="5")]
    pub compressor: ::core::option::Option<i32>,
    /// 业务消息data, 网关作为消息通道透传，  client <-> service
    #[prost(bytes="vec", optional, tag="6")]
    pub biz_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// 初始化信息
    #[prost(message, optional, tag="7")]
    pub init_data: ::core::option::Option<InitData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitData {
    /// biz_data压缩类型
    #[prost(enumeration="CompressorType", tag="1")]
    pub accept_compressor: i32,
    /// 长连接心跳间隔，单位秒
    #[prost(uint32, tag="2")]
    pub ping_interval: u32,
}
/// client消息类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientMsgType {
    /// 未使用
    Unused = 0,
    /// ws长连接初始化消息 client -> bati
    Init = 1,
    /// ws长简介初始化响应消息 client <- bati
    InitResp = 2,
    /// 业务消息 client <-> bati <-> service
    Biz = 3,
    /// ack消息
    Ack = 4,
    /// echo 消息用于测试 client <-> bati
    Echo = 100,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressorType {
    /// 消息biz_data不压缩
    Null = 0,
    /// 使用deflate压缩biz_data
    Deflate = 1,
}
