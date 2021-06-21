use super::roles::LupusRole;
use rand::prelude::SliceRandom;

pub fn get_roles(player_number: usize) -> Vec<LupusRole> {
    let mut rng = rand::thread_rng();

    let complete_roles = vec![
        LupusRole::GUFO,
        LupusRole::WOLF,
        LupusRole::VEGGENTE,
        LupusRole::BODYGUARD {
            self_protected: false,
        },
        LupusRole::MEDIUM,
        LupusRole::VILLICO,
        LupusRole::VILLICO,
        LupusRole::STREGA,
        LupusRole::INDEMONIATO,
        LupusRole::VIGILANTE,
        LupusRole::DORIANGREY,
        LupusRole::SERIALKILLER,
        LupusRole::PUTTANA,
        LupusRole::DOTTORE,
        LupusRole::WOLF,
        LupusRole::CRICETO,
    ];

    let mut filtered_roles = complete_roles
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i < &player_number)
        .map(|(_, role)| role)
        .collect::<Vec<LupusRole>>();

    filtered_roles.shuffle(&mut rng);

    filtered_roles
}
