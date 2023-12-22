// Copyright (c) Tribufu. All Rights Reserved.

pub mod arch;
pub mod future;
pub mod platform;

pub use type_uuid::TypeUuid;
pub use type_uuid::TypeUuidDynamic;
pub use uuid::Uuid;

pub type UuidBytes = type_uuid::Bytes;
