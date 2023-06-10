use anyhow::Result;
use turbo_tasks::{Value, Vc};
use turbo_tasks_fs::FileSystemPath;
use turbopack_core::{
    introspect::{Introspectable, IntrospectableChildren},
    issue::IssueContextExt,
};

use super::{
    ContentSource, ContentSourceContent, ContentSourceData, ContentSourceDataVary,
    ContentSourceResult, ContentSources, GetContentSourceContent,
};

#[turbo_tasks::value]
pub struct IssueContextContentSource {
    context: Option<Vc<FileSystemPath>>,
    description: String,
    source: Vc<Box<dyn ContentSource>>,
}

#[turbo_tasks::value_impl]
impl IssueContextContentSource {
    #[turbo_tasks::function]
    pub fn new_context(
        context: Vc<FileSystemPath>,
        description: String,
        source: Vc<Box<dyn ContentSource>>,
    ) -> Vc<Self> {
        IssueContextContentSource {
            context: Some(context),
            description: description.to_string(),
            source,
        }
        .cell()
    }

    #[turbo_tasks::function]
    pub fn new_description(description: String, source: Vc<Box<dyn ContentSource>>) -> Vc<Self> {
        IssueContextContentSource {
            context: None,
            description: description.to_string(),
            source,
        }
        .cell()
    }
}

#[turbo_tasks::value_impl]
impl ContentSource for IssueContextContentSource {
    #[turbo_tasks::function]
    async fn get(
        self: Vc<Self>,
        path: String,
        data: Value<ContentSourceData>,
    ) -> Result<Vc<ContentSourceResult>> {
        let this = self.await?;
        let result = this
            .source
            .get(path, data)
            .issue_context(this.context, &this.description)
            .await?;
        if let ContentSourceResult::Result {
            get_content,
            specificity,
        } = *result.await?
        {
            Ok(ContentSourceResult::Result {
                get_content: IssueContextGetContentSourceContent {
                    get_content,
                    source: self,
                }
                .cell()
                .into(),
                specificity,
            }
            .cell())
        } else {
            Ok(result)
        }
    }

    #[turbo_tasks::function]
    fn get_children(&self) -> Vc<ContentSources> {
        Vc::cell(vec![self.source])
    }
}

#[turbo_tasks::value]
struct IssueContextGetContentSourceContent {
    get_content: Vc<Box<dyn GetContentSourceContent>>,
    source: Vc<IssueContextContentSource>,
}

#[turbo_tasks::value_impl]
impl GetContentSourceContent for IssueContextGetContentSourceContent {
    #[turbo_tasks::function]
    async fn vary(&self) -> Result<Vc<ContentSourceDataVary>> {
        let source = self.source.await?;
        let result = self
            .get_content
            .vary()
            .issue_context(source.context, &source.description)
            .await?;
        Ok(result)
    }

    #[turbo_tasks::function]
    async fn get(&self, data: Value<ContentSourceData>) -> Result<Vc<ContentSourceContent>> {
        let source = self.source.await?;
        let result = self
            .get_content
            .get(data)
            .issue_context(source.context, &source.description)
            .await?;
        Ok(result)
    }
}

#[turbo_tasks::value_impl]
impl Introspectable for IssueContextContentSource {
    #[turbo_tasks::function]
    async fn ty(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.ty()
            } else {
                Vc::cell("IssueContextContentSource".to_string())
            },
        )
    }

    #[turbo_tasks::function]
    async fn title(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                let title = source.title().await?;
                Vc::cell(format!("{}: {}", self.description, title))
            } else {
                Vc::cell(self.description.clone())
            },
        )
    }

    #[turbo_tasks::function]
    async fn details(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.details()
            } else {
                Vc::cell(String::new())
            },
        )
    }

    #[turbo_tasks::function]
    async fn children(&self) -> Result<Vc<IntrospectableChildren>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.children()
            } else {
                Vc::cell(Default::default())
            },
        )
    }
}
