use pnet::packet::{
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

pub trait GettableEndpoints{
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self)->&[u8];
}

impl<'a> GettableEndpoints for Ipv4Packet<'a>{
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    
    fn get_payload(&self)->&[u8] {
        self.payload()
    }
}

impl<'a> GettableEndpoints for Ipv6Packet<'a>{
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)->&[u8] {
        self.payload()
    }
}

impl<'a> GettableEndpoints for TcpPacket<'a>{
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)->&[u8] {
        self.payload()
    }
}

impl<'a> GettableEndpoints for UdpPacket<'a>{
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self)->&[u8] {
        self.payload()
    }
}