use core::{iter::zip, prelude::v1};

use zerocopy::{AsBytes, FromBytes};

use crate::{
    ipc::{cmif::*, hipc::Header},
    svc,
    util::{align::align_up, result::ResultCode, tls},
};

use self::service::Service;

use super::{cmif, hipc};

mod service;

pub enum BufferKind {
    AutoIn(hipc::Mode),
    AutoOut(hipc::Mode),
    PointerIn,
    PointerOut,
    FixedSizePointerOut,
    MapAliasIn(hipc::Mode),
    MapAliasOut(hipc::Mode),
    MapAliasInOut(hipc::Mode),
}

pub struct Buffer<'a, T> {
    kind: BufferKind,
    data: &'a [T],
}

impl<'a, T> Buffer<'a, T> {
    pub fn from_value(kind: BufferKind, data: &'a T) -> Self {
        Self {
            kind,
            data: core::slice::from_ref(data),
        }
    }

    pub fn from_slice(kind: BufferKind, data: &'a [T]) -> Self { Self { kind, data } }
}

pub struct RequestContext<'a, D: Copy + AsBytes> {
    send_pid: bool,
    objects: heapless::Vec<u32, 8>,
    hipc_send_handles: heapless::Vec<u32, 8>,
    hipc_move_handles: heapless::Vec<u32, 8>,
    hipc_static_index: u8,
    hipc_send_statics: heapless::Vec<hipc::Static, 8>,
    hipc_send_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_receive_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_exchange_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_receive_statics: heapless::Vec<hipc::ReceiveStatic, 8>,
    data: &'a [D],
}

impl<'a, D: Copy + AsBytes> RequestContext<'a, D> {
    pub fn new_from(data: &'a D) -> Self {
        RequestContext::new_from_slice(core::slice::from_ref(data))
    }

    pub fn new_from_slice(data: &'a [D]) -> Self {
        Self {
            send_pid: false,
            objects: heapless::Vec::new(),
            hipc_send_handles: heapless::Vec::new(),
            hipc_move_handles: heapless::Vec::new(),
            hipc_static_index: 0,
            hipc_send_statics: heapless::Vec::new(),
            hipc_send_buffers: heapless::Vec::new(),
            hipc_receive_buffers: heapless::Vec::new(),
            hipc_exchange_buffers: heapless::Vec::new(),
            hipc_receive_statics: heapless::Vec::new(),
            data,
        }
    }

    pub fn set_send_pid(&mut self) { self.send_pid = true; }

    pub fn add_service<Srv>(&mut self, service: Service<Srv>) {
        self.objects
            .push(service.handle.value)
            .map_err(drop)
            .unwrap();
    }

    pub fn add_send_handle<T>(&mut self, handle: svc::Handle<T>) {
        self.hipc_send_handles
            .push(handle.value)
            .map_err(drop)
            .unwrap();
    }

    pub fn add_move_handle<T>(&mut self, handle: svc::Handle<T>) {
        self.hipc_move_handles
            .push(handle.value)
            .map_err(drop)
            .unwrap();
    }

    pub fn add_buffer<T>(&mut self, buffer: &mut Buffer<'a, T>) {
        match buffer.kind {
            BufferKind::AutoIn(mode) => {
                self.hipc_send_statics
                    .push(hipc::Static::new(self.hipc_static_index, 0, 0))
                    .unwrap();
                self.hipc_send_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        mode,
                    ))
                    .unwrap();
                self.hipc_static_index += 1;
            }
            BufferKind::AutoOut(mode) => {
                self.hipc_receive_statics
                    .push(hipc::ReceiveStatic::new(0, 0))
                    .unwrap();
                self.hipc_receive_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        mode,
                    ))
                    .unwrap();
            }
            BufferKind::PointerIn => {
                self.hipc_send_statics
                    .push(hipc::Static::new(
                        self.hipc_static_index,
                        buffer.data.len() as u16,
                        buffer.data.as_ptr() as u64,
                    ))
                    .unwrap();
                self.hipc_static_index += 1;
            }
            BufferKind::PointerOut => {
                self.hipc_receive_statics
                    .push(hipc::ReceiveStatic::new(
                        buffer.data.as_ptr() as u64,
                        buffer.data.len() as u16,
                    ))
                    .unwrap();
            }
            BufferKind::FixedSizePointerOut => {
                self.hipc_exchange_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        hipc::Mode::Normal,
                    ))
                    .unwrap();
            }
            BufferKind::MapAliasIn(mode) => {
                self.hipc_send_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        mode,
                    ))
                    .unwrap();
            }
            BufferKind::MapAliasOut(mode) => {
                self.hipc_receive_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        mode,
                    ))
                    .unwrap();
            }
            BufferKind::MapAliasInOut(mode) => {
                self.hipc_exchange_buffers
                    .push(hipc::Buffer::new(
                        buffer.data.len() as u64,
                        buffer.data.as_ptr() as u64,
                        mode,
                    ))
                    .unwrap();
            }
        }
    }
}

pub struct Response<'a, D: Copy + FromBytes> {
    objects: heapless::Vec<u32, 8>,
    hipc_send_handles: heapless::Vec<u32, 8>,
    hipc_move_handles: heapless::Vec<u32, 8>,
    send_statics: heapless::Vec<hipc::Static, 8>,
    pid: Option<u64>,
    data: &'a [D],
}

impl<'a, D: Copy + FromBytes> Response<'a, D> {
    pub fn new(
        objects: heapless::Vec<u32, 8>,
        hipc_send_handles: heapless::Vec<u32, 8>,
        hipc_move_handles: heapless::Vec<u32, 8>,
        send_statics: heapless::Vec<hipc::Static, 8>,
        pid: Option<u64>,
        data: &'a [D],
    ) -> Self {
        Self {
            objects,
            hipc_send_handles,
            hipc_move_handles,
            send_statics,
            pid,
            data,
        }
    }

