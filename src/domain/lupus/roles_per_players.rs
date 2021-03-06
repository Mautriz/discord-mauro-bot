use super::roles::LupusRole;
use rand::prelude::SliceRandom;

pub fn get_roles(player_number: usize) -> Vec<LupusRole> {
    let mut rng = rand::thread_rng();

    let complete_roles = vec![
        LupusRole::GUFO { is_leader: false },
        LupusRole::VEGGENTE,
        LupusRole::WOLF { is_leader: true },
        LupusRole::BODYGUARD {
            self_protected: false,
        },
        LupusRole::MEDIUM,
        LupusRole::VILLICO,
        LupusRole::VILLICO,
        LupusRole::CRICETO,
        LupusRole::INDEMONIATO,
        LupusRole::VIGILANTE { has_shot: false },
        LupusRole::DORIANGREY {
            has_quadro: true,
            given_to: None,
        },
        LupusRole::SERIALKILLER,
        LupusRole::SEXWORKER,
        LupusRole::DOTTORE { has_healed: false },
        LupusRole::WOLF { is_leader: false },
        LupusRole::STREGA(Box::new(LupusRole::NOTASSIGNED)),
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
