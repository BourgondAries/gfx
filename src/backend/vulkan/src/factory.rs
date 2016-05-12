// Copyright 2016 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{mem, ptr, slice};
use std::os::raw::c_void;
use gfx_core::{self as core, handle as h, factory as f};
use vk;
use command;
use {Resources as R, SharePointer};


#[derive(Copy, Clone)]
pub struct RawMapping {
    pointer: *mut c_void,
}

impl core::mapping::Raw for RawMapping {
    unsafe fn set<T>(&self, index: usize, val: T) {
        *(self.pointer as *mut T).offset(index as isize) = val;
    }

    unsafe fn to_slice<T>(&self, len: usize) -> &[T] {
        slice::from_raw_parts(self.pointer as *const T, len)
    }

    unsafe fn to_mut_slice<T>(&self, len: usize) -> &mut [T] {
        slice::from_raw_parts_mut(self.pointer as *mut T, len)
    }
}

pub struct Factory {
    share: SharePointer,
    command_pool: vk::CommandPool,
}

impl Factory {
    pub fn new(share: SharePointer, qf_index: u32) -> Factory {
        let com_info = vk::CommandPoolCreateInfo {
            sType: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueFamilyIndex: qf_index,
        };
        let com_pool = unsafe {
            let (dev, vk) = share.get_device();
            let mut out = mem::zeroed();
            assert_eq!(vk::SUCCESS, vk.CreateCommandPool(dev, &com_info, ptr::null(), &mut out));
            out
        };
        Factory {
            share: share,
            command_pool: com_pool,
        }
    }

    pub fn create_command_buffer(&mut self) -> command::Buffer {
        let info = vk::CommandBufferAllocateInfo {
            sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: self.command_pool,
            level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };
        let (dev, vk) = self.share.get_device();
        unsafe {
            let mut out = mem::zeroed();
            assert_eq!(vk::SUCCESS, vk.AllocateCommandBuffers(dev, &info, &mut out));
            command::Buffer::new(out)
        }
    }
}

impl Drop for Factory {
    fn drop(&mut self) {
        let (dev, vk) = self.share.get_device();
        unsafe {
            vk.DestroyCommandPool(dev, self.command_pool, ptr::null())
        };
    }
}

impl core::Factory<R> for Factory {
    type Mapper = RawMapping;

    fn get_capabilities(&self) -> &core::Capabilities {
        unimplemented!()
    }

    fn create_buffer_raw(&mut self, _info: f::BufferInfo) -> Result<h::RawBuffer<R>, f::BufferError> {
        unimplemented!()
    }

    fn create_buffer_const_raw(&mut self, _data: &[u8], _stride: usize, _role: f::BufferRole, _bind: f::Bind)
                                -> Result<h::RawBuffer<R>, f::BufferError> {
        unimplemented!()
    }

    fn create_shader(&mut self, _stage: core::shade::Stage, _code: &[u8])
                     -> Result<h::Shader<R>, core::shade::CreateShaderError> {
        unimplemented!()
    }

    fn create_program(&mut self, _shader_set: &core::ShaderSet<R>)
                      -> Result<h::Program<R>, core::shade::CreateProgramError> {
        unimplemented!()
    }

    fn create_pipeline_state_raw(&mut self, _program: &h::Program<R>, _desc: &core::pso::Descriptor)
                                 -> Result<h::RawPipelineState<R>, core::pso::CreationError> {
        unimplemented!()
    }

    fn create_texture_raw(&mut self, _desc: core::tex::Descriptor, _hint: Option<core::format::ChannelType>,
                          _data_opt: Option<&[&[u8]]>) -> Result<h::RawTexture<R>, core::tex::Error> {
        unimplemented!()
    }

    fn view_buffer_as_shader_resource_raw(&mut self, _hbuf: &h::RawBuffer<R>)
                                      -> Result<h::RawShaderResourceView<R>, f::ResourceViewError> {
        Err(f::ResourceViewError::Unsupported) //TODO
    }

    fn view_buffer_as_unordered_access_raw(&mut self, _hbuf: &h::RawBuffer<R>)
                                       -> Result<h::RawUnorderedAccessView<R>, f::ResourceViewError> {
        Err(f::ResourceViewError::Unsupported) //TODO
    }

    fn view_texture_as_shader_resource_raw(&mut self, _htex: &h::RawTexture<R>, _desc: core::tex::ResourceDesc)
                                       -> Result<h::RawShaderResourceView<R>, f::ResourceViewError> {
        unimplemented!()
    }

    fn view_texture_as_unordered_access_raw(&mut self, _htex: &h::RawTexture<R>)
                                        -> Result<h::RawUnorderedAccessView<R>, f::ResourceViewError> {
        Err(f::ResourceViewError::Unsupported) //TODO
    }

    fn view_texture_as_render_target_raw(&mut self, _htex: &h::RawTexture<R>, _desc: core::tex::RenderDesc)
                                         -> Result<h::RawRenderTargetView<R>, f::TargetViewError>
    {
        unimplemented!()
    }

    fn view_texture_as_depth_stencil_raw(&mut self, _htex: &h::RawTexture<R>, _desc: core::tex::DepthStencilDesc)
                                         -> Result<h::RawDepthStencilView<R>, f::TargetViewError>
    {
        unimplemented!()
    }

    fn create_sampler(&mut self, _info: core::tex::SamplerInfo) -> h::Sampler<R> {
        unimplemented!()
    }

    fn map_buffer_raw(&mut self, _buf: &h::RawBuffer<R>, _access: f::MapAccess) -> RawMapping {
        unimplemented!()
    }

    fn unmap_buffer_raw(&mut self, _map: RawMapping) {
        unimplemented!()
    }

    fn map_buffer_readable<T: Copy>(&mut self, _buf: &h::Buffer<R, T>)
                           -> core::mapping::Readable<T, R, Factory> {
        unimplemented!()
    }

    fn map_buffer_writable<T: Copy>(&mut self, _buf: &h::Buffer<R, T>)
                                    -> core::mapping::Writable<T, R, Factory> {
        unimplemented!()
    }

    fn map_buffer_rw<T: Copy>(&mut self, _buf: &h::Buffer<R, T>)
                              -> core::mapping::RW<T, R, Factory> {
        unimplemented!()
    }
}