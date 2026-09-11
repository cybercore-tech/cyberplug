use crate::config;
use crate::discovery;
use crate::models::Plugin;
use crate::omarchy;
use crate::settings;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub const PLACEMENTS: [&str; 3] = ["left", "center", "right"];

pub struct PluginEntry {
    pub plugin: Plugin,
    pub enabled: bool,
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Filter,
    AddUrl,
    Settings,
    ConfirmRemove,
    Placement,
    Discovery,
    Profile,
    ProfilePath,
}

/// Blocking work (network I/O or a shell-out) deferred by one redraw so the
/// UI always paints the "loading…" state the caller set *before* the work
/// runs, instead of appearing frozen for however long the call takes. See
/// `App::perform` and the main loop in `main.rs`.
pub enum Pending {
    LoadDiscovery(bool),
    LoadDescription,
    InstallDiscoverySelected,
}

pub struct App {
    pub plugins: Vec<PluginEntry>,
    pub filtered: Vec<usize>,
    pub selected: usize,
    pub should_quit: bool,
    pub mode: Mode,
    pub filter_text: String,
    pub input_buffer: String,
    pub status: Option<String>,
    pub settings_selected: usize,
    pub settings_editing: bool,
    pub settings_edit_buffer: String,
    pub local_settings: settings::LocalSettings,
    pub placement_selected: usize,
    pub discovery_all: Vec<discovery::Source>,
    pub discovery_sources: Vec<discovery::Source>,
    pub discovery_selected: usize,
    pub discovery_categories: Vec<String>,
    pub discovery_category_index: usize,
    pub discovery_desc_cache: HashMap<String, Option<String>>,
    pub profile_exporting: bool,
    pub pending: Option<Pending>,
}

impl App {
    pub fn new() -> Result<Self> {
        let plugins = load_plugins()?;
        let filtered = (0..plugins.len()).collect();
        let local_settings = settings::LocalSettings::load().unwrap_or_default();
        Ok(Self {
            plugins,
            filtered,
            selected: 0,
            should_quit: false,
            mode: Mode::Normal,
            filter_text: String::new(),
            input_buffer: String::new(),
            status: None,
            settings_selected: 0,
            settings_editing: false,
            settings_edit_buffer: String::new(),
            local_settings,
            placement_selected: 0,
            discovery_all: vec![],
            discovery_sources: vec![],
            discovery_selected: 0,
            discovery_categories: vec![],
            discovery_category_index: 0,
            discovery_desc_cache: HashMap::new(),
            profile_exporting: true,
            pending: None,
        })
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.plugins = load_plugins()?;
        self.apply_filter();
        Ok(())
    }

