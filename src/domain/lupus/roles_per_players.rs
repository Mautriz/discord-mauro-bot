use super::roles::LupusRole;
use rand::prelude::SliceRandom;

pub fn get_roles(player_number: usize) -> Vec<LupusRole> {
    let mut rng = rand::thread_rng();

    let complete_roles = vec![
        LupusRole::GUFO,
        LupusRole::WOLF { is_leader: true },
        LupusRole::VEGGENTE,
        LupusRole::BODYGUARD {
            self_protected: false,
        },
        LupusRole::MEDIUM,
        LupusRole::VILLICO,
        LupusRole::VILLICO,
        LupusRole::STREGA(Box::new(LupusRole::NOTASSIGNED)),
        LupusRole::INDEMONIATO,
        LupusRole::VIGILANTE { has_shot: false },
        LupusRole::DORIANGREY {
            has_quadro: true,
            given_to: None,
        },
        LupusRole::SERIALKILLER,
        LupusRole::SEXWORKER,
        LupusRole::DOTTORE,
        LupusRole::WOLF { is_leader: false },
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