    pub fn get_send_handle<T>(&mut self) -> svc::Handle<T> {
        svc::Handle::new(self.hipc_send_handles.pop().unwrap())
    }

    pub fn get_move_handle<T>(&mut self) -> svc::Handle<T> {
        svc::Handle::new(self.hipc_move_handles.pop().unwrap())
    }

    pub fn get_object(&mut self) -> u32 { self.objects.pop().unwrap() }

    pub fn get_pid(&mut self) -> u64 { self.pid.unwrap() }
}

pub struct HipcSizes{
    hipc_data_size: u16,
    domain_data_size: u16,
}

fn sizes_from_request<Srv, Req: Copy + AsBytes>(service: &Service<Srv>, request_context: &RequestContext<'_, Req>) -> HipcSizes {
    let mut sizes = HipcSizes {
        hipc_data_size: 16,
        domain_data_size: (core::mem::size_of::<cmif::InHeader>()
        + (request_context.data.len() * core::mem::size_of::<Req>())) as u16,
    };
    
    if !service.is_root {
        sizes.hipc_data_size += core::mem::size_of::<cmif::DomainInHeader>() as u16;
        sizes.hipc_data_size += core::mem::size_of::<u32>() as u16 * request_context.objects.len() as u16;
    }
    
    sizes.hipc_data_size += sizes.domain_data_size;

    sizes
}

pub fn invoke_service_request<Srv, Req: Copy + AsBytes, Res: Copy + FromBytes, Out>(
    service: Service<Srv>,
    request_context: &RequestContext<'_, Req>,
    response_handler: fn(&RequestContext<'_, Req>, Response<'_, Res>) -> Result<Out, ResultCode>,
) -> Result<Out, ResultCode> {
    let mut writer = tls::get_writer(0, 0x100);
    let has_special_header = request_context.send_pid
        || request_context.hipc_send_handles.len() > 0
        || request_context.hipc_move_handles.len() > 0;

    let sizes = sizes_from_request(&service, request_context);

    writer.write(Header::default()
        .with_request_type(cmif::CommandType::Request as u16)
        .with_data_words(sizes.hipc_data_size / 4)
        .with_special_header(has_special_header)
        .with_send_static_count(request_context.hipc_send_statics.len() as u8)
        .with_send_buffer_count(request_context.hipc_send_buffers.len() as u8)
        .with_receive_buffer_count(request_context.hipc_receive_buffers.len() as u8)
        .with_exchange_buffer_count(request_context.hipc_exchange_buffers.len() as u8)
    );

    if has_special_header {
        writer.write(hipc::SpecialHeader::new(
            request_context.send_pid,
            request_context.hipc_send_handles.len() as u8,
            request_context.hipc_move_handles.len() as u8,
        ));

        if request_context.send_pid {
            writer.write(0 as usize)
        }

        writer.write_vec(&request_context.hipc_send_handles);
        writer.write_vec(&request_context.hipc_move_handles);
    }

    writer.write_vec(&request_context.hipc_send_statics);
    writer.write_vec(&request_context.hipc_send_buffers);
    writer.write_vec(&request_context.hipc_receive_buffers);
    writer.write_vec(&request_context.hipc_exchange_buffers);

    writer.align_to(16);

    if !service.is_root {
        writer.write(
            cmif::DomainInHeader::default()
                .with_command_type(DomainCommandType::Request)
                .with_object_count(request_context.objects.len() as u8)
                .with_data_size(sizes.domain_data_size),
        );

        writer.write_vec(&request_context.objects);
    }

    writer.write(cmif::InHeader::default().with_command(CommandType::Request as u32));
    writer.write_vec(&request_context.data);

    writer.align_to(16);

    writer.write_vec(&request_context.hipc_receive_statics);

    service.handle.send_sync_request()?;

    let mut reader = tls::get_reader(0, 0x100);

    let header: Header = reader.read();

    let hipc_send_statics = reader.read_vec(header.send_static_count() as usize);

    let mut hipc_send_handles = heapless::Vec::<u32, 8>::new();
    let mut hipc_move_handles = heapless::Vec::<u32, 8>::new();
    let mut pid = None;

    if header.has_special_header() {
        let special_header: hipc::SpecialHeader = reader.read();

        if special_header.has_pid() {
            pid = Some(reader.read());
        }

        reader.read_into_vec(&mut hipc_send_handles, special_header.copy_handle_count() as usize);
        reader.read_into_vec(&mut hipc_move_handles, special_header.move_handle_count() as usize);
    }

    reader.align_to(16);

    let mut data_size = header.data_words() as usize * 4 - core::mem::size_of::<cmif::InHeader>();

    let mut cmif_object_ids = heapless::Vec::<u32, 8>::new();
    if !service.is_root {
        let domain_header = reader.read::<cmif::DomainInHeader>();
        reader.read_into_vec(&mut cmif_object_ids, domain_header.object_count() as usize);
        
        data_size -= core::mem::size_of::<cmif::DomainInHeader>()
            + core::mem::size_of::<u32>() * domain_header.object_count() as usize;
    }

    let cmif_header = reader.read::<cmif::OutHeader>();

    if cmif_header.result().is_failure() {
        return Err(cmif_header.result());
    }
    
    let data = reader.read_slice::<Res>(data_size / core::mem::size_of::<Res>());

    let response = Response::new(
        cmif_object_ids,
        hipc_send_handles,
        hipc_move_handles,
        hipc_send_statics,
        pid,
        data,
    );

    response_handler(request_context, response)
}
