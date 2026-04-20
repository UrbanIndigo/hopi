use anyhow::{Context, Result, anyhow};
use reqwest::{Client, header};
use serde::Deserialize;

pub struct Api {
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct AuthUser {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct Page<T> {
    #[serde(rename = "nextPageCursor")]
    next_page_cursor: Option<String>,
    data: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupInfo {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct CanManageResponse {
    data: Vec<GroupInfo>,
}

/// A universe as returned by the Creator-Hub-backed search API.
#[derive(Debug, Clone, Deserialize)]
pub struct Universe {
    pub id: u64,
    pub name: String,
    #[serde(rename = "rootPlaceId")]
    pub root_place_id: Option<u64>,
    pub creator: Option<Creator>,
    #[serde(rename = "creatorName")]
    pub creator_name_flat: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Creator {
    pub name: Option<String>,
}

impl Universe {
    pub fn creator_name(&self) -> Option<String> {
        self.creator
            .as_ref()
            .and_then(|c| c.name.clone())
            .or_else(|| self.creator_name_flat.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CreatorType {
    User,
    Group,
    Team,
}

impl CreatorType {
    fn as_str(self) -> &'static str {
        match self {
            CreatorType::User => "User",
            CreatorType::Group => "Group",
            CreatorType::Team => "Team",
        }
    }
}

impl Api {
    pub fn new(cookie: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!(".ROBLOSECURITY={cookie}"))
                .context("cookie contains invalid header bytes")?,
        );
        let client = Client::builder()
            .default_headers(headers)
            .user_agent("hopi/0.1")
            .build()?;
        Ok(Self { client })
    }

    pub async fn authenticated_user(&self) -> Result<AuthUser> {
        let resp = self
            .client
            .get("https://users.roblox.com/v1/users/authenticated")
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(anyhow!(
                "Roblox rejected the cookie (401). Re-login to Studio and try again."
            ));
        }
        Ok(resp.error_for_status()?.json().await?)
    }

    /// Groups the authenticated user can manage (edit access).
    pub async fn canmanage_groups(&self) -> Result<Vec<GroupInfo>> {
        let resp: CanManageResponse = self
            .client
            .get("https://develop.roblox.com/v1/user/groups/canmanage")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.data)
    }

    /// Unified Creator-Hub search:
    ///   - CreatorType=User, target=userId → the user's own universes
    ///   - CreatorType=Group, target=groupId → a group's universes
    ///   - CreatorType=Team, target=userId → Team Create "Shared with me"
    pub async fn search_universes(
        &self,
        creator_type: CreatorType,
        target_id: u64,
    ) -> Result<Vec<Universe>> {
        let base = format!(
            "https://apis.roblox.com/universes/v1/search\
             ?CreatorType={}&CreatorTargetId={}\
             &IsArchived=false&Surface=CreatorHubCreations\
             &PageSize=50&SortParam=LastUpdated&SortOrder=Desc",
            creator_type.as_str(),
            target_id,
        );
        let mut out = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let url = match &cursor {
                Some(c) => format!("{base}&Cursor={c}"),
                None => base.clone(),
            };
            let page: Page<Universe> =
                self.client.get(&url).send().await?.error_for_status()?.json().await?;
            out.extend(page.data);
            match page.next_page_cursor {
                Some(c) if !c.is_empty() => cursor = Some(c),
                _ => break,
            }
        }
        Ok(out)
    }
}
