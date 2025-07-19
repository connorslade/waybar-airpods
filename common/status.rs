use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Default, Hash)]
pub struct Status {
    pub metadata: Option<Metadata>,
    pub components: Components,
    pub ear: InEar,
}

#[derive(Hash)]
pub struct Metadata {
    pub name: String,
    pub model: String,
}

#[derive(Default, Hash)]
pub struct Components {
    pub left: Option<ComponentStatus>,
    pub right: Option<ComponentStatus>,
    pub case: Option<ComponentStatus>,
}

#[derive(Default, Hash)]
pub struct InEar {
    pub left: EarStatus,
    pub right: EarStatus,
}

#[derive(Hash)]
pub struct ComponentStatus {
    pub level: u8,
    pub status: BatteryStatus,
}

#[derive(Hash)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Disconnected,
}

#[derive(Default, Hash, Clone, Copy)]
pub enum EarStatus {
    InEar,
    NotInEar,
    InCase,
    #[default]
    Disconnected,
}

impl Status {
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash(self, &mut hasher);
        hasher.finish()
    }

    pub fn is_valid(&self) -> bool {
        let Components { left, right, case } = &self.components;
        left.is_some() || right.is_some() || case.is_some()
    }

    pub fn min_pods(&self) -> u8 {
        let mut out = u8::MAX;

        let Components { left, right, .. } = &self.components;
        for component in [&left, &right] {
            if let Some(component) = &component
                && matches!(component.status, BatteryStatus::Discharging)
            {
                out = out.min(component.level);
            }
        }

        out
    }
}

impl Components {
    pub fn as_arr_mut(&mut self) -> [&mut Option<ComponentStatus>; 3] {
        [&mut self.left, &mut self.right, &mut self.case]
    }
}

impl EarStatus {
    pub fn icon(&self) -> &'static str {
        match self {
            EarStatus::InEar => "󱡏",
            EarStatus::NotInEar => "󱡒",
            EarStatus::InCase => "󱡑",
            EarStatus::Disconnected => "",
        }
    }
}
