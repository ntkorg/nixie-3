use core::{iter::zip, prelude::v1};

use zerocopy::{AsBytes, FromBytes};

use crate::{
    ipc::{cmif::*, hipc::Header},
    svc,
    util::{align::align_up, reader::ReadError, result::ResultCode, tls, writer::WriteError},
};

use self::service::Service;

use super::{
    cmif,
    hipc::{self, Mode},
};

pub mod service;

pub enum Error {
    InvalidRequest(WriteError),
    InvalidResponse(ReadError),
    RequestError(ResultCode),
    ResponseError(ResultCode),
    NoMoveHandle,
    NoSendHandle,
    NoObject,
    NoPid,
    NoStatic,
    NotEnoughData,
    InvalidData,
}

impl From<ReadError> for Error {
    fn from(error: ReadError) -> Self { Error::InvalidResponse(error) }
}

impl From<WriteError> for Error {
    fn from(error: WriteError) -> Self { Error::InvalidRequest(error) }
}

pub struct RequestContext<'a, D: Copy + AsBytes> {
    send_pid: bool,
    command: u32,
    token: u32,
    objects: heapless::Vec<u32, 8>,
    hipc_copy_handles: heapless::Vec<u32, 8>,
    hipc_move_handles: heapless::Vec<u32, 8>,
    hipc_static_index: u8,
    hipc_send_statics: heapless::Vec<hipc::Static, 8>,
    hipc_send_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_receive_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_exchange_buffers: heapless::Vec<hipc::Buffer, 8>,
    hipc_receive_statics: heapless::Vec<hipc::ReceiveStatic, 8>,
    data: &'a [D],
}

impl<'a> RequestContext<'a, ()> {
    pub fn new() -> Self {
        Self {
            send_pid: false,
            command: 0,
            token: 0,
            objects: heapless::Vec::new(),
            hipc_copy_handles: heapless::Vec::new(),
            hipc_move_handles: heapless::Vec::new(),
            hipc_static_index: 0,
            hipc_send_statics: heapless::Vec::new(),
            hipc_send_buffers: heapless::Vec::new(),
            hipc_receive_buffers: heapless::Vec::new(),
            hipc_exchange_buffers: heapless::Vec::new(),
            hipc_receive_statics: heapless::Vec::new(),
            data: &[],
        }
    }
}

impl<'a, D: Copy + AsBytes> RequestContext<'a, D> {
    pub fn new_from(data: &'a D) -> Self {
        RequestContext::new_from_slice(core::slice::from_ref(data))
    }

    pub fn new_from_slice(data: &'a [D]) -> Self {
        Self {
            send_pid: false,
            command: 0,
            token: 0,
            objects: heapless::Vec::new(),
            hipc_copy_handles: heapless::Vec::new(),
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

    pub fn set_send_pid(mut self) -> Self {
        self.send_pid = true;
        self
    }

    pub fn with_command(mut self, command: u32) -> Self {
        self.command = command;
        self
    }

    pub fn with_token(mut self, token: u32) -> Self {
        self.token = token;
        self
    }

    pub fn add_service<Srv>(mut self, service: Service<Srv>) -> Self {
        self.objects
            .push(service.handle.value)
            .map_err(drop)
            .unwrap();

        self
    }

    pub fn add_copy_handle<T>(mut self, handle: svc::Handle<T>) -> Self {
        self.hipc_copy_handles
            .push(handle.value)
            .map_err(drop)
            .unwrap();

        self
    }

    pub fn add_move_handle<T>(mut self, handle: svc::Handle<T>) -> Self {
        self.hipc_move_handles
            .push(handle.value)
            .map_err(drop)
            .unwrap();

        self
    }

    pub fn add_in_autoselect<T>(mut self, mode: Mode, data: &'a [T]) -> Self {
        self.hipc_send_statics
            .push(hipc::Static::new(self.hipc_static_index, 0, 0))
            .unwrap();
        self.hipc_send_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();
        self.hipc_static_index += 1;

        self
    }

    pub fn add_out_autoselect<T>(mut self, mode: Mode, data: &'a mut [T]) -> Self {
        self.hipc_receive_statics
            .push(hipc::ReceiveStatic::new(0, 0))
            .unwrap();
        self.hipc_receive_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();

        self
    }

    pub fn add_in_pointer<T>(mut self, data: &'a [T]) -> Self {
        self.hipc_send_statics
            .push(hipc::Static::new(
                self.hipc_static_index,
                data.len() as u16,
                data.as_ptr() as u64,
            ))
            .unwrap();
        self.hipc_static_index += 1;

        self
    }

    pub fn add_out_pointer<T>(mut self, data: &'a mut [T]) -> Self {
        self.hipc_receive_statics
            .push(hipc::ReceiveStatic::new(
                data.as_ptr() as u64,
                data.len() as u16,
            ))
            .unwrap();

        self
    }

    pub fn add_in_fixed_size_pointer<T>(mut self, mode: Mode, data: &'a [T]) -> Self {
        self.hipc_exchange_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();

        self
    }

    pub fn add_in_map_alias<T>(mut self, mode: Mode, data: &'a [T]) -> Self {
        self.hipc_send_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();

        self
    }

    pub fn add_out_map_alias<T>(mut self, mode: Mode, data: &'a mut [T]) -> Self {
        self.hipc_receive_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();

        self
    }

    pub fn add_inout_map_alias<T>(mut self, mode: Mode, data: &'a mut [T]) -> Self {
        self.hipc_exchange_buffers
            .push(hipc::Buffer::new(
                data.len() as u64,
                data.as_ptr() as u64,
                mode,
            ))
            .unwrap();

        self
    }
}

pub struct Response<'a> {
    objects: heapless::Vec<u32, 8>,
    hipc_copy_handles: heapless::Vec<u32, 8>,
    hipc_move_handles: heapless::Vec<u32, 8>,
    send_statics: heapless::Vec<hipc::Static, 8>,
    pid: Option<u64>,
    data: &'a [u8],
}

impl<'a> Response<'a> {
    pub fn new(
        objects: heapless::Vec<u32, 8>,
        hipc_copy_handles: heapless::Vec<u32, 8>,
        hipc_move_handles: heapless::Vec<u32, 8>,
        send_statics: heapless::Vec<hipc::Static, 8>,
        pid: Option<u64>,
        data: &'a [u8],
    ) -> Self {
        Self {
            objects,
            hipc_copy_handles,
            hipc_move_handles,
            send_statics,
            pid,
            data,
        }
    }

