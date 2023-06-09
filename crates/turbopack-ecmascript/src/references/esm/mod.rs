pub(crate) mod base;
pub(crate) mod binding;
pub(crate) mod dynamic;
pub(crate) mod export;
pub(crate) mod meta;
pub(crate) mod module_id;
pub(crate) mod module_item;
pub(crate) mod url;

use turbo_tasks::Vc;

pub use self::{
    base::EsmAssetReference,
    binding::EsmBinding,
    dynamic::EsmAsyncAssetReference,
    export::EsmExports,
    meta::{ImportMetaBinding, ImportMetaRef},
    module_item::EsmModuleItem,
    url::UrlAssetReference,
};
