use valence::item::ItemKind;

pub fn is_spawn_egg(item: &ItemKind) -> bool {
    match item {
        ItemKind::AllaySpawnEgg => true,
        ItemKind::AxolotlSpawnEgg => true,
        ItemKind::BatSpawnEgg => true,
        ItemKind::BeeSpawnEgg => true,
        ItemKind::BlazeSpawnEgg => true,
        ItemKind::CatSpawnEgg => true,
        ItemKind::CamelSpawnEgg => true,
        ItemKind::CaveSpiderSpawnEgg => true,
        ItemKind::ChickenSpawnEgg => true,
        ItemKind::CodSpawnEgg => true,
        ItemKind::CowSpawnEgg => true,
        ItemKind::CreeperSpawnEgg => true,
        ItemKind::DolphinSpawnEgg => true,
        ItemKind::DonkeySpawnEgg => true,
        ItemKind::DrownedSpawnEgg => true,
        ItemKind::ElderGuardianSpawnEgg => true,
        ItemKind::EnderDragonSpawnEgg => true,
        ItemKind::EndermanSpawnEgg => true,
        ItemKind::EndermiteSpawnEgg => true,
        ItemKind::EvokerSpawnEgg => true,
        ItemKind::FoxSpawnEgg => true,
        ItemKind::FrogSpawnEgg => true,
        ItemKind::GhastSpawnEgg => true,
        ItemKind::GlowSquidSpawnEgg => true,
        ItemKind::GoatSpawnEgg => true,
        ItemKind::GuardianSpawnEgg => true,
        ItemKind::HoglinSpawnEgg => true,
        ItemKind::HorseSpawnEgg => true,
        ItemKind::HuskSpawnEgg => true,
        ItemKind::IronGolemSpawnEgg => true,
        ItemKind::LlamaSpawnEgg => true,
        ItemKind::MagmaCubeSpawnEgg => true,
        ItemKind::MooshroomSpawnEgg => true,
        ItemKind::MuleSpawnEgg => true,
        ItemKind::OcelotSpawnEgg => true,
        ItemKind::PandaSpawnEgg => true,
        ItemKind::ParrotSpawnEgg => true,
        ItemKind::PhantomSpawnEgg => true,
        ItemKind::PigSpawnEgg => true,
        ItemKind::PiglinSpawnEgg => true,
        ItemKind::PiglinBruteSpawnEgg => true,
        ItemKind::PillagerSpawnEgg => true,
        ItemKind::PolarBearSpawnEgg => true,
        ItemKind::PufferfishSpawnEgg => true,
        ItemKind::RabbitSpawnEgg => true,
        ItemKind::RavagerSpawnEgg => true,
        ItemKind::SalmonSpawnEgg => true,
        ItemKind::SheepSpawnEgg => true,
        ItemKind::ShulkerSpawnEgg => true,
        ItemKind::SilverfishSpawnEgg => true,
        ItemKind::SkeletonSpawnEgg => true,
        ItemKind::SkeletonHorseSpawnEgg => true,
        ItemKind::SlimeSpawnEgg => true,
        ItemKind::SnifferSpawnEgg => true,
        ItemKind::SnowGolemSpawnEgg => true,
        ItemKind::SpiderSpawnEgg => true,
        ItemKind::SquidSpawnEgg => true,
        ItemKind::StraySpawnEgg => true,
        ItemKind::StriderSpawnEgg => true,
        ItemKind::TadpoleSpawnEgg => true,
        ItemKind::TraderLlamaSpawnEgg => true,
        ItemKind::TropicalFishSpawnEgg => true,
        ItemKind::TurtleSpawnEgg => true,
        ItemKind::VexSpawnEgg => true,
        ItemKind::VillagerSpawnEgg => true,
        ItemKind::VindicatorSpawnEgg => true,
        ItemKind::WanderingTraderSpawnEgg => true,
        ItemKind::WardenSpawnEgg => true,
        ItemKind::WitchSpawnEgg => true,
        ItemKind::WitherSpawnEgg => true,
        ItemKind::WitherSkeletonSpawnEgg => true,
        ItemKind::WolfSpawnEgg => true,
        ItemKind::ZoglinSpawnEgg => true,
        ItemKind::ZombieSpawnEgg => true,
        ItemKind::ZombieHorseSpawnEgg => true,
        ItemKind::ZombieVillagerSpawnEgg => true,
        ItemKind::ZombifiedPiglinSpawnEgg => true,
        _ => false,
    }
}