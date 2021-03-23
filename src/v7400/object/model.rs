//! `Model` object.

use crate::v7400::object::{ObjectHandle, TypedObjectHandle};

pub use self::{
    camera::CameraHandle, light::LightHandle, limbnode::LimbNodeHandle, mesh::MeshHandle,
    null::NullHandle,
};
use mint::Vector3;
use fbxcel::low::v7400::AttributeValue;
use crate::v7400::object::property::loaders::MintLoader;

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

    /// Gets the root-most model parent of this object in the scene hierarchy.
    ///
    /// If this object has no parents, returns this object.
    ///
    /// There may be multiple root models in a single scene.
    pub fn root_model<'b>(&'b self) -> TypedModelHandle<'a> {
        let mut parent : TypedModelHandle = match self.get_typed() {
            TypedObjectHandle::Model(o) => o,
            _ => panic!("ModelHandle should always have TypedObjectHandle::Model type.")
        };

        while let Some(m) = parent.parent_model() {
            parent = m;
        }
        return parent;
    }

    /// Returns the local rotation (Lcl Rotation) of this model object, if one is present.
    pub fn local_rotation(&self) -> Option<Vector3<f64>> {
        let rotation = self.direct_properties()?.get_property("Lcl Rotation")?;

        // Pull out the first three float attributes.
        let components: Vec<f64> = rotation.node().attributes().iter().filter_map(|attr| {
            return match attr {
                AttributeValue::F64(value) => Some(value.clone()),
                _ => None
            }
        }).collect();

        // todo: Is it ever possible for a rotation to return a quaternion (ie. 4 values)?
        assert_eq!(components.len(), 3);

        return Some(Vector3::from_slice(components.as_slice()));
    }

    /// Returns the local scale (Lcl Scale) of this model object, if one is present.
    pub fn local_scale(&self) -> anyhow::Result<Option<Vector3<f64>>> {
        // `Model` objects have native typename `FbxNode`.
        self.properties_by_native_typename("FbxNode")
            .get_property("Lcl Scaling")
            .map(|prop| prop.load_value(MintLoader::<Vector3<f64>>::new()))
            .transpose()
    }
}