    pub fn move_handles(&self) -> &heapless::Vec<u32, 8> { &self.hipc_move_handles }

    pub fn copy_handles(&self) -> &heapless::Vec<u32, 8> { &self.hipc_copy_handles }

    pub fn objects(&self) -> &heapless::Vec<u32, 8> { &self.objects }

    pub fn statics(&self) -> &heapless::Vec<hipc::Static, 8> { &self.send_statics }

    pub fn get_copy_handle<T>(&mut self) -> Result<svc::Handle<T>, Error> {
        Ok(svc::Handle::new(
            self.hipc_copy_handles.pop().ok_or(Error::NoSendHandle)?,
        ))
    }

    pub fn get_move_handle<T>(&mut self) -> Result<svc::Handle<T>, Error> {
        Ok(svc::Handle::new(
            self.hipc_move_handles.pop().ok_or(Error::NoMoveHandle)?,
        ))
    }

    pub fn get_object(&mut self) -> Result<u32, Error> {
        Ok(self.objects.pop().ok_or(Error::NoObject)?)
    }

    pub fn get_static(&mut self) -> Result<hipc::Static, Error> {
        Ok(self.send_statics.pop().ok_or(Error::NoStatic)?)
    }

    pub fn get_pid(&mut self) -> Result<u64, Error> { Ok(self.pid.ok_or(Error::NoPid)?) }

    pub fn get_data(&self) -> &'a [u8] { self.data }

    pub fn try_get_ref<T>(&self) -> Result<&'a T, Error> {
        if self.data.len() < core::mem::size_of::<T>() {
            return Err(Error::NotEnoughData);
        }

        Ok(unsafe { &*(self.data.as_ptr() as *const T) })
    }
}

pub struct HipcSizes {
    hipc_data_size: u16,
    raw_data_size: u16,
}

fn sizes_from_request<Srv, Req: Copy + AsBytes>(
    service: &Service<Srv>,
    request_context: &RequestContext<'_, Req>,
) -> HipcSizes {
    let mut sizes = HipcSizes {
        hipc_data_size: 16,
        raw_data_size: (core::mem::size_of::<cmif::InHeader>()
            + (request_context.data.len() * core::mem::size_of::<Req>()))
            as u16,
    };

    if !service.is_root {
        sizes.hipc_data_size += core::mem::size_of::<cmif::DomainInHeader>() as u16;
        sizes.hipc_data_size +=
            core::mem::size_of::<u32>() as u16 * request_context.objects.len() as u16;
    }

    sizes.hipc_data_size += sizes.raw_data_size;

    sizes
}

pub fn invoke_service_control<Srv, Req: Copy + AsBytes, Out, ResFn>(
    service: &Service<Srv>,
    request_context: &RequestContext<'_, Req>,
    response_handler: ResFn,
) -> Result<Out, Error>
where
    ResFn: FnOnce(&RequestContext<'_, Req>, Response<'_>) -> Result<Out, Error>,
{
    write_request(&service, cmif::CommandType::Control, request_context)?;
    if request_context.token > 0 {
        svc::panic(0x34, 0, 0)
    }
    service
        .handle
        .send_sync_request()
        .map_err(|res| Error::RequestError(res))?;
    let response = read_response(service)?;
    response_handler(request_context, response)
}

