use zerocopy_derive::{AsBytes, FromZeroes, FromBytes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct Header {
    request_type: u16,
    block_1: u16,
    block_2: u32,
}

impl Header {
    pub fn request_type(&self) -> u16 { self.request_type }
    pub fn send_static_count(&self) -> u8 { (self.block_1 & 0xF) as u8 }
    pub fn send_buffer_count(&self) -> u8 { ((self.block_1 >> 4) & 0xF) as u8 }
    pub fn receive_buffer_count(&self) -> u8 { ((self.block_1 >> 8) & 0xF) as u8 }
    pub fn exchange_buffer_count(&self) -> u8 { ((self.block_1 >> 12) & 0xF) as u8 }
    pub fn data_words(&self) -> u16 { ((self.block_2) & 0x3FF) as u16 }
    pub fn has_special_header(&self) -> bool { ((self.block_2 >> 27) & 1) != 0 }

    pub fn set_request_type(&mut self, request_type: u16) { self.request_type = request_type }
    pub fn set_send_static_count(&mut self, send_static_count: u8) { 
        if send_static_count > 0xF {
            panic!("send_static_count too high! max is 0xF, got: {}", send_static_count);
        }

        self.block_1 &= 0xFFF0;
        self.block_1 |= send_static_count as u16;
    }
    pub fn set_send_buffer_count(&mut self, send_buffer_count: u8) {
        if send_buffer_count > 0xF {
            panic!("send_buffer_count too high! max is 0xF, got: {}", send_buffer_count);
        }

        self.block_1 &= 0xFF0F;
        self.block_1 |= (send_buffer_count as u16) << 4;
    }
    pub fn set_receive_buffer_count(&mut self, receive_buffer_count: u8) {
        if receive_buffer_count > 0xF {
            panic!("receive_buffer_count too high! max is 0xF, got: {}", receive_buffer_count);
        }

        self.block_1 &= 0xF0FF;
        self.block_1 |= (receive_buffer_count as u16) << 8;
    }
    pub fn set_exchange_buffer_count(&mut self, exchange_buffer_count: u8) {
        if exchange_buffer_count > 0xF {
            panic!("exchange_buffer_count too high! max is 0xF, got: {}", exchange_buffer_count);
        }

        self.block_1 &= 0x0FFF;
        self.block_1 |= (exchange_buffer_count as u16) << 12;
    }
    pub fn set_data_words(&mut self, data_words: u16) {
        if data_words > 0x3FF {
            panic!("data_words too high! max is 0x3FF (1kb), got: {}", data_words);
        }

        self.block_2 &= 0xFFFFFC00;
        self.block_2 |= data_words as u32;
    }
    pub fn set_special_header(&mut self, special_header: bool) {
        self.block_2 &= 0x7FFFFFFF;
        self.block_2 |= (if special_header { 1 } else { 0 }) << 31;
    }

    pub fn with_request_type(mut self, request_type: u16) -> Self { self.set_request_type(request_type); self }
    pub fn with_send_static_count(mut self, send_static_count: u8) -> Self { self.set_send_static_count(send_static_count); self }
    pub fn with_send_buffer_count(mut self, send_buffer_count: u8) -> Self { self.set_send_buffer_count(send_buffer_count); self }
    pub fn with_receive_buffer_count(mut self, receive_buffer_count: u8) -> Self { self.set_receive_buffer_count(receive_buffer_count); self }
    pub fn with_exchange_buffer_count(mut self, exchange_buffer_count: u8) -> Self { self.set_exchange_buffer_count(exchange_buffer_count); self }
    pub fn with_data_words(mut self, data_words: u16) -> Self { self.set_data_words(data_words); self }
    pub fn with_special_header(mut self, special_header: bool) -> Self { self.set_special_header(special_header); self }
}
