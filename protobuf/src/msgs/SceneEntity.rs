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

//! Generated file from `foxglove/SceneEntity.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foxglove.SceneEntity)
pub struct SceneEntity {
    // message fields
    // @@protoc_insertion_point(field:foxglove.SceneEntity.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.frame_id)
    pub frame_id: ::std::string::String,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.lifetime)
    pub lifetime: ::protobuf::MessageField<::protobuf::well_known_types::duration::Duration>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.frame_locked)
    pub frame_locked: bool,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.metadata)
    pub metadata: ::std::vec::Vec<super::KeyValuePair::KeyValuePair>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.arrows)
    pub arrows: ::std::vec::Vec<super::ArrowPrimitive::ArrowPrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.cubes)
    pub cubes: ::std::vec::Vec<super::CubePrimitive::CubePrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.spheres)
    pub spheres: ::std::vec::Vec<super::SpherePrimitive::SpherePrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.cylinders)
    pub cylinders: ::std::vec::Vec<super::CylinderPrimitive::CylinderPrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.lines)
    pub lines: ::std::vec::Vec<super::LinePrimitive::LinePrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.triangles)
    pub triangles: ::std::vec::Vec<super::TriangleListPrimitive::TriangleListPrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.texts)
    pub texts: ::std::vec::Vec<super::TextPrimitive::TextPrimitive>,
    // @@protoc_insertion_point(field:foxglove.SceneEntity.models)
    pub models: ::std::vec::Vec<super::ModelPrimitive::ModelPrimitive>,
    // special fields
    // @@protoc_insertion_point(special_field:foxglove.SceneEntity.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEntity {
    fn default() -> &'a SceneEntity {
        <SceneEntity as ::protobuf::Message>::default_instance()
    }
}

