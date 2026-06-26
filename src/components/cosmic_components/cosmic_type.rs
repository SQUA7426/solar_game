#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum CosmicType {
    // Stars
    NeuronStar,
    DwarfStar,
    MidStar,
    LargeStar,
    // Planets
    DwarfPlanet,
    Planet,
    HabitablePlanet,
    GasPlanet,
    // systems
    LowSolarSystem,
    MidSolarSystem,
    HighSolarsystem,
    // other
    Nebula,
    Galaxy,
}