    pub fn apply_filter(&mut self) {
        let needle = self.filter_text.to_lowercase();
        self.filtered = self
            .plugins
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                needle.is_empty()
                    || e.plugin.name.to_lowercase().contains(&needle)
                    || e.plugin.id.to_lowercase().contains(&needle)
                    || e.plugin.description.to_lowercase().contains(&needle)
            })
            .map(|(i, _)| i)
            .collect();
        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }

    pub fn next(&mut self) {
        if !self.filtered.is_empty() {
            self.selected = (self.selected + 1) % self.filtered.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered.is_empty() {
            self.selected = if self.selected == 0 {
                self.filtered.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn selected_entry(&self) -> Option<&PluginEntry> {
        self.filtered
            .get(self.selected)
            .and_then(|&i| self.plugins.get(i))
    }

    pub fn selected_id(&self) -> Option<String> {
        self.selected_entry().map(|e| e.plugin.id.clone())
    }

    /// Runs one piece of deferred blocking work. Called from the main loop
    /// right after it repaints the "loading…" state the key handler set, so
    /// the UI never appears to freeze or drop something (e.g. the Discovery
    /// tabs) — see the module doc on `Pending`.
    pub fn perform(&mut self, work: Pending) -> Result<()> {
        match work {
            Pending::LoadDiscovery(force_refresh) => {
                self.load_discovery(force_refresh)?;
                self.status = Some(format!(
                    "{} plugins available",
                    self.discovery_sources.len()
                ));
            }
            Pending::LoadDescription => self.fetch_discovery_description(),
            Pending::InstallDiscoverySelected => self.install_discovery_selected()?,
        }
        Ok(())
    }

    fn load_discovery(&mut self, force_refresh: bool) -> Result<()> {
        let installed: HashSet<&str> = self.plugins.iter().map(|e| e.plugin.id.as_str()).collect();
        let all = discovery::fetch(force_refresh)?;
        self.discovery_all = all
            .into_iter()
            .filter(|s| !installed.contains(s.id.as_str()))
            .collect();

        let mut cats: Vec<String> = self
            .discovery_all
            .iter()
            .filter_map(|s| s.category.clone())
            .collect();
        cats.sort();
        cats.dedup();
        self.discovery_categories = cats;
        self.discovery_category_index = 0;

        self.apply_discovery_filter();
        Ok(())
    }

    pub fn apply_discovery_filter(&mut self) {
        if self.discovery_category_index == 0 {
            self.discovery_sources = self.discovery_all.clone();
        } else if let Some(cat) = self
            .discovery_categories
            .get(self.discovery_category_index - 1)
        {
            self.discovery_sources = self
                .discovery_all
                .iter()
                .filter(|s| s.category.as_deref() == Some(cat.as_str()))
                .cloned()
                .collect();
        }
        self.discovery_selected = 0;
        self.queue_description_if_needed();
    }

    pub fn discovery_cycle_category(&mut self) {
        let total = self.discovery_categories.len() + 1;
        self.discovery_category_index = (self.discovery_category_index + 1) % total;
        self.apply_discovery_filter();
    }

    pub fn discovery_next(&mut self) {
        if !self.discovery_sources.is_empty() {
            self.discovery_selected = (self.discovery_selected + 1) % self.discovery_sources.len();
        }
        self.queue_description_if_needed();
    }

    pub fn discovery_previous(&mut self) {
        if !self.discovery_sources.is_empty() {
            self.discovery_selected = if self.discovery_selected == 0 {
                self.discovery_sources.len() - 1
            } else {
                self.discovery_selected - 1
            };
        }
        self.queue_description_if_needed();
    }

    /// True if the currently-selected discovery entry has no description yet
    /// and fetching one would need a network call we haven't already made.
    fn discovery_needs_description(&self) -> bool {
        let Some(source) = self.discovery_sources.get(self.discovery_selected) else {
            return false;
        };
        source.description.is_none() && !self.discovery_desc_cache.contains_key(&source.repo)
    }

    /// Queues one deferred network call for the on-screen entry's
    /// description, only when it's actually missing — never pre-fetches the
    /// whole list, and never blocks the caller.
    fn queue_description_if_needed(&mut self) {
        if self.discovery_needs_description() {
            self.pending = Some(Pending::LoadDescription);
        }
    }

    /// The actual (blocking) description fetch. Only reached via
    /// `perform(Pending::LoadDescription)`, one redraw after the caller
    /// asked for it.
    fn fetch_discovery_description(&mut self) {
        let Some(source) = self.discovery_sources.get(self.discovery_selected).cloned() else {
            return;
        };
        if source.description.is_some() || self.discovery_desc_cache.contains_key(&source.repo) {
            return;
        }
        let desc = discovery::fetch_repo_description(&source.repo);
        self.discovery_desc_cache.insert(source.repo.clone(), desc);
    }

    /// Installs the selected Discover entry via `omarchy plugin add` and, if
    /// it registered as a bar widget under the shown id, routes into
    /// placement. Only reached via `perform(Pending::InstallDiscoverySelected)`.
    fn install_discovery_selected(&mut self) -> Result<()> {
        let Some(source) = self.discovery_sources.get(self.discovery_selected).cloned() else {
            return Ok(());
        };
        match omarchy::add(&source.repo, false) {
            Ok(_) => {
                self.refresh()?;
                // Only route into placement if the plugin actually registered
                // as a bar widget under this id — a repo can install without
                // exposing that exact id (e.g. suite repos where the id we
                // showed doesn't map 1:1 to what got installed).
                if self.plugins.iter().any(|e| e.plugin.id == source.id) {
                    self.selected = self
                        .filtered
                        .iter()
                        .position(|&i| self.plugins[i].plugin.id == source.id)
                        .unwrap_or(0);
                    self.mode = Mode::Placement;
                    self.placement_selected = 1;
                    self.status = Some(format!("{} installed — choose bar placement", source.name));
                } else {
                    self.status = Some(format!(
                        "{} installed but not found as '{}' — check `omarchy plugin list` and enable manually",
                        source.name, source.id
                    ));
                }
                self.pending = Some(Pending::LoadDiscovery(false));
            }
            Err(e) => {
                self.status = Some(format!("error: {}", e));
            }
        }
        Ok(())
    }

    pub fn discovery_description(&self) -> Option<String> {
        let source = self.discovery_sources.get(self.discovery_selected)?;
        source.description.clone().or_else(|| {
            self.discovery_desc_cache
                .get(&source.repo)
                .cloned()
                .flatten()
        })
    }
}

fn load_plugins() -> Result<Vec<PluginEntry>> {
    let catalog = omarchy::catalog()?;
    let disabled = config::disabled_plugins().unwrap_or_default();

    Ok(catalog
        .into_iter()
        .map(|plugin| {
            let enabled = !disabled.contains(&plugin.id);
            PluginEntry { plugin, enabled }
        })
        .collect())
}
