//! `Model` object.

use crate::v7400::object::{ObjectHandle, TypedObjectHandle};

pub use self::{
    camera::CameraHandle, light::LightHandle, limbnode::LimbNodeHandle, mesh::MeshHandle,
    null::NullHandle,
};
use crate::v7400::object::property::loaders::MintLoader;
use mint::Vector3;

mod camera;
mod light;
mod limbnode;
mod mesh;
mod null;

define_typed_handle! {
    /// Typed model handle.
    TypedModelHandle(ModelHandle) {
        /// Camera.
        ("Model", "Camera") => Camera(CameraHandle),
        /// Light.
        ("Model", "Light") => Light(LightHandle),
        /// LimbNode.
        ("Model", "LimbNode") => LimbNode(LimbNodeHandle),
        /// Mesh.
        ("Model", "Mesh") => Mesh(MeshHandle),
        /// Null.
        ("Model", "Null") => Null(NullHandle),
    }
}

define_object_subtype! {
    /// `Model` node handle.
    ModelHandle: ObjectHandle
}

impl<'a> ModelHandle<'a> {
    /// Returns the parent model if available.
    pub fn parent_model(&self) -> Option<TypedModelHandle<'a>> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(o) => Some(o),
                _ => None,
            })
            .next()
    }

    /// Returns an iterator of the child models.
    pub fn child_models(&self) -> impl Iterator<Item = TypedModelHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(o) => Some(o),
                _ => None,
            })
    }

    /// Returns the root-most model ancestor of this object in the scene hierarchy.
    ///
    /// If this object has no parent models, returns this object.
    ///
    /// There may be multiple root models in a single scene.
    pub fn root_model<'b>(&'b self) -> TypedModelHandle<'a> {
        let mut parent: TypedModelHandle = match self.get_typed() {
            TypedObjectHandle::Model(o) => o,
            _ => panic!("ModelHandle should always have TypedObjectHandle::Model type."),
        };

        while let Some(m) = parent.parent_model() {
            parent = m;
        }
        parent
    }

    /// Returns the local translation (Lcl Translation) of this model object, if one is present.
    pub fn local_translation(&self) -> anyhow::Result<Option<Vector3<f64>>> {
        // `Model` objects have native typename `FbxNode`.
        self.properties_by_native_typename("FbxNode")
            .get_property("Lcl Translation")
            .map(|prop| prop.load_value(MintLoader::<Vector3<f64>>::new()))
            .transpose()
    }

    /// Returns the local rotation (Lcl Rotation) of this model object, if one is present.
    /// For information on how the FBX SDK uses rotations, see:
    /// https://download.autodesk.com/us/fbx/20112/FBX_SDK_HELP/index.html?url=WS1a9193826455f5ff1f92379812724681e696651.htm,topicNumber=d0e7429
    pub fn local_rotation(&self) -> anyhow::Result<Option<Vector3<f64>>> {
        // `Model` objects have native typename `FbxNode`.
        self.properties_by_native_typename("FbxNode")
            .get_property("Lcl Rotation")
            .map(|prop| prop.load_value(MintLoader::<Vector3<f64>>::new()))
            .transpose()
    }

    /// Returns the pre-rotation (PreRotation) of this model object, if one is present.
    /// For information on how the FBX SDK uses rotations, see:
    /// https://download.autodesk.com/us/fbx/20112/FBX_SDK_HELP/index.html?url=WS1a9193826455f5ff1f92379812724681e696651.htm,topicNumber=d0e7429
    pub fn pre_rotation(&self) -> anyhow::Result<Option<Vector3<f64>>> {
        // `Model` objects have native typename `FbxNode`.
        self.properties_by_native_typename("FbxNode")
            .get_property("PreRotation")
            .map(|prop| prop.load_value(MintLoader::<Vector3<f64>>::new()))
            .transpose()
    }

    /// Returns the local scale (Lcl Scale) of this model object, if one is present.
    pub fn local_scaling(&self) -> anyhow::Result<Option<Vector3<f64>>> {
        // `Model` objects have native typename `FbxNode`.
        self.properties_by_native_typename("FbxNode")
            .get_property("Lcl Scaling")
            .map(|prop| prop.load_value(MintLoader::<Vector3<f64>>::new()))
            .transpose()
    }
}
