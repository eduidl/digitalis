#![doc = include_str!("../README.md")]

mod msgs;
pub use msgs::{
    ArrowPrimitive::ArrowPrimitive, CameraCalibration::CameraCalibration,
    CircleAnnotation::CircleAnnotation, Color::Color, CompressedImage::CompressedImage,
    CubePrimitive::CubePrimitive, CylinderPrimitive::CylinderPrimitive,
    FrameTransform::FrameTransform, FrameTransforms::FrameTransforms, GeoJSON::GeoJSON, Grid::Grid,
    ImageAnnotations::ImageAnnotations, KeyValuePair::KeyValuePair, LaserScan::LaserScan,
    LinePrimitive::LinePrimitive, LocationFix::LocationFix, Log::Log,
    ModelPrimitive::ModelPrimitive, PackedElementField::PackedElementField, Point2::Point2,
    Point3::Point3, PointCloud::PointCloud, PointsAnnotation::PointsAnnotation, Pose::Pose,
    PoseInFrame::PoseInFrame, PosesInFrame::PosesInFrame, Quaternion::Quaternion,
    RawImage::RawImage, SceneEntity::SceneEntity, SceneEntityDeletion::SceneEntityDeletion,
    SceneUpdate::SceneUpdate, SpherePrimitive::SpherePrimitive, TextPrimitive::TextPrimitive,
    TriangleListPrimitive::TriangleListPrimitive, Vector2::Vector2, Vector3::Vector3, *,
};
pub use protobuf; // re-export

pub mod base {
    // re-export
    pub use protobuf::well_known_types::{duration::Duration, timestamp::Timestamp};
}
