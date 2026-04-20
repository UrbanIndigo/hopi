mod api;
mod cookie;
mod launcher;

use anyhow::{Result, anyhow};
use api::{Api, CreatorType, GroupInfo, Universe};
use clap::{Parser, Subcommand};
use inquire::Select;

#[derive(Parser)]
#[command(name = "hopi", about = "Open Roblox Studio places from the CLI")]
struct Cli {
    /// "me", "shared", or a group-name query. Omit for interactive mode.
    owner: Option<String>,
    /// Place query within the selected owner.
    place: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Dump raw shared-with-me results (debug)
    DebugShared,
}

#[derive(Debug, Clone)]
struct Place {
    place_id: u64,
    universe_id: u64,
    name: String,
    owner: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = build_api()?;
    match cli.command {
        Some(Command::DebugShared) => debug_shared(&api).await,
        None => run(&api, cli.owner.as_deref(), cli.place.as_deref()).await,
    }
}

fn build_api() -> Result<Api> {
    let cookie = cookie::load()?;
    Api::new(&cookie)
}

async fn run(api: &Api, owner_q: Option<&str>, place_q: Option<&str>) -> Result<()> {
    let owner_ql = owner_q.map(|s| s.to_lowercase());
    match owner_ql.as_deref() {
        None => interactive(api).await,
        Some("me" | "self" | "mine") => run_me(api, place_q).await,
        Some("shared") => run_shared(api, place_q).await,
        _ => run_group(api, owner_q, place_q).await,
    }
}

async fn interactive(api: &Api) -> Result<()> {
    let categories = vec![
        "My Experiences".to_string(),
        "Shared with me".to_string(),
        "Group Experiences".to_string(),
    ];
    let idx = run_picker(&categories, None, "› ")?;
    match idx {
        0 => run_me(api, None).await,
        1 => run_shared(api, None).await,
        _ => run_group(api, None, None).await,
    }
}

async fn run_me(api: &Api, place_q: Option<&str>) -> Result<()> {
    let me = api.authenticated_user().await?;
    let unis = api.search_universes(CreatorType::User, me.id).await?;
    let places = to_places(unis, PlaceOwner::Fixed(me.name));
    open_one(&places, place_q)
}

async fn run_shared(api: &Api, place_q: Option<&str>) -> Result<()> {
    let me = api.authenticated_user().await?;
    let unis = api.search_universes(CreatorType::Team, me.id).await?;
    let places = to_places(unis, PlaceOwner::PerCreator);
    open_one(&places, place_q)
}

async fn run_group(api: &Api, group_q: Option<&str>, place_q: Option<&str>) -> Result<()> {
    let groups = api.canmanage_groups().await?;
    let group = resolve_group(&groups, group_q)?;
    let unis = api.search_universes(CreatorType::Group, group.id).await?;
    let places = to_places(unis, PlaceOwner::Fixed(group.name));
    open_one(&places, place_q)
}

enum PlaceOwner {
    Fixed(String),
    PerCreator,
}

fn to_places(unis: Vec<Universe>, owner_src: PlaceOwner) -> Vec<Place> {
    unis.into_iter()
        .filter_map(|u| {
            let owner = match &owner_src {
                PlaceOwner::Fixed(s) => s.clone(),
                PlaceOwner::PerCreator => u.creator_name().unwrap_or_else(|| "Shared".into()),
            };
            let universe_id = u.id;
            u.root_place_id.map(|pid| Place {
                place_id: pid,
                universe_id,
                name: u.name,
                owner,
            })
        })
        .collect()
}

fn resolve_group(groups: &[GroupInfo], query: Option<&str>) -> Result<GroupInfo> {
    if groups.is_empty() {
        return Err(anyhow!("no manageable groups"));
    }
    if let Some(q) = query {
        let ql = q.to_lowercase();
        let matches: Vec<&GroupInfo> = groups
            .iter()
            .filter(|g| g.name.to_lowercase().contains(&ql))
            .collect();
        if matches.len() == 1 {
            return Ok(matches[0].clone());
        }
        if matches.is_empty() {
            return Err(anyhow!("no group matched '{q}'"));
        }
        return pick_group(groups, Some(q));
    }
    pick_group(groups, None)
}

fn pick_group(groups: &[GroupInfo], initial: Option<&str>) -> Result<GroupInfo> {
    let labels: Vec<String> = groups.iter().map(|g| g.name.clone()).collect();
    let idx = run_picker(&labels, initial, "group › ")?;
    Ok(groups[idx].clone())
}

fn open_one(places: &[Place], query: Option<&str>) -> Result<()> {
    if places.is_empty() {
        return Err(anyhow!("no places to open"));
    }
    let picked = if let Some(q) = query {
        let ql = q.to_lowercase();
        let matches: Vec<&Place> = places
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&ql))
            .collect();
        match matches.len() {
            0 => return Err(anyhow!("no place matched '{q}'")),
            1 => matches[0].clone(),
            _ => pick_place(places, Some(q))?,
        }
    } else {
        pick_place(places, None)?
    };
    eprintln!("Opening {} ({})", picked.name, picked.place_id);
    launcher::open_place(picked.place_id, picked.universe_id)
}

fn pick_place(places: &[Place], initial: Option<&str>) -> Result<Place> {
    let labels: Vec<String> = places
        .iter()
        .map(|p| format!("{}  ·  {}  ·  #{}", p.name, p.owner, p.place_id))
        .collect();
    let idx = run_picker(&labels, initial, "place › ")?;
    Ok(places[idx].clone())
}

fn run_picker(labels: &[String], initial: Option<&str>, prompt: &'static str) -> Result<usize> {
    let mut select = Select::new(prompt.trim_end_matches(" › "), labels.to_vec());
    if let Some(s) = initial {
        select = select.with_starting_filter_input(s);
    }
    let result = select.raw_prompt().map_err(|e| anyhow!("picker: {e}"))?;
    Ok(result.index)
}

async fn debug_shared(api: &Api) -> Result<()> {
    let me = api.authenticated_user().await?;
    let unis = api.search_universes(CreatorType::Team, me.id).await?;
    for u in unis {
        println!(
            "{:<16} root={:<16} creator={:?}",
            u.id,
            u.root_place_id.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
            u.creator_name()
        );
    }
    Ok(())
}
