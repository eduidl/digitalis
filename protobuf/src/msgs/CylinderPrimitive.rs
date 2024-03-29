// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `foxglove/CylinderPrimitive.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foxglove.CylinderPrimitive)
pub struct CylinderPrimitive {
    // message fields
    // @@protoc_insertion_point(field:foxglove.CylinderPrimitive.pose)
    pub pose: ::protobuf::MessageField<super::Pose::Pose>,
    // @@protoc_insertion_point(field:foxglove.CylinderPrimitive.size)
    pub size: ::protobuf::MessageField<super::Vector3::Vector3>,
    // @@protoc_insertion_point(field:foxglove.CylinderPrimitive.bottom_scale)
    pub bottom_scale: f64,
    // @@protoc_insertion_point(field:foxglove.CylinderPrimitive.top_scale)
    pub top_scale: f64,
    // @@protoc_insertion_point(field:foxglove.CylinderPrimitive.color)
    pub color: ::protobuf::MessageField<super::Color::Color>,
    // special fields
    // @@protoc_insertion_point(special_field:foxglove.CylinderPrimitive.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CylinderPrimitive {
    fn default() -> &'a CylinderPrimitive {
        <CylinderPrimitive as ::protobuf::Message>::default_instance()
    }
}

impl CylinderPrimitive {
    pub fn new() -> CylinderPrimitive {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Pose::Pose>(
            "pose",
            |m: &CylinderPrimitive| { &m.pose },
            |m: &mut CylinderPrimitive| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector3::Vector3>(
            "size",
            |m: &CylinderPrimitive| { &m.size },
            |m: &mut CylinderPrimitive| { &mut m.size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bottom_scale",
            |m: &CylinderPrimitive| { &m.bottom_scale },
            |m: &mut CylinderPrimitive| { &mut m.bottom_scale },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "top_scale",
            |m: &CylinderPrimitive| { &m.top_scale },
            |m: &mut CylinderPrimitive| { &mut m.top_scale },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Color::Color>(
            "color",
            |m: &CylinderPrimitive| { &m.color },
            |m: &mut CylinderPrimitive| { &mut m.color },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CylinderPrimitive>(
            "CylinderPrimitive",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CylinderPrimitive {
    const NAME: &'static str = "CylinderPrimitive";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.size)?;
                },
                25 => {
                    self.bottom_scale = is.read_double()?;
                },
                33 => {
                    self.top_scale = is.read_double()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.color)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.bottom_scale != 0. {
            my_size += 1 + 8;
        }
        if self.top_scale != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.size.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.bottom_scale != 0. {
            os.write_double(3, self.bottom_scale)?;
        }
        if self.top_scale != 0. {
            os.write_double(4, self.top_scale)?;
        }
        if let Some(v) = self.color.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CylinderPrimitive {
        CylinderPrimitive::new()
    }

    fn clear(&mut self) {
        self.pose.clear();
        self.size.clear();
        self.bottom_scale = 0.;
        self.top_scale = 0.;
        self.color.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CylinderPrimitive {
        static instance: CylinderPrimitive = CylinderPrimitive {
            pose: ::protobuf::MessageField::none(),
            size: ::protobuf::MessageField::none(),
            bottom_scale: 0.,
            top_scale: 0.,
            color: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CylinderPrimitive {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CylinderPrimitive").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CylinderPrimitive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CylinderPrimitive {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20foxglove/CylinderPrimitive.proto\x12\x08foxglove\x1a\x14foxglove/C\
    olor.proto\x1a\x13foxglove/Pose.proto\x1a\x16foxglove/Vector3.proto\"\
    \xc5\x01\n\x11CylinderPrimitive\x12\"\n\x04pose\x18\x01\x20\x01(\x0b2\
    \x0e.foxglove.PoseR\x04pose\x12%\n\x04size\x18\x02\x20\x01(\x0b2\x11.fox\
    glove.Vector3R\x04size\x12!\n\x0cbottom_scale\x18\x03\x20\x01(\x01R\x0bb\
    ottomScale\x12\x1b\n\ttop_scale\x18\x04\x20\x01(\x01R\x08topScale\x12%\n\
    \x05color\x18\x05\x20\x01(\x0b2\x0f.foxglove.ColorR\x05colorb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::Color::file_descriptor().clone());
            deps.push(super::Pose::file_descriptor().clone());
            deps.push(super::Vector3::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CylinderPrimitive::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
