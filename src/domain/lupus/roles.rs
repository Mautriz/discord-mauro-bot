use serenity::model::id::UserId;

// LUPI: durante la notte uccidono una persona; sono visti cattivi dal veggente
// VEGGENTE: durante la notte vede il ruolo di una persona da lui scelta; il master dirà lui se è buono o cattivo
// CRICETO: viene visto buono; il suo scopo è quello di farsi impiccare durante il giorno in modo tale da vincere il gioco (se si è in tanti, ammazza due di quelli che l'hanno votato); non muore di notte se non dal vigilante
// BODYGUARD: durante la notte può proteggere una persona; può inoltre proteggere se stesso ma una volta a game; viene visto buono dal veggente
// VIGILANTE: ha la possibilità di uccidere durante la notte una persona a game; viene visto buono dal veggente
// MEDIUM: viene a conoscenza durante la notte dal master del ruolo della persona impiccata la giornata prima; visto buono dal veggente
// GUFO: gioca con i lupi; durante la notte decide una persona che se vista dal veggente, verrà vista cattiva; una volta morti i lupi, uccide anche lui; viene visto cattivo dal veggente
// MASSONI: villici che sanno il proprio reciproco ruolo; visti buoni dal veggente
// DORIAN GREY: visto buono; ogni notte dà il quadro ad una persona; nel momento in cui il Dorian grey muore, muore al suo posto la persona in possesso del quadro; il quadro si annulla ogni notte; una volta perso il quadro, non lo ha più
// VILLICO MANNARO: visto cattivo; nel momento in cui muore di notte (non dal vigilante o Dorian) diventa a sua volta un lupo
// PUTTANA: vista buona; di notte va a letto con una persona  bloccando il ruolo della persona in questione
// SERIAL KILLER: visto cattivo; ogni notte può uccidere una persona, vince da solo; non muore di notte
// DOTTORE: visto buono; una volta a game può resuscitare una persona
// INDEMONIATO: villico visto buono; vince però se vincono i lupi;
// STREGA: vista in base al ruolo che prende; gioca da sola, di notte ruba il ruolo di una persona da lei scelta; blocca tutti tranne il veggente
// ANGELO: visto buono; il master da lui un nome; lui vince se la persona che gli è stata affidata vince; è neutrale; se la persona che gli è stata affidata muore, il suo obiettivo sarà quello di rimane in vita fino alla fine
// AMNESIA: inizialmente visto buono; targetta uno morto, UNA VOLTA a partita decide un morto di cui prenderà il ruolo; se il ruolo della persona scelta è buono, rimane buono, se il ruolo è cattivo, diventa cattivo
// VILLICO: visto buono, non fa niente
#[derive(Clone)]
pub enum LupusRole {
    VEGGENTE,
    CRICETO,
    BODYGUARD,
    VIGILANTE,
    MEDIUM,
    GUFO,
    DORIAN_GREY,
    VILLICO_MANNARO,
    PUTTANA,
    SERIAL_KILLER,
    DOTTORE,
    INDEMONIATO,
    STREGA,
    ANGELO,
    AMNESIA,
    VILLICO,
    WOLF,
    NOT_ASSIGNED,
}

pub enum Nature {
    GOOD,
    EVIL,
    UNKNOWN,
}

impl LupusRole {
    fn is_evil(self) -> Nature {
        match self {
            Self::VEGGENTE
            | Self::ANGELO
            | Self::CRICETO
            | Self::BODYGUARD
            | Self::VIGILANTE
            | Self::MEDIUM
            | Self::DORIAN_GREY
            | Self::VILLICO
            | Self::PUTTANA
            | Self::DOTTORE
            | Self::INDEMONIATO => Nature::GOOD,
            Self::WOLF | Self::SERIAL_KILLER | Self::VILLICO_MANNARO | Self::GUFO => Nature::EVIL,
            _ => Nature::UNKNOWN,
        }
    }
}

impl Default for LupusRole {
    fn default() -> Self {
        LupusRole::NOT_ASSIGNED
    }
}

#[derive(Clone)]
pub enum LupusCommand {
    RoleBlock { user_id: UserId },
    Frame { user_id: UserId },
    GiveQuadro { user_id: UserId },
    Kill { user_id: UserId },
    // WolfVote {  }
    TrueSight { user_id: UserId },
    Heal { user_id: UserId },
    Remember { user_id: UserId },
    Possess { user_id: UserId },
}