pub fn invoke_service_request<Srv, Req: Copy + AsBytes, Out, ResFn>(
    service: &Service<Srv>,
    request_context: &RequestContext<'_, Req>,
    response_handler: ResFn,
) -> Result<Out, Error>
where
    ResFn: FnOnce(&RequestContext<'_, Req>, Response<'_>) -> Result<Out, Error>,
{
    write_request(&service, cmif::CommandType::Request, request_context)?;
    if request_context.token > 0 {
        svc::panic(0x34, 0, 0)
    }
    service
        .handle
        .send_sync_request()
        .map_err(|res| Error::RequestError(res))?;
    let response = read_response(service)?;
    response_handler(request_context, response)
}

fn write_request<Srv, Req: Copy + AsBytes>(
    service: &Service<Srv>,
    command_type: cmif::CommandType,
    request_context: &RequestContext<'_, Req>,
) -> Result<(), WriteError> {
    let mut writer = tls::get_writer(0, 0x100);
    let has_special_header = request_context.send_pid
        || request_context.hipc_copy_handles.len() > 0
        || request_context.hipc_move_handles.len() > 0;

    let sizes = sizes_from_request(&service, request_context);

    writer.write(
        Header::default()
            .with_request_type(command_type as u16)
            .with_data_words(align_up(sizes.hipc_data_size as usize, 4) as u16 / 4)
            .with_special_header(has_special_header)
            .with_send_static_count(request_context.hipc_send_statics.len() as u8)
            .with_send_buffer_count(request_context.hipc_send_buffers.len() as u8)
            .with_receive_buffer_count(request_context.hipc_receive_buffers.len() as u8)
            .with_exchange_buffer_count(request_context.hipc_exchange_buffers.len() as u8),
    )?;

    if has_special_header {
        writer.write(hipc::SpecialHeader::new(
            request_context.send_pid,
            request_context.hipc_copy_handles.len() as u8,
            request_context.hipc_move_handles.len() as u8,
        ))?;

        if request_context.send_pid {
            writer.write(0u64)?;
        }

        writer.write_vec(&request_context.hipc_copy_handles)?;
        writer.write_vec(&request_context.hipc_move_handles)?;
    }

    writer.write_vec(&request_context.hipc_send_statics)?;
    writer.write_vec(&request_context.hipc_send_buffers)?;
    writer.write_vec(&request_context.hipc_receive_buffers)?;
    writer.write_vec(&request_context.hipc_exchange_buffers)?;

    writer.align_to(16);

    if service.is_domain() {
        svc::panic(0x16, 0x32, 0x64);
        writer.write(
            cmif::DomainInHeader::default()
                .with_command_type(DomainCommandType::Request)
                .with_object_count(request_context.objects.len() as u8)
                .with_data_size(sizes.raw_data_size),
        )?;
    }

    writer.write(
        cmif::InHeader::default()
            .with_command(request_context.command)
            .with_token(request_context.token),
    )?;
    writer.write_vec(&request_context.data)?;
    writer.align_to(4);

    if service.is_domain() {
        writer.write_vec(&request_context.objects)?;
    }

    writer.align_to(16);

    writer.write_vec(&request_context.hipc_receive_statics)?;

    Ok(())
}

fn read_response<'a, Srv>(service: &Service<Srv>) -> Result<Response<'a>, Error> {
    let mut reader = tls::get_reader(0, 0x100);

    let header: hipc::Header = reader.read()?;

    let mut hipc_copy_handles = heapless::Vec::<u32, 8>::new();
    let mut hipc_move_handles = heapless::Vec::<u32, 8>::new();
    let mut pid = None;

    if header.has_special_header() {
        let special_header: hipc::SpecialHeader = reader.read()?;

        if special_header.has_pid() {
            pid = Some(reader.read()?);
        }

        reader.read_into_vec(
            &mut hipc_copy_handles,
            special_header.copy_handle_count() as usize,
        )?;
        reader.read_into_vec(
            &mut hipc_move_handles,
            special_header.move_handle_count() as usize,
        )?;
    }

    let hipc_send_statics = reader.read_vec(header.send_static_count() as usize)?;

    reader.align_to(16);

    let mut data_words = header.data_words() as usize;

    let mut cmif_object_ids = heapless::Vec::<u32, 8>::new();
    if service.is_domain() {
        let domain_header = reader.read::<cmif::DomainInHeader>()?;
        reader.read_into_vec(&mut cmif_object_ids, domain_header.object_count() as usize)?;

        data_words -= 4 + domain_header.object_count() as usize;
    }

    let cmif_header = reader.read::<cmif::OutHeader>()?;
    data_words -= 4;

    if cmif_header.result().is_failure() {
        return Err(Error::ResponseError(cmif_header.result()));
    }

    let data = reader.read_slice(data_words * 4)?;

    Ok(Response::new(
        cmif_object_ids,
        hipc_copy_handles,
        hipc_move_handles,
        hipc_send_statics,
        pid,
        data,
    ))
}