impl SceneEntity {
    pub fn new() -> SceneEntity {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &SceneEntity| { &m.timestamp },
            |m: &mut SceneEntity| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "frame_id",
            |m: &SceneEntity| { &m.frame_id },
            |m: &mut SceneEntity| { &mut m.frame_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &SceneEntity| { &m.id },
            |m: &mut SceneEntity| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::duration::Duration>(
            "lifetime",
            |m: &SceneEntity| { &m.lifetime },
            |m: &mut SceneEntity| { &mut m.lifetime },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "frame_locked",
            |m: &SceneEntity| { &m.frame_locked },
            |m: &mut SceneEntity| { &mut m.frame_locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "metadata",
            |m: &SceneEntity| { &m.metadata },
            |m: &mut SceneEntity| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "arrows",
            |m: &SceneEntity| { &m.arrows },
            |m: &mut SceneEntity| { &mut m.arrows },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "cubes",
            |m: &SceneEntity| { &m.cubes },
            |m: &mut SceneEntity| { &mut m.cubes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "spheres",
            |m: &SceneEntity| { &m.spheres },
            |m: &mut SceneEntity| { &mut m.spheres },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "cylinders",
            |m: &SceneEntity| { &m.cylinders },
            |m: &mut SceneEntity| { &mut m.cylinders },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lines",
            |m: &SceneEntity| { &m.lines },
            |m: &mut SceneEntity| { &mut m.lines },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "triangles",
            |m: &SceneEntity| { &m.triangles },
            |m: &mut SceneEntity| { &mut m.triangles },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "texts",
            |m: &SceneEntity| { &m.texts },
            |m: &mut SceneEntity| { &mut m.texts },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "models",
            |m: &SceneEntity| { &m.models },
            |m: &mut SceneEntity| { &mut m.models },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEntity>(
            "SceneEntity",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEntity {
    const NAME: &'static str = "SceneEntity";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                18 => {
                    self.frame_id = is.read_string()?;
                },
                26 => {
                    self.id = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.lifetime)?;
                },
                40 => {
                    self.frame_locked = is.read_bool()?;
                },
                50 => {
                    self.metadata.push(is.read_message()?);
                },
                58 => {
                    self.arrows.push(is.read_message()?);
                },
                66 => {
                    self.cubes.push(is.read_message()?);
                },
                74 => {
                    self.spheres.push(is.read_message()?);
                },
                82 => {
                    self.cylinders.push(is.read_message()?);
                },
                90 => {
                    self.lines.push(is.read_message()?);
                },
                98 => {
                    self.triangles.push(is.read_message()?);
                },
                106 => {
                    self.texts.push(is.read_message()?);
                },
                114 => {
                    self.models.push(is.read_message()?);
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
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.frame_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.frame_id);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.id);
        }
        if let Some(v) = self.lifetime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.frame_locked != false {
            my_size += 1 + 1;
        }
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.arrows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.cubes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.spheres {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.cylinders {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.lines {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.triangles {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.texts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.models {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.frame_id.is_empty() {
            os.write_string(2, &self.frame_id)?;
        }
        if !self.id.is_empty() {
            os.write_string(3, &self.id)?;
        }
        if let Some(v) = self.lifetime.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.frame_locked != false {
            os.write_bool(5, self.frame_locked)?;
        }
        for v in &self.metadata {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.arrows {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        for v in &self.cubes {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.spheres {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.cylinders {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.lines {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.triangles {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.texts {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for v in &self.models {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SceneEntity {
        SceneEntity::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.frame_id.clear();
        self.id.clear();
        self.lifetime.clear();
        self.frame_locked = false;
        self.metadata.clear();
        self.arrows.clear();
        self.cubes.clear();
        self.spheres.clear();
        self.cylinders.clear();
        self.lines.clear();
        self.triangles.clear();
        self.texts.clear();
        self.models.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEntity {
        static instance: SceneEntity = SceneEntity {
            timestamp: ::protobuf::MessageField::none(),
            frame_id: ::std::string::String::new(),
            id: ::std::string::String::new(),
            lifetime: ::protobuf::MessageField::none(),
            frame_locked: false,
            metadata: ::std::vec::Vec::new(),
            arrows: ::std::vec::Vec::new(),
            cubes: ::std::vec::Vec::new(),
            spheres: ::std::vec::Vec::new(),
            cylinders: ::std::vec::Vec::new(),
            lines: ::std::vec::Vec::new(),
            triangles: ::std::vec::Vec::new(),
            texts: ::std::vec::Vec::new(),
            models: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEntity {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEntity").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEntity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEntity {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1afoxglove/SceneEntity.proto\x12\x08foxglove\x1a\x1dfoxglove/ArrowPr\
    imitive.proto\x1a\x1cfoxglove/CubePrimitive.proto\x1a\x20foxglove/Cylind\
    erPrimitive.proto\x1a\x1bfoxglove/KeyValuePair.proto\x1a\x1cfoxglove/Lin\
    ePrimitive.proto\x1a\x1dfoxglove/ModelPrimitive.proto\x1a\x1efoxglove/Sp\
    herePrimitive.proto\x1a\x1cfoxglove/TextPrimitive.proto\x1a$foxglove/Tri\
    angleListPrimitive.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\x1fgo\
    ogle/protobuf/timestamp.proto\"\xa0\x05\n\x0bSceneEntity\x128\n\ttimesta\
    mp\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ttimestamp\x12\
    \x19\n\x08frame_id\x18\x02\x20\x01(\tR\x07frameId\x12\x0e\n\x02id\x18\
    \x03\x20\x01(\tR\x02id\x125\n\x08lifetime\x18\x04\x20\x01(\x0b2\x19.goog\
    le.protobuf.DurationR\x08lifetime\x12!\n\x0cframe_locked\x18\x05\x20\x01\
    (\x08R\x0bframeLocked\x122\n\x08metadata\x18\x06\x20\x03(\x0b2\x16.foxgl\
    ove.KeyValuePairR\x08metadata\x120\n\x06arrows\x18\x07\x20\x03(\x0b2\x18\
    .foxglove.ArrowPrimitiveR\x06arrows\x12-\n\x05cubes\x18\x08\x20\x03(\x0b\
    2\x17.foxglove.CubePrimitiveR\x05cubes\x123\n\x07spheres\x18\t\x20\x03(\
    \x0b2\x19.foxglove.SpherePrimitiveR\x07spheres\x129\n\tcylinders\x18\n\
    \x20\x03(\x0b2\x1b.foxglove.CylinderPrimitiveR\tcylinders\x12-\n\x05line\
    s\x18\x0b\x20\x03(\x0b2\x17.foxglove.LinePrimitiveR\x05lines\x12=\n\ttri\
    angles\x18\x0c\x20\x03(\x0b2\x1f.foxglove.TriangleListPrimitiveR\ttriang\
    les\x12-\n\x05texts\x18\r\x20\x03(\x0b2\x17.foxglove.TextPrimitiveR\x05t\
    exts\x120\n\x06models\x18\x0e\x20\x03(\x0b2\x18.foxglove.ModelPrimitiveR\
    \x06modelsb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::ArrowPrimitive::file_descriptor().clone());
            deps.push(super::CubePrimitive::file_descriptor().clone());
            deps.push(super::CylinderPrimitive::file_descriptor().clone());
            deps.push(super::KeyValuePair::file_descriptor().clone());
            deps.push(super::LinePrimitive::file_descriptor().clone());
            deps.push(super::ModelPrimitive::file_descriptor().clone());
            deps.push(super::SpherePrimitive::file_descriptor().clone());
            deps.push(super::TextPrimitive::file_descriptor().clone());
            deps.push(super::TriangleListPrimitive::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::duration::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEntity::generated_message_descriptor_data());
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