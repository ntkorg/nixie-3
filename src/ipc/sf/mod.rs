use core::{iter::zip, prelude::v1};

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

pub struct RequestContext<'a, D> {
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

impl<'a, D> RequestContext<'a, D> {
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

pub struct Response<'a, D> {
    objects: heapless::Vec<u32, 8>,
    hipc_send_handles: heapless::Vec<u32, 8>,
    hipc_move_handles: heapless::Vec<u32, 8>,
    send_statics: heapless::Vec<hipc::Static, 8>,
    pid: Option<u64>,
    data: &'a [D],
}

impl<'a, D> Response<'a, D> {
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

fn copy_to_tls<T: Copy>(tls_offset: usize, data: &[T]) {
    let tls_data = tls::slice_offset_mut(tls_offset, data.len());
    for (tls_data, data) in zip(tls_data.iter_mut(), data.iter()) {
        *tls_data = *data;
    }
}

fn copy_from_tls<T: Copy>(tls_offset: usize, data: &mut heapless::Vec<T, 8>) {
    let tls_data = tls::slice_offset(tls_offset, data.len());
    for tls_data in tls_data.iter() {
        data.push(*tls_data).unwrap();
    }
}

pub fn invoke_service_request<Srv, Req, Res, Out>(
    service: Service<Srv>,
    request_context: &RequestContext<'_, Req>,
    response_handler: fn(&RequestContext<'_, Req>, Response<'_, Res>) -> Result<Out, ResultCode>,
) -> Result<Out, ResultCode> {
    let mut tls_offset = 0;
    let mut header: &mut Header = unsafe { tls::transmute_offset_mut(tls_offset) };

    header.set_send_static_count(request_context.hipc_send_statics.len() as u8);
    header.set_send_buffer_count(request_context.hipc_send_buffers.len() as u8);
    header.set_receive_buffer_count(request_context.hipc_receive_buffers.len() as u8);
    header.set_exchange_buffer_count(request_context.hipc_exchange_buffers.len() as u8);
    header.set_special_header(
        request_context.send_pid
    || !request_context.hipc_move_handles.is_empty()
    || !request_context.hipc_send_handles.is_empty(),
    );

    tls_offset += core::mem::size_of::<Header>();

    if header.has_special_header() {
        unsafe {
            *tls::transmute_offset_mut(tls_offset) = hipc::SpecialHeader::new(
                request_context.send_pid,
                request_context.hipc_send_handles.len() as u8,
                request_context.hipc_move_handles.len() as u8,
            );
        }

        tls_offset += core::mem::size_of::<hipc::SpecialHeader>();

        if request_context.send_pid {
            unsafe {
                *tls::transmute_offset_mut(tls_offset) = 0usize;
            }

            tls_offset += core::mem::size_of::<u64>();
        }

        copy_to_tls(tls_offset, &request_context.hipc_send_handles);
        tls_offset += core::mem::size_of::<u32>() * request_context.hipc_send_handles.len();

        copy_to_tls(tls_offset, &request_context.hipc_move_handles);
        tls_offset += core::mem::size_of::<u32>() * request_context.hipc_move_handles.len();
    }

    tls_offset = align_up(tls_offset, 4);

    copy_to_tls(tls_offset, &request_context.hipc_send_statics);
    tls_offset += core::mem::size_of::<hipc::Static>() * request_context.hipc_send_statics.len();

    copy_to_tls(tls_offset, &request_context.hipc_send_buffers);
    tls_offset += core::mem::size_of::<hipc::Buffer>() * request_context.hipc_send_buffers.len();

    copy_to_tls(tls_offset, &request_context.hipc_receive_buffers);
    tls_offset += core::mem::size_of::<hipc::Buffer>() * request_context.hipc_receive_buffers.len();

    copy_to_tls(tls_offset, &request_context.hipc_exchange_buffers);
    tls_offset +=
        core::mem::size_of::<hipc::Buffer>() * request_context.hipc_exchange_buffers.len();

    tls_offset = align_up(tls_offset, 8);

    let in_size = (core::mem::size_of::<cmif::InHeader>()
        + (request_context.data.len() * core::mem::size_of::<Req>()))
        as u16;
    let mut data_size = 16 + in_size as u16;
    if !service.is_root {
        let cmif_domain_header =
            unsafe { tls::transmute_offset_mut::<cmif::DomainInHeader>(tls_offset) };
        cmif_domain_header.set_object_count(request_context.objects.len() as u8);
        cmif_domain_header.set_data_size(in_size);
        data_size += core::mem::size_of::<cmif::DomainInHeader>() as u16;
        tls_offset += core::mem::size_of::<cmif::DomainInHeader>();

        copy_to_tls(tls_offset, &request_context.objects);
        tls_offset += core::mem::size_of::<u32>() * request_context.objects.len();
    }

    header.set_data_words(data_size / 4);

    copy_to_tls(tls_offset, &request_context.hipc_receive_statics);

    service.handle.send_sync_request()?;

    let mut tls_offset = 0;
    let mut header: &Header = unsafe { tls::transmute_offset(tls_offset) };
    tls_offset += core::mem::size_of::<Header>();

    let mut hipc_send_handles = heapless::Vec::<u32, 8>::new();
    let mut hipc_move_handles = heapless::Vec::<u32, 8>::new();
    let mut pid = None;
    if header.has_special_header() {
        let special_header: &hipc::SpecialHeader = unsafe { tls::transmute_offset(tls_offset) };
        tls_offset += core::mem::size_of::<hipc::SpecialHeader>();

        if special_header.has_pid() {
            pid = Some(unsafe { *tls::transmute_offset::<u64>(tls_offset) });
            tls_offset += core::mem::size_of::<u64>();
        }

        copy_from_tls(tls_offset, &mut hipc_send_handles);
        tls_offset += core::mem::size_of::<u32>() * special_header.copy_handle_count() as usize;

        copy_from_tls(tls_offset, &mut hipc_move_handles);
        tls_offset += core::mem::size_of::<u32>() * special_header.move_handle_count() as usize;
    }

    let mut hipc_send_statics = heapless::Vec::<hipc::Static, 8>::new();
    copy_from_tls(tls_offset, &mut hipc_send_statics);
    tls_offset += core::mem::size_of::<hipc::Static>() * header.send_static_count() as usize;

    let mut data_size = header.data_words() as usize * 4 - core::mem::size_of::<cmif::InHeader>();

    tls_offset = align_up(tls_offset, 8);
    let mut cmif_object_ids = heapless::Vec::<u32, 8>::new();
    if !service.is_root {
        let cmif_domain_header =
            unsafe { tls::transmute_offset::<cmif::DomainOutHeader>(tls_offset) };
        tls_offset += core::mem::size_of::<cmif::DomainOutHeader>();

        copy_from_tls(tls_offset, &mut cmif_object_ids);
        tls_offset += core::mem::size_of::<u32>() * cmif_domain_header.object_count() as usize;

        data_size -= core::mem::size_of::<cmif::DomainOutHeader>()
            + core::mem::size_of::<u32>() * cmif_domain_header.object_count() as usize;
    }

    let cmif_header = unsafe { tls::transmute_offset::<cmif::OutHeader>(tls_offset) };
    tls_offset += core::mem::size_of::<cmif::OutHeader>();

    if cmif_header.result().is_failure() {
        return Err(cmif_header.result());
    }

    let data = tls::slice_offset(tls_offset, data_size / core::mem::size_of::<Res>());

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
