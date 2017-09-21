// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Action {
    // message fields
    pub command_digest: ::protobuf::SingularPtrField<Digest>,
    pub input_root_digest: ::protobuf::SingularPtrField<Digest>,
    pub output_files: ::protobuf::RepeatedField<::std::string::String>,
    pub output_directories: ::protobuf::RepeatedField<::std::string::String>,
    pub platform: ::protobuf::SingularPtrField<Platform>,
    pub timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub do_not_cache: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Action {}

impl Action {
    pub fn new() -> Action {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Action {
        static mut instance: ::protobuf::lazy::Lazy<Action> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Action,
        };
        unsafe {
            instance.get(Action::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.Digest command_digest = 1;

    pub fn clear_command_digest(&mut self) {
        self.command_digest.clear();
    }

    pub fn has_command_digest(&self) -> bool {
        self.command_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command_digest(&mut self, v: Digest) {
        self.command_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command_digest(&mut self) -> &mut Digest {
        if self.command_digest.is_none() {
            self.command_digest.set_default();
        }
        self.command_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_command_digest(&mut self) -> Digest {
        self.command_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_command_digest(&self) -> &Digest {
        self.command_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_command_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.command_digest
    }

    fn mut_command_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.command_digest
    }

    // .google.devtools.remoteexecution.v1test.Digest input_root_digest = 2;

    pub fn clear_input_root_digest(&mut self) {
        self.input_root_digest.clear();
    }

    pub fn has_input_root_digest(&self) -> bool {
        self.input_root_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_root_digest(&mut self, v: Digest) {
        self.input_root_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_root_digest(&mut self) -> &mut Digest {
        if self.input_root_digest.is_none() {
            self.input_root_digest.set_default();
        }
        self.input_root_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_input_root_digest(&mut self) -> Digest {
        self.input_root_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_input_root_digest(&self) -> &Digest {
        self.input_root_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_input_root_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.input_root_digest
    }

    fn mut_input_root_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.input_root_digest
    }

    // repeated string output_files = 3;

    pub fn clear_output_files(&mut self) {
        self.output_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_files(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.output_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_files(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.output_files
    }

    // Take field
    pub fn take_output_files(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.output_files, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_files(&self) -> &[::std::string::String] {
        &self.output_files
    }

    fn get_output_files_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.output_files
    }

    fn mut_output_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.output_files
    }

    // repeated string output_directories = 4;

    pub fn clear_output_directories(&mut self) {
        self.output_directories.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_directories(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.output_directories = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_directories(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.output_directories
    }

    // Take field
    pub fn take_output_directories(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.output_directories, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_directories(&self) -> &[::std::string::String] {
        &self.output_directories
    }

    fn get_output_directories_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.output_directories
    }

    fn mut_output_directories_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.output_directories
    }

    // .google.devtools.remoteexecution.v1test.Platform platform = 5;

    pub fn clear_platform(&mut self) {
        self.platform.clear();
    }

    pub fn has_platform(&self) -> bool {
        self.platform.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: Platform) {
        self.platform = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_platform(&mut self) -> &mut Platform {
        if self.platform.is_none() {
            self.platform.set_default();
        }
        self.platform.as_mut().unwrap()
    }

    // Take field
    pub fn take_platform(&mut self) -> Platform {
        self.platform.take().unwrap_or_else(|| Platform::new())
    }

    pub fn get_platform(&self) -> &Platform {
        self.platform.as_ref().unwrap_or_else(|| Platform::default_instance())
    }

    fn get_platform_for_reflect(&self) -> &::protobuf::SingularPtrField<Platform> {
        &self.platform
    }

    fn mut_platform_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Platform> {
        &mut self.platform
    }

    // .google.protobuf.Duration timeout = 6;

    pub fn clear_timeout(&mut self) {
        self.timeout.clear();
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.timeout.is_none() {
            self.timeout.set_default();
        }
        self.timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.timeout
    }

    // bool do_not_cache = 7;

    pub fn clear_do_not_cache(&mut self) {
        self.do_not_cache = false;
    }

    // Param is passed by value, moved
    pub fn set_do_not_cache(&mut self, v: bool) {
        self.do_not_cache = v;
    }

    pub fn get_do_not_cache(&self) -> bool {
        self.do_not_cache
    }

    fn get_do_not_cache_for_reflect(&self) -> &bool {
        &self.do_not_cache
    }

    fn mut_do_not_cache_for_reflect(&mut self) -> &mut bool {
        &mut self.do_not_cache
    }
}

impl ::protobuf::Message for Action {
    fn is_initialized(&self) -> bool {
        for v in &self.command_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.input_root_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.platform {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.command_digest)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.input_root_digest)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.output_files)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.output_directories)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.platform)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timeout)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.do_not_cache = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.command_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.input_root_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.output_files {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.output_directories {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.platform.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.do_not_cache != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.command_digest.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.input_root_digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.output_files {
            os.write_string(3, &v)?;
        };
        for v in &self.output_directories {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.platform.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.timeout.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.do_not_cache != false {
            os.write_bool(7, self.do_not_cache)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Action {
    fn new() -> Action {
        Action::new()
    }

    fn descriptor_static(_: ::std::option::Option<Action>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "command_digest",
                    Action::get_command_digest_for_reflect,
                    Action::mut_command_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "input_root_digest",
                    Action::get_input_root_digest_for_reflect,
                    Action::mut_input_root_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "output_files",
                    Action::get_output_files_for_reflect,
                    Action::mut_output_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "output_directories",
                    Action::get_output_directories_for_reflect,
                    Action::mut_output_directories_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Platform>>(
                    "platform",
                    Action::get_platform_for_reflect,
                    Action::mut_platform_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "timeout",
                    Action::get_timeout_for_reflect,
                    Action::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "do_not_cache",
                    Action::get_do_not_cache_for_reflect,
                    Action::mut_do_not_cache_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Action>(
                    "Action",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Action {
    fn clear(&mut self) {
        self.clear_command_digest();
        self.clear_input_root_digest();
        self.clear_output_files();
        self.clear_output_directories();
        self.clear_platform();
        self.clear_timeout();
        self.clear_do_not_cache();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Command {
    // message fields
    pub arguments: ::protobuf::RepeatedField<::std::string::String>,
    pub environment_variables: ::protobuf::RepeatedField<Command_EnvironmentVariable>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Command {}

impl Command {
    pub fn new() -> Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Command {
        static mut instance: ::protobuf::lazy::Lazy<Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Command,
        };
        unsafe {
            instance.get(Command::new)
        }
    }

    // repeated string arguments = 1;

    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.arguments
    }

    // Take field
    pub fn take_arguments(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.arguments, ::protobuf::RepeatedField::new())
    }

    pub fn get_arguments(&self) -> &[::std::string::String] {
        &self.arguments
    }

    fn get_arguments_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.arguments
    }

    fn mut_arguments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.arguments
    }

    // repeated .google.devtools.remoteexecution.v1test.Command.EnvironmentVariable environment_variables = 2;

    pub fn clear_environment_variables(&mut self) {
        self.environment_variables.clear();
    }

    // Param is passed by value, moved
    pub fn set_environment_variables(&mut self, v: ::protobuf::RepeatedField<Command_EnvironmentVariable>) {
        self.environment_variables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_environment_variables(&mut self) -> &mut ::protobuf::RepeatedField<Command_EnvironmentVariable> {
        &mut self.environment_variables
    }

    // Take field
    pub fn take_environment_variables(&mut self) -> ::protobuf::RepeatedField<Command_EnvironmentVariable> {
        ::std::mem::replace(&mut self.environment_variables, ::protobuf::RepeatedField::new())
    }

    pub fn get_environment_variables(&self) -> &[Command_EnvironmentVariable] {
        &self.environment_variables
    }

    fn get_environment_variables_for_reflect(&self) -> &::protobuf::RepeatedField<Command_EnvironmentVariable> {
        &self.environment_variables
    }

    fn mut_environment_variables_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Command_EnvironmentVariable> {
        &mut self.environment_variables
    }
}

impl ::protobuf::Message for Command {
    fn is_initialized(&self) -> bool {
        for v in &self.environment_variables {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.arguments)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.environment_variables)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.arguments {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.environment_variables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.arguments {
            os.write_string(1, &v)?;
        };
        for v in &self.environment_variables {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Command {
    fn new() -> Command {
        Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "arguments",
                    Command::get_arguments_for_reflect,
                    Command::mut_arguments_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Command_EnvironmentVariable>>(
                    "environment_variables",
                    Command::get_environment_variables_for_reflect,
                    Command::mut_environment_variables_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Command>(
                    "Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Command {
    fn clear(&mut self) {
        self.clear_arguments();
        self.clear_environment_variables();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Command {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Command_EnvironmentVariable {
    // message fields
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Command_EnvironmentVariable {}

impl Command_EnvironmentVariable {
    pub fn new() -> Command_EnvironmentVariable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Command_EnvironmentVariable {
        static mut instance: ::protobuf::lazy::Lazy<Command_EnvironmentVariable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Command_EnvironmentVariable,
        };
        unsafe {
            instance.get(Command_EnvironmentVariable::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for Command_EnvironmentVariable {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Command_EnvironmentVariable {
    fn new() -> Command_EnvironmentVariable {
        Command_EnvironmentVariable::new()
    }

    fn descriptor_static(_: ::std::option::Option<Command_EnvironmentVariable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Command_EnvironmentVariable::get_name_for_reflect,
                    Command_EnvironmentVariable::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    Command_EnvironmentVariable::get_value_for_reflect,
                    Command_EnvironmentVariable::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Command_EnvironmentVariable>(
                    "Command_EnvironmentVariable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Command_EnvironmentVariable {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Command_EnvironmentVariable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Command_EnvironmentVariable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Platform {
    // message fields
    pub properties: ::protobuf::RepeatedField<Platform_Property>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Platform {}

impl Platform {
    pub fn new() -> Platform {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Platform {
        static mut instance: ::protobuf::lazy::Lazy<Platform> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Platform,
        };
        unsafe {
            instance.get(Platform::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.Platform.Property properties = 1;

    pub fn clear_properties(&mut self) {
        self.properties.clear();
    }

    // Param is passed by value, moved
    pub fn set_properties(&mut self, v: ::protobuf::RepeatedField<Platform_Property>) {
        self.properties = v;
    }

    // Mutable pointer to the field.
    pub fn mut_properties(&mut self) -> &mut ::protobuf::RepeatedField<Platform_Property> {
        &mut self.properties
    }

    // Take field
    pub fn take_properties(&mut self) -> ::protobuf::RepeatedField<Platform_Property> {
        ::std::mem::replace(&mut self.properties, ::protobuf::RepeatedField::new())
    }

    pub fn get_properties(&self) -> &[Platform_Property] {
        &self.properties
    }

    fn get_properties_for_reflect(&self) -> &::protobuf::RepeatedField<Platform_Property> {
        &self.properties
    }

    fn mut_properties_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Platform_Property> {
        &mut self.properties
    }
}

impl ::protobuf::Message for Platform {
    fn is_initialized(&self) -> bool {
        for v in &self.properties {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.properties)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.properties {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.properties {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Platform {
    fn new() -> Platform {
        Platform::new()
    }

    fn descriptor_static(_: ::std::option::Option<Platform>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Platform_Property>>(
                    "properties",
                    Platform::get_properties_for_reflect,
                    Platform::mut_properties_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Platform>(
                    "Platform",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Platform {
    fn clear(&mut self) {
        self.clear_properties();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Platform {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Platform_Property {
    // message fields
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Platform_Property {}

impl Platform_Property {
    pub fn new() -> Platform_Property {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Platform_Property {
        static mut instance: ::protobuf::lazy::Lazy<Platform_Property> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Platform_Property,
        };
        unsafe {
            instance.get(Platform_Property::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for Platform_Property {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Platform_Property {
    fn new() -> Platform_Property {
        Platform_Property::new()
    }

    fn descriptor_static(_: ::std::option::Option<Platform_Property>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Platform_Property::get_name_for_reflect,
                    Platform_Property::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    Platform_Property::get_value_for_reflect,
                    Platform_Property::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Platform_Property>(
                    "Platform_Property",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Platform_Property {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Platform_Property {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Platform_Property {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Directory {
    // message fields
    pub files: ::protobuf::RepeatedField<FileNode>,
    pub directories: ::protobuf::RepeatedField<DirectoryNode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Directory {}

impl Directory {
    pub fn new() -> Directory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Directory {
        static mut instance: ::protobuf::lazy::Lazy<Directory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Directory,
        };
        unsafe {
            instance.get(Directory::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.FileNode files = 1;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<FileNode>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<FileNode> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<FileNode> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[FileNode] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<FileNode> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileNode> {
        &mut self.files
    }

    // repeated .google.devtools.remoteexecution.v1test.DirectoryNode directories = 2;

    pub fn clear_directories(&mut self) {
        self.directories.clear();
    }

    // Param is passed by value, moved
    pub fn set_directories(&mut self, v: ::protobuf::RepeatedField<DirectoryNode>) {
        self.directories = v;
    }

    // Mutable pointer to the field.
    pub fn mut_directories(&mut self) -> &mut ::protobuf::RepeatedField<DirectoryNode> {
        &mut self.directories
    }

    // Take field
    pub fn take_directories(&mut self) -> ::protobuf::RepeatedField<DirectoryNode> {
        ::std::mem::replace(&mut self.directories, ::protobuf::RepeatedField::new())
    }

    pub fn get_directories(&self) -> &[DirectoryNode] {
        &self.directories
    }

    fn get_directories_for_reflect(&self) -> &::protobuf::RepeatedField<DirectoryNode> {
        &self.directories
    }

    fn mut_directories_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DirectoryNode> {
        &mut self.directories
    }
}

impl ::protobuf::Message for Directory {
    fn is_initialized(&self) -> bool {
        for v in &self.files {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.directories {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.files)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.directories)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.directories {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.files {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.directories {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Directory {
    fn new() -> Directory {
        Directory::new()
    }

    fn descriptor_static(_: ::std::option::Option<Directory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileNode>>(
                    "files",
                    Directory::get_files_for_reflect,
                    Directory::mut_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DirectoryNode>>(
                    "directories",
                    Directory::get_directories_for_reflect,
                    Directory::mut_directories_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Directory>(
                    "Directory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Directory {
    fn clear(&mut self) {
        self.clear_files();
        self.clear_directories();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Directory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Directory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileNode {
    // message fields
    pub name: ::std::string::String,
    pub digest: ::protobuf::SingularPtrField<Digest>,
    pub is_executable: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileNode {}

impl FileNode {
    pub fn new() -> FileNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileNode {
        static mut instance: ::protobuf::lazy::Lazy<FileNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileNode,
        };
        unsafe {
            instance.get(FileNode::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .google.devtools.remoteexecution.v1test.Digest digest = 2;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: Digest) {
        self.digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut Digest {
        if self.digest.is_none() {
            self.digest.set_default();
        }
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> Digest {
        self.digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_digest(&self) -> &Digest {
        self.digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.digest
    }

    // bool is_executable = 4;

    pub fn clear_is_executable(&mut self) {
        self.is_executable = false;
    }

    // Param is passed by value, moved
    pub fn set_is_executable(&mut self, v: bool) {
        self.is_executable = v;
    }

    pub fn get_is_executable(&self) -> bool {
        self.is_executable
    }

    fn get_is_executable_for_reflect(&self) -> &bool {
        &self.is_executable
    }

    fn mut_is_executable_for_reflect(&mut self) -> &mut bool {
        &mut self.is_executable
    }
}

impl ::protobuf::Message for FileNode {
    fn is_initialized(&self) -> bool {
        for v in &self.digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.digest)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_executable = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.is_executable != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.is_executable != false {
            os.write_bool(4, self.is_executable)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileNode {
    fn new() -> FileNode {
        FileNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FileNode::get_name_for_reflect,
                    FileNode::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "digest",
                    FileNode::get_digest_for_reflect,
                    FileNode::mut_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_executable",
                    FileNode::get_is_executable_for_reflect,
                    FileNode::mut_is_executable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileNode>(
                    "FileNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileNode {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_digest();
        self.clear_is_executable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DirectoryNode {
    // message fields
    pub name: ::std::string::String,
    pub digest: ::protobuf::SingularPtrField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirectoryNode {}

impl DirectoryNode {
    pub fn new() -> DirectoryNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirectoryNode {
        static mut instance: ::protobuf::lazy::Lazy<DirectoryNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirectoryNode,
        };
        unsafe {
            instance.get(DirectoryNode::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .google.devtools.remoteexecution.v1test.Digest digest = 2;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: Digest) {
        self.digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut Digest {
        if self.digest.is_none() {
            self.digest.set_default();
        }
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> Digest {
        self.digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_digest(&self) -> &Digest {
        self.digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.digest
    }
}

impl ::protobuf::Message for DirectoryNode {
    fn is_initialized(&self) -> bool {
        for v in &self.digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.digest)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DirectoryNode {
    fn new() -> DirectoryNode {
        DirectoryNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirectoryNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DirectoryNode::get_name_for_reflect,
                    DirectoryNode::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "digest",
                    DirectoryNode::get_digest_for_reflect,
                    DirectoryNode::mut_digest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirectoryNode>(
                    "DirectoryNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirectoryNode {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_digest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirectoryNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirectoryNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Digest {
    // message fields
    pub hash: ::std::string::String,
    pub size_bytes: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Digest {}

impl Digest {
    pub fn new() -> Digest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Digest {
        static mut instance: ::protobuf::lazy::Lazy<Digest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Digest,
        };
        unsafe {
            instance.get(Digest::new)
        }
    }

    // string hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hash, ::std::string::String::new())
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::string::String {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // int64 size_bytes = 2;

    pub fn clear_size_bytes(&mut self) {
        self.size_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_size_bytes(&mut self, v: i64) {
        self.size_bytes = v;
    }

    pub fn get_size_bytes(&self) -> i64 {
        self.size_bytes
    }

    fn get_size_bytes_for_reflect(&self) -> &i64 {
        &self.size_bytes
    }

    fn mut_size_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.size_bytes
    }
}

impl ::protobuf::Message for Digest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.size_bytes = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.hash);
        }
        if self.size_bytes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_string(1, &self.hash)?;
        }
        if self.size_bytes != 0 {
            os.write_int64(2, self.size_bytes)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Digest {
    fn new() -> Digest {
        Digest::new()
    }

    fn descriptor_static(_: ::std::option::Option<Digest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hash",
                    Digest::get_hash_for_reflect,
                    Digest::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "size_bytes",
                    Digest::get_size_bytes_for_reflect,
                    Digest::mut_size_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Digest>(
                    "Digest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Digest {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_size_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Digest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Digest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionResult {
    // message fields
    pub output_files: ::protobuf::RepeatedField<OutputFile>,
    pub output_directories: ::protobuf::RepeatedField<OutputDirectory>,
    pub exit_code: i32,
    pub stdout_raw: ::std::vec::Vec<u8>,
    pub stdout_digest: ::protobuf::SingularPtrField<Digest>,
    pub stderr_raw: ::std::vec::Vec<u8>,
    pub stderr_digest: ::protobuf::SingularPtrField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionResult {}

impl ActionResult {
    pub fn new() -> ActionResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionResult {
        static mut instance: ::protobuf::lazy::Lazy<ActionResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionResult,
        };
        unsafe {
            instance.get(ActionResult::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.OutputFile output_files = 2;

    pub fn clear_output_files(&mut self) {
        self.output_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_files(&mut self, v: ::protobuf::RepeatedField<OutputFile>) {
        self.output_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_files(&mut self) -> &mut ::protobuf::RepeatedField<OutputFile> {
        &mut self.output_files
    }

    // Take field
    pub fn take_output_files(&mut self) -> ::protobuf::RepeatedField<OutputFile> {
        ::std::mem::replace(&mut self.output_files, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_files(&self) -> &[OutputFile] {
        &self.output_files
    }

    fn get_output_files_for_reflect(&self) -> &::protobuf::RepeatedField<OutputFile> {
        &self.output_files
    }

    fn mut_output_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OutputFile> {
        &mut self.output_files
    }

    // repeated .google.devtools.remoteexecution.v1test.OutputDirectory output_directories = 3;

    pub fn clear_output_directories(&mut self) {
        self.output_directories.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_directories(&mut self, v: ::protobuf::RepeatedField<OutputDirectory>) {
        self.output_directories = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_directories(&mut self) -> &mut ::protobuf::RepeatedField<OutputDirectory> {
        &mut self.output_directories
    }

    // Take field
    pub fn take_output_directories(&mut self) -> ::protobuf::RepeatedField<OutputDirectory> {
        ::std::mem::replace(&mut self.output_directories, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_directories(&self) -> &[OutputDirectory] {
        &self.output_directories
    }

    fn get_output_directories_for_reflect(&self) -> &::protobuf::RepeatedField<OutputDirectory> {
        &self.output_directories
    }

    fn mut_output_directories_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OutputDirectory> {
        &mut self.output_directories
    }

    // int32 exit_code = 4;

    pub fn clear_exit_code(&mut self) {
        self.exit_code = 0;
    }

    // Param is passed by value, moved
    pub fn set_exit_code(&mut self, v: i32) {
        self.exit_code = v;
    }

    pub fn get_exit_code(&self) -> i32 {
        self.exit_code
    }

    fn get_exit_code_for_reflect(&self) -> &i32 {
        &self.exit_code
    }

    fn mut_exit_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.exit_code
    }

    // bytes stdout_raw = 5;

    pub fn clear_stdout_raw(&mut self) {
        self.stdout_raw.clear();
    }

    // Param is passed by value, moved
    pub fn set_stdout_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.stdout_raw = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stdout_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stdout_raw
    }

    // Take field
    pub fn take_stdout_raw(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.stdout_raw, ::std::vec::Vec::new())
    }

    pub fn get_stdout_raw(&self) -> &[u8] {
        &self.stdout_raw
    }

    fn get_stdout_raw_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.stdout_raw
    }

    fn mut_stdout_raw_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stdout_raw
    }

    // .google.devtools.remoteexecution.v1test.Digest stdout_digest = 6;

    pub fn clear_stdout_digest(&mut self) {
        self.stdout_digest.clear();
    }

    pub fn has_stdout_digest(&self) -> bool {
        self.stdout_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stdout_digest(&mut self, v: Digest) {
        self.stdout_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stdout_digest(&mut self) -> &mut Digest {
        if self.stdout_digest.is_none() {
            self.stdout_digest.set_default();
        }
        self.stdout_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stdout_digest(&mut self) -> Digest {
        self.stdout_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_stdout_digest(&self) -> &Digest {
        self.stdout_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_stdout_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.stdout_digest
    }

    fn mut_stdout_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.stdout_digest
    }

    // bytes stderr_raw = 7;

    pub fn clear_stderr_raw(&mut self) {
        self.stderr_raw.clear();
    }

    // Param is passed by value, moved
    pub fn set_stderr_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.stderr_raw = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stderr_raw(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stderr_raw
    }

    // Take field
    pub fn take_stderr_raw(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.stderr_raw, ::std::vec::Vec::new())
    }

    pub fn get_stderr_raw(&self) -> &[u8] {
        &self.stderr_raw
    }

    fn get_stderr_raw_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.stderr_raw
    }

    fn mut_stderr_raw_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stderr_raw
    }

    // .google.devtools.remoteexecution.v1test.Digest stderr_digest = 8;

    pub fn clear_stderr_digest(&mut self) {
        self.stderr_digest.clear();
    }

    pub fn has_stderr_digest(&self) -> bool {
        self.stderr_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stderr_digest(&mut self, v: Digest) {
        self.stderr_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stderr_digest(&mut self) -> &mut Digest {
        if self.stderr_digest.is_none() {
            self.stderr_digest.set_default();
        }
        self.stderr_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stderr_digest(&mut self) -> Digest {
        self.stderr_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_stderr_digest(&self) -> &Digest {
        self.stderr_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_stderr_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.stderr_digest
    }

    fn mut_stderr_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.stderr_digest
    }
}

impl ::protobuf::Message for ActionResult {
    fn is_initialized(&self) -> bool {
        for v in &self.output_files {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output_directories {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stdout_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stderr_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_files)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_directories)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.exit_code = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.stdout_raw)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stdout_digest)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.stderr_raw)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stderr_digest)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.output_files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_directories {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.exit_code != 0 {
            my_size += ::protobuf::rt::value_size(4, self.exit_code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.stdout_raw.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.stdout_raw);
        }
        if let Some(ref v) = self.stdout_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.stderr_raw.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.stderr_raw);
        }
        if let Some(ref v) = self.stderr_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.output_files {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_directories {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.exit_code != 0 {
            os.write_int32(4, self.exit_code)?;
        }
        if !self.stdout_raw.is_empty() {
            os.write_bytes(5, &self.stdout_raw)?;
        }
        if let Some(ref v) = self.stdout_digest.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.stderr_raw.is_empty() {
            os.write_bytes(7, &self.stderr_raw)?;
        }
        if let Some(ref v) = self.stderr_digest.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ActionResult {
    fn new() -> ActionResult {
        ActionResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputFile>>(
                    "output_files",
                    ActionResult::get_output_files_for_reflect,
                    ActionResult::mut_output_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputDirectory>>(
                    "output_directories",
                    ActionResult::get_output_directories_for_reflect,
                    ActionResult::mut_output_directories_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "exit_code",
                    ActionResult::get_exit_code_for_reflect,
                    ActionResult::mut_exit_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stdout_raw",
                    ActionResult::get_stdout_raw_for_reflect,
                    ActionResult::mut_stdout_raw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "stdout_digest",
                    ActionResult::get_stdout_digest_for_reflect,
                    ActionResult::mut_stdout_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stderr_raw",
                    ActionResult::get_stderr_raw_for_reflect,
                    ActionResult::mut_stderr_raw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "stderr_digest",
                    ActionResult::get_stderr_digest_for_reflect,
                    ActionResult::mut_stderr_digest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionResult>(
                    "ActionResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionResult {
    fn clear(&mut self) {
        self.clear_output_files();
        self.clear_output_directories();
        self.clear_exit_code();
        self.clear_stdout_raw();
        self.clear_stdout_digest();
        self.clear_stderr_raw();
        self.clear_stderr_digest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputFile {
    // message fields
    pub path: ::std::string::String,
    pub digest: ::protobuf::SingularPtrField<Digest>,
    pub content: ::std::vec::Vec<u8>,
    pub is_executable: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputFile {}

impl OutputFile {
    pub fn new() -> OutputFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputFile {
        static mut instance: ::protobuf::lazy::Lazy<OutputFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputFile,
        };
        unsafe {
            instance.get(OutputFile::new)
        }
    }

    // string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // .google.devtools.remoteexecution.v1test.Digest digest = 2;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: Digest) {
        self.digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut Digest {
        if self.digest.is_none() {
            self.digest.set_default();
        }
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> Digest {
        self.digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_digest(&self) -> &Digest {
        self.digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.digest
    }

    // bytes content = 3;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // bool is_executable = 4;

    pub fn clear_is_executable(&mut self) {
        self.is_executable = false;
    }

    // Param is passed by value, moved
    pub fn set_is_executable(&mut self, v: bool) {
        self.is_executable = v;
    }

    pub fn get_is_executable(&self) -> bool {
        self.is_executable
    }

    fn get_is_executable_for_reflect(&self) -> &bool {
        &self.is_executable
    }

    fn mut_is_executable_for_reflect(&mut self) -> &mut bool {
        &mut self.is_executable
    }
}

impl ::protobuf::Message for OutputFile {
    fn is_initialized(&self) -> bool {
        for v in &self.digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.digest)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_executable = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        if let Some(ref v) = self.digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.content);
        }
        if self.is_executable != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        }
        if let Some(ref v) = self.digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.content.is_empty() {
            os.write_bytes(3, &self.content)?;
        }
        if self.is_executable != false {
            os.write_bool(4, self.is_executable)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OutputFile {
    fn new() -> OutputFile {
        OutputFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    OutputFile::get_path_for_reflect,
                    OutputFile::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "digest",
                    OutputFile::get_digest_for_reflect,
                    OutputFile::mut_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    OutputFile::get_content_for_reflect,
                    OutputFile::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_executable",
                    OutputFile::get_is_executable_for_reflect,
                    OutputFile::mut_is_executable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputFile>(
                    "OutputFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputFile {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_digest();
        self.clear_content();
        self.clear_is_executable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tree {
    // message fields
    pub root: ::protobuf::SingularPtrField<Directory>,
    pub children: ::protobuf::RepeatedField<Directory>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tree {}

impl Tree {
    pub fn new() -> Tree {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tree {
        static mut instance: ::protobuf::lazy::Lazy<Tree> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tree,
        };
        unsafe {
            instance.get(Tree::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.Directory root = 1;

    pub fn clear_root(&mut self) {
        self.root.clear();
    }

    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: Directory) {
        self.root = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root(&mut self) -> &mut Directory {
        if self.root.is_none() {
            self.root.set_default();
        }
        self.root.as_mut().unwrap()
    }

    // Take field
    pub fn take_root(&mut self) -> Directory {
        self.root.take().unwrap_or_else(|| Directory::new())
    }

    pub fn get_root(&self) -> &Directory {
        self.root.as_ref().unwrap_or_else(|| Directory::default_instance())
    }

    fn get_root_for_reflect(&self) -> &::protobuf::SingularPtrField<Directory> {
        &self.root
    }

    fn mut_root_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Directory> {
        &mut self.root
    }

    // repeated .google.devtools.remoteexecution.v1test.Directory children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Directory>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Directory> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Directory> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Directory] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<Directory> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Directory> {
        &mut self.children
    }
}

impl ::protobuf::Message for Tree {
    fn is_initialized(&self) -> bool {
        for v in &self.root {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.root)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.root.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.root.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.children {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tree {
    fn new() -> Tree {
        Tree::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tree>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Directory>>(
                    "root",
                    Tree::get_root_for_reflect,
                    Tree::mut_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Directory>>(
                    "children",
                    Tree::get_children_for_reflect,
                    Tree::mut_children_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tree>(
                    "Tree",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tree {
    fn clear(&mut self) {
        self.clear_root();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tree {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputDirectory {
    // message fields
    pub path: ::std::string::String,
    pub digest: ::protobuf::SingularPtrField<Digest>,
    pub tree_digest: ::protobuf::SingularPtrField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputDirectory {}

impl OutputDirectory {
    pub fn new() -> OutputDirectory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputDirectory {
        static mut instance: ::protobuf::lazy::Lazy<OutputDirectory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputDirectory,
        };
        unsafe {
            instance.get(OutputDirectory::new)
        }
    }

    // string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // .google.devtools.remoteexecution.v1test.Digest digest = 2;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: Digest) {
        self.digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut Digest {
        if self.digest.is_none() {
            self.digest.set_default();
        }
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> Digest {
        self.digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_digest(&self) -> &Digest {
        self.digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.digest
    }

    // .google.devtools.remoteexecution.v1test.Digest tree_digest = 3;

    pub fn clear_tree_digest(&mut self) {
        self.tree_digest.clear();
    }

    pub fn has_tree_digest(&self) -> bool {
        self.tree_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tree_digest(&mut self, v: Digest) {
        self.tree_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_digest(&mut self) -> &mut Digest {
        if self.tree_digest.is_none() {
            self.tree_digest.set_default();
        }
        self.tree_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_tree_digest(&mut self) -> Digest {
        self.tree_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_tree_digest(&self) -> &Digest {
        self.tree_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_tree_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.tree_digest
    }

    fn mut_tree_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.tree_digest
    }
}

impl ::protobuf::Message for OutputDirectory {
    fn is_initialized(&self) -> bool {
        for v in &self.digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tree_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.digest)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tree_digest)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        if let Some(ref v) = self.digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.tree_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        }
        if let Some(ref v) = self.digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.tree_digest.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OutputDirectory {
    fn new() -> OutputDirectory {
        OutputDirectory::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputDirectory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    OutputDirectory::get_path_for_reflect,
                    OutputDirectory::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "digest",
                    OutputDirectory::get_digest_for_reflect,
                    OutputDirectory::mut_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "tree_digest",
                    OutputDirectory::get_tree_digest_for_reflect,
                    OutputDirectory::mut_tree_digest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputDirectory>(
                    "OutputDirectory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputDirectory {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_digest();
        self.clear_tree_digest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputDirectory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputDirectory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecuteRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub action: ::protobuf::SingularPtrField<Action>,
    pub skip_cache_lookup: bool,
    pub total_input_file_count: i32,
    pub total_input_file_bytes: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecuteRequest {}

impl ExecuteRequest {
    pub fn new() -> ExecuteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecuteRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExecuteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecuteRequest,
        };
        unsafe {
            instance.get(ExecuteRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // .google.devtools.remoteexecution.v1test.Action action = 2;

    pub fn clear_action(&mut self) {
        self.action.clear();
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: Action) {
        self.action = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action(&mut self) -> &mut Action {
        if self.action.is_none() {
            self.action.set_default();
        }
        self.action.as_mut().unwrap()
    }

    // Take field
    pub fn take_action(&mut self) -> Action {
        self.action.take().unwrap_or_else(|| Action::new())
    }

    pub fn get_action(&self) -> &Action {
        self.action.as_ref().unwrap_or_else(|| Action::default_instance())
    }

    fn get_action_for_reflect(&self) -> &::protobuf::SingularPtrField<Action> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Action> {
        &mut self.action
    }

    // bool skip_cache_lookup = 3;

    pub fn clear_skip_cache_lookup(&mut self) {
        self.skip_cache_lookup = false;
    }

    // Param is passed by value, moved
    pub fn set_skip_cache_lookup(&mut self, v: bool) {
        self.skip_cache_lookup = v;
    }

    pub fn get_skip_cache_lookup(&self) -> bool {
        self.skip_cache_lookup
    }

    fn get_skip_cache_lookup_for_reflect(&self) -> &bool {
        &self.skip_cache_lookup
    }

    fn mut_skip_cache_lookup_for_reflect(&mut self) -> &mut bool {
        &mut self.skip_cache_lookup
    }

    // int32 total_input_file_count = 4;

    pub fn clear_total_input_file_count(&mut self) {
        self.total_input_file_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_input_file_count(&mut self, v: i32) {
        self.total_input_file_count = v;
    }

    pub fn get_total_input_file_count(&self) -> i32 {
        self.total_input_file_count
    }

    fn get_total_input_file_count_for_reflect(&self) -> &i32 {
        &self.total_input_file_count
    }

    fn mut_total_input_file_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.total_input_file_count
    }

    // int64 total_input_file_bytes = 5;

    pub fn clear_total_input_file_bytes(&mut self) {
        self.total_input_file_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_input_file_bytes(&mut self, v: i64) {
        self.total_input_file_bytes = v;
    }

    pub fn get_total_input_file_bytes(&self) -> i64 {
        self.total_input_file_bytes
    }

    fn get_total_input_file_bytes_for_reflect(&self) -> &i64 {
        &self.total_input_file_bytes
    }

    fn mut_total_input_file_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_input_file_bytes
    }
}

impl ::protobuf::Message for ExecuteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.action {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.skip_cache_lookup = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.total_input_file_count = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_input_file_bytes = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        if let Some(ref v) = self.action.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.skip_cache_lookup != false {
            my_size += 2;
        }
        if self.total_input_file_count != 0 {
            my_size += ::protobuf::rt::value_size(4, self.total_input_file_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_input_file_bytes != 0 {
            my_size += ::protobuf::rt::value_size(5, self.total_input_file_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        if let Some(ref v) = self.action.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.skip_cache_lookup != false {
            os.write_bool(3, self.skip_cache_lookup)?;
        }
        if self.total_input_file_count != 0 {
            os.write_int32(4, self.total_input_file_count)?;
        }
        if self.total_input_file_bytes != 0 {
            os.write_int64(5, self.total_input_file_bytes)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecuteRequest {
    fn new() -> ExecuteRequest {
        ExecuteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecuteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    ExecuteRequest::get_instance_name_for_reflect,
                    ExecuteRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Action>>(
                    "action",
                    ExecuteRequest::get_action_for_reflect,
                    ExecuteRequest::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "skip_cache_lookup",
                    ExecuteRequest::get_skip_cache_lookup_for_reflect,
                    ExecuteRequest::mut_skip_cache_lookup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "total_input_file_count",
                    ExecuteRequest::get_total_input_file_count_for_reflect,
                    ExecuteRequest::mut_total_input_file_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_input_file_bytes",
                    ExecuteRequest::get_total_input_file_bytes_for_reflect,
                    ExecuteRequest::mut_total_input_file_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecuteRequest>(
                    "ExecuteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecuteRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_action();
        self.clear_skip_cache_lookup();
        self.clear_total_input_file_count();
        self.clear_total_input_file_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecuteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecuteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogFile {
    // message fields
    pub digest: ::protobuf::SingularPtrField<Digest>,
    pub human_readable: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogFile {}

impl LogFile {
    pub fn new() -> LogFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogFile {
        static mut instance: ::protobuf::lazy::Lazy<LogFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogFile,
        };
        unsafe {
            instance.get(LogFile::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.Digest digest = 1;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    pub fn has_digest(&self) -> bool {
        self.digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: Digest) {
        self.digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_digest(&mut self) -> &mut Digest {
        if self.digest.is_none() {
            self.digest.set_default();
        }
        self.digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_digest(&mut self) -> Digest {
        self.digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_digest(&self) -> &Digest {
        self.digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.digest
    }

    fn mut_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.digest
    }

    // bool human_readable = 2;

    pub fn clear_human_readable(&mut self) {
        self.human_readable = false;
    }

    // Param is passed by value, moved
    pub fn set_human_readable(&mut self, v: bool) {
        self.human_readable = v;
    }

    pub fn get_human_readable(&self) -> bool {
        self.human_readable
    }

    fn get_human_readable_for_reflect(&self) -> &bool {
        &self.human_readable
    }

    fn mut_human_readable_for_reflect(&mut self) -> &mut bool {
        &mut self.human_readable
    }
}

impl ::protobuf::Message for LogFile {
    fn is_initialized(&self) -> bool {
        for v in &self.digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.digest)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.human_readable = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.human_readable != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.digest.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.human_readable != false {
            os.write_bool(2, self.human_readable)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogFile {
    fn new() -> LogFile {
        LogFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "digest",
                    LogFile::get_digest_for_reflect,
                    LogFile::mut_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "human_readable",
                    LogFile::get_human_readable_for_reflect,
                    LogFile::mut_human_readable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogFile>(
                    "LogFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogFile {
    fn clear(&mut self) {
        self.clear_digest();
        self.clear_human_readable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecuteResponse {
    // message fields
    pub result: ::protobuf::SingularPtrField<ActionResult>,
    pub cached_result: bool,
    pub status: ::protobuf::SingularPtrField<super::status::Status>,
    pub server_logs: ::std::collections::HashMap<::std::string::String, LogFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecuteResponse {}

impl ExecuteResponse {
    pub fn new() -> ExecuteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecuteResponse {
        static mut instance: ::protobuf::lazy::Lazy<ExecuteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecuteResponse,
        };
        unsafe {
            instance.get(ExecuteResponse::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.ActionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ActionResult) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ActionResult {
        if self.result.is_none() {
            self.result.set_default();
        }
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> ActionResult {
        self.result.take().unwrap_or_else(|| ActionResult::new())
    }

    pub fn get_result(&self) -> &ActionResult {
        self.result.as_ref().unwrap_or_else(|| ActionResult::default_instance())
    }

    fn get_result_for_reflect(&self) -> &::protobuf::SingularPtrField<ActionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ActionResult> {
        &mut self.result
    }

    // bool cached_result = 2;

    pub fn clear_cached_result(&mut self) {
        self.cached_result = false;
    }

    // Param is passed by value, moved
    pub fn set_cached_result(&mut self, v: bool) {
        self.cached_result = v;
    }

    pub fn get_cached_result(&self) -> bool {
        self.cached_result
    }

    fn get_cached_result_for_reflect(&self) -> &bool {
        &self.cached_result
    }

    fn mut_cached_result_for_reflect(&mut self) -> &mut bool {
        &mut self.cached_result
    }

    // .google.rpc.Status status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::status::Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::status::Status {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::status::Status {
        self.status.take().unwrap_or_else(|| super::status::Status::new())
    }

    pub fn get_status(&self) -> &super::status::Status {
        self.status.as_ref().unwrap_or_else(|| super::status::Status::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::status::Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::status::Status> {
        &mut self.status
    }

    // repeated .google.devtools.remoteexecution.v1test.ExecuteResponse.ServerLogsEntry server_logs = 4;

    pub fn clear_server_logs(&mut self) {
        self.server_logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_logs(&mut self, v: ::std::collections::HashMap<::std::string::String, LogFile>) {
        self.server_logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_logs(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, LogFile> {
        &mut self.server_logs
    }

    // Take field
    pub fn take_server_logs(&mut self) -> ::std::collections::HashMap<::std::string::String, LogFile> {
        ::std::mem::replace(&mut self.server_logs, ::std::collections::HashMap::new())
    }

    pub fn get_server_logs(&self) -> &::std::collections::HashMap<::std::string::String, LogFile> {
        &self.server_logs
    }

    fn get_server_logs_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, LogFile> {
        &self.server_logs
    }

    fn mut_server_logs_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, LogFile> {
        &mut self.server_logs
    }
}

impl ::protobuf::Message for ExecuteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.result {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cached_result = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<LogFile>>(wire_type, is, &mut self.server_logs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.cached_result != false {
            my_size += 2;
        }
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<LogFile>>(4, &self.server_logs);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.result.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.cached_result != false {
            os.write_bool(2, self.cached_result)?;
        }
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<LogFile>>(4, &self.server_logs, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecuteResponse {
    fn new() -> ExecuteResponse {
        ExecuteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecuteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActionResult>>(
                    "result",
                    ExecuteResponse::get_result_for_reflect,
                    ExecuteResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cached_result",
                    ExecuteResponse::get_cached_result_for_reflect,
                    ExecuteResponse::mut_cached_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::status::Status>>(
                    "status",
                    ExecuteResponse::get_status_for_reflect,
                    ExecuteResponse::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<LogFile>>(
                    "server_logs",
                    ExecuteResponse::get_server_logs_for_reflect,
                    ExecuteResponse::mut_server_logs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecuteResponse>(
                    "ExecuteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecuteResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_cached_result();
        self.clear_status();
        self.clear_server_logs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecuteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecuteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExecuteOperationMetadata {
    // message fields
    pub stage: ExecuteOperationMetadata_Stage,
    pub action_digest: ::protobuf::SingularPtrField<Digest>,
    pub stdout_stream_name: ::std::string::String,
    pub stderr_stream_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExecuteOperationMetadata {}

impl ExecuteOperationMetadata {
    pub fn new() -> ExecuteOperationMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecuteOperationMetadata {
        static mut instance: ::protobuf::lazy::Lazy<ExecuteOperationMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecuteOperationMetadata,
        };
        unsafe {
            instance.get(ExecuteOperationMetadata::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.ExecuteOperationMetadata.Stage stage = 1;

    pub fn clear_stage(&mut self) {
        self.stage = ExecuteOperationMetadata_Stage::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: ExecuteOperationMetadata_Stage) {
        self.stage = v;
    }

    pub fn get_stage(&self) -> ExecuteOperationMetadata_Stage {
        self.stage
    }

    fn get_stage_for_reflect(&self) -> &ExecuteOperationMetadata_Stage {
        &self.stage
    }

    fn mut_stage_for_reflect(&mut self) -> &mut ExecuteOperationMetadata_Stage {
        &mut self.stage
    }

    // .google.devtools.remoteexecution.v1test.Digest action_digest = 2;

    pub fn clear_action_digest(&mut self) {
        self.action_digest.clear();
    }

    pub fn has_action_digest(&self) -> bool {
        self.action_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_digest(&mut self, v: Digest) {
        self.action_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_digest(&mut self) -> &mut Digest {
        if self.action_digest.is_none() {
            self.action_digest.set_default();
        }
        self.action_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_digest(&mut self) -> Digest {
        self.action_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_action_digest(&self) -> &Digest {
        self.action_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_action_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.action_digest
    }

    fn mut_action_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.action_digest
    }

    // string stdout_stream_name = 3;

    pub fn clear_stdout_stream_name(&mut self) {
        self.stdout_stream_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_stdout_stream_name(&mut self, v: ::std::string::String) {
        self.stdout_stream_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stdout_stream_name(&mut self) -> &mut ::std::string::String {
        &mut self.stdout_stream_name
    }

    // Take field
    pub fn take_stdout_stream_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.stdout_stream_name, ::std::string::String::new())
    }

    pub fn get_stdout_stream_name(&self) -> &str {
        &self.stdout_stream_name
    }

    fn get_stdout_stream_name_for_reflect(&self) -> &::std::string::String {
        &self.stdout_stream_name
    }

    fn mut_stdout_stream_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.stdout_stream_name
    }

    // string stderr_stream_name = 4;

    pub fn clear_stderr_stream_name(&mut self) {
        self.stderr_stream_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_stderr_stream_name(&mut self, v: ::std::string::String) {
        self.stderr_stream_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stderr_stream_name(&mut self) -> &mut ::std::string::String {
        &mut self.stderr_stream_name
    }

    // Take field
    pub fn take_stderr_stream_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.stderr_stream_name, ::std::string::String::new())
    }

    pub fn get_stderr_stream_name(&self) -> &str {
        &self.stderr_stream_name
    }

    fn get_stderr_stream_name_for_reflect(&self) -> &::std::string::String {
        &self.stderr_stream_name
    }

    fn mut_stderr_stream_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.stderr_stream_name
    }
}

impl ::protobuf::Message for ExecuteOperationMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.action_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.stage = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_digest)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.stdout_stream_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.stderr_stream_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.stage != ExecuteOperationMetadata_Stage::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.stage);
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.stdout_stream_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.stdout_stream_name);
        }
        if !self.stderr_stream_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.stderr_stream_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.stage != ExecuteOperationMetadata_Stage::UNKNOWN {
            os.write_enum(1, self.stage.value())?;
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.stdout_stream_name.is_empty() {
            os.write_string(3, &self.stdout_stream_name)?;
        }
        if !self.stderr_stream_name.is_empty() {
            os.write_string(4, &self.stderr_stream_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecuteOperationMetadata {
    fn new() -> ExecuteOperationMetadata {
        ExecuteOperationMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecuteOperationMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExecuteOperationMetadata_Stage>>(
                    "stage",
                    ExecuteOperationMetadata::get_stage_for_reflect,
                    ExecuteOperationMetadata::mut_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "action_digest",
                    ExecuteOperationMetadata::get_action_digest_for_reflect,
                    ExecuteOperationMetadata::mut_action_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stdout_stream_name",
                    ExecuteOperationMetadata::get_stdout_stream_name_for_reflect,
                    ExecuteOperationMetadata::mut_stdout_stream_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stderr_stream_name",
                    ExecuteOperationMetadata::get_stderr_stream_name_for_reflect,
                    ExecuteOperationMetadata::mut_stderr_stream_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecuteOperationMetadata>(
                    "ExecuteOperationMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecuteOperationMetadata {
    fn clear(&mut self) {
        self.clear_stage();
        self.clear_action_digest();
        self.clear_stdout_stream_name();
        self.clear_stderr_stream_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExecuteOperationMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecuteOperationMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExecuteOperationMetadata_Stage {
    UNKNOWN = 0,
    CACHE_CHECK = 1,
    QUEUED = 2,
    EXECUTING = 3,
    COMPLETED = 4,
}

impl ::protobuf::ProtobufEnum for ExecuteOperationMetadata_Stage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExecuteOperationMetadata_Stage> {
        match value {
            0 => ::std::option::Option::Some(ExecuteOperationMetadata_Stage::UNKNOWN),
            1 => ::std::option::Option::Some(ExecuteOperationMetadata_Stage::CACHE_CHECK),
            2 => ::std::option::Option::Some(ExecuteOperationMetadata_Stage::QUEUED),
            3 => ::std::option::Option::Some(ExecuteOperationMetadata_Stage::EXECUTING),
            4 => ::std::option::Option::Some(ExecuteOperationMetadata_Stage::COMPLETED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExecuteOperationMetadata_Stage] = &[
            ExecuteOperationMetadata_Stage::UNKNOWN,
            ExecuteOperationMetadata_Stage::CACHE_CHECK,
            ExecuteOperationMetadata_Stage::QUEUED,
            ExecuteOperationMetadata_Stage::EXECUTING,
            ExecuteOperationMetadata_Stage::COMPLETED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ExecuteOperationMetadata_Stage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExecuteOperationMetadata_Stage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExecuteOperationMetadata_Stage {
}

impl ::std::default::Default for ExecuteOperationMetadata_Stage {
    fn default() -> Self {
        ExecuteOperationMetadata_Stage::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for ExecuteOperationMetadata_Stage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetActionResultRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub action_digest: ::protobuf::SingularPtrField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetActionResultRequest {}

impl GetActionResultRequest {
    pub fn new() -> GetActionResultRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetActionResultRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetActionResultRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetActionResultRequest,
        };
        unsafe {
            instance.get(GetActionResultRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // .google.devtools.remoteexecution.v1test.Digest action_digest = 2;

    pub fn clear_action_digest(&mut self) {
        self.action_digest.clear();
    }

    pub fn has_action_digest(&self) -> bool {
        self.action_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_digest(&mut self, v: Digest) {
        self.action_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_digest(&mut self) -> &mut Digest {
        if self.action_digest.is_none() {
            self.action_digest.set_default();
        }
        self.action_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_digest(&mut self) -> Digest {
        self.action_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_action_digest(&self) -> &Digest {
        self.action_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_action_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.action_digest
    }

    fn mut_action_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.action_digest
    }
}

impl ::protobuf::Message for GetActionResultRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.action_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_digest)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetActionResultRequest {
    fn new() -> GetActionResultRequest {
        GetActionResultRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetActionResultRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    GetActionResultRequest::get_instance_name_for_reflect,
                    GetActionResultRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "action_digest",
                    GetActionResultRequest::get_action_digest_for_reflect,
                    GetActionResultRequest::mut_action_digest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetActionResultRequest>(
                    "GetActionResultRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetActionResultRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_action_digest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetActionResultRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetActionResultRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateActionResultRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub action_digest: ::protobuf::SingularPtrField<Digest>,
    pub action_result: ::protobuf::SingularPtrField<ActionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateActionResultRequest {}

impl UpdateActionResultRequest {
    pub fn new() -> UpdateActionResultRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateActionResultRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateActionResultRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateActionResultRequest,
        };
        unsafe {
            instance.get(UpdateActionResultRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // .google.devtools.remoteexecution.v1test.Digest action_digest = 2;

    pub fn clear_action_digest(&mut self) {
        self.action_digest.clear();
    }

    pub fn has_action_digest(&self) -> bool {
        self.action_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_digest(&mut self, v: Digest) {
        self.action_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_digest(&mut self) -> &mut Digest {
        if self.action_digest.is_none() {
            self.action_digest.set_default();
        }
        self.action_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_digest(&mut self) -> Digest {
        self.action_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_action_digest(&self) -> &Digest {
        self.action_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_action_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.action_digest
    }

    fn mut_action_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.action_digest
    }

    // .google.devtools.remoteexecution.v1test.ActionResult action_result = 3;

    pub fn clear_action_result(&mut self) {
        self.action_result.clear();
    }

    pub fn has_action_result(&self) -> bool {
        self.action_result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_result(&mut self, v: ActionResult) {
        self.action_result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_result(&mut self) -> &mut ActionResult {
        if self.action_result.is_none() {
            self.action_result.set_default();
        }
        self.action_result.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_result(&mut self) -> ActionResult {
        self.action_result.take().unwrap_or_else(|| ActionResult::new())
    }

    pub fn get_action_result(&self) -> &ActionResult {
        self.action_result.as_ref().unwrap_or_else(|| ActionResult::default_instance())
    }

    fn get_action_result_for_reflect(&self) -> &::protobuf::SingularPtrField<ActionResult> {
        &self.action_result
    }

    fn mut_action_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ActionResult> {
        &mut self.action_result
    }
}

impl ::protobuf::Message for UpdateActionResultRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.action_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.action_result {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_digest)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_result)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.action_result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        if let Some(ref v) = self.action_digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.action_result.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateActionResultRequest {
    fn new() -> UpdateActionResultRequest {
        UpdateActionResultRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateActionResultRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    UpdateActionResultRequest::get_instance_name_for_reflect,
                    UpdateActionResultRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "action_digest",
                    UpdateActionResultRequest::get_action_digest_for_reflect,
                    UpdateActionResultRequest::mut_action_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActionResult>>(
                    "action_result",
                    UpdateActionResultRequest::get_action_result_for_reflect,
                    UpdateActionResultRequest::mut_action_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateActionResultRequest>(
                    "UpdateActionResultRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateActionResultRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_action_digest();
        self.clear_action_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateActionResultRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateActionResultRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FindMissingBlobsRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub blob_digests: ::protobuf::RepeatedField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FindMissingBlobsRequest {}

impl FindMissingBlobsRequest {
    pub fn new() -> FindMissingBlobsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FindMissingBlobsRequest {
        static mut instance: ::protobuf::lazy::Lazy<FindMissingBlobsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FindMissingBlobsRequest,
        };
        unsafe {
            instance.get(FindMissingBlobsRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // repeated .google.devtools.remoteexecution.v1test.Digest blob_digests = 2;

    pub fn clear_blob_digests(&mut self) {
        self.blob_digests.clear();
    }

    // Param is passed by value, moved
    pub fn set_blob_digests(&mut self, v: ::protobuf::RepeatedField<Digest>) {
        self.blob_digests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blob_digests(&mut self) -> &mut ::protobuf::RepeatedField<Digest> {
        &mut self.blob_digests
    }

    // Take field
    pub fn take_blob_digests(&mut self) -> ::protobuf::RepeatedField<Digest> {
        ::std::mem::replace(&mut self.blob_digests, ::protobuf::RepeatedField::new())
    }

    pub fn get_blob_digests(&self) -> &[Digest] {
        &self.blob_digests
    }

    fn get_blob_digests_for_reflect(&self) -> &::protobuf::RepeatedField<Digest> {
        &self.blob_digests
    }

    fn mut_blob_digests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Digest> {
        &mut self.blob_digests
    }
}

impl ::protobuf::Message for FindMissingBlobsRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.blob_digests {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blob_digests)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        for value in &self.blob_digests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        for v in &self.blob_digests {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FindMissingBlobsRequest {
    fn new() -> FindMissingBlobsRequest {
        FindMissingBlobsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<FindMissingBlobsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    FindMissingBlobsRequest::get_instance_name_for_reflect,
                    FindMissingBlobsRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "blob_digests",
                    FindMissingBlobsRequest::get_blob_digests_for_reflect,
                    FindMissingBlobsRequest::mut_blob_digests_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FindMissingBlobsRequest>(
                    "FindMissingBlobsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FindMissingBlobsRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_blob_digests();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FindMissingBlobsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FindMissingBlobsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FindMissingBlobsResponse {
    // message fields
    pub missing_blob_digests: ::protobuf::RepeatedField<Digest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FindMissingBlobsResponse {}

impl FindMissingBlobsResponse {
    pub fn new() -> FindMissingBlobsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FindMissingBlobsResponse {
        static mut instance: ::protobuf::lazy::Lazy<FindMissingBlobsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FindMissingBlobsResponse,
        };
        unsafe {
            instance.get(FindMissingBlobsResponse::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.Digest missing_blob_digests = 2;

    pub fn clear_missing_blob_digests(&mut self) {
        self.missing_blob_digests.clear();
    }

    // Param is passed by value, moved
    pub fn set_missing_blob_digests(&mut self, v: ::protobuf::RepeatedField<Digest>) {
        self.missing_blob_digests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_missing_blob_digests(&mut self) -> &mut ::protobuf::RepeatedField<Digest> {
        &mut self.missing_blob_digests
    }

    // Take field
    pub fn take_missing_blob_digests(&mut self) -> ::protobuf::RepeatedField<Digest> {
        ::std::mem::replace(&mut self.missing_blob_digests, ::protobuf::RepeatedField::new())
    }

    pub fn get_missing_blob_digests(&self) -> &[Digest] {
        &self.missing_blob_digests
    }

    fn get_missing_blob_digests_for_reflect(&self) -> &::protobuf::RepeatedField<Digest> {
        &self.missing_blob_digests
    }

    fn mut_missing_blob_digests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Digest> {
        &mut self.missing_blob_digests
    }
}

impl ::protobuf::Message for FindMissingBlobsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.missing_blob_digests {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.missing_blob_digests)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.missing_blob_digests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.missing_blob_digests {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FindMissingBlobsResponse {
    fn new() -> FindMissingBlobsResponse {
        FindMissingBlobsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FindMissingBlobsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "missing_blob_digests",
                    FindMissingBlobsResponse::get_missing_blob_digests_for_reflect,
                    FindMissingBlobsResponse::mut_missing_blob_digests_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FindMissingBlobsResponse>(
                    "FindMissingBlobsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FindMissingBlobsResponse {
    fn clear(&mut self) {
        self.clear_missing_blob_digests();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FindMissingBlobsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FindMissingBlobsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateBlobRequest {
    // message fields
    pub content_digest: ::protobuf::SingularPtrField<Digest>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateBlobRequest {}

impl UpdateBlobRequest {
    pub fn new() -> UpdateBlobRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateBlobRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateBlobRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateBlobRequest,
        };
        unsafe {
            instance.get(UpdateBlobRequest::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.Digest content_digest = 1;

    pub fn clear_content_digest(&mut self) {
        self.content_digest.clear();
    }

    pub fn has_content_digest(&self) -> bool {
        self.content_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_digest(&mut self, v: Digest) {
        self.content_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_digest(&mut self) -> &mut Digest {
        if self.content_digest.is_none() {
            self.content_digest.set_default();
        }
        self.content_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_digest(&mut self) -> Digest {
        self.content_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_content_digest(&self) -> &Digest {
        self.content_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_content_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.content_digest
    }

    fn mut_content_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.content_digest
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for UpdateBlobRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.content_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.content_digest)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.content_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.content_digest.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateBlobRequest {
    fn new() -> UpdateBlobRequest {
        UpdateBlobRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateBlobRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "content_digest",
                    UpdateBlobRequest::get_content_digest_for_reflect,
                    UpdateBlobRequest::mut_content_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    UpdateBlobRequest::get_data_for_reflect,
                    UpdateBlobRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateBlobRequest>(
                    "UpdateBlobRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateBlobRequest {
    fn clear(&mut self) {
        self.clear_content_digest();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateBlobRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateBlobRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchUpdateBlobsRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub requests: ::protobuf::RepeatedField<UpdateBlobRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchUpdateBlobsRequest {}

impl BatchUpdateBlobsRequest {
    pub fn new() -> BatchUpdateBlobsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchUpdateBlobsRequest {
        static mut instance: ::protobuf::lazy::Lazy<BatchUpdateBlobsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchUpdateBlobsRequest,
        };
        unsafe {
            instance.get(BatchUpdateBlobsRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // repeated .google.devtools.remoteexecution.v1test.UpdateBlobRequest requests = 2;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<UpdateBlobRequest>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<UpdateBlobRequest> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<UpdateBlobRequest> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests(&self) -> &[UpdateBlobRequest] {
        &self.requests
    }

    fn get_requests_for_reflect(&self) -> &::protobuf::RepeatedField<UpdateBlobRequest> {
        &self.requests
    }

    fn mut_requests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UpdateBlobRequest> {
        &mut self.requests
    }
}

impl ::protobuf::Message for BatchUpdateBlobsRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.requests {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        for v in &self.requests {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchUpdateBlobsRequest {
    fn new() -> BatchUpdateBlobsRequest {
        BatchUpdateBlobsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchUpdateBlobsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    BatchUpdateBlobsRequest::get_instance_name_for_reflect,
                    BatchUpdateBlobsRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpdateBlobRequest>>(
                    "requests",
                    BatchUpdateBlobsRequest::get_requests_for_reflect,
                    BatchUpdateBlobsRequest::mut_requests_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchUpdateBlobsRequest>(
                    "BatchUpdateBlobsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchUpdateBlobsRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchUpdateBlobsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchUpdateBlobsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchUpdateBlobsResponse {
    // message fields
    pub responses: ::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchUpdateBlobsResponse {}

impl BatchUpdateBlobsResponse {
    pub fn new() -> BatchUpdateBlobsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchUpdateBlobsResponse {
        static mut instance: ::protobuf::lazy::Lazy<BatchUpdateBlobsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchUpdateBlobsResponse,
        };
        unsafe {
            instance.get(BatchUpdateBlobsResponse::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.BatchUpdateBlobsResponse.Response responses = 1;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[BatchUpdateBlobsResponse_Response] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BatchUpdateBlobsResponse_Response> {
        &mut self.responses
    }
}

impl ::protobuf::Message for BatchUpdateBlobsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.responses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.responses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchUpdateBlobsResponse {
    fn new() -> BatchUpdateBlobsResponse {
        BatchUpdateBlobsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchUpdateBlobsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BatchUpdateBlobsResponse_Response>>(
                    "responses",
                    BatchUpdateBlobsResponse::get_responses_for_reflect,
                    BatchUpdateBlobsResponse::mut_responses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchUpdateBlobsResponse>(
                    "BatchUpdateBlobsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchUpdateBlobsResponse {
    fn clear(&mut self) {
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchUpdateBlobsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchUpdateBlobsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchUpdateBlobsResponse_Response {
    // message fields
    pub blob_digest: ::protobuf::SingularPtrField<Digest>,
    pub status: ::protobuf::SingularPtrField<super::status::Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchUpdateBlobsResponse_Response {}

impl BatchUpdateBlobsResponse_Response {
    pub fn new() -> BatchUpdateBlobsResponse_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchUpdateBlobsResponse_Response {
        static mut instance: ::protobuf::lazy::Lazy<BatchUpdateBlobsResponse_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchUpdateBlobsResponse_Response,
        };
        unsafe {
            instance.get(BatchUpdateBlobsResponse_Response::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.Digest blob_digest = 1;

    pub fn clear_blob_digest(&mut self) {
        self.blob_digest.clear();
    }

    pub fn has_blob_digest(&self) -> bool {
        self.blob_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blob_digest(&mut self, v: Digest) {
        self.blob_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blob_digest(&mut self) -> &mut Digest {
        if self.blob_digest.is_none() {
            self.blob_digest.set_default();
        }
        self.blob_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_blob_digest(&mut self) -> Digest {
        self.blob_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_blob_digest(&self) -> &Digest {
        self.blob_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_blob_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.blob_digest
    }

    fn mut_blob_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.blob_digest
    }

    // .google.rpc.Status status = 2;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::status::Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::status::Status {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::status::Status {
        self.status.take().unwrap_or_else(|| super::status::Status::new())
    }

    pub fn get_status(&self) -> &super::status::Status {
        self.status.as_ref().unwrap_or_else(|| super::status::Status::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<super::status::Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::status::Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for BatchUpdateBlobsResponse_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.blob_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blob_digest)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.blob_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blob_digest.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchUpdateBlobsResponse_Response {
    fn new() -> BatchUpdateBlobsResponse_Response {
        BatchUpdateBlobsResponse_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchUpdateBlobsResponse_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "blob_digest",
                    BatchUpdateBlobsResponse_Response::get_blob_digest_for_reflect,
                    BatchUpdateBlobsResponse_Response::mut_blob_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::status::Status>>(
                    "status",
                    BatchUpdateBlobsResponse_Response::get_status_for_reflect,
                    BatchUpdateBlobsResponse_Response::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchUpdateBlobsResponse_Response>(
                    "BatchUpdateBlobsResponse_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchUpdateBlobsResponse_Response {
    fn clear(&mut self) {
        self.clear_blob_digest();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchUpdateBlobsResponse_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchUpdateBlobsResponse_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTreeRequest {
    // message fields
    pub instance_name: ::std::string::String,
    pub root_digest: ::protobuf::SingularPtrField<Digest>,
    pub page_size: i32,
    pub page_token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTreeRequest {}

impl GetTreeRequest {
    pub fn new() -> GetTreeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTreeRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetTreeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTreeRequest,
        };
        unsafe {
            instance.get(GetTreeRequest::new)
        }
    }

    // string instance_name = 1;

    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }

    fn get_instance_name_for_reflect(&self) -> &::std::string::String {
        &self.instance_name
    }

    fn mut_instance_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // .google.devtools.remoteexecution.v1test.Digest root_digest = 2;

    pub fn clear_root_digest(&mut self) {
        self.root_digest.clear();
    }

    pub fn has_root_digest(&self) -> bool {
        self.root_digest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root_digest(&mut self, v: Digest) {
        self.root_digest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root_digest(&mut self) -> &mut Digest {
        if self.root_digest.is_none() {
            self.root_digest.set_default();
        }
        self.root_digest.as_mut().unwrap()
    }

    // Take field
    pub fn take_root_digest(&mut self) -> Digest {
        self.root_digest.take().unwrap_or_else(|| Digest::new())
    }

    pub fn get_root_digest(&self) -> &Digest {
        self.root_digest.as_ref().unwrap_or_else(|| Digest::default_instance())
    }

    fn get_root_digest_for_reflect(&self) -> &::protobuf::SingularPtrField<Digest> {
        &self.root_digest
    }

    fn mut_root_digest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Digest> {
        &mut self.root_digest
    }

    // int32 page_size = 3;

    pub fn clear_page_size(&mut self) {
        self.page_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_page_size(&mut self, v: i32) {
        self.page_size = v;
    }

    pub fn get_page_size(&self) -> i32 {
        self.page_size
    }

    fn get_page_size_for_reflect(&self) -> &i32 {
        &self.page_size
    }

    fn mut_page_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.page_size
    }

    // string page_token = 4;

    pub fn clear_page_token(&mut self) {
        self.page_token.clear();
    }

    // Param is passed by value, moved
    pub fn set_page_token(&mut self, v: ::std::string::String) {
        self.page_token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_page_token(&mut self) -> &mut ::std::string::String {
        &mut self.page_token
    }

    // Take field
    pub fn take_page_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.page_token, ::std::string::String::new())
    }

    pub fn get_page_token(&self) -> &str {
        &self.page_token
    }

    fn get_page_token_for_reflect(&self) -> &::std::string::String {
        &self.page_token
    }

    fn mut_page_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.page_token
    }
}

impl ::protobuf::Message for GetTreeRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.root_digest {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.root_digest)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.page_size = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.page_token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.instance_name);
        }
        if let Some(ref v) = self.root_digest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.page_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.page_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.page_token.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.page_token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.instance_name.is_empty() {
            os.write_string(1, &self.instance_name)?;
        }
        if let Some(ref v) = self.root_digest.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.page_size != 0 {
            os.write_int32(3, self.page_size)?;
        }
        if !self.page_token.is_empty() {
            os.write_string(4, &self.page_token)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTreeRequest {
    fn new() -> GetTreeRequest {
        GetTreeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTreeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "instance_name",
                    GetTreeRequest::get_instance_name_for_reflect,
                    GetTreeRequest::mut_instance_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Digest>>(
                    "root_digest",
                    GetTreeRequest::get_root_digest_for_reflect,
                    GetTreeRequest::mut_root_digest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "page_size",
                    GetTreeRequest::get_page_size_for_reflect,
                    GetTreeRequest::mut_page_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "page_token",
                    GetTreeRequest::get_page_token_for_reflect,
                    GetTreeRequest::mut_page_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTreeRequest>(
                    "GetTreeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTreeRequest {
    fn clear(&mut self) {
        self.clear_instance_name();
        self.clear_root_digest();
        self.clear_page_size();
        self.clear_page_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTreeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTreeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTreeResponse {
    // message fields
    pub directories: ::protobuf::RepeatedField<Directory>,
    pub next_page_token: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTreeResponse {}

impl GetTreeResponse {
    pub fn new() -> GetTreeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTreeResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetTreeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTreeResponse,
        };
        unsafe {
            instance.get(GetTreeResponse::new)
        }
    }

    // repeated .google.devtools.remoteexecution.v1test.Directory directories = 1;

    pub fn clear_directories(&mut self) {
        self.directories.clear();
    }

    // Param is passed by value, moved
    pub fn set_directories(&mut self, v: ::protobuf::RepeatedField<Directory>) {
        self.directories = v;
    }

    // Mutable pointer to the field.
    pub fn mut_directories(&mut self) -> &mut ::protobuf::RepeatedField<Directory> {
        &mut self.directories
    }

    // Take field
    pub fn take_directories(&mut self) -> ::protobuf::RepeatedField<Directory> {
        ::std::mem::replace(&mut self.directories, ::protobuf::RepeatedField::new())
    }

    pub fn get_directories(&self) -> &[Directory] {
        &self.directories
    }

    fn get_directories_for_reflect(&self) -> &::protobuf::RepeatedField<Directory> {
        &self.directories
    }

    fn mut_directories_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Directory> {
        &mut self.directories
    }

    // string next_page_token = 2;

    pub fn clear_next_page_token(&mut self) {
        self.next_page_token.clear();
    }

    // Param is passed by value, moved
    pub fn set_next_page_token(&mut self, v: ::std::string::String) {
        self.next_page_token = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_next_page_token(&mut self) -> &mut ::std::string::String {
        &mut self.next_page_token
    }

    // Take field
    pub fn take_next_page_token(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.next_page_token, ::std::string::String::new())
    }

    pub fn get_next_page_token(&self) -> &str {
        &self.next_page_token
    }

    fn get_next_page_token_for_reflect(&self) -> &::std::string::String {
        &self.next_page_token
    }

    fn mut_next_page_token_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.next_page_token
    }
}

impl ::protobuf::Message for GetTreeResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.directories {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.directories)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.next_page_token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.directories {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.next_page_token.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.next_page_token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.directories {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.next_page_token.is_empty() {
            os.write_string(2, &self.next_page_token)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetTreeResponse {
    fn new() -> GetTreeResponse {
        GetTreeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTreeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Directory>>(
                    "directories",
                    GetTreeResponse::get_directories_for_reflect,
                    GetTreeResponse::mut_directories_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "next_page_token",
                    GetTreeResponse::get_next_page_token_for_reflect,
                    GetTreeResponse::mut_next_page_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTreeResponse>(
                    "GetTreeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTreeResponse {
    fn clear(&mut self) {
        self.clear_directories();
        self.clear_next_page_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTreeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTreeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolDetails {
    // message fields
    pub tool_name: ::std::string::String,
    pub tool_version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolDetails {}

impl ToolDetails {
    pub fn new() -> ToolDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolDetails {
        static mut instance: ::protobuf::lazy::Lazy<ToolDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolDetails,
        };
        unsafe {
            instance.get(ToolDetails::new)
        }
    }

    // string tool_name = 1;

    pub fn clear_tool_name(&mut self) {
        self.tool_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_tool_name(&mut self, v: ::std::string::String) {
        self.tool_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tool_name(&mut self) -> &mut ::std::string::String {
        &mut self.tool_name
    }

    // Take field
    pub fn take_tool_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tool_name, ::std::string::String::new())
    }

    pub fn get_tool_name(&self) -> &str {
        &self.tool_name
    }

    fn get_tool_name_for_reflect(&self) -> &::std::string::String {
        &self.tool_name
    }

    fn mut_tool_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tool_name
    }

    // string tool_version = 2;

    pub fn clear_tool_version(&mut self) {
        self.tool_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_tool_version(&mut self, v: ::std::string::String) {
        self.tool_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tool_version(&mut self) -> &mut ::std::string::String {
        &mut self.tool_version
    }

    // Take field
    pub fn take_tool_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tool_version, ::std::string::String::new())
    }

    pub fn get_tool_version(&self) -> &str {
        &self.tool_version
    }

    fn get_tool_version_for_reflect(&self) -> &::std::string::String {
        &self.tool_version
    }

    fn mut_tool_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tool_version
    }
}

impl ::protobuf::Message for ToolDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tool_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tool_version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.tool_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tool_name);
        }
        if !self.tool_version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.tool_version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tool_name.is_empty() {
            os.write_string(1, &self.tool_name)?;
        }
        if !self.tool_version.is_empty() {
            os.write_string(2, &self.tool_version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ToolDetails {
    fn new() -> ToolDetails {
        ToolDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tool_name",
                    ToolDetails::get_tool_name_for_reflect,
                    ToolDetails::mut_tool_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tool_version",
                    ToolDetails::get_tool_version_for_reflect,
                    ToolDetails::mut_tool_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolDetails>(
                    "ToolDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolDetails {
    fn clear(&mut self) {
        self.clear_tool_name();
        self.clear_tool_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestMetadata {
    // message fields
    pub tool_details: ::protobuf::SingularPtrField<ToolDetails>,
    pub action_id: ::std::string::String,
    pub tool_invocation_id: ::std::string::String,
    pub correlated_invocations_id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestMetadata {}

impl RequestMetadata {
    pub fn new() -> RequestMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestMetadata {
        static mut instance: ::protobuf::lazy::Lazy<RequestMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestMetadata,
        };
        unsafe {
            instance.get(RequestMetadata::new)
        }
    }

    // .google.devtools.remoteexecution.v1test.ToolDetails tool_details = 1;

    pub fn clear_tool_details(&mut self) {
        self.tool_details.clear();
    }

    pub fn has_tool_details(&self) -> bool {
        self.tool_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool_details(&mut self, v: ToolDetails) {
        self.tool_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tool_details(&mut self) -> &mut ToolDetails {
        if self.tool_details.is_none() {
            self.tool_details.set_default();
        }
        self.tool_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_tool_details(&mut self) -> ToolDetails {
        self.tool_details.take().unwrap_or_else(|| ToolDetails::new())
    }

    pub fn get_tool_details(&self) -> &ToolDetails {
        self.tool_details.as_ref().unwrap_or_else(|| ToolDetails::default_instance())
    }

    fn get_tool_details_for_reflect(&self) -> &::protobuf::SingularPtrField<ToolDetails> {
        &self.tool_details
    }

    fn mut_tool_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ToolDetails> {
        &mut self.tool_details
    }

    // string action_id = 2;

    pub fn clear_action_id(&mut self) {
        self.action_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_action_id(&mut self, v: ::std::string::String) {
        self.action_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_id(&mut self) -> &mut ::std::string::String {
        &mut self.action_id
    }

    // Take field
    pub fn take_action_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.action_id, ::std::string::String::new())
    }

    pub fn get_action_id(&self) -> &str {
        &self.action_id
    }

    fn get_action_id_for_reflect(&self) -> &::std::string::String {
        &self.action_id
    }

    fn mut_action_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.action_id
    }

    // string tool_invocation_id = 3;

    pub fn clear_tool_invocation_id(&mut self) {
        self.tool_invocation_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_tool_invocation_id(&mut self, v: ::std::string::String) {
        self.tool_invocation_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tool_invocation_id(&mut self) -> &mut ::std::string::String {
        &mut self.tool_invocation_id
    }

    // Take field
    pub fn take_tool_invocation_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tool_invocation_id, ::std::string::String::new())
    }

    pub fn get_tool_invocation_id(&self) -> &str {
        &self.tool_invocation_id
    }

    fn get_tool_invocation_id_for_reflect(&self) -> &::std::string::String {
        &self.tool_invocation_id
    }

    fn mut_tool_invocation_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tool_invocation_id
    }

    // string correlated_invocations_id = 4;

    pub fn clear_correlated_invocations_id(&mut self) {
        self.correlated_invocations_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_correlated_invocations_id(&mut self, v: ::std::string::String) {
        self.correlated_invocations_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_correlated_invocations_id(&mut self) -> &mut ::std::string::String {
        &mut self.correlated_invocations_id
    }

    // Take field
    pub fn take_correlated_invocations_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.correlated_invocations_id, ::std::string::String::new())
    }

    pub fn get_correlated_invocations_id(&self) -> &str {
        &self.correlated_invocations_id
    }

    fn get_correlated_invocations_id_for_reflect(&self) -> &::std::string::String {
        &self.correlated_invocations_id
    }

    fn mut_correlated_invocations_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.correlated_invocations_id
    }
}

impl ::protobuf::Message for RequestMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.tool_details {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tool_details)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.action_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tool_invocation_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.correlated_invocations_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.tool_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.action_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.action_id);
        }
        if !self.tool_invocation_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.tool_invocation_id);
        }
        if !self.correlated_invocations_id.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.correlated_invocations_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.tool_details.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.action_id.is_empty() {
            os.write_string(2, &self.action_id)?;
        }
        if !self.tool_invocation_id.is_empty() {
            os.write_string(3, &self.tool_invocation_id)?;
        }
        if !self.correlated_invocations_id.is_empty() {
            os.write_string(4, &self.correlated_invocations_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestMetadata {
    fn new() -> RequestMetadata {
        RequestMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ToolDetails>>(
                    "tool_details",
                    RequestMetadata::get_tool_details_for_reflect,
                    RequestMetadata::mut_tool_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "action_id",
                    RequestMetadata::get_action_id_for_reflect,
                    RequestMetadata::mut_action_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tool_invocation_id",
                    RequestMetadata::get_tool_invocation_id_for_reflect,
                    RequestMetadata::mut_tool_invocation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "correlated_invocations_id",
                    RequestMetadata::get_correlated_invocations_id_for_reflect,
                    RequestMetadata::mut_correlated_invocations_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestMetadata>(
                    "RequestMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestMetadata {
    fn clear(&mut self) {
        self.clear_tool_details();
        self.clear_action_id();
        self.clear_tool_invocation_id();
        self.clear_correlated_invocations_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n=google/devtools/remoteexecution/v1test/remote_execution.proto\x12&goo\
    gle.devtools.remoteexecution.v1test\x1a\x1cgoogle/api/annotations.proto\
    \x1a#google/longrunning/operations.proto\x1a\x1egoogle/protobuf/duration\
    .proto\x1a\x17google/rpc/status.proto\"\xb2\x03\n\x06Action\x12U\n\x0eco\
    mmand_digest\x18\x01\x20\x01(\x0b2..google.devtools.remoteexecution.v1te\
    st.DigestR\rcommandDigest\x12Z\n\x11input_root_digest\x18\x02\x20\x01(\
    \x0b2..google.devtools.remoteexecution.v1test.DigestR\x0finputRootDigest\
    \x12!\n\x0coutput_files\x18\x03\x20\x03(\tR\x0boutputFiles\x12-\n\x12out\
    put_directories\x18\x04\x20\x03(\tR\x11outputDirectories\x12L\n\x08platf\
    orm\x18\x05\x20\x01(\x0b20.google.devtools.remoteexecution.v1test.Platfo\
    rmR\x08platform\x123\n\x07timeout\x18\x06\x20\x01(\x0b2\x19.google.proto\
    buf.DurationR\x07timeout\x12\x20\n\x0cdo_not_cache\x18\x07\x20\x01(\x08R\
    \ndoNotCache\"\xe2\x01\n\x07Command\x12\x1c\n\targuments\x18\x01\x20\x03\
    (\tR\targuments\x12x\n\x15environment_variables\x18\x02\x20\x03(\x0b2C.g\
    oogle.devtools.remoteexecution.v1test.Command.EnvironmentVariableR\x14en\
    vironmentVariables\x1a?\n\x13EnvironmentVariable\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value\
    \"\x9b\x01\n\x08Platform\x12Y\n\nproperties\x18\x01\x20\x03(\x0b29.googl\
    e.devtools.remoteexecution.v1test.Platform.PropertyR\nproperties\x1a4\n\
    \x08Property\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05\
    value\x18\x02\x20\x01(\tR\x05value\"\xac\x01\n\tDirectory\x12F\n\x05file\
    s\x18\x01\x20\x03(\x0b20.google.devtools.remoteexecution.v1test.FileNode\
    R\x05files\x12W\n\x0bdirectories\x18\x02\x20\x03(\x0b25.google.devtools.\
    remoteexecution.v1test.DirectoryNodeR\x0bdirectories\"\x8b\x01\n\x08File\
    Node\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12F\n\x06digest\x18\
    \x02\x20\x01(\x0b2..google.devtools.remoteexecution.v1test.DigestR\x06di\
    gest\x12#\n\ris_executable\x18\x04\x20\x01(\x08R\x0cisExecutable\"k\n\rD\
    irectoryNode\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12F\n\x06dig\
    est\x18\x02\x20\x01(\x0b2..google.devtools.remoteexecution.v1test.Digest\
    R\x06digest\";\n\x06Digest\x12\x12\n\x04hash\x18\x01\x20\x01(\tR\x04hash\
    \x12\x1d\n\nsize_bytes\x18\x02\x20\x01(\x03R\tsizeBytes\"\xd2\x03\n\x0cA\
    ctionResult\x12U\n\x0coutput_files\x18\x02\x20\x03(\x0b22.google.devtool\
    s.remoteexecution.v1test.OutputFileR\x0boutputFiles\x12f\n\x12output_dir\
    ectories\x18\x03\x20\x03(\x0b27.google.devtools.remoteexecution.v1test.O\
    utputDirectoryR\x11outputDirectories\x12\x1b\n\texit_code\x18\x04\x20\
    \x01(\x05R\x08exitCode\x12\x1d\n\nstdout_raw\x18\x05\x20\x01(\x0cR\tstdo\
    utRaw\x12S\n\rstdout_digest\x18\x06\x20\x01(\x0b2..google.devtools.remot\
    eexecution.v1test.DigestR\x0cstdoutDigest\x12\x1d\n\nstderr_raw\x18\x07\
    \x20\x01(\x0cR\tstderrRaw\x12S\n\rstderr_digest\x18\x08\x20\x01(\x0b2..g\
    oogle.devtools.remoteexecution.v1test.DigestR\x0cstderrDigest\"\xa7\x01\
    \n\nOutputFile\x12\x12\n\x04path\x18\x01\x20\x01(\tR\x04path\x12F\n\x06d\
    igest\x18\x02\x20\x01(\x0b2..google.devtools.remoteexecution.v1test.Dige\
    stR\x06digest\x12\x18\n\x07content\x18\x03\x20\x01(\x0cR\x07content\x12#\
    \n\ris_executable\x18\x04\x20\x01(\x08R\x0cisExecutable\"\x9c\x01\n\x04T\
    ree\x12E\n\x04root\x18\x01\x20\x01(\x0b21.google.devtools.remoteexecutio\
    n.v1test.DirectoryR\x04root\x12M\n\x08children\x18\x02\x20\x03(\x0b21.go\
    ogle.devtools.remoteexecution.v1test.DirectoryR\x08children\"\xbe\x01\n\
    \x0fOutputDirectory\x12\x12\n\x04path\x18\x01\x20\x01(\tR\x04path\x12F\n\
    \x06digest\x18\x02\x20\x01(\x0b2..google.devtools.remoteexecution.v1test\
    .DigestR\x06digest\x12O\n\x0btree_digest\x18\x03\x20\x01(\x0b2..google.d\
    evtools.remoteexecution.v1test.DigestR\ntreeDigest\"\x93\x02\n\x0eExecut\
    eRequest\x12#\n\rinstance_name\x18\x01\x20\x01(\tR\x0cinstanceName\x12F\
    \n\x06action\x18\x02\x20\x01(\x0b2..google.devtools.remoteexecution.v1te\
    st.ActionR\x06action\x12*\n\x11skip_cache_lookup\x18\x03\x20\x01(\x08R\
    \x0fskipCacheLookup\x123\n\x16total_input_file_count\x18\x04\x20\x01(\
    \x05R\x13totalInputFileCount\x123\n\x16total_input_file_bytes\x18\x05\
    \x20\x01(\x03R\x13totalInputFileBytes\"x\n\x07LogFile\x12F\n\x06digest\
    \x18\x01\x20\x01(\x0b2..google.devtools.remoteexecution.v1test.DigestR\
    \x06digest\x12%\n\x0ehuman_readable\x18\x02\x20\x01(\x08R\rhumanReadable\
    \"\x8a\x03\n\x0fExecuteResponse\x12L\n\x06result\x18\x01\x20\x01(\x0b24.\
    google.devtools.remoteexecution.v1test.ActionResultR\x06result\x12#\n\rc\
    ached_result\x18\x02\x20\x01(\x08R\x0ccachedResult\x12*\n\x06status\x18\
    \x03\x20\x01(\x0b2\x12.google.rpc.StatusR\x06status\x12h\n\x0bserver_log\
    s\x18\x04\x20\x03(\x0b2G.google.devtools.remoteexecution.v1test.ExecuteR\
    esponse.ServerLogsEntryR\nserverLogs\x1an\n\x0fServerLogsEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03key\x12E\n\x05value\x18\x02\x20\x01(\x0b2\
    /.google.devtools.remoteexecution.v1test.LogFileR\x05value:\x028\x01\"\
    \xfa\x02\n\x18ExecuteOperationMetadata\x12\\\n\x05stage\x18\x01\x20\x01(\
    \x0e2F.google.devtools.remoteexecution.v1test.ExecuteOperationMetadata.S\
    tageR\x05stage\x12S\n\raction_digest\x18\x02\x20\x01(\x0b2..google.devto\
    ols.remoteexecution.v1test.DigestR\x0cactionDigest\x12,\n\x12stdout_stre\
    am_name\x18\x03\x20\x01(\tR\x10stdoutStreamName\x12,\n\x12stderr_stream_\
    name\x18\x04\x20\x01(\tR\x10stderrStreamName\"O\n\x05Stage\x12\x0b\n\x07\
    UNKNOWN\x10\0\x12\x0f\n\x0bCACHE_CHECK\x10\x01\x12\n\n\x06QUEUED\x10\x02\
    \x12\r\n\tEXECUTING\x10\x03\x12\r\n\tCOMPLETED\x10\x04\"\x92\x01\n\x16Ge\
    tActionResultRequest\x12#\n\rinstance_name\x18\x01\x20\x01(\tR\x0cinstan\
    ceName\x12S\n\raction_digest\x18\x02\x20\x01(\x0b2..google.devtools.remo\
    teexecution.v1test.DigestR\x0cactionDigest\"\xf0\x01\n\x19UpdateActionRe\
    sultRequest\x12#\n\rinstance_name\x18\x01\x20\x01(\tR\x0cinstanceName\
    \x12S\n\raction_digest\x18\x02\x20\x01(\x0b2..google.devtools.remoteexec\
    ution.v1test.DigestR\x0cactionDigest\x12Y\n\raction_result\x18\x03\x20\
    \x01(\x0b24.google.devtools.remoteexecution.v1test.ActionResultR\x0cacti\
    onResult\"\x91\x01\n\x17FindMissingBlobsRequest\x12#\n\rinstance_name\
    \x18\x01\x20\x01(\tR\x0cinstanceName\x12Q\n\x0cblob_digests\x18\x02\x20\
    \x03(\x0b2..google.devtools.remoteexecution.v1test.DigestR\x0bblobDigest\
    s\"|\n\x18FindMissingBlobsResponse\x12`\n\x14missing_blob_digests\x18\
    \x02\x20\x03(\x0b2..google.devtools.remoteexecution.v1test.DigestR\x12mi\
    ssingBlobDigests\"~\n\x11UpdateBlobRequest\x12U\n\x0econtent_digest\x18\
    \x01\x20\x01(\x0b2..google.devtools.remoteexecution.v1test.DigestR\rcont\
    entDigest\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"\x95\x01\n\
    \x17BatchUpdateBlobsRequest\x12#\n\rinstance_name\x18\x01\x20\x01(\tR\
    \x0cinstanceName\x12U\n\x08requests\x18\x02\x20\x03(\x0b29.google.devtoo\
    ls.remoteexecution.v1test.UpdateBlobRequestR\x08requests\"\x8d\x02\n\x18\
    BatchUpdateBlobsResponse\x12g\n\tresponses\x18\x01\x20\x03(\x0b2I.google\
    .devtools.remoteexecution.v1test.BatchUpdateBlobsResponse.ResponseR\tres\
    ponses\x1a\x87\x01\n\x08Response\x12O\n\x0bblob_digest\x18\x01\x20\x01(\
    \x0b2..google.devtools.remoteexecution.v1test.DigestR\nblobDigest\x12*\n\
    \x06status\x18\x02\x20\x01(\x0b2\x12.google.rpc.StatusR\x06status\"\xc2\
    \x01\n\x0eGetTreeRequest\x12#\n\rinstance_name\x18\x01\x20\x01(\tR\x0cin\
    stanceName\x12O\n\x0broot_digest\x18\x02\x20\x01(\x0b2..google.devtools.\
    remoteexecution.v1test.DigestR\nrootDigest\x12\x1b\n\tpage_size\x18\x03\
    \x20\x01(\x05R\x08pageSize\x12\x1d\n\npage_token\x18\x04\x20\x01(\tR\tpa\
    geToken\"\x8e\x01\n\x0fGetTreeResponse\x12S\n\x0bdirectories\x18\x01\x20\
    \x03(\x0b21.google.devtools.remoteexecution.v1test.DirectoryR\x0bdirecto\
    ries\x12&\n\x0fnext_page_token\x18\x02\x20\x01(\tR\rnextPageToken\"M\n\
    \x0bToolDetails\x12\x1b\n\ttool_name\x18\x01\x20\x01(\tR\x08toolName\x12\
    !\n\x0ctool_version\x18\x02\x20\x01(\tR\x0btoolVersion\"\xf0\x01\n\x0fRe\
    questMetadata\x12V\n\x0ctool_details\x18\x01\x20\x01(\x0b23.google.devto\
    ols.remoteexecution.v1test.ToolDetailsR\x0btoolDetails\x12\x1b\n\taction\
    _id\x18\x02\x20\x01(\tR\x08actionId\x12,\n\x12tool_invocation_id\x18\x03\
    \x20\x01(\tR\x10toolInvocationId\x12:\n\x19correlated_invocations_id\x18\
    \x04\x20\x01(\tR\x17correlatedInvocationsId2\xa5\x01\n\tExecution\x12\
    \x97\x01\n\x07Execute\x126.google.devtools.remoteexecution.v1test.Execut\
    eRequest\x1a\x1d.google.longrunning.Operation\"5\x82\xd3\xe4\x93\x02/\"*\
    /v1test/{instance_name=**}/actions:execute:\x01*2\xfa\x03\n\x0bActionCac\
    he\x12\xe9\x01\n\x0fGetActionResult\x12>.google.devtools.remoteexecution\
    .v1test.GetActionResultRequest\x1a4.google.devtools.remoteexecution.v1te\
    st.ActionResult\"`\x82\xd3\xe4\x93\x02Z\x12X/v1test/{instance_name=**}/a\
    ctionResults/{action_digest.hash}/{action_digest.size_bytes}\x12\xfe\x01\
    \n\x12UpdateActionResult\x12A.google.devtools.remoteexecution.v1test.Upd\
    ateActionResultRequest\x1a4.google.devtools.remoteexecution.v1test.Actio\
    nResult\"o\x82\xd3\xe4\x93\x02i\x1aX/v1test/{instance_name=**}/actionRes\
    ults/{action_digest.hash}/{action_digest.size_bytes}:\raction_result2\
    \x98\x05\n\x19ContentAddressableStorage\x12\xce\x01\n\x10FindMissingBlob\
    s\x12?.google.devtools.remoteexecution.v1test.FindMissingBlobsRequest\
    \x1a@.google.devtools.remoteexecution.v1test.FindMissingBlobsResponse\"7\
    \x82\xd3\xe4\x93\x021\",/v1test/{instance_name=**}/blobs:findMissing:\
    \x01*\x12\xce\x01\n\x10BatchUpdateBlobs\x12?.google.devtools.remoteexecu\
    tion.v1test.BatchUpdateBlobsRequest\x1a@.google.devtools.remoteexecution\
    .v1test.BatchUpdateBlobsResponse\"7\x82\xd3\xe4\x93\x021\",/v1test/{inst\
    ance_name=**}/blobs:batchUpdate:\x01*\x12\xd8\x01\n\x07GetTree\x126.goog\
    le.devtools.remoteexecution.v1test.GetTreeRequest\x1a7.google.devtools.r\
    emoteexecution.v1test.GetTreeResponse\"\\\x82\xd3\xe4\x93\x02V\x12T/v1te\
    st/{instance_name=**}/blobs/{root_digest.hash}/{root_digest.size_bytes}:\
    getTreeB\xc1\x01\n*com.google.devtools.remoteexecution.v1testB\x14Remote\
    ExecutionProtoP\x01ZUgoogle.golang.org/genproto/googleapis/devtools/remo\
    teexecution/v1test;remoteexecution\xa2\x02\x03REX\xaa\x02\x1dGoogle.Remo\
    teExecution.V1TestJ\x82\xe4\x02\n\x07\x12\x05\x0e\0\xbc\x07\x01\n\xbd\
    \x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\
    \x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Versio\
    n\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08.\n\t\n\x02\x03\0\
    \x12\x03\x12\x07%\n\t\n\x02\x03\x01\x12\x03\x13\x07,\n\t\n\x02\x03\x02\
    \x12\x03\x14\x07'\n\t\n\x02\x03\x03\x12\x03\x15\x07\x20\n\x08\n\x01\x08\
    \x12\x03\x17\0:\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x17\0:\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x17\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x17\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x17\x07\x17\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x17\x1a9\n\x08\n\x01\x08\x12\x03\
    \x18\0l\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x18\0l\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x18\x07\x11\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x18\x07\x11\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x18\x07\x11\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x18\x14k\n\x08\n\x01\x08\x12\x03\
    \x19\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x19\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x19\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x19\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x19\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x19\x1d!\n\x08\n\x01\x08\x12\x03\
    \x1a\05\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x1a\05\n\x0c\n\x05\x08\xe7\
    \x07\x03\x02\x12\x03\x1a\x07\x1b\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\
    \x1a\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x1a\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x1a\x1e4\n\x08\n\x01\x08\x12\x03\
    \x1b\0C\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x1b\0C\n\x0c\n\x05\x08\xe7\
    \x07\x04\x02\x12\x03\x1b\x07\x13\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03\
    \x1b\x07\x13\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\x1b\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x04\x07\x12\x03\x1b\x16B\n\x08\n\x01\x08\x12\x03\
    \x1c\0!\n\x0b\n\x04\x08\xe7\x07\x05\x12\x03\x1c\0!\n\x0c\n\x05\x08\xe7\
    \x07\x05\x02\x12\x03\x1c\x07\x18\n\r\n\x06\x08\xe7\x07\x05\x02\0\x12\x03\
    \x1c\x07\x18\n\x0e\n\x07\x08\xe7\x07\x05\x02\0\x01\x12\x03\x1c\x07\x18\n\
    \x0c\n\x05\x08\xe7\x07\x05\x07\x12\x03\x1c\x1b\x20\n\x91\x03\n\x02\x06\0\
    \x12\x04'\0j\x01\x1a\x84\x03\x20The\x20Remote\x20Execution\x20API\x20is\
    \x20used\x20to\x20execute\x20an\n\x20[Action][google.devtools.remoteexec\
    ution.v1test.Action]\x20on\x20the\x20remote\n\x20workers.\n\n\x20As\x20w\
    ith\x20other\x20services\x20in\x20the\x20Remote\x20Execution\x20API,\x20\
    any\x20call\x20may\x20return\x20an\n\x20error\x20with\x20a\x20[RetryInfo\
    ][google.rpc.RetryInfo]\x20error\x20detail\x20providing\n\x20information\
    \x20about\x20when\x20the\x20client\x20should\x20retry\x20the\x20request;\
    \x20clients\x20SHOULD\n\x20respect\x20the\x20information\x20provided.\n\
    \n\n\n\x03\x06\0\x01\x12\x03'\x08\x11\n\xec\x1a\n\x04\x06\0\x02\0\x12\
    \x04g\x02i\x03\x1a\xdd\x1a\x20Execute\x20an\x20action\x20remotely.\n\n\
    \x20In\x20order\x20to\x20execute\x20an\x20action,\x20the\x20client\x20mu\
    st\x20first\x20upload\x20all\x20of\x20the\n\x20inputs,\x20as\x20well\x20\
    as\x20the\n\x20[Command][google.devtools.remoteexecution.v1test.Command]\
    \x20to\x20run,\x20into\x20the\n\x20[ContentAddressableStorage][google.de\
    vtools.remoteexecution.v1test.ContentAddressableStorage].\n\x20It\x20the\
    n\x20calls\x20`Execute`\x20with\x20an\n\x20[Action][google.devtools.remo\
    teexecution.v1test.Action]\x20referring\x20to\x20them.\n\x20The\x20serve\
    r\x20will\x20run\x20the\x20action\x20and\x20eventually\x20return\x20the\
    \x20result.\n\n\x20The\x20input\x20`Action`'s\x20fields\x20MUST\x20meet\
    \x20the\x20various\x20canonicalization\n\x20requirements\x20specified\
    \x20in\x20the\x20documentation\x20for\x20their\x20types\x20so\x20that\
    \x20it\x20has\n\x20the\x20same\x20digest\x20as\x20other\x20logically\x20\
    equivalent\x20`Action`s.\x20The\x20server\x20MAY\n\x20enforce\x20the\x20\
    requirements\x20and\x20return\x20errors\x20if\x20a\x20non-canonical\x20i\
    nput\x20is\n\x20received.\x20It\x20MAY\x20also\x20proceed\x20without\x20\
    verifying\x20some\x20or\x20all\x20of\x20the\n\x20requirements,\x20such\
    \x20as\x20for\x20performance\x20reasons.\x20If\x20the\x20server\x20does\
    \x20not\n\x20verify\x20the\x20requirement,\x20then\x20it\x20will\x20trea\
    t\x20the\x20`Action`\x20as\x20distinct\x20from\n\x20another\x20logically\
    \x20equivalent\x20action\x20if\x20they\x20hash\x20differently.\n\n\x20Re\
    turns\x20a\x20[google.longrunning.Operation][google.longrunning.Operatio\
    n]\n\x20describing\x20the\x20resulting\x20execution,\x20with\x20eventual\
    \x20`response`\n\x20[ExecuteResponse][google.devtools.remoteexecution.v1\
    test.ExecuteResponse].\n\x20The\x20`metadata`\x20on\x20the\x20operation\
    \x20is\x20of\x20type\n\x20[ExecuteOperationMetadata][google.devtools.rem\
    oteexecution.v1test.ExecuteOperationMetadata].\n\n\x20To\x20query\x20the\
    \x20operation,\x20you\x20can\x20use\x20the\n\x20[Operations\x20API][goog\
    le.longrunning.Operations.GetOperation].\x20If\x20you\x20wish\n\x20to\
    \x20allow\x20the\x20server\x20to\x20stream\x20operations\x20updates,\x20\
    rather\x20than\x20requiring\n\x20client\x20polling,\x20you\x20can\x20use\
    \x20the\n\x20[Watcher\x20API][google.watcher.v1.Watcher.Watch]\x20with\
    \x20the\x20Operation's\x20`name`\n\x20as\x20the\x20`target`.\n\n\x20When\
    \x20using\x20the\x20Watcher\x20API,\x20the\x20initial\x20`data`\x20will\
    \x20be\x20the\x20`Operation`\x20at\n\x20the\x20time\x20of\x20the\x20requ\
    est.\x20Updates\x20will\x20be\x20provided\x20periodically\x20by\x20the\n\
    \x20server\x20until\x20the\x20`Operation`\x20completes,\x20at\x20which\
    \x20point\x20the\x20response\x20message\n\x20will\x20(assuming\x20no\x20\
    error)\x20be\x20at\x20`data.response`.\n\n\x20The\x20server\x20NEED\x20N\
    OT\x20implement\x20other\x20methods\x20or\x20functionality\x20of\x20the\
    \n\x20Operation\x20and\x20Watcher\x20APIs.\n\n\x20Errors\x20discovered\
    \x20during\x20creation\x20of\x20the\x20`Operation`\x20will\x20be\x20repo\
    rted\n\x20as\x20gRPC\x20Status\x20errors,\x20while\x20errors\x20that\x20\
    occurred\x20while\x20running\x20the\n\x20action\x20will\x20be\x20reporte\
    d\x20in\x20the\x20`status`\x20field\x20of\x20the\x20`ExecuteResponse`.\
    \x20The\n\x20server\x20MUST\x20NOT\x20set\x20the\x20`error`\x20field\x20\
    of\x20the\x20`Operation`\x20proto.\n\x20The\x20possible\x20errors\x20inc\
    lude:\n\x20*\x20`INVALID_ARGUMENT`:\x20One\x20or\x20more\x20arguments\
    \x20are\x20invalid.\n\x20*\x20`FAILED_PRECONDITION`:\x20One\x20or\x20mor\
    e\x20errors\x20occurred\x20in\x20setting\x20up\x20the\n\x20\x20\x20actio\
    n\x20requested,\x20such\x20as\x20a\x20missing\x20input\x20or\x20command\
    \x20or\x20no\x20worker\x20being\n\x20\x20\x20available.\x20The\x20client\
    \x20may\x20be\x20able\x20to\x20fix\x20the\x20errors\x20and\x20retry.\n\
    \x20*\x20`RESOURCE_EXHAUSTED`:\x20There\x20is\x20insufficient\x20quota\
    \x20of\x20some\x20resource\x20to\x20run\n\x20\x20\x20the\x20action.\n\
    \x20*\x20`UNAVAILABLE`:\x20Due\x20to\x20a\x20transient\x20condition,\x20\
    such\x20as\x20all\x20workers\x20being\n\x20\x20\x20occupied\x20(and\x20t\
    he\x20server\x20does\x20not\x20support\x20a\x20queue),\x20the\x20action\
    \x20could\x20not\n\x20\x20\x20be\x20started.\x20The\x20client\x20should\
    \x20retry.\n\x20*\x20`INTERNAL`:\x20An\x20internal\x20error\x20occurred\
    \x20in\x20the\x20execution\x20engine\x20or\x20the\n\x20\x20\x20worker.\n\
    \x20*\x20`DEADLINE_EXCEEDED`:\x20The\x20execution\x20timed\x20out.\n\n\
    \x20In\x20the\x20case\x20of\x20a\x20missing\x20input\x20or\x20command,\
    \x20the\x20server\x20SHOULD\x20additionally\n\x20send\x20a\x20[Precondit\
    ionFailure][google.rpc.PreconditionFailure]\x20error\x20detail\n\x20wher\
    e,\x20for\x20each\x20requested\x20blob\x20not\x20present\x20in\x20the\
    \x20CAS,\x20there\x20is\x20a\n\x20`Violation`\x20with\x20a\x20`type`\x20\
    of\x20`MISSING`\x20and\x20a\x20`subject`\x20of\n\x20`\"blobs/{hash}/{siz\
    e}\"`\x20indicating\x20the\x20digest\x20of\x20the\x20missing\x20blob.\n\
    \n\x0c\n\x05\x06\0\x02\0\x01\x12\x03g\x06\r\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03g\x0e\x1c\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03g'C\n\x0c\n\x05\x06\
    \0\x02\0\x04\x12\x03h\x04`\n\x0f\n\x08\x06\0\x02\0\x04\xe7\x07\0\x12\x03\
    h\x04`\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\0\x02\x12\x03h\x0b\x1c\n\x11\n\
    \n\x06\0\x02\0\x04\xe7\x07\0\x02\0\x12\x03h\x0b\x1c\n\x12\n\x0b\x06\0\
    \x02\0\x04\xe7\x07\0\x02\0\x01\x12\x03h\x0c\x1b\n\x10\n\t\x06\0\x02\0\
    \x04\xe7\x07\0\x08\x12\x03h\x1f_\n\x81\t\n\x02\x06\x01\x12\x05\x7f\0\x96\
    \x01\x01\x1a\xf3\x08\x20The\x20action\x20cache\x20API\x20is\x20used\x20t\
    o\x20query\x20whether\x20a\x20given\x20action\x20has\x20already\x20been\
    \n\x20performed\x20and,\x20if\x20so,\x20retrieve\x20its\x20result.\x20Un\
    like\x20the\n\x20[ContentAddressableStorage][google.devtools.remoteexecu\
    tion.v1test.ContentAddressableStorage],\n\x20which\x20addresses\x20blobs\
    \x20by\x20their\x20own\x20content,\x20the\x20action\x20cache\x20addresse\
    s\x20the\n\x20[ActionResult][google.devtools.remoteexecution.v1test.Acti\
    onResult]\x20by\x20a\n\x20digest\x20of\x20the\x20encoded\x20[Action][goo\
    gle.devtools.remoteexecution.v1test.Action]\n\x20which\x20produced\x20th\
    em.\n\n\x20The\x20lifetime\x20of\x20entries\x20in\x20the\x20action\x20ca\
    che\x20is\x20implementation-specific,\x20but\n\x20the\x20server\x20SHOUL\
    D\x20assume\x20that\x20more\x20recently\x20used\x20entries\x20are\x20mor\
    e\x20likely\x20to\n\x20be\x20used\x20again.\x20Additionally,\x20action\
    \x20cache\x20implementations\x20SHOULD\x20ensure\x20that\n\x20any\x20blo\
    bs\x20referenced\x20in\x20the\n\x20[ContentAddressableStorage][google.de\
    vtools.remoteexecution.v1test.ContentAddressableStorage]\n\x20are\x20sti\
    ll\x20valid\x20when\x20returning\x20a\x20result.\n\n\x20As\x20with\x20ot\
    her\x20services\x20in\x20the\x20Remote\x20Execution\x20API,\x20any\x20ca\
    ll\x20may\x20return\x20an\n\x20error\x20with\x20a\x20[RetryInfo][google.\
    rpc.RetryInfo]\x20error\x20detail\x20providing\n\x20information\x20about\
    \x20when\x20the\x20client\x20should\x20retry\x20the\x20request;\x20clien\
    ts\x20SHOULD\n\x20respect\x20the\x20information\x20provided.\n\n\n\n\x03\
    \x06\x01\x01\x12\x03\x7f\x08\x13\n\x81\x01\n\x04\x06\x01\x02\0\x12\x06\
    \x84\x01\x02\x86\x01\x03\x1aq\x20Retrieve\x20a\x20cached\x20execution\
    \x20result.\n\n\x20Errors:\n\x20*\x20`NOT_FOUND`:\x20The\x20requested\
    \x20`ActionResult`\x20is\x20not\x20in\x20the\x20cache.\n\n\r\n\x05\x06\
    \x01\x02\0\x01\x12\x04\x84\x01\x06\x15\n\r\n\x05\x06\x01\x02\0\x02\x12\
    \x04\x84\x01\x16,\n\r\n\x05\x06\x01\x02\0\x03\x12\x04\x84\x017C\n\x0e\n\
    \x05\x06\x01\x02\0\x04\x12\x05\x85\x01\x04\x83\x01\n\x11\n\x08\x06\x01\
    \x02\0\x04\xe7\x07\0\x12\x05\x85\x01\x04\x83\x01\n\x11\n\t\x06\x01\x02\0\
    \x04\xe7\x07\0\x02\x12\x04\x85\x01\x0b\x1c\n\x12\n\n\x06\x01\x02\0\x04\
    \xe7\x07\0\x02\0\x12\x04\x85\x01\x0b\x1c\n\x13\n\x0b\x06\x01\x02\0\x04\
    \xe7\x07\0\x02\0\x01\x12\x04\x85\x01\x0c\x1b\n\x12\n\t\x06\x01\x02\0\x04\
    \xe7\x07\0\x08\x12\x05\x85\x01\x1f\x82\x01\n\xb9\x03\n\x04\x06\x01\x02\
    \x01\x12\x06\x93\x01\x02\x95\x01\x03\x1a\xa8\x03\x20Upload\x20a\x20new\
    \x20execution\x20result.\n\n\x20This\x20method\x20is\x20intended\x20for\
    \x20servers\x20which\x20implement\x20the\x20distributed\x20cache\n\x20in\
    dependently\x20of\x20the\n\x20[Execution][google.devtools.remoteexecutio\
    n.v1test.Execution]\x20API.\x20As\x20a\n\x20result,\x20it\x20is\x20OPTIO\
    NAL\x20for\x20servers\x20to\x20implement.\n\n\x20Errors:\n\x20*\x20`NOT_\
    IMPLEMENTED`:\x20This\x20method\x20is\x20not\x20supported\x20by\x20the\
    \x20server.\n\x20*\x20`RESOURCE_EXHAUSTED`:\x20There\x20is\x20insufficie\
    nt\x20storage\x20space\x20to\x20add\x20the\n\x20\x20\x20entry\x20to\x20t\
    he\x20cache.\n\n\r\n\x05\x06\x01\x02\x01\x01\x12\x04\x93\x01\x06\x18\n\r\
    \n\x05\x06\x01\x02\x01\x02\x12\x04\x93\x01\x192\n\r\n\x05\x06\x01\x02\
    \x01\x03\x12\x04\x93\x01=I\n\x0e\n\x05\x06\x01\x02\x01\x04\x12\x05\x94\
    \x01\x04\x99\x01\n\x11\n\x08\x06\x01\x02\x01\x04\xe7\x07\0\x12\x05\x94\
    \x01\x04\x99\x01\n\x11\n\t\x06\x01\x02\x01\x04\xe7\x07\0\x02\x12\x04\x94\
    \x01\x0b\x1c\n\x12\n\n\x06\x01\x02\x01\x04\xe7\x07\0\x02\0\x12\x04\x94\
    \x01\x0b\x1c\n\x13\n\x0b\x06\x01\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x04\
    \x94\x01\x0c\x1b\n\x12\n\t\x06\x01\x02\x01\x04\xe7\x07\0\x08\x12\x05\x94\
    \x01\x1f\x98\x01\n\xe4\x1f\n\x02\x06\x02\x12\x06\xd9\x01\0\xff\x01\x01\
    \x1a\xd5\x1f\x20The\x20CAS\x20(content-addressable\x20storage)\x20is\x20\
    used\x20to\x20store\x20the\x20inputs\x20to\x20and\n\x20outputs\x20from\
    \x20the\x20execution\x20service.\x20Each\x20piece\x20of\x20content\x20is\
    \x20addressed\x20by\x20the\n\x20digest\x20of\x20its\x20binary\x20data.\n\
    \n\x20Most\x20of\x20the\x20binary\x20data\x20stored\x20in\x20the\x20CAS\
    \x20is\x20opaque\x20to\x20the\x20execution\x20engine,\n\x20and\x20is\x20\
    only\x20used\x20as\x20a\x20communication\x20medium.\x20In\x20order\x20to\
    \x20build\x20an\n\x20[Action][google.devtools.remoteexecution.v1test.Act\
    ion],\n\x20however,\x20the\x20client\x20will\x20need\x20to\x20also\x20up\
    load\x20the\n\x20[Command][google.devtools.remoteexecution.v1test.Comman\
    d]\x20and\x20input\x20root\n\x20[Directory][google.devtools.remoteexecut\
    ion.v1test.Directory]\x20for\x20the\x20Action.\n\x20The\x20Command\x20an\
    d\x20Directory\x20messages\x20must\x20be\x20marshalled\x20to\x20wire\x20\
    format\x20and\x20then\n\x20uploaded\x20under\x20the\x20hash\x20as\x20wit\
    h\x20any\x20other\x20piece\x20of\x20content.\x20In\x20practice,\x20the\n\
    \x20input\x20root\x20directory\x20is\x20likely\x20to\x20refer\x20to\x20o\
    ther\x20Directories\x20in\x20its\n\x20hierarchy,\x20which\x20must\x20als\
    o\x20each\x20be\x20uploaded\x20on\x20their\x20own.\n\n\x20For\x20small\
    \x20file\x20uploads\x20the\x20client\x20should\x20group\x20them\x20toget\
    her\x20and\x20call\n\x20[BatchUpdateBlobs][google.devtools.remoteexecuti\
    on.v1test.ContentAddressableStorage.BatchUpdateBlobs]\n\x20on\x20chunks\
    \x20of\x20no\x20more\x20than\x2010\x20MiB.\x20For\x20large\x20uploads,\
    \x20the\x20client\x20must\x20use\x20the\n\x20[Write\x20method][google.by\
    testream.ByteStream.Write]\x20of\x20the\x20ByteStream\x20API.\x20The\n\
    \x20`resource_name`\x20is\x20`{instance_name}/uploads/{uuid}/blobs/{hash\
    }/{size}`,\n\x20where\x20`instance_name`\x20is\x20as\x20described\x20in\
    \x20the\x20next\x20paragraph,\x20`uuid`\x20is\x20a\n\x20version\x204\x20\
    UUID\x20generated\x20by\x20the\x20client,\x20and\x20`hash`\x20and\x20`si\
    ze`\x20are\x20the\n\x20[Digest][google.devtools.remoteexecution.v1test.D\
    igest]\x20of\x20the\x20blob.\x20The\n\x20`uuid`\x20is\x20used\x20only\
    \x20to\x20avoid\x20collisions\x20when\x20multiple\x20clients\x20try\x20t\
    o\x20upload\n\x20the\x20same\x20file\x20(or\x20the\x20same\x20client\x20\
    tries\x20to\x20upload\x20the\x20file\x20multiple\x20times\x20at\n\x20onc\
    e\x20on\x20different\x20threads),\x20so\x20the\x20client\x20MAY\x20reuse\
    \x20the\x20`uuid`\x20for\x20uploading\n\x20different\x20blobs.\x20The\
    \x20`resource_name`\x20may\x20optionally\x20have\x20a\x20trailing\x20fil\
    ename\n\x20(or\x20other\x20metadata)\x20for\x20a\x20client\x20to\x20use\
    \x20if\x20it\x20is\x20storing\x20URLs,\x20as\x20in\n\x20`{instance}/uplo\
    ads/{uuid}/blobs/{hash}/{size}/foo/bar/baz.cc`.\x20Anything\n\x20after\
    \x20the\x20`size`\x20is\x20ignored.\n\n\x20A\x20single\x20server\x20MAY\
    \x20support\x20multiple\x20instances\x20of\x20the\x20execution\x20system\
    ,\x20each\n\x20with\x20their\x20own\x20workers,\x20storage,\x20cache,\
    \x20etc.\x20The\x20exact\x20relationship\x20between\n\x20instances\x20is\
    \x20up\x20to\x20the\x20server.\x20If\x20the\x20server\x20does,\x20then\
    \x20the\x20`instance_name`\n\x20is\x20an\x20identifier,\x20possibly\x20c\
    ontaining\x20multiple\x20path\x20segments,\x20used\x20to\n\x20distinguis\
    h\x20between\x20the\x20various\x20instances\x20on\x20the\x20server,\x20i\
    n\x20a\x20manner\x20defined\n\x20by\x20the\x20server.\x20For\x20servers\
    \x20which\x20do\x20not\x20support\x20multiple\x20instances,\x20then\x20t\
    he\n\x20`instance_name`\x20is\x20the\x20empty\x20path\x20and\x20the\x20l\
    eading\x20slash\x20is\x20omitted,\x20so\x20that\n\x20the\x20`resource_na\
    me`\x20becomes\x20`uploads/{uuid}/blobs/{hash}/{size}`.\n\n\x20When\x20a\
    ttempting\x20an\x20upload,\x20if\x20another\x20client\x20has\x20already\
    \x20completed\x20the\x20upload\n\x20(which\x20may\x20occur\x20in\x20the\
    \x20middle\x20of\x20a\x20single\x20upload\x20if\x20another\x20client\x20\
    uploads\n\x20the\x20same\x20blob\x20concurrently),\x20the\x20request\x20\
    will\x20terminate\x20immediately\x20with\n\x20a\x20response\x20whose\x20\
    `committed_size`\x20is\x20the\x20full\x20size\x20of\x20the\x20uploaded\
    \x20file\n\x20(regardless\x20of\x20how\x20much\x20data\x20was\x20transmi\
    tted\x20by\x20the\x20client).\x20If\x20the\x20client\n\x20completes\x20t\
    he\x20upload\x20but\x20the\n\x20[Digest][google.devtools.remoteexecution\
    .v1test.Digest]\x20does\x20not\x20match,\x20an\n\x20`INVALID_ARGUMENT`\
    \x20error\x20will\x20be\x20returned.\x20In\x20either\x20case,\x20the\x20\
    client\x20should\n\x20not\x20attempt\x20to\x20retry\x20the\x20upload.\n\
    \n\x20For\x20downloading\x20blobs,\x20the\x20client\x20must\x20use\x20th\
    e\n\x20[Read\x20method][google.bytestream.ByteStream.Read]\x20of\x20the\
    \x20ByteStream\x20API,\x20with\n\x20a\x20`resource_name`\x20of\x20`\"{in\
    stance_name}/blobs/{hash}/{size}\"`,\x20where\n\x20`instance_name`\x20is\
    \x20the\x20instance\x20name\x20(see\x20above),\x20and\x20`hash`\x20and\
    \x20`size`\x20are\n\x20the\x20[Digest][google.devtools.remoteexecution.v\
    1test.Digest]\x20of\x20the\x20blob.\n\n\x20The\x20lifetime\x20of\x20entr\
    ies\x20in\x20the\x20CAS\x20is\x20implementation\x20specific,\x20but\x20i\
    t\x20SHOULD\n\x20be\x20long\x20enough\x20to\x20allow\x20for\x20newly-add\
    ed\x20and\x20recently\x20looked-up\x20entries\x20to\x20be\n\x20used\x20i\
    n\x20subsequent\x20calls\x20(e.g.\x20to\n\x20[Execute][google.devtools.r\
    emoteexecution.v1test.Execution.Execute]).\n\n\x20As\x20with\x20other\
    \x20services\x20in\x20the\x20Remote\x20Execution\x20API,\x20any\x20call\
    \x20may\x20return\x20an\n\x20error\x20with\x20a\x20[RetryInfo][google.rp\
    c.RetryInfo]\x20error\x20detail\x20providing\n\x20information\x20about\
    \x20when\x20the\x20client\x20should\x20retry\x20the\x20request;\x20clien\
    ts\x20SHOULD\n\x20respect\x20the\x20information\x20provided.\n\n\x0b\n\
    \x03\x06\x02\x01\x12\x04\xd9\x01\x08!\n\xf4\x01\n\x04\x06\x02\x02\0\x12\
    \x06\xe0\x01\x02\xe2\x01\x03\x1a\xe3\x01\x20Determine\x20if\x20blobs\x20\
    are\x20present\x20in\x20the\x20CAS.\n\n\x20Clients\x20can\x20use\x20this\
    \x20API\x20before\x20uploading\x20blobs\x20to\x20determine\x20which\x20o\
    nes\x20are\n\x20already\x20present\x20in\x20the\x20CAS\x20and\x20do\x20n\
    ot\x20need\x20to\x20be\x20uploaded\x20again.\n\n\x20There\x20are\x20no\
    \x20method-specific\x20errors.\n\n\r\n\x05\x06\x02\x02\0\x01\x12\x04\xe0\
    \x01\x06\x16\n\r\n\x05\x06\x02\x02\0\x02\x12\x04\xe0\x01\x17.\n\r\n\x05\
    \x06\x02\x02\0\x03\x12\x04\xe0\x019Q\n\r\n\x05\x06\x02\x02\0\x04\x12\x04\
    \xe1\x01\x04b\n\x10\n\x08\x06\x02\x02\0\x04\xe7\x07\0\x12\x04\xe1\x01\
    \x04b\n\x11\n\t\x06\x02\x02\0\x04\xe7\x07\0\x02\x12\x04\xe1\x01\x0b\x1c\
    \n\x12\n\n\x06\x02\x02\0\x04\xe7\x07\0\x02\0\x12\x04\xe1\x01\x0b\x1c\n\
    \x13\n\x0b\x06\x02\x02\0\x04\xe7\x07\0\x02\0\x01\x12\x04\xe1\x01\x0c\x1b\
    \n\x11\n\t\x06\x02\x02\0\x04\xe7\x07\0\x08\x12\x04\xe1\x01\x1fa\n\x95\
    \x06\n\x04\x06\x02\x02\x01\x12\x06\xf7\x01\x02\xf9\x01\x03\x1a\x84\x06\
    \x20Upload\x20many\x20blobs\x20at\x20once.\n\n\x20The\x20client\x20MUST\
    \x20NOT\x20upload\x20blobs\x20with\x20a\x20combined\x20total\x20size\x20\
    of\x20more\x20than\x2010\n\x20MiB\x20using\x20this\x20API.\x20Such\x20re\
    quests\x20should\x20either\x20be\x20split\x20into\x20smaller\n\x20chunks\
    \x20or\x20uploaded\x20using\x20the\n\x20[ByteStream\x20API][google.bytes\
    tream.ByteStream],\x20as\x20appropriate.\n\n\x20This\x20request\x20is\
    \x20equivalent\x20to\x20calling\x20[UpdateBlob][]\x20on\x20each\x20indiv\
    idual\n\x20blob,\x20in\x20parallel.\x20The\x20requests\x20may\x20succeed\
    \x20or\x20fail\x20independently.\n\n\x20Errors:\n\x20*\x20`INVALID_ARGUM\
    ENT`:\x20The\x20client\x20attempted\x20to\x20upload\x20more\x20than\x201\
    0\x20MiB\x20of\n\x20\x20\x20data.\n\n\x20Individual\x20requests\x20may\
    \x20return\x20the\x20following\x20errors,\x20additionally:\n\x20*\x20`RE\
    SOURCE_EXHAUSTED`:\x20There\x20is\x20insufficient\x20disk\x20quota\x20to\
    \x20store\x20the\x20blob.\n\x20*\x20`INVALID_ARGUMENT`:\x20The\n\x20[Dig\
    est][google.devtools.remoteexecution.v1test.Digest]\x20does\x20not\x20ma\
    tch\x20the\n\x20provided\x20data.\n\n\r\n\x05\x06\x02\x02\x01\x01\x12\
    \x04\xf7\x01\x06\x16\n\r\n\x05\x06\x02\x02\x01\x02\x12\x04\xf7\x01\x17.\
    \n\r\n\x05\x06\x02\x02\x01\x03\x12\x04\xf7\x019Q\n\r\n\x05\x06\x02\x02\
    \x01\x04\x12\x04\xf8\x01\x04b\n\x10\n\x08\x06\x02\x02\x01\x04\xe7\x07\0\
    \x12\x04\xf8\x01\x04b\n\x11\n\t\x06\x02\x02\x01\x04\xe7\x07\0\x02\x12\
    \x04\xf8\x01\x0b\x1c\n\x12\n\n\x06\x02\x02\x01\x04\xe7\x07\0\x02\0\x12\
    \x04\xf8\x01\x0b\x1c\n\x13\n\x0b\x06\x02\x02\x01\x04\xe7\x07\0\x02\0\x01\
    \x12\x04\xf8\x01\x0c\x1b\n\x11\n\t\x06\x02\x02\x01\x04\xe7\x07\0\x08\x12\
    \x04\xf8\x01\x1fa\nU\n\x04\x06\x02\x02\x02\x12\x06\xfc\x01\x02\xfe\x01\
    \x03\x1aE\x20DEPRECATED:\x20This\x20method\x20is\x20deprecated\x20and\
    \x20should\x20no\x20longer\x20be\x20used.\n\n\r\n\x05\x06\x02\x02\x02\
    \x01\x12\x04\xfc\x01\x06\r\n\r\n\x05\x06\x02\x02\x02\x02\x12\x04\xfc\x01\
    \x0e\x1c\n\r\n\x05\x06\x02\x02\x02\x03\x12\x04\xfc\x01'6\n\r\n\x05\x06\
    \x02\x02\x02\x04\x12\x04\xfd\x01\x04\x7f\n\x10\n\x08\x06\x02\x02\x02\x04\
    \xe7\x07\0\x12\x04\xfd\x01\x04\x7f\n\x11\n\t\x06\x02\x02\x02\x04\xe7\x07\
    \0\x02\x12\x04\xfd\x01\x0b\x1c\n\x12\n\n\x06\x02\x02\x02\x04\xe7\x07\0\
    \x02\0\x12\x04\xfd\x01\x0b\x1c\n\x13\n\x0b\x06\x02\x02\x02\x04\xe7\x07\0\
    \x02\0\x01\x12\x04\xfd\x01\x0c\x1b\n\x11\n\t\x06\x02\x02\x02\x04\xe7\x07\
    \0\x08\x12\x04\xfd\x01\x1f~\n\xf5\x08\n\x02\x04\0\x12\x06\x94\x02\0\xdc\
    \x02\x01\x1a\xe6\x08\x20An\x20`Action`\x20captures\x20all\x20the\x20info\
    rmation\x20about\x20an\x20execution\x20which\x20is\x20required\n\x20to\
    \x20reproduce\x20it.\n\n\x20`Action`s\x20are\x20the\x20core\x20component\
    \x20of\x20the\x20[Execution]\x20service.\x20A\x20single\n\x20`Action`\
    \x20represents\x20a\x20repeatable\x20action\x20that\x20can\x20be\x20perf\
    ormed\x20by\x20the\n\x20execution\x20service.\x20`Action`s\x20can\x20be\
    \x20succinctly\x20identified\x20by\x20the\x20digest\x20of\n\x20their\x20\
    wire\x20format\x20encoding\x20and,\x20once\x20an\x20`Action`\x20has\x20b\
    een\x20executed,\x20will\x20be\n\x20cached\x20in\x20the\x20action\x20cac\
    he.\x20Future\x20requests\x20can\x20then\x20use\x20the\x20cached\x20resu\
    lt\n\x20rather\x20than\x20needing\x20to\x20run\x20afresh.\n\n\x20When\
    \x20a\x20server\x20completes\x20execution\x20of\x20an\n\x20[Action][goog\
    le.devtools.remoteexecution.v1test.Action],\x20it\x20MAY\x20choose\x20to\
    \n\x20cache\x20the\x20[result][google.devtools.remoteexecution.v1test.Ac\
    tionResult]\x20in\n\x20the\x20[ActionCache][google.devtools.remoteexecut\
    ion.v1test.ActionCache]\x20unless\n\x20`do_not_cache`\x20is\x20`true`.\
    \x20Clients\x20SHOULD\x20expect\x20the\x20server\x20to\x20do\x20so.\x20B\
    y\n\x20default,\x20future\x20calls\x20to\x20[Execute][]\x20the\x20same\
    \x20`Action`\x20will\x20also\x20serve\x20their\n\x20results\x20from\x20t\
    he\x20cache.\x20Clients\x20must\x20take\x20care\x20to\x20understand\x20t\
    he\x20caching\n\x20behaviour.\x20Ideally,\x20all\x20`Action`s\x20will\
    \x20be\x20reproducible\x20so\x20that\x20serving\x20a\n\x20result\x20from\
    \x20cache\x20is\x20always\x20desirable\x20and\x20correct.\n\n\x0b\n\x03\
    \x04\0\x01\x12\x04\x94\x02\x08\x0e\n\xe2\x01\n\x04\x04\0\x02\0\x12\x04\
    \x98\x02\x02\x1c\x1a\xd3\x01\x20The\x20digest\x20of\x20the\x20[Command][\
    google.devtools.remoteexecution.v1test.Command]\n\x20to\x20run,\x20which\
    \x20MUST\x20be\x20present\x20in\x20the\n\x20[ContentAddressableStorage][\
    google.devtools.remoteexecution.v1test.ContentAddressableStorage].\n\n\
    \x0f\n\x05\x04\0\x02\0\x04\x12\x06\x98\x02\x02\x94\x02\x10\n\r\n\x05\x04\
    \0\x02\0\x06\x12\x04\x98\x02\x02\x08\n\r\n\x05\x04\0\x02\0\x01\x12\x04\
    \x98\x02\t\x17\n\r\n\x05\x04\0\x02\0\x03\x12\x04\x98\x02\x1a\x1b\n\xb9\
    \x03\n\x04\x04\0\x02\x01\x12\x04\xa1\x02\x02\x1f\x1a\xaa\x03\x20The\x20d\
    igest\x20of\x20the\x20root\n\x20[Directory][google.devtools.remoteexecut\
    ion.v1test.Directory]\x20for\x20the\x20input\n\x20files.\x20The\x20files\
    \x20in\x20the\x20directory\x20tree\x20are\x20available\x20in\x20the\x20c\
    orrect\n\x20location\x20on\x20the\x20build\x20machine\x20before\x20the\
    \x20command\x20is\x20executed.\x20The\x20root\n\x20directory,\x20as\x20w\
    ell\x20as\x20every\x20subdirectory\x20and\x20content\x20blob\x20referred\
    \x20to,\x20MUST\n\x20be\x20in\x20the\n\x20[ContentAddressableStorage][go\
    ogle.devtools.remoteexecution.v1test.ContentAddressableStorage].\n\n\x0f\
    \n\x05\x04\0\x02\x01\x04\x12\x06\xa1\x02\x02\x98\x02\x1c\n\r\n\x05\x04\0\
    \x02\x01\x06\x12\x04\xa1\x02\x02\x08\n\r\n\x05\x04\0\x02\x01\x01\x12\x04\
    \xa1\x02\t\x1a\n\r\n\x05\x04\0\x02\x01\x03\x12\x04\xa1\x02\x1d\x1e\n\x81\
    \x05\n\x04\x04\0\x02\x02\x12\x04\xaf\x02\x02#\x1a\xf2\x04\x20A\x20list\
    \x20of\x20the\x20output\x20files\x20that\x20the\x20client\x20expects\x20\
    to\x20retrieve\x20from\x20the\n\x20action.\x20Only\x20the\x20listed\x20f\
    iles,\x20as\x20well\x20as\x20directories\x20listed\x20in\n\x20`output_di\
    rectories`,\x20will\x20be\x20returned\x20to\x20the\x20client\x20as\x20ou\
    tput.\n\x20Other\x20files\x20that\x20may\x20be\x20created\x20during\x20c\
    ommand\x20execution\x20are\x20discarded.\n\n\x20The\x20paths\x20are\x20s\
    pecified\x20using\x20forward\x20slashes\x20(`/`)\x20as\x20path\x20separa\
    tors,\n\x20even\x20if\x20the\x20execution\x20platform\x20natively\x20use\
    s\x20a\x20different\x20separator.\x20The\n\x20path\x20MUST\x20NOT\x20inc\
    lude\x20a\x20trailing\x20slash.\n\n\x20In\x20order\x20to\x20ensure\x20co\
    nsistent\x20hashing\x20of\x20the\x20same\x20Action,\x20the\x20output\x20\
    paths\n\x20MUST\x20be\x20sorted\x20lexicographically\x20by\x20code\x20po\
    int\x20(or,\x20equivalently,\x20by\x20UTF-8\n\x20bytes).\n\n\r\n\x05\x04\
    \0\x02\x02\x04\x12\x04\xaf\x02\x02\n\n\r\n\x05\x04\0\x02\x02\x05\x12\x04\
    \xaf\x02\x0b\x11\n\r\n\x05\x04\0\x02\x02\x01\x12\x04\xaf\x02\x12\x1e\n\r\
    \n\x05\x04\0\x02\x02\x03\x12\x04\xaf\x02!\"\n\xc3\x06\n\x04\x04\0\x02\
    \x03\x12\x04\xc0\x02\x02)\x1a\xb4\x06\x20A\x20list\x20of\x20the\x20outpu\
    t\x20directories\x20that\x20the\x20client\x20expects\x20to\x20retrieve\
    \x20from\n\x20the\x20action.\x20Only\x20the\x20contents\x20of\x20the\x20\
    indicated\x20directories\x20(recursively\n\x20including\x20the\x20conten\
    ts\x20of\x20their\x20subdirectories)\x20will\x20be\n\x20returned,\x20as\
    \x20well\x20as\x20files\x20listed\x20in\x20`output_files`.\x20Other\x20f\
    iles\x20that\x20may\n\x20be\x20created\x20during\x20command\x20execution\
    \x20are\x20discarded.\n\n\x20The\x20paths\x20are\x20specified\x20using\
    \x20forward\x20slashes\x20(`/`)\x20as\x20path\x20separators,\n\x20even\
    \x20if\x20the\x20execution\x20platform\x20natively\x20uses\x20a\x20diffe\
    rent\x20separator.\x20The\n\x20path\x20MUST\x20NOT\x20include\x20a\x20tr\
    ailing\x20slash,\x20unless\x20the\x20path\x20is\x20`\"/\"`\x20(which,\n\
    \x20although\x20not\x20recommended,\x20can\x20be\x20used\x20to\x20captur\
    e\x20the\x20entire\x20working\n\x20directory\x20tree,\x20including\x20in\
    puts).\n\n\x20In\x20order\x20to\x20ensure\x20consistent\x20hashing\x20of\
    \x20the\x20same\x20Action,\x20the\x20output\x20paths\n\x20MUST\x20be\x20\
    sorted\x20lexicographically\x20by\x20code\x20point\x20(or,\x20equivalent\
    ly,\x20by\x20UTF-8\n\x20bytes).\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\
    \xc0\x02\x02\n\n\r\n\x05\x04\0\x02\x03\x05\x12\x04\xc0\x02\x0b\x11\n\r\n\
    \x05\x04\0\x02\x03\x01\x12\x04\xc0\x02\x12$\n\r\n\x05\x04\0\x02\x03\x03\
    \x12\x04\xc0\x02'(\n\x85\x02\n\x04\x04\0\x02\x04\x12\x04\xc6\x02\x02\x18\
    \x1a\xf6\x01\x20The\x20platform\x20requirements\x20for\x20the\x20executi\
    on\x20environment.\x20The\x20server\x20MAY\n\x20choose\x20to\x20execute\
    \x20the\x20action\x20on\x20any\x20worker\x20satisfying\x20the\x20require\
    ments,\x20so\n\x20the\x20client\x20SHOULD\x20ensure\x20that\x20running\
    \x20the\x20action\x20on\x20any\x20such\x20worker\x20will\n\x20have\x20th\
    e\x20same\x20result.\n\n\x0f\n\x05\x04\0\x02\x04\x04\x12\x06\xc6\x02\x02\
    \xc0\x02)\n\r\n\x05\x04\0\x02\x04\x06\x12\x04\xc6\x02\x02\n\n\r\n\x05\
    \x04\0\x02\x04\x01\x12\x04\xc6\x02\x0b\x13\n\r\n\x05\x04\0\x02\x04\x03\
    \x12\x04\xc6\x02\x16\x17\n\xf3\x07\n\x04\x04\0\x02\x05\x12\x04\xd8\x02\
    \x02'\x1a\xe4\x07\x20A\x20timeout\x20after\x20which\x20the\x20execution\
    \x20should\x20be\x20killed.\x20If\x20the\x20timeout\x20is\n\x20absent,\
    \x20then\x20the\x20client\x20is\x20specifying\x20that\x20the\x20executio\
    n\x20should\x20continue\n\x20as\x20long\x20as\x20the\x20server\x20will\
    \x20let\x20it.\x20The\x20server\x20SHOULD\x20impose\x20a\x20timeout\x20i\
    f\n\x20the\x20client\x20does\x20not\x20specify\x20one,\x20however,\x20if\
    \x20the\x20client\x20does\x20specify\x20a\n\x20timeout\x20that\x20is\x20\
    longer\x20than\x20the\x20server's\x20maximum\x20timeout,\x20the\x20serve\
    r\x20MUST\n\x20reject\x20the\x20request.\n\n\x20The\x20timeout\x20is\x20\
    a\x20part\x20of\x20the\n\x20[Action][google.devtools.remoteexecution.v1t\
    est.Action]\x20message,\x20and\n\x20therefore\x20two\x20`Actions`\x20wit\
    h\x20different\x20timeouts\x20are\x20different,\x20even\x20if\x20they\n\
    \x20are\x20otherwise\x20identical.\x20This\x20is\x20because,\x20if\x20th\
    ey\x20were\x20not,\x20running\x20an\n\x20`Action`\x20with\x20a\x20lower\
    \x20timeout\x20than\x20is\x20required\x20might\x20result\x20in\x20a\x20c\
    ache\x20hit\n\x20from\x20an\x20execution\x20run\x20with\x20a\x20longer\
    \x20timeout,\x20hiding\x20the\x20fact\x20that\x20the\n\x20timeout\x20is\
    \x20too\x20short.\x20By\x20encoding\x20it\x20directly\x20in\x20the\x20`A\
    ction`,\x20a\x20lower\n\x20timeout\x20will\x20result\x20in\x20a\x20cache\
    \x20miss\x20and\x20the\x20execution\x20timeout\x20will\x20fail\n\x20imme\
    diately,\x20rather\x20than\x20whenever\x20the\x20cache\x20entry\x20gets\
    \x20evicted.\n\n\x0f\n\x05\x04\0\x02\x05\x04\x12\x06\xd8\x02\x02\xc6\x02\
    \x18\n\r\n\x05\x04\0\x02\x05\x06\x12\x04\xd8\x02\x02\x1a\n\r\n\x05\x04\0\
    \x02\x05\x01\x12\x04\xd8\x02\x1b\"\n\r\n\x05\x04\0\x02\x05\x03\x12\x04\
    \xd8\x02%&\nE\n\x04\x04\0\x02\x06\x12\x04\xdb\x02\x02\x18\x1a7\x20If\x20\
    true,\x20then\x20the\x20`Action`'s\x20result\x20cannot\x20be\x20cached.\
    \n\n\x0f\n\x05\x04\0\x02\x06\x04\x12\x06\xdb\x02\x02\xd8\x02'\n\r\n\x05\
    \x04\0\x02\x06\x05\x12\x04\xdb\x02\x02\x06\n\r\n\x05\x04\0\x02\x06\x01\
    \x12\x04\xdb\x02\x07\x13\n\r\n\x05\x04\0\x02\x06\x03\x12\x04\xdb\x02\x16\
    \x17\n\xef\x02\n\x02\x04\x01\x12\x06\xe4\x02\0\x80\x03\x01\x1a\xe0\x02\
    \x20A\x20`Command`\x20is\x20the\x20actual\x20command\x20executed\x20by\
    \x20a\x20worker\x20running\x20an\n\x20[Action][google.devtools.remoteexe\
    cution.v1test.Action].\n\n\x20Except\x20as\x20otherwise\x20required,\x20\
    the\x20environment\x20(such\x20as\x20which\x20system\n\x20libraries\x20o\
    r\x20binaries\x20are\x20available,\x20and\x20what\x20filesystems\x20are\
    \x20mounted\x20where)\n\x20is\x20defined\x20by\x20and\x20specific\x20to\
    \x20the\x20implementation\x20of\x20the\x20remote\x20execution\x20API.\n\
    \n\x0b\n\x03\x04\x01\x01\x12\x04\xe4\x02\x08\x0f\nh\n\x04\x04\x01\x03\0\
    \x12\x06\xe7\x02\x02\xed\x02\x03\x1aX\x20An\x20`EnvironmentVariable`\x20\
    is\x20one\x20variable\x20to\x20set\x20in\x20the\x20running\x20program's\
    \n\x20environment.\n\n\r\n\x05\x04\x01\x03\0\x01\x12\x04\xe7\x02\n\x1d\n\
    $\n\x06\x04\x01\x03\0\x02\0\x12\x04\xe9\x02\x04\x14\x1a\x14\x20The\x20va\
    riable\x20name.\n\n\x11\n\x07\x04\x01\x03\0\x02\0\x04\x12\x06\xe9\x02\
    \x04\xe7\x02\x1f\n\x0f\n\x07\x04\x01\x03\0\x02\0\x05\x12\x04\xe9\x02\x04\
    \n\n\x0f\n\x07\x04\x01\x03\0\x02\0\x01\x12\x04\xe9\x02\x0b\x0f\n\x0f\n\
    \x07\x04\x01\x03\0\x02\0\x03\x12\x04\xe9\x02\x12\x13\n%\n\x06\x04\x01\
    \x03\0\x02\x01\x12\x04\xec\x02\x04\x15\x1a\x15\x20The\x20variable\x20val\
    ue.\n\n\x11\n\x07\x04\x01\x03\0\x02\x01\x04\x12\x06\xec\x02\x04\xe9\x02\
    \x14\n\x0f\n\x07\x04\x01\x03\0\x02\x01\x05\x12\x04\xec\x02\x04\n\n\x0f\n\
    \x07\x04\x01\x03\0\x02\x01\x01\x12\x04\xec\x02\x0b\x10\n\x0f\n\x07\x04\
    \x01\x03\0\x02\x01\x03\x12\x04\xec\x02\x13\x14\n\x97\x03\n\x04\x04\x01\
    \x02\0\x12\x04\xf6\x02\x02\x20\x1a\x88\x03\x20The\x20arguments\x20to\x20\
    the\x20command.\x20The\x20first\x20argument\x20must\x20be\x20the\x20path\
    \x20to\x20the\n\x20executable,\x20which\x20must\x20be\x20either\x20a\x20\
    relative\x20path,\x20in\x20which\x20case\x20it\x20is\n\x20evaluated\x20w\
    ith\x20respect\x20to\x20the\x20input\x20root,\x20or\x20an\x20absolute\
    \x20path.\x20The\x20`PATH`\n\x20environment\x20variable,\x20or\x20simila\
    r\x20functionality\x20on\x20other\x20systems,\x20is\x20not\n\x20used\x20\
    to\x20determine\x20which\x20executable\x20to\x20run.\n\n\x20The\x20worki\
    ng\x20directory\x20will\x20always\x20be\x20the\x20input\x20root.\n\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\xf6\x02\x02\n\n\r\n\x05\x04\x01\x02\0\x05\
    \x12\x04\xf6\x02\x0b\x11\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\xf6\x02\x12\
    \x1b\n\r\n\x05\x04\x01\x02\0\x03\x12\x04\xf6\x02\x1e\x1f\n\xcb\x03\n\x04\
    \x04\x01\x02\x01\x12\x04\xff\x02\x029\x1a\xbc\x03\x20The\x20environment\
    \x20variables\x20to\x20set\x20when\x20running\x20the\x20program.\x20The\
    \x20worker\x20may\n\x20provide\x20its\x20own\x20default\x20environment\
    \x20variables;\x20these\x20defaults\x20can\x20be\n\x20overridden\x20usin\
    g\x20this\x20field.\x20Additional\x20variables\x20can\x20also\x20be\x20s\
    pecified.\n\n\x20In\x20order\x20to\x20ensure\x20that\x20equivalent\x20`C\
    ommand`s\x20always\x20hash\x20to\x20the\x20same\n\x20value,\x20the\x20en\
    vironment\x20variables\x20MUST\x20be\x20lexicographically\x20sorted\x20b\
    y\x20name.\n\x20Sorting\x20of\x20strings\x20is\x20done\x20by\x20code\x20\
    point,\x20equivalently,\x20by\x20the\x20UTF-8\x20bytes.\n\n\r\n\x05\x04\
    \x01\x02\x01\x04\x12\x04\xff\x02\x02\n\n\r\n\x05\x04\x01\x02\x01\x06\x12\
    \x04\xff\x02\x0b\x1e\n\r\n\x05\x04\x01\x02\x01\x01\x12\x04\xff\x02\x1f4\
    \n\r\n\x05\x04\x01\x02\x01\x03\x12\x04\xff\x0278\n\xac\x03\n\x02\x04\x02\
    \x12\x06\x8a\x03\0\xa9\x03\x01\x1a\x9d\x03\x20A\x20`Platform`\x20is\x20a\
    \x20set\x20of\x20requirements,\x20such\x20as\x20hardware,\x20operation\
    \x20system,\x20or\n\x20compiler\x20toolchain,\x20for\x20an\n\x20[Action]\
    [google.devtools.remoteexecution.v1test.Action]'s\x20execution\n\x20envi\
    ronment.\x20A\x20`Platform`\x20is\x20represented\x20as\x20a\x20series\
    \x20of\x20key-value\x20pairs\n\x20representing\x20the\x20properties\x20t\
    hat\x20are\x20required\x20of\x20the\x20platform.\n\n\x20This\x20message\
    \x20is\x20currently\x20being\x20redeveloped\x20since\x20it\x20is\x20an\
    \x20overly\x20simplistic\n\x20model\x20of\x20platforms.\n\n\x0b\n\x03\
    \x04\x02\x01\x12\x04\x8a\x03\x08\x10\n\xa4\x07\n\x04\x04\x02\x03\0\x12\
    \x06\x9c\x03\x02\xa2\x03\x03\x1a\x93\x07\x20A\x20single\x20property\x20f\
    or\x20the\x20environment.\x20The\x20server\x20is\x20responsible\x20for\n\
    \x20specifying\x20the\x20property\x20`name`s\x20that\x20it\x20accepts.\
    \x20If\x20an\x20unknown\x20`name`\x20is\n\x20provided\x20in\x20the\x20re\
    quirements\x20for\x20an\n\x20[Action][google.devtools.remoteexecution.v1\
    test.Action],\x20the\x20server\x20SHOULD\n\x20reject\x20the\x20execution\
    \x20request.\x20If\x20permitted\x20by\x20the\x20server,\x20the\x20same\
    \x20`name`\n\x20may\x20occur\x20multiple\x20times.\n\n\x20The\x20server\
    \x20is\x20also\x20responsible\x20for\x20specifying\x20the\x20interpretat\
    ion\x20of\n\x20property\x20`value`s.\x20For\x20instance,\x20a\x20propert\
    y\x20describing\x20how\x20much\x20RAM\x20must\x20be\n\x20available\x20ma\
    y\x20be\x20interpreted\x20as\x20allowing\x20a\x20worker\x20with\x2016GB\
    \x20to\x20fulfill\x20a\n\x20request\x20for\x208GB,\x20while\x20a\x20prop\
    erty\x20describing\x20the\x20OS\x20environment\x20on\x20which\n\x20the\
    \x20action\x20must\x20be\x20performed\x20may\x20require\x20an\x20exact\
    \x20match\x20with\x20the\x20worker's\n\x20OS.\n\n\x20The\x20server\x20MA\
    Y\x20use\x20the\x20`value`\x20of\x20one\x20or\x20more\x20properties\x20t\
    o\x20determine\x20how\n\x20it\x20sets\x20up\x20the\x20execution\x20envir\
    onment,\x20such\x20as\x20by\x20making\x20specific\x20system\n\x20files\
    \x20available\x20to\x20the\x20worker.\n\n\r\n\x05\x04\x02\x03\0\x01\x12\
    \x04\x9c\x03\n\x12\n$\n\x06\x04\x02\x03\0\x02\0\x12\x04\x9e\x03\x04\x14\
    \x1a\x14\x20The\x20property\x20name.\n\n\x11\n\x07\x04\x02\x03\0\x02\0\
    \x04\x12\x06\x9e\x03\x04\x9c\x03\x14\n\x0f\n\x07\x04\x02\x03\0\x02\0\x05\
    \x12\x04\x9e\x03\x04\n\n\x0f\n\x07\x04\x02\x03\0\x02\0\x01\x12\x04\x9e\
    \x03\x0b\x0f\n\x0f\n\x07\x04\x02\x03\0\x02\0\x03\x12\x04\x9e\x03\x12\x13\
    \n%\n\x06\x04\x02\x03\0\x02\x01\x12\x04\xa1\x03\x04\x15\x1a\x15\x20The\
    \x20property\x20value.\n\n\x11\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x06\
    \xa1\x03\x04\x9e\x03\x14\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x04\
    \xa1\x03\x04\n\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x04\xa1\x03\x0b\
    \x10\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x04\xa1\x03\x13\x14\n\xa4\
    \x02\n\x04\x04\x02\x02\0\x12\x04\xa8\x03\x02#\x1a\x95\x02\x20The\x20prop\
    erties\x20that\x20make\x20up\x20this\x20platform.\x20In\x20order\x20to\
    \x20ensure\x20that\n\x20equivalent\x20`Platform`s\x20always\x20hash\x20t\
    o\x20the\x20same\x20value,\x20the\x20properties\x20MUST\n\x20be\x20lexic\
    ographically\x20sorted\x20by\x20name,\x20and\x20then\x20by\x20value.\x20\
    Sorting\x20of\x20strings\n\x20is\x20done\x20by\x20code\x20point,\x20equi\
    valently,\x20by\x20the\x20UTF-8\x20bytes.\n\n\r\n\x05\x04\x02\x02\0\x04\
    \x12\x04\xa8\x03\x02\n\n\r\n\x05\x04\x02\x02\0\x06\x12\x04\xa8\x03\x0b\
    \x13\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\xa8\x03\x14\x1e\n\r\n\x05\x04\
    \x02\x02\0\x03\x12\x04\xa8\x03!\"\n\xf2\r\n\x02\x04\x03\x12\x06\xe7\x03\
    \0\xed\x03\x01\x1a\xe3\r\x20A\x20`Directory`\x20represents\x20a\x20direc\
    tory\x20node\x20in\x20a\x20file\x20tree,\x20containing\x20zero\x20or\n\
    \x20more\x20children\x20[FileNodes][google.devtools.remoteexecution.v1te\
    st.FileNode]\n\x20and\x20[DirectoryNodes][google.devtools.remoteexecutio\
    n.v1test.DirectoryNode].\n\x20Each\x20`Node`\x20contains\x20its\x20name\
    \x20in\x20the\x20directory,\x20the\x20digest\x20of\x20its\x20content\n\
    \x20(either\x20a\x20file\x20blob\x20or\x20a\x20`Directory`\x20proto),\
    \x20as\x20well\x20as\x20possibly\x20some\n\x20metadata\x20about\x20the\
    \x20file\x20or\x20directory.\n\n\x20In\x20order\x20to\x20ensure\x20that\
    \x20two\x20equivalent\x20directory\x20trees\x20hash\x20to\x20the\x20same\
    \n\x20value,\x20the\x20following\x20restrictions\x20MUST\x20be\x20obeyed\
    \x20when\x20constructing\x20a\n\x20a\x20`Directory`:\n\x20\x20\x20-\x20E\
    very\x20child\x20in\x20the\x20directory\x20must\x20have\x20a\x20path\x20\
    of\x20exactly\x20one\x20segment.\n\x20\x20\x20\x20\x20Multiple\x20levels\
    \x20of\x20directory\x20hierarchy\x20may\x20not\x20be\x20collapsed.\n\x20\
    \x20\x20-\x20Each\x20child\x20in\x20the\x20directory\x20must\x20have\x20\
    a\x20unique\x20path\x20segment\x20(file\x20name).\n\x20\x20\x20-\x20The\
    \x20files\x20and\x20directories\x20in\x20the\x20directory\x20must\x20eac\
    h\x20be\x20sorted\x20in\n\x20\x20\x20\x20\x20lexicographical\x20order\
    \x20by\x20path.\x20The\x20path\x20strings\x20must\x20be\x20sorted\x20by\
    \x20code\n\x20\x20\x20\x20\x20point,\x20equivalently,\x20by\x20UTF-8\x20\
    bytes.\n\n\x20A\x20`Directory`\x20that\x20obeys\x20the\x20restrictions\
    \x20is\x20said\x20to\x20be\x20in\x20canonical\x20form.\n\n\x20As\x20an\
    \x20example,\x20the\x20following\x20could\x20be\x20used\x20for\x20a\x20f\
    ile\x20named\x20`bar`\x20and\x20a\n\x20directory\x20named\x20`foo`\x20wi\
    th\x20an\x20executable\x20file\x20named\x20`baz`\x20(hashes\x20shortened\
    \n\x20for\x20readability):\n\n\x20```json\n\x20//\x20(Directory\x20proto\
    )\n\x20{\n\x20\x20\x20files:\x20[\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20name:\x20\"bar\",\n\x20\x20\x20\x20\x20\x20\x20digest:\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20hash:\x20\"4a73bc9d03...\",\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20size:\x2065534\n\x20\x20\x20\x20\x20\
    \x20\x20}\n\x20\x20\x20\x20\x20}\n\x20\x20\x20],\n\x20\x20\x20directorie\
    s:\x20[\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\x20\x20\x20\x20name:\x20\"f\
    oo\",\n\x20\x20\x20\x20\x20\x20\x20digest:\x20{\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20hash:\x20\"4cf2eda940...\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20size:\x2043\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\
    \x20}\n\x20\x20\x20]\n\x20}\n\n\x20//\x20(Directory\x20proto\x20with\x20\
    hash\x20\"4cf2eda940...\"\x20and\x20size\x2043)\n\x20{\n\x20\x20\x20file\
    s:\x20[\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\x20\x20\x20\x20name:\x20\"b\
    az\",\n\x20\x20\x20\x20\x20\x20\x20digest:\x20{\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20hash:\x20\"b2c941073e...\",\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20size:\x201294,\n\x20\x20\x20\x20\x20\x20\x20},\n\x20\x20\x20\
    \x20\x20\x20\x20is_executable:\x20true\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20]\n\x20}\n\x20```\n\n\x0b\n\x03\x04\x03\x01\x12\x04\xe7\x03\x08\x11\
    \n+\n\x04\x04\x03\x02\0\x12\x04\xe9\x03\x02\x1e\x1a\x1d\x20The\x20files\
    \x20in\x20the\x20directory.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\xe9\
    \x03\x02\n\n\r\n\x05\x04\x03\x02\0\x06\x12\x04\xe9\x03\x0b\x13\n\r\n\x05\
    \x04\x03\x02\0\x01\x12\x04\xe9\x03\x14\x19\n\r\n\x05\x04\x03\x02\0\x03\
    \x12\x04\xe9\x03\x1c\x1d\n4\n\x04\x04\x03\x02\x01\x12\x04\xec\x03\x02)\
    \x1a&\x20The\x20subdirectories\x20in\x20the\x20directory.\n\n\r\n\x05\
    \x04\x03\x02\x01\x04\x12\x04\xec\x03\x02\n\n\r\n\x05\x04\x03\x02\x01\x06\
    \x12\x04\xec\x03\x0b\x18\n\r\n\x05\x04\x03\x02\x01\x01\x12\x04\xec\x03\
    \x19$\n\r\n\x05\x04\x03\x02\x01\x03\x12\x04\xec\x03'(\nN\n\x02\x04\x04\
    \x12\x06\xf0\x03\0\xf9\x03\x01\x1a@\x20A\x20`FileNode`\x20represents\x20\
    a\x20single\x20file\x20and\x20associated\x20metadata.\n\n\x0b\n\x03\x04\
    \x04\x01\x12\x04\xf0\x03\x08\x10\n%\n\x04\x04\x04\x02\0\x12\x04\xf2\x03\
    \x02\x12\x1a\x17\x20The\x20name\x20of\x20the\x20file.\n\n\x0f\n\x05\x04\
    \x04\x02\0\x04\x12\x06\xf2\x03\x02\xf0\x03\x12\n\r\n\x05\x04\x04\x02\0\
    \x05\x12\x04\xf2\x03\x02\x08\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\xf2\x03\
    \t\r\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xf2\x03\x10\x11\n1\n\x04\x04\
    \x04\x02\x01\x12\x04\xf5\x03\x02\x14\x1a#\x20The\x20digest\x20of\x20the\
    \x20file's\x20content.\n\n\x0f\n\x05\x04\x04\x02\x01\x04\x12\x06\xf5\x03\
    \x02\xf2\x03\x12\n\r\n\x05\x04\x04\x02\x01\x06\x12\x04\xf5\x03\x02\x08\n\
    \r\n\x05\x04\x04\x02\x01\x01\x12\x04\xf5\x03\t\x0f\n\r\n\x05\x04\x04\x02\
    \x01\x03\x12\x04\xf5\x03\x12\x13\n<\n\x04\x04\x04\x02\x02\x12\x04\xf8\
    \x03\x02\x19\x1a.\x20True\x20if\x20file\x20is\x20executable,\x20false\
    \x20otherwise.\n\n\x0f\n\x05\x04\x04\x02\x02\x04\x12\x06\xf8\x03\x02\xf5\
    \x03\x14\n\r\n\x05\x04\x04\x02\x02\x05\x12\x04\xf8\x03\x02\x06\n\r\n\x05\
    \x04\x04\x02\x02\x01\x12\x04\xf8\x03\x07\x14\n\r\n\x05\x04\x04\x02\x02\
    \x03\x12\x04\xf8\x03\x17\x18\n\xb5\x01\n\x02\x04\x05\x12\x06\xfe\x03\0\
    \x87\x04\x01\x1a\xa6\x01\x20A\x20`DirectoryNode`\x20represents\x20a\x20c\
    hild\x20of\x20a\n\x20[Directory][google.devtools.remoteexecution.v1test.\
    Directory]\x20which\x20is\x20itself\n\x20a\x20`Directory`\x20and\x20its\
    \x20associated\x20metadata.\n\n\x0b\n\x03\x04\x05\x01\x12\x04\xfe\x03\
    \x08\x15\n*\n\x04\x04\x05\x02\0\x12\x04\x80\x04\x02\x12\x1a\x1c\x20The\
    \x20name\x20of\x20the\x20directory.\n\n\x0f\n\x05\x04\x05\x02\0\x04\x12\
    \x06\x80\x04\x02\xfe\x03\x17\n\r\n\x05\x04\x05\x02\0\x05\x12\x04\x80\x04\
    \x02\x08\n\r\n\x05\x04\x05\x02\0\x01\x12\x04\x80\x04\t\r\n\r\n\x05\x04\
    \x05\x02\0\x03\x12\x04\x80\x04\x10\x11\n\xf4\x01\n\x04\x04\x05\x02\x01\
    \x12\x04\x86\x04\x02\x14\x1a\xe5\x01\x20The\x20digest\x20of\x20the\n\x20\
    [Directory][google.devtools.remoteexecution.v1test.Directory]\x20object\
    \n\x20represented.\x20See\x20[Digest][google.devtools.remoteexecution.v1\
    test.Digest]\n\x20for\x20information\x20about\x20how\x20to\x20take\x20th\
    e\x20digest\x20of\x20a\x20proto\x20message.\n\n\x0f\n\x05\x04\x05\x02\
    \x01\x04\x12\x06\x86\x04\x02\x80\x04\x12\n\r\n\x05\x04\x05\x02\x01\x06\
    \x12\x04\x86\x04\x02\x08\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\x86\x04\t\
    \x0f\n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\x86\x04\x12\x13\n\xb3\x0e\n\
    \x02\x04\x06\x12\x06\xa8\x04\0\xaf\x04\x01\x1a\xa4\x0e\x20A\x20content\
    \x20digest.\x20A\x20digest\x20for\x20a\x20given\x20blob\x20consists\x20o\
    f\x20the\x20size\x20of\x20the\x20blob\n\x20and\x20its\x20hash.\x20The\
    \x20hash\x20algorithm\x20to\x20use\x20is\x20defined\x20by\x20the\x20serv\
    er,\x20but\x20servers\n\x20SHOULD\x20use\x20SHA-256.\n\n\x20The\x20size\
    \x20is\x20considered\x20to\x20be\x20an\x20integral\x20part\x20of\x20the\
    \x20digest\x20and\x20cannot\x20be\n\x20separated.\x20That\x20is,\x20even\
    \x20if\x20the\x20`hash`\x20field\x20is\x20correctly\x20specified\x20but\
    \n\x20`size_bytes`\x20is\x20not,\x20the\x20server\x20MUST\x20reject\x20t\
    he\x20request.\n\n\x20The\x20reason\x20for\x20including\x20the\x20size\
    \x20in\x20the\x20digest\x20is\x20as\x20follows:\x20in\x20a\x20great\n\
    \x20many\x20cases,\x20the\x20server\x20needs\x20to\x20know\x20the\x20siz\
    e\x20of\x20the\x20blob\x20it\x20is\x20about\x20to\x20work\n\x20with\x20p\
    rior\x20to\x20starting\x20an\x20operation\x20with\x20it,\x20such\x20as\
    \x20flattening\x20Merkle\x20tree\n\x20structures\x20or\x20streaming\x20i\
    t\x20to\x20a\x20worker.\x20Technically,\x20the\x20server\x20could\n\x20i\
    mplement\x20a\x20separate\x20metadata\x20store,\x20but\x20this\x20result\
    s\x20in\x20a\x20significantly\x20more\n\x20complicated\x20implementation\
    \x20as\x20opposed\x20to\x20having\x20the\x20client\x20specify\x20the\x20\
    size\n\x20up-front\x20(or\x20storing\x20the\x20size\x20along\x20with\x20\
    the\x20digest\x20in\x20every\x20message\x20where\n\x20digests\x20are\x20\
    embedded).\x20This\x20does\x20mean\x20that\x20the\x20API\x20leaks\x20som\
    e\x20implementation\n\x20details\x20of\x20(what\x20we\x20consider\x20to\
    \x20be)\x20a\x20reasonable\x20server\x20implementation,\x20but\n\x20we\
    \x20consider\x20this\x20to\x20be\x20a\x20worthwhile\x20tradeoff.\n\n\x20\
    When\x20a\x20`Digest`\x20is\x20used\x20to\x20refer\x20to\x20a\x20proto\
    \x20message,\x20it\x20always\x20refers\x20to\x20the\n\x20message\x20in\
    \x20binary\x20encoded\x20form.\x20To\x20ensure\x20consistent\x20hashing,\
    \x20clients\x20and\n\x20servers\x20MUST\x20ensure\x20that\x20they\x20ser\
    ialize\x20messages\x20according\x20to\x20the\x20following\n\x20rules,\
    \x20even\x20if\x20there\x20are\x20alternate\x20valid\x20encodings\x20for\
    \x20the\x20same\x20message.\n\x20-\x20Fields\x20are\x20serialized\x20in\
    \x20tag\x20order.\n\x20-\x20There\x20are\x20no\x20unknown\x20fields.\n\
    \x20-\x20There\x20are\x20no\x20duplicate\x20fields.\n\x20-\x20Fields\x20\
    are\x20serialized\x20according\x20to\x20the\x20default\x20semantics\x20f\
    or\x20their\x20type.\n\n\x20Most\x20protocol\x20buffer\x20implementation\
    s\x20will\x20always\x20follow\x20these\x20rules\x20when\n\x20serializing\
    ,\x20but\x20care\x20should\x20be\x20taken\x20to\x20avoid\x20shortcuts.\
    \x20For\x20instance,\n\x20concatenating\x20two\x20messages\x20to\x20merg\
    e\x20them\x20may\x20produce\x20duplicate\x20fields.\n\n\x0b\n\x03\x04\
    \x06\x01\x12\x04\xa8\x04\x08\x0e\nw\n\x04\x04\x06\x02\0\x12\x04\xab\x04\
    \x02\x12\x1ai\x20The\x20hash.\x20In\x20the\x20case\x20of\x20SHA-256,\x20\
    it\x20will\x20always\x20be\x20a\x20lowercase\x20hex\x20string\n\x20exact\
    ly\x2064\x20characters\x20long.\n\n\x0f\n\x05\x04\x06\x02\0\x04\x12\x06\
    \xab\x04\x02\xa8\x04\x10\n\r\n\x05\x04\x06\x02\0\x05\x12\x04\xab\x04\x02\
    \x08\n\r\n\x05\x04\x06\x02\0\x01\x12\x04\xab\x04\t\r\n\r\n\x05\x04\x06\
    \x02\0\x03\x12\x04\xab\x04\x10\x11\n/\n\x04\x04\x06\x02\x01\x12\x04\xae\
    \x04\x02\x17\x1a!\x20The\x20size\x20of\x20the\x20blob,\x20in\x20bytes.\n\
    \n\x0f\n\x05\x04\x06\x02\x01\x04\x12\x06\xae\x04\x02\xab\x04\x12\n\r\n\
    \x05\x04\x06\x02\x01\x05\x12\x04\xae\x04\x02\x07\n\r\n\x05\x04\x06\x02\
    \x01\x01\x12\x04\xae\x04\x08\x12\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\
    \xae\x04\x15\x16\n\x7f\n\x02\x04\x07\x12\x06\xb3\x04\0\xe3\x04\x01\x1aq\
    \x20An\x20ActionResult\x20represents\x20the\x20result\x20of\x20an\n\x20[\
    Action][google.devtools.remoteexecution.v1test.Action]\x20being\x20run.\
    \n\n\x0b\n\x03\x04\x07\x01\x12\x04\xb3\x04\x08\x14\n\xe7\x03\n\x04\x04\
    \x07\x02\0\x12\x04\xbc\x04\x02'\x1a\xd8\x03\x20The\x20output\x20files\
    \x20of\x20the\x20action.\x20For\x20each\x20output\x20file\x20requested,\
    \x20if\x20the\n\x20corresponding\x20file\x20existed\x20after\x20the\x20a\
    ction\x20completed,\x20a\x20single\x20entry\x20will\n\x20be\x20present\
    \x20in\x20the\x20output\x20list.\n\n\x20If\x20the\x20action\x20does\x20n\
    ot\x20produce\x20the\x20requested\x20output,\x20or\x20produces\x20a\n\
    \x20directory\x20where\x20a\x20regular\x20file\x20is\x20expected\x20or\
    \x20vice\x20versa,\x20then\x20that\x20output\n\x20will\x20be\x20omitted\
    \x20from\x20the\x20list.\x20The\x20server\x20is\x20free\x20to\x20arrange\
    \x20the\x20output\n\x20list\x20as\x20desired;\x20clients\x20MUST\x20NOT\
    \x20assume\x20that\x20the\x20output\x20list\x20is\x20sorted.\n\n\r\n\x05\
    \x04\x07\x02\0\x04\x12\x04\xbc\x04\x02\n\n\r\n\x05\x04\x07\x02\0\x06\x12\
    \x04\xbc\x04\x0b\x15\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\xbc\x04\x16\"\n\
    \r\n\x05\x04\x07\x02\0\x03\x12\x04\xbc\x04%&\n\xd4\x02\n\x04\x04\x07\x02\
    \x01\x12\x04\xc3\x04\x022\x1a\xc5\x02\x20The\x20output\x20directories\
    \x20of\x20the\x20action.\x20For\x20each\x20output\x20directory\x20reques\
    ted,\n\x20if\x20the\x20corresponding\x20directory\x20existed\x20after\
    \x20the\x20action\x20completed,\x20a\x20single\n\x20entry\x20will\x20be\
    \x20present\x20in\x20the\x20output\x20list,\x20which\x20will\x20contain\
    \x20the\x20digest\x20of\n\x20a\x20[Tree][google.devtools.remoteexecution\
    .v1.test.Tree]\x20message\x20containing\n\x20the\x20directory\x20tree.\n\
    \n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\xc3\x04\x02\n\n\r\n\x05\x04\x07\
    \x02\x01\x06\x12\x04\xc3\x04\x0b\x1a\n\r\n\x05\x04\x07\x02\x01\x01\x12\
    \x04\xc3\x04\x1b-\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\xc3\x0401\n-\n\
    \x04\x04\x07\x02\x02\x12\x04\xc6\x04\x02\x16\x1a\x1f\x20The\x20exit\x20c\
    ode\x20of\x20the\x20command.\n\n\x0f\n\x05\x04\x07\x02\x02\x04\x12\x06\
    \xc6\x04\x02\xc3\x042\n\r\n\x05\x04\x07\x02\x02\x05\x12\x04\xc6\x04\x02\
    \x07\n\r\n\x05\x04\x07\x02\x02\x01\x12\x04\xc6\x04\x08\x11\n\r\n\x05\x04\
    \x07\x02\x02\x03\x12\x04\xc6\x04\x14\x15\n\x96\x03\n\x04\x04\x07\x02\x03\
    \x12\x04\xce\x04\x02\x17\x1a\x87\x03\x20The\x20standard\x20output\x20buf\
    fer\x20of\x20the\x20action.\x20The\x20server\x20will\x20determine,\x20ba\
    sed\n\x20on\x20the\x20size\x20of\x20the\x20buffer,\x20whether\x20to\x20r\
    eturn\x20it\x20in\x20raw\x20form\x20or\x20to\x20return\n\x20a\x20digest\
    \x20in\x20`stdout_digest`\x20that\x20points\x20to\x20the\x20buffer.\x20I\
    f\x20neither\x20is\x20set,\n\x20then\x20the\x20buffer\x20is\x20empty.\
    \x20The\x20client\x20SHOULD\x20NOT\x20assume\x20it\x20will\x20get\x20one\
    \x20of\n\x20the\x20raw\x20buffer\x20or\x20a\x20digest\x20on\x20any\x20gi\
    ven\x20request\x20and\x20should\x20be\x20prepared\x20to\n\x20handle\x20e\
    ither.\n\n\x0f\n\x05\x04\x07\x02\x03\x04\x12\x06\xce\x04\x02\xc6\x04\x16\
    \n\r\n\x05\x04\x07\x02\x03\x05\x12\x04\xce\x04\x02\x07\n\r\n\x05\x04\x07\
    \x02\x03\x01\x12\x04\xce\x04\x08\x12\n\r\n\x05\x04\x07\x02\x03\x03\x12\
    \x04\xce\x04\x15\x16\n\x82\x02\n\x04\x04\x07\x02\x04\x12\x04\xd4\x04\x02\
    \x1b\x1a\xf3\x01\x20The\x20digest\x20for\x20a\x20blob\x20containing\x20t\
    he\x20standard\x20output\x20of\x20the\x20action,\x20which\n\x20can\x20be\
    \x20retrieved\x20from\x20the\n\x20[ContentAddressableStorage][google.dev\
    tools.remoteexecution.v1test.ContentAddressableStorage].\n\x20See\x20`st\
    dout_raw`\x20for\x20when\x20this\x20will\x20be\x20set.\n\n\x0f\n\x05\x04\
    \x07\x02\x04\x04\x12\x06\xd4\x04\x02\xce\x04\x17\n\r\n\x05\x04\x07\x02\
    \x04\x06\x12\x04\xd4\x04\x02\x08\n\r\n\x05\x04\x07\x02\x04\x01\x12\x04\
    \xd4\x04\t\x16\n\r\n\x05\x04\x07\x02\x04\x03\x12\x04\xd4\x04\x19\x1a\n\
    \x95\x03\n\x04\x04\x07\x02\x05\x12\x04\xdc\x04\x02\x17\x1a\x86\x03\x20Th\
    e\x20standard\x20error\x20buffer\x20of\x20the\x20action.\x20The\x20serve\
    r\x20will\x20determine,\x20based\n\x20on\x20the\x20size\x20of\x20the\x20\
    buffer,\x20whether\x20to\x20return\x20it\x20in\x20raw\x20form\x20or\x20t\
    o\x20return\n\x20a\x20digest\x20in\x20`stderr_digest`\x20that\x20points\
    \x20to\x20the\x20buffer.\x20If\x20neither\x20is\x20set,\n\x20then\x20the\
    \x20buffer\x20is\x20empty.\x20The\x20client\x20SHOULD\x20NOT\x20assume\
    \x20it\x20will\x20get\x20one\x20of\n\x20the\x20raw\x20buffer\x20or\x20a\
    \x20digest\x20on\x20any\x20given\x20request\x20and\x20should\x20be\x20pr\
    epared\x20to\n\x20handle\x20either.\n\n\x0f\n\x05\x04\x07\x02\x05\x04\
    \x12\x06\xdc\x04\x02\xd4\x04\x1b\n\r\n\x05\x04\x07\x02\x05\x05\x12\x04\
    \xdc\x04\x02\x07\n\r\n\x05\x04\x07\x02\x05\x01\x12\x04\xdc\x04\x08\x12\n\
    \r\n\x05\x04\x07\x02\x05\x03\x12\x04\xdc\x04\x15\x16\n\x81\x02\n\x04\x04\
    \x07\x02\x06\x12\x04\xe2\x04\x02\x1b\x1a\xf2\x01\x20The\x20digest\x20for\
    \x20a\x20blob\x20containing\x20the\x20standard\x20error\x20of\x20the\x20\
    action,\x20which\n\x20can\x20be\x20retrieved\x20from\x20the\n\x20[Conten\
    tAddressableStorage][google.devtools.remoteexecution.v1test.ContentAddre\
    ssableStorage].\n\x20See\x20`stderr_raw`\x20for\x20when\x20this\x20will\
    \x20be\x20set.\n\n\x0f\n\x05\x04\x07\x02\x06\x04\x12\x06\xe2\x04\x02\xdc\
    \x04\x17\n\r\n\x05\x04\x07\x02\x06\x06\x12\x04\xe2\x04\x02\x08\n\r\n\x05\
    \x04\x07\x02\x06\x01\x12\x04\xe2\x04\t\x16\n\r\n\x05\x04\x07\x02\x06\x03\
    \x12\x04\xe2\x04\x19\x1a\n\xc6\x02\n\x02\x04\x08\x12\x06\xeb\x04\0\x81\
    \x05\x01\x1a\xb7\x02\x20An\x20`OutputFile`\x20is\x20similar\x20to\x20a\n\
    \x20[FileNode][google.devtools.remoteexecution.v1test.FileNode],\x20but\
    \x20it\x20is\n\x20tailored\x20for\x20output\x20as\x20part\x20of\x20an\
    \x20`ActionResult`.\x20It\x20allows\x20a\x20full\x20file\x20path\n\x20ra\
    ther\x20than\x20only\x20a\x20name,\x20and\x20allows\x20the\x20server\x20\
    to\x20include\x20content\x20inline.\n\n\x20`OutputFile`\x20is\x20binary-\
    compatible\x20with\x20`FileNode`.\n\n\x0b\n\x03\x04\x08\x01\x12\x04\xeb\
    \x04\x08\x12\n\x89\x01\n\x04\x04\x08\x02\0\x12\x04\xee\x04\x02\x12\x1a{\
    \x20The\x20full\x20path\x20of\x20the\x20file\x20relative\x20to\x20the\
    \x20input\x20root,\x20including\x20the\n\x20filename.\x20The\x20path\x20\
    separator\x20is\x20a\x20forward\x20slash\x20`/`.\n\n\x0f\n\x05\x04\x08\
    \x02\0\x04\x12\x06\xee\x04\x02\xeb\x04\x14\n\r\n\x05\x04\x08\x02\0\x05\
    \x12\x04\xee\x04\x02\x08\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\xee\x04\t\r\
    \n\r\n\x05\x04\x08\x02\0\x03\x12\x04\xee\x04\x10\x11\n1\n\x04\x04\x08\
    \x02\x01\x12\x04\xf1\x04\x02\x14\x1a#\x20The\x20digest\x20of\x20the\x20f\
    ile's\x20content.\n\n\x0f\n\x05\x04\x08\x02\x01\x04\x12\x06\xf1\x04\x02\
    \xee\x04\x12\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\xf1\x04\x02\x08\n\r\n\
    \x05\x04\x08\x02\x01\x01\x12\x04\xf1\x04\t\x0f\n\r\n\x05\x04\x08\x02\x01\
    \x03\x12\x04\xf1\x04\x12\x13\n\xbd\x03\n\x04\x04\x08\x02\x02\x12\x04\xfd\
    \x04\x02\x14\x1a\xae\x03\x20The\x20raw\x20content\x20of\x20the\x20file.\
    \n\n\x20This\x20field\x20may\x20be\x20used\x20by\x20the\x20server\x20to\
    \x20provide\x20the\x20content\x20of\x20a\x20file\n\x20inline\x20in\x20an\
    \n\x20[ActionResult][google.devtools.remoteexecution.v1test.ActionResult\
    ]\x20and\n\x20avoid\x20requiring\x20that\x20the\x20client\x20make\x20a\
    \x20separate\x20call\x20to\n\x20[ContentAddressableStorage.GetBlob]\x20t\
    o\x20retrieve\x20it.\n\n\x20The\x20client\x20SHOULD\x20NOT\x20assume\x20\
    that\x20it\x20will\x20get\x20raw\x20content\x20with\x20any\x20request,\n\
    \x20and\x20always\x20be\x20prepared\x20to\x20retrieve\x20it\x20via\x20`d\
    igest`.\n\n\x0f\n\x05\x04\x08\x02\x02\x04\x12\x06\xfd\x04\x02\xf1\x04\
    \x14\n\r\n\x05\x04\x08\x02\x02\x05\x12\x04\xfd\x04\x02\x07\n\r\n\x05\x04\
    \x08\x02\x02\x01\x12\x04\xfd\x04\x08\x0f\n\r\n\x05\x04\x08\x02\x02\x03\
    \x12\x04\xfd\x04\x12\x13\n<\n\x04\x04\x08\x02\x03\x12\x04\x80\x05\x02\
    \x19\x1a.\x20True\x20if\x20file\x20is\x20executable,\x20false\x20otherwi\
    se.\n\n\x0f\n\x05\x04\x08\x02\x03\x04\x12\x06\x80\x05\x02\xfd\x04\x14\n\
    \r\n\x05\x04\x08\x02\x03\x05\x12\x04\x80\x05\x02\x06\n\r\n\x05\x04\x08\
    \x02\x03\x01\x12\x04\x80\x05\x07\x14\n\r\n\x05\x04\x08\x02\x03\x03\x12\
    \x04\x80\x05\x17\x18\n\xb1\x01\n\x02\x04\t\x12\x06\x86\x05\0\x8f\x05\x01\
    \x1a\xa2\x01\x20A\x20`Tree`\x20contains\x20all\x20the\n\x20[Directory][g\
    oogle.devtools.remoteexecution.v1test.Directory]\x20protos\x20in\x20a\n\
    \x20single\x20directory\x20Merkle\x20tree,\x20compressed\x20into\x20one\
    \x20message.\n\n\x0b\n\x03\x04\t\x01\x12\x04\x86\x05\x08\x0c\n/\n\x04\
    \x04\t\x02\0\x12\x04\x88\x05\x02\x15\x1a!\x20The\x20root\x20directory\
    \x20in\x20the\x20tree.\n\n\x0f\n\x05\x04\t\x02\0\x04\x12\x06\x88\x05\x02\
    \x86\x05\x0e\n\r\n\x05\x04\t\x02\0\x06\x12\x04\x88\x05\x02\x0b\n\r\n\x05\
    \x04\t\x02\0\x01\x12\x04\x88\x05\x0c\x10\n\r\n\x05\x04\t\x02\0\x03\x12\
    \x04\x88\x05\x13\x14\n\x9b\x02\n\x04\x04\t\x02\x01\x12\x04\x8e\x05\x02\"\
    \x1a\x8c\x02\x20All\x20the\x20child\x20directories:\x20the\x20directorie\
    s\x20referred\x20to\x20by\x20the\x20root\x20and,\n\x20recursively,\x20al\
    l\x20its\x20children.\x20In\x20order\x20to\x20reconstruct\x20the\x20dire\
    ctory\x20tree,\n\x20the\x20client\x20must\x20take\x20the\x20digests\x20o\
    f\x20each\x20of\x20the\x20child\x20directories\x20and\x20then\n\x20build\
    \x20up\x20a\x20tree\x20starting\x20from\x20the\x20`root`.\n\n\r\n\x05\
    \x04\t\x02\x01\x04\x12\x04\x8e\x05\x02\n\n\r\n\x05\x04\t\x02\x01\x06\x12\
    \x04\x8e\x05\x0b\x14\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\x8e\x05\x15\x1d\
    \n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x8e\x05\x20!\n\x91\x01\n\x02\x04\n\
    \x12\x06\x93\x05\0\x9f\x05\x01\x1a\x82\x01\x20An\x20`OutputDirectory`\
    \x20is\x20the\x20output\x20in\x20an\x20`ActionResult`\x20corresponding\
    \x20to\x20a\n\x20directory's\x20full\x20contents\x20rather\x20than\x20a\
    \x20single\x20file.\n\n\x0b\n\x03\x04\n\x01\x12\x04\x93\x05\x08\x17\n\
    \x8f\x01\n\x04\x04\n\x02\0\x12\x04\x96\x05\x02\x12\x1a\x80\x01\x20The\
    \x20full\x20path\x20of\x20the\x20directory\x20relative\x20to\x20the\x20i\
    nput\x20root,\x20including\x20the\n\x20filename.\x20The\x20path\x20separ\
    ator\x20is\x20a\x20forward\x20slash\x20`/`.\n\n\x0f\n\x05\x04\n\x02\0\
    \x04\x12\x06\x96\x05\x02\x93\x05\x19\n\r\n\x05\x04\n\x02\0\x05\x12\x04\
    \x96\x05\x02\x08\n\r\n\x05\x04\n\x02\0\x01\x12\x04\x96\x05\t\r\n\r\n\x05\
    \x04\n\x02\0\x03\x12\x04\x96\x05\x10\x11\nR\n\x04\x04\n\x02\x01\x12\x04\
    \x99\x05\x02\x14\x1aD\x20DEPRECATED:\x20This\x20field\x20is\x20deprecate\
    d\x20and\x20should\x20no\x20longer\x20be\x20used.\n\n\x0f\n\x05\x04\n\
    \x02\x01\x04\x12\x06\x99\x05\x02\x96\x05\x12\n\r\n\x05\x04\n\x02\x01\x06\
    \x12\x04\x99\x05\x02\x08\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\x99\x05\t\
    \x0f\n\r\n\x05\x04\n\x02\x01\x03\x12\x04\x99\x05\x12\x13\n\x8a\x01\n\x04\
    \x04\n\x02\x02\x12\x04\x9e\x05\x02\x19\x1a|\x20The\x20digest\x20of\x20th\
    e\x20encoded\n\x20[Tree][google.devtools.remoteexecution.v1test.Tree]\
    \x20proto\x20containing\x20the\n\x20directory's\x20contents.\n\n\x0f\n\
    \x05\x04\n\x02\x02\x04\x12\x06\x9e\x05\x02\x99\x05\x14\n\r\n\x05\x04\n\
    \x02\x02\x06\x12\x04\x9e\x05\x02\x08\n\r\n\x05\x04\n\x02\x02\x01\x12\x04\
    \x9e\x05\t\x14\n\r\n\x05\x04\n\x02\x02\x03\x12\x04\x9e\x05\x17\x18\nu\n\
    \x02\x04\x0b\x12\x06\xa3\x05\0\xba\x05\x01\x1ag\x20A\x20request\x20messa\
    ge\x20for\n\x20[Execution.Execute][google.devtools.remoteexecution.v1tes\
    t.Execution.Execute].\n\n\x0b\n\x03\x04\x0b\x01\x12\x04\xa3\x05\x08\x16\
    \n\xc1\x02\n\x04\x04\x0b\x02\0\x12\x04\xa9\x05\x02\x1b\x1a\xb2\x02\x20Th\
    e\x20instance\x20of\x20the\x20execution\x20system\x20to\x20operate\x20ag\
    ainst.\x20A\x20server\x20may\n\x20support\x20multiple\x20instances\x20of\
    \x20the\x20execution\x20system\x20(with\x20their\x20own\x20workers,\n\
    \x20storage,\x20caches,\x20etc.).\x20The\x20server\x20MAY\x20require\x20\
    use\x20of\x20this\x20field\x20to\x20select\n\x20between\x20them\x20in\
    \x20an\x20implementation-defined\x20fashion,\x20otherwise\x20it\x20can\
    \x20be\n\x20omitted.\n\n\x0f\n\x05\x04\x0b\x02\0\x04\x12\x06\xa9\x05\x02\
    \xa3\x05\x18\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\xa9\x05\x02\x08\n\r\n\
    \x05\x04\x0b\x02\0\x01\x12\x04\xa9\x05\t\x16\n\r\n\x05\x04\x0b\x02\0\x03\
    \x12\x04\xa9\x05\x19\x1a\n+\n\x04\x04\x0b\x02\x01\x12\x04\xac\x05\x02\
    \x14\x1a\x1d\x20The\x20action\x20to\x20be\x20performed.\n\n\x0f\n\x05\
    \x04\x0b\x02\x01\x04\x12\x06\xac\x05\x02\xa9\x05\x1b\n\r\n\x05\x04\x0b\
    \x02\x01\x06\x12\x04\xac\x05\x02\x08\n\r\n\x05\x04\x0b\x02\x01\x01\x12\
    \x04\xac\x05\t\x0f\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\xac\x05\x12\x13\
    \n\xe0\x01\n\x04\x04\x0b\x02\x02\x12\x04\xb1\x05\x02\x1d\x1a\xd1\x01\x20\
    If\x20true,\x20the\x20action\x20will\x20be\x20executed\x20anew\x20even\
    \x20if\x20its\x20result\x20was\x20already\n\x20present\x20in\x20the\x20c\
    ache.\x20If\x20false,\x20the\x20result\x20may\x20be\x20served\x20from\
    \x20the\n\x20[ActionCache][google.devtools.remoteexecution.v1test.Action\
    Cache].\n\n\x0f\n\x05\x04\x0b\x02\x02\x04\x12\x06\xb1\x05\x02\xac\x05\
    \x14\n\r\n\x05\x04\x0b\x02\x02\x05\x12\x04\xb1\x05\x02\x06\n\r\n\x05\x04\
    \x0b\x02\x02\x01\x12\x04\xb1\x05\x07\x18\n\r\n\x05\x04\x0b\x02\x02\x03\
    \x12\x04\xb1\x05\x1b\x1c\ne\n\x04\x04\x0b\x02\x03\x12\x04\xb5\x05\x02#\
    \x1aW\x20DEPRECATED:\x20This\x20field\x20should\x20be\x20ignored\x20by\
    \x20clients\x20and\x20servers\x20and\x20will\x20be\n\x20removed.\n\n\x0f\
    \n\x05\x04\x0b\x02\x03\x04\x12\x06\xb5\x05\x02\xb1\x05\x1d\n\r\n\x05\x04\
    \x0b\x02\x03\x05\x12\x04\xb5\x05\x02\x07\n\r\n\x05\x04\x0b\x02\x03\x01\
    \x12\x04\xb5\x05\x08\x1e\n\r\n\x05\x04\x0b\x02\x03\x03\x12\x04\xb5\x05!\
    \"\ne\n\x04\x04\x0b\x02\x04\x12\x04\xb9\x05\x02#\x1aW\x20DEPRECATED:\x20\
    This\x20field\x20should\x20be\x20ignored\x20by\x20clients\x20and\x20serv\
    ers\x20and\x20will\x20be\n\x20removed.\n\n\x0f\n\x05\x04\x0b\x02\x04\x04\
    \x12\x06\xb9\x05\x02\xb5\x05#\n\r\n\x05\x04\x0b\x02\x04\x05\x12\x04\xb9\
    \x05\x02\x07\n\r\n\x05\x04\x0b\x02\x04\x01\x12\x04\xb9\x05\x08\x1e\n\r\n\
    \x05\x04\x0b\x02\x04\x03\x12\x04\xb9\x05!\"\n7\n\x02\x04\x0c\x12\x06\xbd\
    \x05\0\xc7\x05\x01\x1a)\x20A\x20`LogFile`\x20is\x20a\x20log\x20stored\
    \x20in\x20the\x20CAS.\n\n\x0b\n\x03\x04\x0c\x01\x12\x04\xbd\x05\x08\x0f\
    \n/\n\x04\x04\x0c\x02\0\x12\x04\xbf\x05\x02\x14\x1a!\x20The\x20digest\
    \x20of\x20the\x20log\x20contents.\n\n\x0f\n\x05\x04\x0c\x02\0\x04\x12\
    \x06\xbf\x05\x02\xbd\x05\x11\n\r\n\x05\x04\x0c\x02\0\x06\x12\x04\xbf\x05\
    \x02\x08\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\xbf\x05\t\x0f\n\r\n\x05\x04\
    \x0c\x02\0\x03\x12\x04\xbf\x05\x12\x13\n\xd3\x02\n\x04\x04\x0c\x02\x01\
    \x12\x04\xc6\x05\x02\x1a\x1a\xc4\x02\x20This\x20is\x20a\x20hint\x20as\
    \x20to\x20the\x20purpose\x20of\x20the\x20log,\x20and\x20is\x20set\x20to\
    \x20true\x20if\x20the\x20log\n\x20is\x20human-readable\x20text\x20that\
    \x20can\x20be\x20usefully\x20displayed\x20to\x20a\x20user,\x20and\x20fal\
    se\n\x20otherwise.\x20For\x20instance,\x20if\x20a\x20command-line\x20cli\
    ent\x20wishes\x20to\x20print\x20the\n\x20server\x20logs\x20to\x20the\x20\
    terminal\x20for\x20a\x20failed\x20action,\x20this\x20allows\x20it\x20to\
    \x20avoid\n\x20displaying\x20a\x20binary\x20file.\n\n\x0f\n\x05\x04\x0c\
    \x02\x01\x04\x12\x06\xc6\x05\x02\xbf\x05\x14\n\r\n\x05\x04\x0c\x02\x01\
    \x05\x12\x04\xc6\x05\x02\x06\n\r\n\x05\x04\x0c\x02\x01\x01\x12\x04\xc6\
    \x05\x07\x15\n\r\n\x05\x04\x0c\x02\x01\x03\x12\x04\xc6\x05\x18\x19\n\x85\
    \x02\n\x02\x04\r\x12\x06\xce\x05\0\xe9\x05\x01\x1a\xf6\x01\x20The\x20res\
    ponse\x20message\x20for\n\x20[Execution.Execute][google.devtools.remotee\
    xecution.v1test.Execution.Execute],\n\x20which\x20will\x20be\x20containe\
    d\x20in\x20the\x20[response\n\x20field][google.longrunning.Operation.res\
    ponse]\x20of\x20the\n\x20[Operation][google.longrunning.Operation].\n\n\
    \x0b\n\x03\x04\r\x01\x12\x04\xce\x05\x08\x17\n)\n\x04\x04\r\x02\0\x12\
    \x04\xd0\x05\x02\x1a\x1a\x1b\x20The\x20result\x20of\x20the\x20action.\n\
    \n\x0f\n\x05\x04\r\x02\0\x04\x12\x06\xd0\x05\x02\xce\x05\x19\n\r\n\x05\
    \x04\r\x02\0\x06\x12\x04\xd0\x05\x02\x0e\n\r\n\x05\x04\r\x02\0\x01\x12\
    \x04\xd0\x05\x0f\x15\n\r\n\x05\x04\r\x02\0\x03\x12\x04\xd0\x05\x18\x19\n\
    S\n\x04\x04\r\x02\x01\x12\x04\xd3\x05\x02\x19\x1aE\x20True\x20if\x20the\
    \x20result\x20was\x20served\x20from\x20cache,\x20false\x20if\x20it\x20wa\
    s\x20executed.\n\n\x0f\n\x05\x04\r\x02\x01\x04\x12\x06\xd3\x05\x02\xd0\
    \x05\x1a\n\r\n\x05\x04\r\x02\x01\x05\x12\x04\xd3\x05\x02\x06\n\r\n\x05\
    \x04\r\x02\x01\x01\x12\x04\xd3\x05\x07\x14\n\r\n\x05\x04\r\x02\x01\x03\
    \x12\x04\xd3\x05\x17\x18\n\xf3\x04\n\x04\x04\r\x02\x02\x12\x04\xdf\x05\
    \x02\x1f\x1a\xe4\x04\x20If\x20the\x20status\x20has\x20a\x20code\x20other\
    \x20than\x20`OK`,\x20it\x20indicates\x20that\x20the\x20action\x20did\n\
    \x20not\x20finish\x20execution.\x20For\x20example,\x20if\x20the\x20opera\
    tion\x20times\x20out\x20during\n\x20execution,\x20the\x20status\x20will\
    \x20have\x20a\x20`DEADLINE_EXCEEDED`\x20code.\x20Servers\x20MUST\n\x20us\
    e\x20this\x20field\x20for\x20errors\x20in\x20execution,\x20rather\x20tha\
    n\x20the\x20error\x20field\x20on\x20the\n\x20`Operation`\x20object.\n\n\
    \x20If\x20the\x20status\x20code\x20is\x20other\x20than\x20`OK`,\x20then\
    \x20the\x20result\x20MUST\x20NOT\x20be\x20cached.\n\x20For\x20an\x20erro\
    r\x20status,\x20the\x20`result`\x20field\x20is\x20optional;\x20the\x20se\
    rver\x20may\n\x20populate\x20the\x20output-,\x20stdout-,\x20and\x20stder\
    r-related\x20fields\x20if\x20it\x20has\x20any\n\x20information\x20availa\
    ble,\x20such\x20as\x20the\x20stdout\x20and\x20stderr\x20of\x20a\x20timed\
    -out\x20action.\n\n\x0f\n\x05\x04\r\x02\x02\x04\x12\x06\xdf\x05\x02\xd3\
    \x05\x19\n\r\n\x05\x04\r\x02\x02\x06\x12\x04\xdf\x05\x02\x13\n\r\n\x05\
    \x04\r\x02\x02\x01\x12\x04\xdf\x05\x14\x1a\n\r\n\x05\x04\r\x02\x02\x03\
    \x12\x04\xdf\x05\x1d\x1e\n\xdc\x03\n\x04\x04\r\x02\x03\x12\x04\xe8\x05\
    \x02'\x1a\xcd\x03\x20An\x20optional\x20list\x20of\x20additional\x20log\
    \x20outputs\x20the\x20server\x20wishes\x20to\x20provide.\x20A\n\x20serve\
    r\x20can\x20use\x20this\x20to\x20return\x20execution-specific\x20logs\
    \x20however\x20it\x20wishes.\n\x20This\x20is\x20intended\x20primarily\
    \x20to\x20make\x20it\x20easier\x20for\x20users\x20to\x20debug\x20issues\
    \x20that\n\x20may\x20be\x20outside\x20of\x20the\x20actual\x20job\x20exec\
    ution,\x20such\x20as\x20by\x20identifying\x20the\n\x20worker\x20executin\
    g\x20the\x20action\x20or\x20by\x20providing\x20logs\x20from\x20the\x20wo\
    rker's\x20setup\n\x20phase.\x20The\x20keys\x20SHOULD\x20be\x20human\x20r\
    eadable\x20so\x20that\x20a\x20client\x20can\x20display\x20them\n\x20to\
    \x20a\x20user.\n\n\x0f\n\x05\x04\r\x02\x03\x04\x12\x06\xe8\x05\x02\xdf\
    \x05\x1f\n\r\n\x05\x04\r\x02\x03\x06\x12\x04\xe8\x05\x02\x16\n\r\n\x05\
    \x04\r\x02\x03\x01\x12\x04\xe8\x05\x17\"\n\r\n\x05\x04\r\x02\x03\x03\x12\
    \x04\xe8\x05%&\n\xfe\x01\n\x02\x04\x0e\x12\x06\xf0\x05\0\x91\x06\x01\x1a\
    \xef\x01\x20Metadata\x20about\x20an\x20ongoing\n\x20[execution][google.d\
    evtools.remoteexecution.v1test.Execution.Execute],\x20which\n\x20will\
    \x20be\x20contained\x20in\x20the\x20[metadata\n\x20field][google.longrun\
    ning.Operation.response]\x20of\x20the\n\x20[Operation][google.longrunnin\
    g.Operation].\n\n\x0b\n\x03\x04\x0e\x01\x12\x04\xf0\x05\x08\x20\n1\n\x04\
    \x04\x0e\x04\0\x12\x06\xf2\x05\x02\x80\x06\x03\x1a!\x20The\x20current\
    \x20stage\x20of\x20execution.\n\n\r\n\x05\x04\x0e\x04\0\x01\x12\x04\xf2\
    \x05\x07\x0c\n\x0e\n\x06\x04\x0e\x04\0\x02\0\x12\x04\xf3\x05\x04\x10\n\
    \x0f\n\x07\x04\x0e\x04\0\x02\0\x01\x12\x04\xf3\x05\x04\x0b\n\x0f\n\x07\
    \x04\x0e\x04\0\x02\0\x02\x12\x04\xf3\x05\x0e\x0f\n8\n\x06\x04\x0e\x04\0\
    \x02\x01\x12\x04\xf6\x05\x04\x14\x1a(\x20Checking\x20the\x20result\x20ag\
    ainst\x20the\x20cache.\n\n\x0f\n\x07\x04\x0e\x04\0\x02\x01\x01\x12\x04\
    \xf6\x05\x04\x0f\n\x0f\n\x07\x04\x0e\x04\0\x02\x01\x02\x12\x04\xf6\x05\
    \x12\x13\nE\n\x06\x04\x0e\x04\0\x02\x02\x12\x04\xf9\x05\x04\x0f\x1a5\x20\
    Currently\x20idle,\x20awaiting\x20a\x20free\x20machine\x20to\x20execute.\
    \n\n\x0f\n\x07\x04\x0e\x04\0\x02\x02\x01\x12\x04\xf9\x05\x04\n\n\x0f\n\
    \x07\x04\x0e\x04\0\x02\x02\x02\x12\x04\xf9\x05\r\x0e\n7\n\x06\x04\x0e\
    \x04\0\x02\x03\x12\x04\xfc\x05\x04\x12\x1a'\x20Currently\x20being\x20exe\
    cuted\x20by\x20a\x20worker.\n\n\x0f\n\x07\x04\x0e\x04\0\x02\x03\x01\x12\
    \x04\xfc\x05\x04\r\n\x0f\n\x07\x04\x0e\x04\0\x02\x03\x02\x12\x04\xfc\x05\
    \x10\x11\n%\n\x06\x04\x0e\x04\0\x02\x04\x12\x04\xff\x05\x04\x12\x1a\x15\
    \x20Finished\x20execution.\n\n\x0f\n\x07\x04\x0e\x04\0\x02\x04\x01\x12\
    \x04\xff\x05\x04\r\n\x0f\n\x07\x04\x0e\x04\0\x02\x04\x02\x12\x04\xff\x05\
    \x10\x11\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\x82\x06\x02\x12\n\x0f\n\x05\
    \x04\x0e\x02\0\x04\x12\x06\x82\x06\x02\x80\x06\x03\n\r\n\x05\x04\x0e\x02\
    \0\x06\x12\x04\x82\x06\x02\x07\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\x82\
    \x06\x08\r\n\r\n\x05\x04\x0e\x02\0\x03\x12\x04\x82\x06\x10\x11\nj\n\x04\
    \x04\x0e\x02\x01\x12\x04\x86\x06\x02\x1b\x1a\\\x20The\x20digest\x20of\
    \x20the\x20[Action][google.devtools.remoteexecution.v1test.Action]\n\x20\
    being\x20executed.\n\n\x0f\n\x05\x04\x0e\x02\x01\x04\x12\x06\x86\x06\x02\
    \x82\x06\x12\n\r\n\x05\x04\x0e\x02\x01\x06\x12\x04\x86\x06\x02\x08\n\r\n\
    \x05\x04\x0e\x02\x01\x01\x12\x04\x86\x06\t\x16\n\r\n\x05\x04\x0e\x02\x01\
    \x03\x12\x04\x86\x06\x19\x1a\n\x90\x01\n\x04\x04\x0e\x02\x02\x12\x04\x8b\
    \x06\x02\x20\x1a\x81\x01\x20If\x20set,\x20the\x20client\x20can\x20use\
    \x20this\x20name\x20with\n\x20[ByteStream.Read][google.bytestream.ByteSt\
    ream.Read]\x20to\x20stream\x20the\n\x20standard\x20output.\n\n\x0f\n\x05\
    \x04\x0e\x02\x02\x04\x12\x06\x8b\x06\x02\x86\x06\x1b\n\r\n\x05\x04\x0e\
    \x02\x02\x05\x12\x04\x8b\x06\x02\x08\n\r\n\x05\x04\x0e\x02\x02\x01\x12\
    \x04\x8b\x06\t\x1b\n\r\n\x05\x04\x0e\x02\x02\x03\x12\x04\x8b\x06\x1e\x1f\
    \n\x8f\x01\n\x04\x04\x0e\x02\x03\x12\x04\x90\x06\x02\x20\x1a\x80\x01\x20\
    If\x20set,\x20the\x20client\x20can\x20use\x20this\x20name\x20with\n\x20[\
    ByteStream.Read][google.bytestream.ByteStream.Read]\x20to\x20stream\x20t\
    he\n\x20standard\x20error.\n\n\x0f\n\x05\x04\x0e\x02\x03\x04\x12\x06\x90\
    \x06\x02\x8b\x06\x20\n\r\n\x05\x04\x0e\x02\x03\x05\x12\x04\x90\x06\x02\
    \x08\n\r\n\x05\x04\x0e\x02\x03\x01\x12\x04\x90\x06\t\x1b\n\r\n\x05\x04\
    \x0e\x02\x03\x03\x12\x04\x90\x06\x1e\x1f\n\x89\x01\n\x02\x04\x0f\x12\x06\
    \x95\x06\0\xa0\x06\x01\x1a{\x20A\x20request\x20message\x20for\n\x20[Acti\
    onCache.GetActionResult][google.devtools.remoteexecution.v1test.ActionCa\
    che.GetActionResult].\n\n\x0b\n\x03\x04\x0f\x01\x12\x04\x95\x06\x08\x1e\
    \n\xc1\x02\n\x04\x04\x0f\x02\0\x12\x04\x9b\x06\x02\x1b\x1a\xb2\x02\x20Th\
    e\x20instance\x20of\x20the\x20execution\x20system\x20to\x20operate\x20ag\
    ainst.\x20A\x20server\x20may\n\x20support\x20multiple\x20instances\x20of\
    \x20the\x20execution\x20system\x20(with\x20their\x20own\x20workers,\n\
    \x20storage,\x20caches,\x20etc.).\x20The\x20server\x20MAY\x20require\x20\
    use\x20of\x20this\x20field\x20to\x20select\n\x20between\x20them\x20in\
    \x20an\x20implementation-defined\x20fashion,\x20otherwise\x20it\x20can\
    \x20be\n\x20omitted.\n\n\x0f\n\x05\x04\x0f\x02\0\x04\x12\x06\x9b\x06\x02\
    \x95\x06\x20\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\x9b\x06\x02\x08\n\r\n\
    \x05\x04\x0f\x02\0\x01\x12\x04\x9b\x06\t\x16\n\r\n\x05\x04\x0f\x02\0\x03\
    \x12\x04\x9b\x06\x19\x1a\nu\n\x04\x04\x0f\x02\x01\x12\x04\x9f\x06\x02\
    \x1b\x1ag\x20The\x20digest\x20of\x20the\x20[Action][google.devtools.remo\
    teexecution.v1test.Action]\n\x20whose\x20result\x20is\x20requested.\n\n\
    \x0f\n\x05\x04\x0f\x02\x01\x04\x12\x06\x9f\x06\x02\x9b\x06\x1b\n\r\n\x05\
    \x04\x0f\x02\x01\x06\x12\x04\x9f\x06\x02\x08\n\r\n\x05\x04\x0f\x02\x01\
    \x01\x12\x04\x9f\x06\t\x16\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\x9f\x06\
    \x19\x1a\n\x90\x01\n\x02\x04\x10\x12\x06\xa4\x06\0\xb3\x06\x01\x1a\x81\
    \x01\x20A\x20request\x20message\x20for\n\x20[ActionCache.UpdateActionRes\
    ult][google.devtools.remoteexecution.v1test.ActionCache.UpdateActionResu\
    lt].\n\n\x0b\n\x03\x04\x10\x01\x12\x04\xa4\x06\x08!\n\xc1\x02\n\x04\x04\
    \x10\x02\0\x12\x04\xaa\x06\x02\x1b\x1a\xb2\x02\x20The\x20instance\x20of\
    \x20the\x20execution\x20system\x20to\x20operate\x20against.\x20A\x20serv\
    er\x20may\n\x20support\x20multiple\x20instances\x20of\x20the\x20executio\
    n\x20system\x20(with\x20their\x20own\x20workers,\n\x20storage,\x20caches\
    ,\x20etc.).\x20The\x20server\x20MAY\x20require\x20use\x20of\x20this\x20f\
    ield\x20to\x20select\n\x20between\x20them\x20in\x20an\x20implementation-\
    defined\x20fashion,\x20otherwise\x20it\x20can\x20be\n\x20omitted.\n\n\
    \x0f\n\x05\x04\x10\x02\0\x04\x12\x06\xaa\x06\x02\xa4\x06#\n\r\n\x05\x04\
    \x10\x02\0\x05\x12\x04\xaa\x06\x02\x08\n\r\n\x05\x04\x10\x02\0\x01\x12\
    \x04\xaa\x06\t\x16\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\xaa\x06\x19\x1a\n\
    z\n\x04\x04\x10\x02\x01\x12\x04\xae\x06\x02\x1b\x1al\x20The\x20digest\
    \x20of\x20the\x20[Action][google.devtools.remoteexecution.v1test.Action]\
    \n\x20whose\x20result\x20is\x20being\x20uploaded.\n\n\x0f\n\x05\x04\x10\
    \x02\x01\x04\x12\x06\xae\x06\x02\xaa\x06\x1b\n\r\n\x05\x04\x10\x02\x01\
    \x06\x12\x04\xae\x06\x02\x08\n\r\n\x05\x04\x10\x02\x01\x01\x12\x04\xae\
    \x06\t\x16\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\xae\x06\x19\x1a\no\n\
    \x04\x04\x10\x02\x02\x12\x04\xb2\x06\x02!\x1aa\x20The\x20[ActionResult][\
    google.devtools.remoteexecution.v1test.ActionResult]\n\x20to\x20store\
    \x20in\x20the\x20cache.\n\n\x0f\n\x05\x04\x10\x02\x02\x04\x12\x06\xb2\
    \x06\x02\xae\x06\x1b\n\r\n\x05\x04\x10\x02\x02\x06\x12\x04\xb2\x06\x02\
    \x0e\n\r\n\x05\x04\x10\x02\x02\x01\x12\x04\xb2\x06\x0f\x1c\n\r\n\x05\x04\
    \x10\x02\x02\x03\x12\x04\xb2\x06\x1f\x20\n\xa8\x01\n\x02\x04\x11\x12\x06\
    \xb7\x06\0\xc1\x06\x01\x1a\x99\x01\x20A\x20request\x20message\x20for\n\
    \x20[ContentAddressableStorage.FindMissingBlobs][google.devtools.remotee\
    xecution.v1test.ContentAddressableStorage.FindMissingBlobs].\n\n\x0b\n\
    \x03\x04\x11\x01\x12\x04\xb7\x06\x08\x1f\n\xc1\x02\n\x04\x04\x11\x02\0\
    \x12\x04\xbd\x06\x02\x1b\x1a\xb2\x02\x20The\x20instance\x20of\x20the\x20\
    execution\x20system\x20to\x20operate\x20against.\x20A\x20server\x20may\n\
    \x20support\x20multiple\x20instances\x20of\x20the\x20execution\x20system\
    \x20(with\x20their\x20own\x20workers,\n\x20storage,\x20caches,\x20etc.).\
    \x20The\x20server\x20MAY\x20require\x20use\x20of\x20this\x20field\x20to\
    \x20select\n\x20between\x20them\x20in\x20an\x20implementation-defined\
    \x20fashion,\x20otherwise\x20it\x20can\x20be\n\x20omitted.\n\n\x0f\n\x05\
    \x04\x11\x02\0\x04\x12\x06\xbd\x06\x02\xb7\x06!\n\r\n\x05\x04\x11\x02\0\
    \x05\x12\x04\xbd\x06\x02\x08\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xbd\x06\
    \t\x16\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xbd\x06\x19\x1a\n-\n\x04\x04\
    \x11\x02\x01\x12\x04\xc0\x06\x02#\x1a\x1f\x20A\x20list\x20of\x20the\x20b\
    lobs\x20to\x20check.\n\n\r\n\x05\x04\x11\x02\x01\x04\x12\x04\xc0\x06\x02\
    \n\n\r\n\x05\x04\x11\x02\x01\x06\x12\x04\xc0\x06\x0b\x11\n\r\n\x05\x04\
    \x11\x02\x01\x01\x12\x04\xc0\x06\x12\x1e\n\r\n\x05\x04\x11\x02\x01\x03\
    \x12\x04\xc0\x06!\"\n\xa9\x01\n\x02\x04\x12\x12\x06\xc5\x06\0\xc8\x06\
    \x01\x1a\x9a\x01\x20A\x20response\x20message\x20for\n\x20[ContentAddress\
    ableStorage.FindMissingBlobs][google.devtools.remoteexecution.v1test.Con\
    tentAddressableStorage.FindMissingBlobs].\n\n\x0b\n\x03\x04\x12\x01\x12\
    \x04\xc5\x06\x08\x20\nK\n\x04\x04\x12\x02\0\x12\x04\xc7\x06\x02+\x1a=\
    \x20A\x20list\x20of\x20the\x20blobs\x20requested\x20*not*\x20present\x20\
    in\x20the\x20storage.\n\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xc7\x06\x02\
    \n\n\r\n\x05\x04\x12\x02\0\x06\x12\x04\xc7\x06\x0b\x11\n\r\n\x05\x04\x12\
    \x02\0\x01\x12\x04\xc7\x06\x12&\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xc7\
    \x06)*\n\xaf\x01\n\x02\x04\x13\x12\x06\xcc\x06\0\xd2\x06\x01\x1a\xa0\x01\
    \x20A\x20single\x20request\x20message\x20for\n\x20[ContentAddressableSto\
    rage.BatchUpdateBlobs][google.devtools.remoteexecution.v1test.ContentAdd\
    ressableStorage.BatchUpdateBlobs].\n\n\x0b\n\x03\x04\x13\x01\x12\x04\xcc\
    \x06\x08\x19\nJ\n\x04\x04\x13\x02\0\x12\x04\xce\x06\x02\x1c\x1a<\x20The\
    \x20digest\x20of\x20the\x20blob.\x20This\x20MUST\x20be\x20the\x20digest\
    \x20of\x20`data`.\n\n\x0f\n\x05\x04\x13\x02\0\x04\x12\x06\xce\x06\x02\
    \xcc\x06\x1b\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\xce\x06\x02\x08\n\r\n\
    \x05\x04\x13\x02\0\x01\x12\x04\xce\x06\t\x17\n\r\n\x05\x04\x13\x02\0\x03\
    \x12\x04\xce\x06\x1a\x1b\n$\n\x04\x04\x13\x02\x01\x12\x04\xd1\x06\x02\
    \x11\x1a\x16\x20The\x20raw\x20binary\x20data.\n\n\x0f\n\x05\x04\x13\x02\
    \x01\x04\x12\x06\xd1\x06\x02\xce\x06\x1c\n\r\n\x05\x04\x13\x02\x01\x05\
    \x12\x04\xd1\x06\x02\x07\n\r\n\x05\x04\x13\x02\x01\x01\x12\x04\xd1\x06\
    \x08\x0c\n\r\n\x05\x04\x13\x02\x01\x03\x12\x04\xd1\x06\x0f\x10\n\xa8\x01\
    \n\x02\x04\x14\x12\x06\xd6\x06\0\xe0\x06\x01\x1a\x99\x01\x20A\x20request\
    \x20message\x20for\n\x20[ContentAddressableStorage.BatchUpdateBlobs][goo\
    gle.devtools.remoteexecution.v1test.ContentAddressableStorage.BatchUpdat\
    eBlobs].\n\n\x0b\n\x03\x04\x14\x01\x12\x04\xd6\x06\x08\x1f\n\xc1\x02\n\
    \x04\x04\x14\x02\0\x12\x04\xdc\x06\x02\x1b\x1a\xb2\x02\x20The\x20instanc\
    e\x20of\x20the\x20execution\x20system\x20to\x20operate\x20against.\x20A\
    \x20server\x20may\n\x20support\x20multiple\x20instances\x20of\x20the\x20\
    execution\x20system\x20(with\x20their\x20own\x20workers,\n\x20storage,\
    \x20caches,\x20etc.).\x20The\x20server\x20MAY\x20require\x20use\x20of\
    \x20this\x20field\x20to\x20select\n\x20between\x20them\x20in\x20an\x20im\
    plementation-defined\x20fashion,\x20otherwise\x20it\x20can\x20be\n\x20om\
    itted.\n\n\x0f\n\x05\x04\x14\x02\0\x04\x12\x06\xdc\x06\x02\xd6\x06!\n\r\
    \n\x05\x04\x14\x02\0\x05\x12\x04\xdc\x06\x02\x08\n\r\n\x05\x04\x14\x02\0\
    \x01\x12\x04\xdc\x06\t\x16\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\xdc\x06\
    \x19\x1a\n/\n\x04\x04\x14\x02\x01\x12\x04\xdf\x06\x02*\x1a!\x20The\x20in\
    dividual\x20upload\x20requests.\n\n\r\n\x05\x04\x14\x02\x01\x04\x12\x04\
    \xdf\x06\x02\n\n\r\n\x05\x04\x14\x02\x01\x06\x12\x04\xdf\x06\x0b\x1c\n\r\
    \n\x05\x04\x14\x02\x01\x01\x12\x04\xdf\x06\x1d%\n\r\n\x05\x04\x14\x02\
    \x01\x03\x12\x04\xdf\x06()\n\xa9\x01\n\x02\x04\x15\x12\x06\xe4\x06\0\xf0\
    \x06\x01\x1a\x9a\x01\x20A\x20response\x20message\x20for\n\x20[ContentAdd\
    ressableStorage.BatchUpdateBlobs][google.devtools.remoteexecution.v1test\
    .ContentAddressableStorage.BatchUpdateBlobs].\n\n\x0b\n\x03\x04\x15\x01\
    \x12\x04\xe4\x06\x08\x20\n\\\n\x04\x04\x15\x03\0\x12\x06\xe6\x06\x02\xec\
    \x06\x03\x1aL\x20A\x20response\x20corresponding\x20to\x20a\x20single\x20\
    blob\x20that\x20the\x20client\x20tried\x20to\x20upload.\n\n\r\n\x05\x04\
    \x15\x03\0\x01\x12\x04\xe6\x06\n\x12\n@\n\x06\x04\x15\x03\0\x02\0\x12\
    \x04\xe8\x06\x04\x1b\x1a0\x20The\x20digest\x20to\x20which\x20this\x20res\
    ponse\x20corresponds.\n\n\x11\n\x07\x04\x15\x03\0\x02\0\x04\x12\x06\xe8\
    \x06\x04\xe6\x06\x14\n\x0f\n\x07\x04\x15\x03\0\x02\0\x06\x12\x04\xe8\x06\
    \x04\n\n\x0f\n\x07\x04\x15\x03\0\x02\0\x01\x12\x04\xe8\x06\x0b\x16\n\x0f\
    \n\x07\x04\x15\x03\0\x02\0\x03\x12\x04\xe8\x06\x19\x1a\n?\n\x06\x04\x15\
    \x03\0\x02\x01\x12\x04\xeb\x06\x04!\x1a/\x20The\x20result\x20of\x20attem\
    pting\x20to\x20upload\x20that\x20blob.\n\n\x11\n\x07\x04\x15\x03\0\x02\
    \x01\x04\x12\x06\xeb\x06\x04\xe8\x06\x1b\n\x0f\n\x07\x04\x15\x03\0\x02\
    \x01\x06\x12\x04\xeb\x06\x04\x15\n\x0f\n\x07\x04\x15\x03\0\x02\x01\x01\
    \x12\x04\xeb\x06\x16\x1c\n\x0f\n\x07\x04\x15\x03\0\x02\x01\x03\x12\x04\
    \xeb\x06\x1f\x20\n.\n\x04\x04\x15\x02\0\x12\x04\xef\x06\x02\"\x1a\x20\
    \x20The\x20responses\x20to\x20the\x20requests.\n\n\r\n\x05\x04\x15\x02\0\
    \x04\x12\x04\xef\x06\x02\n\n\r\n\x05\x04\x15\x02\0\x06\x12\x04\xef\x06\
    \x0b\x13\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\xef\x06\x14\x1d\n\r\n\x05\
    \x04\x15\x02\0\x03\x12\x04\xef\x06\x20!\n\xd0\x01\n\x02\x04\x16\x12\x06\
    \xf5\x06\0\x8d\x07\x01\x1a\xc1\x01\x20A\x20request\x20message\x20for\n\
    \x20[ContentAddressableStorage.GetTree][google.devtools.remoteexecution.\
    v1test.ContentAddressableStorage.GetTree].\n\x20This\x20message\x20is\
    \x20deprecated\x20and\x20should\x20no\x20longer\x20be\x20used.\n\n\x0b\n\
    \x03\x04\x16\x01\x12\x04\xf5\x06\x08\x16\n\xc1\x02\n\x04\x04\x16\x02\0\
    \x12\x04\xfb\x06\x02\x1b\x1a\xb2\x02\x20The\x20instance\x20of\x20the\x20\
    execution\x20system\x20to\x20operate\x20against.\x20A\x20server\x20may\n\
    \x20support\x20multiple\x20instances\x20of\x20the\x20execution\x20system\
    \x20(with\x20their\x20own\x20workers,\n\x20storage,\x20caches,\x20etc.).\
    \x20The\x20server\x20MAY\x20require\x20use\x20of\x20this\x20field\x20to\
    \x20select\n\x20between\x20them\x20in\x20an\x20implementation-defined\
    \x20fashion,\x20otherwise\x20it\x20can\x20be\n\x20omitted.\n\n\x0f\n\x05\
    \x04\x16\x02\0\x04\x12\x06\xfb\x06\x02\xf5\x06\x18\n\r\n\x05\x04\x16\x02\
    \0\x05\x12\x04\xfb\x06\x02\x08\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xfb\
    \x06\t\x16\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\xfb\x06\x19\x1a\n\xf7\x01\
    \n\x04\x04\x16\x02\x01\x12\x04\x81\x07\x02\x19\x1a\xe8\x01\x20The\x20dig\
    est\x20of\x20the\x20root,\x20which\x20must\x20be\x20an\x20encoded\n\x20[\
    Directory][google.devtools.remoteexecution.v1test.Directory]\x20message\
    \n\x20stored\x20in\x20the\n\x20[ContentAddressableStorage][google.devtoo\
    ls.remoteexecution.v1test.ContentAddressableStorage].\n\n\x0f\n\x05\x04\
    \x16\x02\x01\x04\x12\x06\x81\x07\x02\xfb\x06\x1b\n\r\n\x05\x04\x16\x02\
    \x01\x06\x12\x04\x81\x07\x02\x08\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\
    \x81\x07\t\x14\n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\x81\x07\x17\x18\n\
    \xb8\x02\n\x04\x04\x16\x02\x02\x12\x04\x87\x07\x02\x16\x1a\xa9\x02\x20A\
    \x20maximum\x20page\x20size\x20to\x20request.\x20If\x20present,\x20the\
    \x20server\x20will\x20request\x20no\x20more\n\x20than\x20this\x20many\
    \x20items.\x20Regardless\x20of\x20whether\x20a\x20page\x20size\x20is\x20\
    specified,\x20the\n\x20server\x20may\x20place\x20its\x20own\x20limit\x20\
    on\x20the\x20number\x20of\x20items\x20to\x20be\x20returned\x20and\n\x20r\
    equire\x20the\x20client\x20to\x20retrieve\x20more\x20items\x20using\x20a\
    \x20subsequent\x20request.\n\n\x0f\n\x05\x04\x16\x02\x02\x04\x12\x06\x87\
    \x07\x02\x81\x07\x19\n\r\n\x05\x04\x16\x02\x02\x05\x12\x04\x87\x07\x02\
    \x07\n\r\n\x05\x04\x16\x02\x02\x01\x12\x04\x87\x07\x08\x11\n\r\n\x05\x04\
    \x16\x02\x02\x03\x12\x04\x87\x07\x14\x15\n\xe4\x01\n\x04\x04\x16\x02\x03\
    \x12\x04\x8c\x07\x02\x18\x1a\xd5\x01\x20A\x20page\x20token,\x20which\x20\
    must\x20be\x20a\x20value\x20received\x20in\x20a\x20previous\n\x20[GetTre\
    eResponse][google.devtools.remoteexecution.v1test.GetTreeResponse].\n\
    \x20If\x20present,\x20the\x20server\x20will\x20use\x20it\x20to\x20return\
    \x20the\x20following\x20page\x20of\x20results.\n\n\x0f\n\x05\x04\x16\x02\
    \x03\x04\x12\x06\x8c\x07\x02\x87\x07\x16\n\r\n\x05\x04\x16\x02\x03\x05\
    \x12\x04\x8c\x07\x02\x08\n\r\n\x05\x04\x16\x02\x03\x01\x12\x04\x8c\x07\t\
    \x13\n\r\n\x05\x04\x16\x02\x03\x03\x12\x04\x8c\x07\x16\x17\n\xd1\x01\n\
    \x02\x04\x17\x12\x06\x92\x07\0\x9b\x07\x01\x1a\xc2\x01\x20A\x20response\
    \x20message\x20for\n\x20[ContentAddressableStorage.GetTree][google.devto\
    ols.remoteexecution.v1test.ContentAddressableStorage.GetTree].\n\x20This\
    \x20message\x20is\x20deprecated\x20and\x20should\x20no\x20longer\x20be\
    \x20used.\n\n\x0b\n\x03\x04\x17\x01\x12\x04\x92\x07\x08\x17\nB\n\x04\x04\
    \x17\x02\0\x12\x04\x94\x07\x02%\x1a4\x20The\x20directories\x20descended\
    \x20from\x20the\x20requested\x20root.\n\n\r\n\x05\x04\x17\x02\0\x04\x12\
    \x04\x94\x07\x02\n\n\r\n\x05\x04\x17\x02\0\x06\x12\x04\x94\x07\x0b\x14\n\
    \r\n\x05\x04\x17\x02\0\x01\x12\x04\x94\x07\x15\x20\n\r\n\x05\x04\x17\x02\
    \0\x03\x12\x04\x94\x07#$\n\x92\x02\n\x04\x04\x17\x02\x01\x12\x04\x9a\x07\
    \x02\x1d\x1a\x83\x02\x20If\x20present,\x20signifies\x20that\x20there\x20\
    are\x20more\x20results\x20which\x20the\x20client\x20can\n\x20retrieve\
    \x20by\x20passing\x20this\x20as\x20the\x20page_token\x20in\x20a\x20subse\
    quent\n\x20[request][google.devtools.remoteexecution.v1test.GetTreeReque\
    st].\n\x20If\x20empty,\x20signifies\x20that\x20this\x20is\x20the\x20last\
    \x20page\x20of\x20results.\n\n\x0f\n\x05\x04\x17\x02\x01\x04\x12\x06\x9a\
    \x07\x02\x94\x07%\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\x9a\x07\x02\x08\
    \n\r\n\x05\x04\x17\x02\x01\x01\x12\x04\x9a\x07\t\x18\n\r\n\x05\x04\x17\
    \x02\x01\x03\x12\x04\x9a\x07\x1b\x1c\n:\n\x02\x04\x18\x12\x06\x9e\x07\0\
    \xa4\x07\x01\x1a,\x20Details\x20for\x20the\x20tool\x20used\x20to\x20call\
    \x20the\x20API.\n\n\x0b\n\x03\x04\x18\x01\x12\x04\x9e\x07\x08\x13\n-\n\
    \x04\x04\x18\x02\0\x12\x04\xa0\x07\x02\x17\x1a\x1f\x20Name\x20of\x20the\
    \x20tool,\x20e.g.\x20bazel.\n\n\x0f\n\x05\x04\x18\x02\0\x04\x12\x06\xa0\
    \x07\x02\x9e\x07\x15\n\r\n\x05\x04\x18\x02\0\x05\x12\x04\xa0\x07\x02\x08\
    \n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xa0\x07\t\x12\n\r\n\x05\x04\x18\x02\
    \0\x03\x12\x04\xa0\x07\x15\x16\nE\n\x04\x04\x18\x02\x01\x12\x04\xa3\x07\
    \x02\x1a\x1a7\x20Version\x20of\x20the\x20tool\x20used\x20for\x20the\x20r\
    equest,\x20e.g.\x205.0.3.\n\n\x0f\n\x05\x04\x18\x02\x01\x04\x12\x06\xa3\
    \x07\x02\xa0\x07\x17\n\r\n\x05\x04\x18\x02\x01\x05\x12\x04\xa3\x07\x02\
    \x08\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\xa3\x07\t\x15\n\r\n\x05\x04\
    \x18\x02\x01\x03\x12\x04\xa3\x07\x18\x19\n\x98\x03\n\x02\x04\x19\x12\x06\
    \xac\x07\0\xbc\x07\x01\x1a\x89\x03\x20An\x20optional\x20Metadata\x20to\
    \x20attach\x20to\x20any\x20RPC\x20request\x20to\x20tell\x20the\x20server\
    \x20about\x20an\n\x20external\x20context\x20of\x20the\x20request.\x20The\
    \x20server\x20may\x20use\x20this\x20for\x20logging\x20or\x20other\n\x20p\
    urposes.\x20To\x20use\x20it,\x20the\x20client\x20attaches\x20the\x20head\
    er\x20to\x20the\x20call\x20using\x20the\n\x20canonical\x20proto\x20seria\
    lization:\n\x20name:\x20google.devtools.remoteexecution.v1test.requestme\
    tadata-bin\n\x20contents:\x20the\x20base64\x20encoded\x20binary\x20Reque\
    stMetadata\x20message.\n\n\x0b\n\x03\x04\x19\x01\x12\x04\xac\x07\x08\x17\
    \n?\n\x04\x04\x19\x02\0\x12\x04\xae\x07\x02\x1f\x1a1\x20The\x20details\
    \x20for\x20the\x20tool\x20invoking\x20the\x20requests.\n\n\x0f\n\x05\x04\
    \x19\x02\0\x04\x12\x06\xae\x07\x02\xac\x07\x19\n\r\n\x05\x04\x19\x02\0\
    \x06\x12\x04\xae\x07\x02\r\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xae\x07\
    \x0e\x1a\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xae\x07\x1d\x1e\n\xc0\x01\n\
    \x04\x04\x19\x02\x01\x12\x04\xb3\x07\x02\x17\x1a\xb1\x01\x20An\x20identi\
    fier\x20that\x20ties\x20multiple\x20requests\x20to\x20the\x20same\x20act\
    ion.\n\x20For\x20example,\x20multiple\x20requests\x20to\x20the\x20CAS,\
    \x20Action\x20Cache,\x20and\x20Execution\n\x20API\x20are\x20used\x20in\
    \x20order\x20to\x20compile\x20foo.cc.\n\n\x0f\n\x05\x04\x19\x02\x01\x04\
    \x12\x06\xb3\x07\x02\xae\x07\x1f\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\
    \xb3\x07\x02\x08\n\r\n\x05\x04\x19\x02\x01\x01\x12\x04\xb3\x07\t\x12\n\r\
    \n\x05\x04\x19\x02\x01\x03\x12\x04\xb3\x07\x15\x16\n\x9c\x01\n\x04\x04\
    \x19\x02\x02\x12\x04\xb7\x07\x02\x20\x1a\x8d\x01\x20An\x20identifier\x20\
    that\x20ties\x20multiple\x20actions\x20together\x20to\x20a\x20final\x20r\
    esult.\n\x20For\x20example,\x20multiple\x20actions\x20are\x20required\
    \x20to\x20build\x20and\x20run\x20foo_test.\n\n\x0f\n\x05\x04\x19\x02\x02\
    \x04\x12\x06\xb7\x07\x02\xb3\x07\x17\n\r\n\x05\x04\x19\x02\x02\x05\x12\
    \x04\xb7\x07\x02\x08\n\r\n\x05\x04\x19\x02\x02\x01\x12\x04\xb7\x07\t\x1b\
    \n\r\n\x05\x04\x19\x02\x02\x03\x12\x04\xb7\x07\x1e\x1f\n\xa2\x01\n\x04\
    \x04\x19\x02\x03\x12\x04\xbb\x07\x02'\x1a\x93\x01\x20An\x20identifier\
    \x20to\x20tie\x20multiple\x20tool\x20invocations\x20together.\x20For\x20\
    example,\n\x20runs\x20of\x20foo_test,\x20bar_test\x20and\x20baz_test\x20\
    on\x20a\x20post-submit\x20of\x20a\x20given\x20patch.\n\n\x0f\n\x05\x04\
    \x19\x02\x03\x04\x12\x06\xbb\x07\x02\xb7\x07\x20\n\r\n\x05\x04\x19\x02\
    \x03\x05\x12\x04\xbb\x07\x02\x08\n\r\n\x05\x04\x19\x02\x03\x01\x12\x04\
    \xbb\x07\t\"\n\r\n\x05\x04\x19\x02\x03\x03\x12\x04\xbb\x07%&b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
