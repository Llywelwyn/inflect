//! Word lists and patterns ported from Python inflect 7.5.0.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub(crate) const PL_SB_IRREGULAR_S: &[(&str, &str)] = &[
    ("corpus", "corpuses|corpora"),
    ("opus", "opuses|opera"),
    ("genus", "genera"),
    ("mythos", "mythoi"),
    ("penis", "penises|penes"),
    ("testis", "testes"),
    ("atlas", "atlases|atlantes"),
    ("yes", "yeses"),
];

pub(crate) const PL_SB_IRREGULAR_BASE: &[(&str, &str)] = &[
    ("child", "children"),
    ("chili", "chilis|chilies"),
    ("brother", "brothers|brethren"),
    ("infinity", "infinities|infinity"),
    ("loaf", "loaves"),
    ("lore", "lores|lore"),
    ("hoof", "hoofs|hooves"),
    ("beef", "beefs|beeves"),
    ("thief", "thiefs|thieves"),
    ("money", "monies"),
    ("mongoose", "mongooses"),
    ("ox", "oxen"),
    ("cow", "cows|kine"),
    ("graffito", "graffiti"),
    ("octopus", "octopuses|octopodes"),
    ("genie", "genies|genii"),
    ("ganglion", "ganglions|ganglia"),
    ("trilby", "trilbys"),
    ("turf", "turfs|turves"),
    ("numen", "numina"),
    ("atman", "atmas"),
    ("occiput", "occiputs|occipita"),
    ("sabretooth", "sabretooths"),
    ("sabertooth", "sabertooths"),
    ("lowlife", "lowlifes"),
    ("flatfoot", "flatfoots"),
    ("tenderfoot", "tenderfoots"),
    ("romany", "romanies"),
    ("jerry", "jerries"),
    ("mary", "maries"),
    ("talouse", "talouses"),
    ("rom", "roma"),
    ("carmen", "carmina"),
];

pub(crate) const PL_SB_IRREGULAR_CAPS: &[(&str, &str)] = &[
    ("Romany", "Romanies"),
    ("Jerry", "Jerrys"),
    ("Mary", "Marys"),
    ("Rom", "Roma"),
];

pub(crate) const PL_SB_IRREGULAR_COMPOUND: &[(&str, &str)] =
    &[("prima donna", "prima donnas|prime donne")];

pub(crate) const PL_SB_Z_ZES_LIST: &[&str] = &["quartz", "topaz"];

pub(crate) const PL_SB_ZE_ZES_LIST: &[&str] = &["snooze"];

pub(crate) const PL_SB_C_IS_IDES_COMPLETE: &[&str] =
    &["ephemeris", "iris", "clitoris", "chrysalis", "epididymis"];

pub(crate) const PL_SB_C_IS_IDES_ENDINGS: &[&str] = &["itis"];

pub(crate) const PL_SB_C_A_ATA_LIST: &[&str] = &[
    "anathema",
    "bema",
    "carcinoma",
    "charisma",
    "diploma",
    "dogma",
    "drama",
    "edema",
    "enema",
    "enigma",
    "lemma",
    "lymphoma",
    "magma",
    "melisma",
    "miasma",
    "oedema",
    "sarcoma",
    "schema",
    "soma",
    "stigma",
    "stoma",
    "trauma",
    "gumma",
    "pragma",
];

pub(crate) const PL_SB_U_A_AE_LIST: &[&str] = &["alumna", "alga", "vertebra", "persona", "vita"];

pub(crate) const PL_SB_C_A_AE_LIST: &[&str] = &[
    "amoeba",
    "antenna",
    "formula",
    "hyperbola",
    "medusa",
    "nebula",
    "parabola",
    "abscissa",
    "hydra",
    "nova",
    "lacuna",
    "aurora",
    "umbra",
    "flora",
    "fauna",
];

pub(crate) const PL_SB_C_EN_INA_LIST: &[&str] = &["stamen", "foramen", "lumen"];

pub(crate) const PL_SB_U_UM_A_LIST: &[&str] = &[
    "bacterium",
    "agendum",
    "desideratum",
    "erratum",
    "stratum",
    "datum",
    "ovum",
    "extremum",
    "candelabrum",
];

pub(crate) const PL_SB_C_UM_A_LIST: &[&str] = &[
    "maximum",
    "minimum",
    "momentum",
    "optimum",
    "quantum",
    "cranium",
    "curriculum",
    "dictum",
    "phylum",
    "aquarium",
    "compendium",
    "emporium",
    "encomium",
    "gymnasium",
    "honorarium",
    "interregnum",
    "lustrum",
    "memorandum",
    "millennium",
    "rostrum",
    "spectrum",
    "speculum",
    "stadium",
    "trapezium",
    "ultimatum",
    "medium",
    "vacuum",
    "velum",
    "consortium",
    "arboretum",
];

pub(crate) const PL_SB_U_US_I_LIST: &[&str] = &[
    "alumnus",
    "alveolus",
    "bacillus",
    "bronchus",
    "locus",
    "nucleus",
    "stimulus",
    "meniscus",
    "sarcophagus",
];

pub(crate) const PL_SB_C_US_I_LIST: &[&str] = &[
    "focus",
    "radius",
    "genius",
    "incubus",
    "succubus",
    "nimbus",
    "fungus",
    "nucleolus",
    "stylus",
    "torus",
    "umbilicus",
    "uterus",
    "hippopotamus",
    "cactus",
];

pub(crate) const PL_SB_C_US_US: &[&str] = &[
    "status",
    "apparatus",
    "prospectus",
    "sinus",
    "hiatus",
    "impetus",
    "plexus",
];

pub(crate) const PL_SB_U_ON_A_LIST: &[&str] = &[
    "criterion",
    "perihelion",
    "aphelion",
    "phenomenon",
    "prolegomenon",
    "noumenon",
    "organon",
    "asyndeton",
    "hyperbaton",
];

pub(crate) const PL_SB_C_ON_A_LIST: &[&str] = &["oxymoron"];

pub(crate) const PL_SB_C_O_I: &[&str] = &[
    "solo",
    "soprano",
    "basso",
    "alto",
    "contralto",
    "tempo",
    "piano",
    "virtuoso",
];

pub(crate) const PL_SB_U_O_OS_COMPLETE: &[&str] = &["ISO", "NATO", "NCO", "NGO", "ado", "oto"];

pub(crate) const PL_SB_U_O_OS_ENDINGS_BASE: &[&str] = &[
    "aficionado",
    "aggro",
    "albino",
    "allegro",
    "ammo",
    "Antananarivo",
    "archipelago",
    "armadillo",
    "auto",
    "avocado",
    "Bamako",
    "Barquisimeto",
    "bimbo",
    "bingo",
    "Biro",
    "bolero",
    "Bolzano",
    "bongo",
    "Boto",
    "burro",
    "Cairo",
    "canto",
    "cappuccino",
    "casino",
    "cello",
    "Chicago",
    "Chimango",
    "cilantro",
    "cochito",
    "coco",
    "Colombo",
    "Colorado",
    "commando",
    "concertino",
    "contango",
    "credo",
    "crescendo",
    "cyano",
    "demo",
    "ditto",
    "Draco",
    "dynamo",
    "embryo",
    "Esperanto",
    "espresso",
    "euro",
    "falsetto",
    "Faro",
    "fiasco",
    "Filipino",
    "flamenco",
    "furioso",
    "generalissimo",
    "Gestapo",
    "ghetto",
    "gigolo",
    "gizmo",
    "Greensboro",
    "gringo",
    "Guaiabero",
    "guano",
    "gumbo",
    "gyro",
    "hairdo",
    "hippo",
    "Idaho",
    "impetigo",
    "inferno",
    "info",
    "intermezzo",
    "intertrigo",
    "Iquico",
    "jumbo",
    "junto",
    "Kakapo",
    "kilo",
    "Kinkimavo",
    "Kokako",
    "Kosovo",
    "Lesotho",
    "libero",
    "libido",
    "libretto",
    "lido",
    "Lilo",
    "limbo",
    "limo",
    "lineno",
    "lingo",
    "lino",
    "livedo",
    "loco",
    "logo",
    "lumbago",
    "macho",
    "macro",
    "mafioso",
    "magneto",
    "magnifico",
    "Majuro",
    "Malabo",
    "manifesto",
    "Maputo",
    "Maracaibo",
    "medico",
    "memo",
    "metro",
    "Mexico",
    "micro",
    "Milano",
    "Monaco",
    "mono",
    "Montenegro",
    "Morocco",
    "Muqdisho",
    "myo",
    "neutrino",
    "Ningbo",
    "octavo",
    "oregano",
    "Orinoco",
    "Orlando",
    "Oslo",
    "panto",
    "Paramaribo",
    "Pardusco",
    "pedalo",
    "photo",
    "pimento",
    "pinto",
    "pleco",
    "Pluto",
    "pogo",
    "polo",
    "poncho",
    "Porto-Novo",
    "Porto",
    "pro",
    "psycho",
    "pueblo",
    "quarto",
    "Quito",
    "repo",
    "rhino",
    "risotto",
    "rococo",
    "rondo",
    "Sacramento",
    "saddo",
    "sago",
    "salvo",
    "Santiago",
    "Sapporo",
    "Sarajevo",
    "scherzando",
    "scherzo",
    "silo",
    "sirocco",
    "sombrero",
    "staccato",
    "sterno",
    "stucco",
    "stylo",
    "sumo",
    "Taiko",
    "techno",
    "terrazzo",
    "testudo",
    "timpano",
    "tiro",
    "tobacco",
    "Togo",
    "Tokyo",
    "torero",
    "Torino",
    "Toronto",
    "torso",
    "tremolo",
    "typo",
    "tyro",
    "ufo",
    "UNESCO",
    "vaquero",
    "vermicello",
    "verso",
    "vibrato",
    "violoncello",
    "Virgo",
    "weirdo",
    "WHO",
    "WTO",
    "Yamoussoukro",
    "yo-yo",
    "zero",
    "Zibo",
];

pub(crate) const PL_SB_U_CH_CHS_LIST: &[&str] = &["czech", "eunuch", "stomach"];

pub(crate) const PL_SB_U_EX_ICES_LIST: &[&str] = &["codex", "murex", "silex"];

pub(crate) const PL_SB_U_IX_ICES_LIST: &[&str] = &["radix", "helix"];

pub(crate) const PL_SB_C_EX_ICES_LIST: &[&str] = &[
    "vortex", "vertex", "cortex", "latex", "pontifex", "apex", "index", "simplex",
];

pub(crate) const PL_SB_C_IX_ICES_LIST: &[&str] = &["appendix"];

pub(crate) const PL_SB_C_I_LIST: &[&str] = &["afrit", "afreet", "efreet"];

pub(crate) const PL_SB_C_IM_LIST: &[&str] = &["goy", "seraph", "cherub"];

pub(crate) const PL_SB_U_MAN_MANS_LIST: &[&str] = &[
    "ataman", "caiman", "cayman", "ceriman", "desman", "dolman", "farman", "harman", "hetman",
    "human", "leman", "ottoman", "shaman", "talisman",
];

pub(crate) const PL_SB_U_MAN_MANS_CAPS_LIST: &[&str] = &[
    "Alabaman",
    "Bahaman",
    "Burman",
    "German",
    "Hiroshiman",
    "Liman",
    "Nakayaman",
    "Norman",
    "Oklahoman",
    "Panaman",
    "Roman",
    "Selman",
    "Sonaman",
    "Tacoman",
    "Yakiman",
    "Yokohaman",
    "Yuman",
];

pub(crate) const PL_SB_U_LOUSE_LICE_LIST: &[&str] =
    &["booklouse", "grapelouse", "louse", "woodlouse"];

pub(crate) const PL_SB_UNINFLECTED_S_COMPLETE: &[&str] = &[
    "breeches",
    "britches",
    "pajamas",
    "pyjamas",
    "clippers",
    "gallows",
    "hijinks",
    "headquarters",
    "pliers",
    "scissors",
    "testes",
    "herpes",
    "pincers",
    "shears",
    "proceedings",
    "trousers",
    "cantus",
    "coitus",
    "nexus",
    "contretemps",
    "corps",
    "debris",
    "siemens",
    "mumps",
    "diabetes",
    "jackanapes",
    "series",
    "species",
    "subspecies",
    "rabies",
    "chassis",
    "innings",
    "news",
    "mews",
    "haggis",
];

pub(crate) const PL_SB_UNINFLECTED_S_ENDINGS: &[&str] = &["ois", "measles"];

pub(crate) const PL_SB_UNINFLECTED_HERD: &[&str] = &[
    "wildebeest",
    "swine",
    "eland",
    "bison",
    "buffalo",
    "cattle",
    "elk",
    "rhinoceros",
    "zucchini",
    "caribou",
    "dace",
    "grouse",
    "guinea fowl",
    "guinea-fowl",
    "haddock",
    "hake",
    "halibut",
    "herring",
    "mackerel",
    "pickerel",
    "pike",
    "roe",
    "seed",
    "shad",
    "snipe",
    "teal",
    "turbot",
    "water fowl",
    "water-fowl",
];

pub(crate) const PL_SB_UNINFLECTED_COMPLETE_BASE: &[&str] = &[
    "tuna",
    "salmon",
    "mackerel",
    "trout",
    "bream",
    "sea-bass",
    "sea bass",
    "carp",
    "cod",
    "flounder",
    "whiting",
    "moose",
    "graffiti",
    "djinn",
    "samuri",
    "offspring",
    "pence",
    "quid",
    "hertz",
];

pub(crate) const PL_SB_UNINFLECTED_CAPS: &[&str] = &[
    "Portuguese",
    "Amoyese",
    "Borghese",
    "Congoese",
    "Faroese",
    "Foochowese",
    "Genevese",
    "Genoese",
    "Gilbertese",
    "Hottentotese",
    "Kiplingese",
    "Kongoese",
    "Lucchese",
    "Maltese",
    "Nankingese",
    "Niasese",
    "Pekingese",
    "Piedmontese",
    "Pistoiese",
    "Sarawakese",
    "Shavese",
    "Vermontese",
    "Wenchowese",
    "Yengeese",
];

pub(crate) const PL_SB_UNINFLECTED_ENDINGS_BASE: &[&str] = &[
    "butter",
    "cash",
    "furniture",
    "information",
    "fish",
    "deer",
    "sheep",
    "nese",
    "rese",
    "lese",
    "mese",
    "pox",
    "craft",
];

pub(crate) const PL_SB_SINGULAR_S_COMPLETE_BASE: &[&str] = &[
    "acropolis",
    "aegis",
    "alias",
    "asbestos",
    "bathos",
    "bias",
    "bronchitis",
    "bursitis",
    "caddis",
    "cannabis",
    "canvas",
    "chaos",
    "cosmos",
    "dais",
    "digitalis",
    "epidermis",
    "ethos",
    "eyas",
    "gas",
    "glottis",
    "hubris",
    "ibis",
    "lens",
    "mantis",
    "marquis",
    "metropolis",
    "pathos",
    "pelvis",
    "polis",
    "rhinoceros",
    "sassafras",
    "trellis",
];

pub(crate) const SI_SB_OIS_OI_CASE: &[&str] = &["Bolshois", "Hanois"];

pub(crate) const SI_SB_USES_USE_CASE: &[&str] =
    &["Betelgeuses", "Duses", "Meuses", "Syracuses", "Toulouses"];

pub(crate) const SI_SB_USES_USE: &[&str] = &[
    "abuses",
    "applauses",
    "blouses",
    "carouses",
    "causes",
    "chartreuses",
    "clauses",
    "contuses",
    "douses",
    "excuses",
    "fuses",
    "grouses",
    "hypotenuses",
    "masseuses",
    "menopauses",
    "misuses",
    "muses",
    "overuses",
    "pauses",
    "peruses",
    "profuses",
    "recluses",
    "reuses",
    "ruses",
    "souses",
    "spouses",
    "suffuses",
    "transfuses",
    "uses",
];

pub(crate) const SI_SB_IES_IE_CASE: &[&str] = &[
    "Addies",
    "Aggies",
    "Allies",
    "Amies",
    "Angies",
    "Annies",
    "Annmaries",
    "Archies",
    "Arties",
    "Aussies",
    "Barbies",
    "Barries",
    "Basies",
    "Bennies",
    "Bernies",
    "Berties",
    "Bessies",
    "Betties",
    "Billies",
    "Blondies",
    "Bobbies",
    "Bonnies",
    "Bowies",
    "Brandies",
    "Bries",
    "Brownies",
    "Callies",
    "Carnegies",
    "Carries",
    "Cassies",
    "Charlies",
    "Cheries",
    "Christies",
    "Connies",
    "Curies",
    "Dannies",
    "Debbies",
    "Dixies",
    "Dollies",
    "Donnies",
    "Drambuies",
    "Eddies",
    "Effies",
    "Ellies",
    "Elsies",
    "Eries",
    "Ernies",
    "Essies",
    "Eugenies",
    "Fannies",
    "Flossies",
    "Frankies",
    "Freddies",
    "Gillespies",
    "Goldies",
    "Gracies",
    "Guthries",
    "Hallies",
    "Hatties",
    "Hetties",
    "Hollies",
    "Jackies",
    "Jamies",
    "Janies",
    "Jannies",
    "Jeanies",
    "Jeannies",
    "Jennies",
    "Jessies",
    "Jimmies",
    "Jodies",
    "Johnies",
    "Johnnies",
    "Josies",
    "Julies",
    "Kalgoorlies",
    "Kathies",
    "Katies",
    "Kellies",
    "Kewpies",
    "Kristies",
    "Laramies",
    "Lassies",
    "Lauries",
    "Leslies",
    "Lessies",
    "Lillies",
    "Lizzies",
    "Lonnies",
    "Lories",
    "Lorries",
    "Lotties",
    "Louies",
    "Mackenzies",
    "Maggies",
    "Maisies",
    "Mamies",
    "Marcies",
    "Margies",
    "Maries",
    "Marjories",
    "Matties",
    "McKenzies",
    "Melanies",
    "Mickies",
    "Millies",
    "Minnies",
    "Mollies",
    "Mounties",
    "Nannies",
    "Natalies",
    "Nellies",
    "Netties",
    "Ollies",
    "Ozzies",
    "Pearlies",
    "Pottawatomies",
    "Reggies",
    "Richies",
    "Rickies",
    "Robbies",
    "Ronnies",
    "Rosalies",
    "Rosemaries",
    "Rosies",
    "Roxies",
    "Rushdies",
    "Ruthies",
    "Sadies",
    "Sallies",
    "Sammies",
    "Scotties",
    "Selassies",
    "Sherries",
    "Sophies",
    "Stacies",
    "Stefanies",
    "Stephanies",
    "Stevies",
    "Susies",
    "Sylvies",
    "Tammies",
    "Terries",
    "Tessies",
    "Tommies",
    "Tracies",
    "Trekkies",
    "Valaries",
    "Valeries",
    "Valkyries",
    "Vickies",
    "Virgies",
    "Willies",
    "Winnies",
    "Wylies",
    "Yorkies",
];

pub(crate) const SI_SB_IES_IE: &[&str] = &[
    "aeries",
    "baggies",
    "belies",
    "biggies",
    "birdies",
    "bogies",
    "bonnies",
    "boogies",
    "bookies",
    "bourgeoisies",
    "brownies",
    "budgies",
    "caddies",
    "calories",
    "camaraderies",
    "cockamamies",
    "collies",
    "cookies",
    "coolies",
    "cooties",
    "coteries",
    "crappies",
    "curies",
    "cutesies",
    "dogies",
    "eyries",
    "floozies",
    "footsies",
    "freebies",
    "genies",
    "goalies",
    "groupies",
    "hies",
    "jalousies",
    "junkies",
    "kiddies",
    "laddies",
    "lassies",
    "lies",
    "lingeries",
    "magpies",
    "menageries",
    "mommies",
    "movies",
    "neckties",
    "newbies",
    "nighties",
    "oldies",
    "organdies",
    "overlies",
    "pies",
    "pinkies",
    "pixies",
    "potpies",
    "prairies",
    "quickies",
    "reveries",
    "rookies",
    "rotisseries",
    "softies",
    "sorties",
    "species",
    "stymies",
    "sweeties",
    "ties",
    "underlies",
    "unties",
    "veggies",
    "vies",
    "yuppies",
    "zombies",
];

pub(crate) const SI_SB_OES_OE_CASE: &[&str] = &[
    "Chloes",
    "Crusoes",
    "Defoes",
    "Faeroes",
    "Ivanhoes",
    "Joes",
    "McEnroes",
    "Moes",
    "Monroes",
    "Noes",
    "Poes",
    "Roscoes",
    "Tahoes",
    "Tippecanoes",
    "Zoes",
];

pub(crate) const SI_SB_OES_OE: &[&str] = &[
    "aloes",
    "backhoes",
    "canoes",
    "does",
    "floes",
    "foes",
    "hoes",
    "mistletoes",
    "oboes",
    "pekoes",
    "roes",
    "sloes",
    "throes",
    "tiptoes",
    "toes",
    "woes",
];

pub(crate) const SI_SB_Z_ZES: &[&str] = &["quartzes", "topazes"];

pub(crate) const SI_SB_ZZES_ZZ: &[&str] = &["buzzes", "fizzes", "frizzes", "razzes"];

pub(crate) const SI_SB_CHES_CHE_CASE: &[&str] = &[
    "Andromaches",
    "Apaches",
    "Blanches",
    "Comanches",
    "Nietzsches",
    "Porsches",
    "Roches",
];

pub(crate) const SI_SB_CHES_CHE: &[&str] = &[
    "aches",
    "avalanches",
    "backaches",
    "bellyaches",
    "caches",
    "cloches",
    "creches",
    "douches",
    "earaches",
    "fiches",
    "headaches",
    "heartaches",
    "microfiches",
    "niches",
    "pastiches",
    "psyches",
    "quiches",
    "stomachaches",
    "toothaches",
    "tranches",
];

pub(crate) const SI_SB_XES_XE: &[&str] = &["annexes", "axes", "deluxes", "pickaxes"];

pub(crate) const SI_SB_SSES_SSE_CASE: &[&str] = &["Hesses", "Jesses", "Larousses", "Matisses"];

pub(crate) const SI_SB_SSES_SSE: &[&str] = &[
    "bouillabaisses",
    "crevasses",
    "demitasses",
    "impasses",
    "mousses",
    "posses",
];

pub(crate) const SI_SB_VES_VE_CASE: &[&str] = &["Clives", "Palmolives"];

pub(crate) const SI_SB_VES_VE: &[&str] = &[
    "interweaves",
    "weaves",
    "olives",
    "bivalves",
    "dissolves",
    "resolves",
    "salves",
    "twelves",
    "valves",
];

pub(crate) const SI_SB_ES_IS: &[&str] = &[
    "amanuenses",
    "amniocenteses",
    "analyses",
    "antitheses",
    "apotheoses",
    "arterioscleroses",
    "atheroscleroses",
    "axes",
    "catalyses",
    "catharses",
    "chasses",
    "cirrhoses",
    "cocces",
    "crises",
    "diagnoses",
    "dialyses",
    "diereses",
    "electrolyses",
    "emphases",
    "exegeses",
    "geneses",
    "halitoses",
    "hydrolyses",
    "hypnoses",
    "hypotheses",
    "hystereses",
    "metamorphoses",
    "metastases",
    "misdiagnoses",
    "mitoses",
    "mononucleoses",
    "narcoses",
    "necroses",
    "nemeses",
    "neuroses",
    "oases",
    "osmoses",
    "osteoporoses",
    "paralyses",
    "parentheses",
    "parthenogeneses",
    "periphrases",
    "photosyntheses",
    "probosces",
    "prognoses",
    "prophylaxes",
    "prostheses",
    "preces",
    "psoriases",
    "psychoanalyses",
    "psychokineses",
    "psychoses",
    "scleroses",
    "scolioses",
    "sepses",
    "silicoses",
    "symbioses",
    "synopses",
    "syntheses",
    "taxes",
    "telekineses",
    "theses",
    "thromboses",
    "tuberculoses",
    "urinalyses",
];

pub(crate) const PL_PREP_LIST: &[&str] = &[
    "about", "above", "across", "after", "among", "around", "at", "athwart", "before", "behind",
    "below", "beneath", "beside", "besides", "between", "betwixt", "beyond", "but", "by", "during",
    "except", "for", "from", "in", "into", "near", "of", "off", "on", "onto", "out", "over",
    "since", "till", "to", "under", "until", "unto", "upon", "with",
];

pub(crate) const PL_PRON_NOM: &[(&str, &str)] = &[
    ("i", "we"),
    ("myself", "ourselves"),
    ("you", "you"),
    ("yourself", "yourselves"),
    ("she", "they"),
    ("herself", "themselves"),
    ("he", "they"),
    ("himself", "themselves"),
    ("it", "they"),
    ("itself", "themselves"),
    ("they", "they"),
    ("themself", "themselves"),
    ("mine", "ours"),
    ("yours", "yours"),
    ("hers", "theirs"),
    ("his", "theirs"),
    ("its", "theirs"),
    ("theirs", "theirs"),
];

pub(crate) const PL_PRON_ACC: &[(&str, &str)] = &[
    ("me", "us"),
    ("myself", "ourselves"),
    ("you", "you"),
    ("yourself", "yourselves"),
    ("her", "them"),
    ("herself", "themselves"),
    ("him", "them"),
    ("himself", "themselves"),
    ("it", "them"),
    ("itself", "themselves"),
    ("them", "them"),
    ("themself", "themselves"),
];

pub(crate) const PLVERB_IRREGULAR_PRES: &[(&str, &str)] = &[
    ("am", "are"),
    ("are", "are"),
    ("is", "are"),
    ("was", "were"),
    ("were", "were"),
    ("have", "have"),
    ("has", "have"),
    ("do", "do"),
    ("does", "do"),
];

pub(crate) const PLVERB_AMBIGUOUS_PRES: &[(&str, &str)] = &[
    ("act", "act"),
    ("acts", "act"),
    ("blame", "blame"),
    ("blames", "blame"),
    ("can", "can"),
    ("must", "must"),
    ("fly", "fly"),
    ("flies", "fly"),
    ("copy", "copy"),
    ("copies", "copy"),
    ("drink", "drink"),
    ("drinks", "drink"),
    ("fight", "fight"),
    ("fights", "fight"),
    ("fire", "fire"),
    ("fires", "fire"),
    ("like", "like"),
    ("likes", "like"),
    ("look", "look"),
    ("looks", "look"),
    ("make", "make"),
    ("makes", "make"),
    ("reach", "reach"),
    ("reaches", "reach"),
    ("run", "run"),
    ("runs", "run"),
    ("sink", "sink"),
    ("sinks", "sink"),
    ("sleep", "sleep"),
    ("sleeps", "sleep"),
    ("view", "view"),
    ("views", "view"),
];

pub(crate) const PLVERB_IRREGULAR_NON_PRES: &[&str] = &[
    "did", "had", "ate", "made", "put", "spent", "fought", "sank", "gave", "sought", "shall",
    "could", "ought", "should",
];

pub(crate) const PL_V_OES_OE: &[&str] = &["canoes", "floes", "oboes", "roes", "throes", "woes"];

pub(crate) const PL_V_OES_OE_ENDINGS_SIZE4: &[&str] = &["hoes", "toes"];

pub(crate) const PL_V_OES_OE_ENDINGS_SIZE5: &[&str] = &["shoes"];

pub(crate) const PL_COUNT_ZERO: &[&str] = &["0", "no", "zero", "nil"];

pub(crate) const PL_COUNT_ONE: &[&str] = &["1", "a", "an", "one", "each", "every", "this", "that"];

pub(crate) const PL_ADJ_SPECIAL: &[(&str, &str)] = &[
    ("a", "some"),
    ("an", "some"),
    ("this", "these"),
    ("that", "those"),
];

pub(crate) const PL_ADJ_POSS: &[(&str, &str)] = &[
    ("my", "our"),
    ("your", "your"),
    ("its", "their"),
    ("her", "their"),
    ("his", "their"),
    ("their", "their"),
];

pub(crate) const ORDINAL: &[(&str, &str)] = &[
    ("ty", "tieth"),
    ("one", "first"),
    ("two", "second"),
    ("three", "third"),
    ("five", "fifth"),
    ("eight", "eighth"),
    ("nine", "ninth"),
    ("twelve", "twelfth"),
];

pub(crate) const UNIT: &[&str] = &[
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub(crate) const TEEN: &[&str] = &[
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

pub(crate) const TEN: &[&str] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub(crate) const MILL: &[&str] = &[
    " ",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
    " sextillion",
    " septillion",
    " octillion",
    " nonillion",
    " decillion",
];

pub(crate) type Set = HashSet<&'static str>;
pub(crate) type Map = HashMap<&'static str, &'static str>;

pub(crate) fn enclose(s: &str) -> String {
    format!("(?:{s})")
}

pub(crate) fn joinstem<S: AsRef<str>>(cutpoint: Option<usize>, words: &[S]) -> String {
    let stems: Vec<&str> = words
        .iter()
        .map(|w| match cutpoint {
            Some(cut) => crate::text::chop(w.as_ref(), cut),
            None => w.as_ref(),
        })
        .collect();
    enclose(&stems.join("|"))
}

fn si_list(words: &[&str], cut: Option<usize>, ending: &str) -> Vec<String> {
    words
        .iter()
        .map(|w| match cut {
            Some(cut) => format!("{}{ending}", crate::text::chop(w, cut)),
            None => format!("{w}{ending}"),
        })
        .collect()
}

fn set(lists: &[&[&'static str]]) -> Set {
    lists.iter().flat_map(|l| l.iter().copied()).collect()
}

fn invert(pairs: impl IntoIterator<Item = (&'static str, &'static str)>) -> Map {
    let mut inverted = Map::new();
    for (singular, plural) in pairs {
        for form in plural.split('|') {
            inverted.insert(form, singular);
        }
    }
    inverted
}

pub(crate) static PL_SB_IRREGULAR: LazyLock<Map> = LazyLock::new(|| {
    PL_SB_IRREGULAR_BASE
        .iter()
        .chain(PL_SB_IRREGULAR_S)
        .copied()
        .collect()
});
pub(crate) static PL_SB_IRREGULAR_CAPS_MAP: LazyLock<Map> =
    LazyLock::new(|| PL_SB_IRREGULAR_CAPS.iter().copied().collect());
pub(crate) static PL_SB_IRREGULAR_COMPOUND_MAP: LazyLock<Map> =
    LazyLock::new(|| PL_SB_IRREGULAR_COMPOUND.iter().copied().collect());
pub(crate) static SI_SB_IRREGULAR: LazyLock<Map> = LazyLock::new(|| {
    invert(
        PL_SB_IRREGULAR_BASE
            .iter()
            .chain(PL_SB_IRREGULAR_S)
            .copied(),
    )
});
pub(crate) static SI_SB_IRREGULAR_CAPS: LazyLock<Map> =
    LazyLock::new(|| invert(PL_SB_IRREGULAR_CAPS.iter().copied()));
pub(crate) static SI_SB_IRREGULAR_COMPOUND: LazyLock<Map> =
    LazyLock::new(|| invert(PL_SB_IRREGULAR_COMPOUND.iter().copied()));
pub(crate) static PL_SB_IRREGULAR_VALUES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    PL_SB_IRREGULAR_BASE
        .iter()
        .chain(PL_SB_IRREGULAR_S)
        .chain(PL_SB_IRREGULAR_CAPS)
        .map(|&(_, plural)| plural)
        .collect()
});

pub(crate) static PL_SB_C_IS_IDES_LIST: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PL_SB_C_IS_IDES_COMPLETE
        .iter()
        .chain(PL_SB_C_IS_IDES_ENDINGS)
        .copied()
        .collect()
});
pub(crate) static PL_SB_C_IS_IDES: LazyLock<String> = LazyLock::new(|| {
    let words: Vec<String> = PL_SB_C_IS_IDES_COMPLETE
        .iter()
        .map(|w| w.to_string())
        .chain(PL_SB_C_IS_IDES_ENDINGS.iter().map(|w| format!(".*{w}")))
        .collect();
    joinstem(Some(2), &words)
});

pub(crate) struct Family {
    pub(crate) plurals: Vec<String>,
    pub(crate) stem: String,
}

fn family(words: &[&str], ending: &str, cut: Option<usize>) -> Family {
    Family {
        plurals: si_list(words, cut, ending),
        stem: joinstem(cut, words),
    }
}

pub(crate) static SI_SB_C_IS_IDES: LazyLock<Vec<String>> =
    LazyLock::new(|| si_list(&PL_SB_C_IS_IDES_LIST, Some(2), "ides"));
pub(crate) static C_A_ATA: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_A_ATA_LIST, "ata", Some(1)));
pub(crate) static U_A_AE: LazyLock<Family> = LazyLock::new(|| family(PL_SB_U_A_AE_LIST, "e", None));
pub(crate) static C_A_AE: LazyLock<Family> = LazyLock::new(|| family(PL_SB_C_A_AE_LIST, "e", None));
pub(crate) static C_EN_INA: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_EN_INA_LIST, "ina", Some(2)));
pub(crate) static U_UM_A: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_UM_A_LIST, "a", Some(2)));
pub(crate) static C_UM_A: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_UM_A_LIST, "a", Some(2)));
pub(crate) static U_US_I: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_US_I_LIST, "i", Some(2)));
pub(crate) static C_US_I: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_US_I_LIST, "i", Some(2)));
pub(crate) static U_ON_A: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_ON_A_LIST, "a", Some(2)));
pub(crate) static C_ON_A: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_ON_A_LIST, "a", Some(2)));
pub(crate) static U_CH_CHS: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_CH_CHS_LIST, "s", None));
pub(crate) static U_EX_ICES: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_EX_ICES_LIST, "ices", Some(2)));
pub(crate) static U_IX_ICES: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_IX_ICES_LIST, "ices", Some(2)));
pub(crate) static C_EX_ICES: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_EX_ICES_LIST, "ices", Some(2)));
pub(crate) static C_IX_ICES: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_C_IX_ICES_LIST, "ices", Some(2)));
pub(crate) static C_I: LazyLock<Family> = LazyLock::new(|| family(PL_SB_C_I_LIST, "i", None));
pub(crate) static C_IM: LazyLock<Family> = LazyLock::new(|| family(PL_SB_C_IM_LIST, "im", None));
pub(crate) static C_O_I: LazyLock<Family> = LazyLock::new(|| family(PL_SB_C_O_I, "i", Some(1)));
pub(crate) static U_MAN_MANS: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_MAN_MANS_LIST, "s", None));
pub(crate) static U_MAN_MANS_CAPS: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_MAN_MANS_CAPS_LIST, "s", None));
pub(crate) static U_LOUSE_LICE: LazyLock<Family> =
    LazyLock::new(|| family(PL_SB_U_LOUSE_LICE_LIST, "lice", Some(5)));

pub(crate) static PL_SB_U_O_OS_ENDINGS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PL_SB_U_O_OS_ENDINGS_BASE
        .iter()
        .chain(PL_SB_C_O_I)
        .copied()
        .collect()
});
pub(crate) static SI_SB_U_O_OS_ENDINGS: LazyLock<Vec<String>> =
    LazyLock::new(|| si_list(&PL_SB_U_O_OS_ENDINGS, None, "s"));
pub(crate) static PL_SB_U_O_OS_COMPLETE_SET: LazyLock<Set> =
    LazyLock::new(|| set(&[PL_SB_U_O_OS_COMPLETE]));
pub(crate) static SI_SB_U_O_OS_COMPLETE: LazyLock<HashSet<String>> = LazyLock::new(|| {
    PL_SB_U_O_OS_COMPLETE
        .iter()
        .map(|w| format!("{w}s"))
        .collect()
});

pub(crate) static PL_SB_UNINFLECTED_COMPLETE: LazyLock<Set> = LazyLock::new(|| {
    set(&[
        PL_SB_UNINFLECTED_COMPLETE_BASE,
        PL_SB_UNINFLECTED_S_COMPLETE,
    ])
});
pub(crate) static PL_SB_UNINFLECTED_CAPS_SET: LazyLock<Set> =
    LazyLock::new(|| set(&[PL_SB_UNINFLECTED_CAPS]));
pub(crate) static PL_SB_UNINFLECTED_HERD_SET: LazyLock<Set> =
    LazyLock::new(|| set(&[PL_SB_UNINFLECTED_HERD]));
pub(crate) static PL_SB_UNINFLECTED_ENDINGS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PL_SB_UNINFLECTED_ENDINGS_BASE
        .iter()
        .chain(PL_SB_UNINFLECTED_S_ENDINGS)
        .copied()
        .collect()
});
pub(crate) static PL_SB_UNINFLECTED_S: LazyLock<Vec<String>> = LazyLock::new(|| {
    PL_SB_UNINFLECTED_S_COMPLETE
        .iter()
        .map(|w| w.to_string())
        .chain(PL_SB_UNINFLECTED_S_ENDINGS.iter().map(|w| format!(".*{w}")))
        .collect()
});
pub(crate) static PL_SB_C_US_US_SET: LazyLock<Set> = LazyLock::new(|| set(&[PL_SB_C_US_US]));

pub(crate) static PL_SB_SINGULAR_S_COMPLETE: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PL_SB_SINGULAR_S_COMPLETE_BASE
        .iter()
        .chain(PL_SB_C_IS_IDES_COMPLETE)
        .copied()
        .collect()
});
pub(crate) static PL_SB_SINGULAR_S_ENDINGS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["ss", "us"]
        .iter()
        .chain(PL_SB_C_IS_IDES_ENDINGS)
        .copied()
        .collect()
});
pub(crate) static SI_SB_SINGULAR_S_COMPLETE: LazyLock<HashSet<String>> = LazyLock::new(|| {
    PL_SB_SINGULAR_S_COMPLETE
        .iter()
        .map(|w| format!("{w}es"))
        .collect()
});
pub(crate) static SI_SB_SINGULAR_S_ENDINGS: LazyLock<Vec<String>> =
    LazyLock::new(|| si_list(&PL_SB_SINGULAR_S_ENDINGS, None, "es"));
pub(crate) static PL_SB_SINGULAR_S: LazyLock<String> = LazyLock::new(|| {
    let alternatives: Vec<String> = PL_SB_SINGULAR_S_COMPLETE
        .iter()
        .map(|w| w.to_string())
        .chain(PL_SB_SINGULAR_S_ENDINGS.iter().map(|w| format!(".*{w}")))
        .chain(["[A-Z].*es".to_string()])
        .collect();
    enclose(&alternatives.join("|"))
});

pub(crate) static PLVERB_SPECIAL_S: LazyLock<String> = LazyLock::new(|| {
    let alternatives: Vec<String> = [PL_SB_SINGULAR_S.clone()]
        .into_iter()
        .chain(PL_SB_UNINFLECTED_S.iter().cloned())
        .chain(PL_SB_IRREGULAR_S.iter().map(|(k, _)| k.to_string()))
        .chain(["(.*[csx])is", "(.*)ceps", "[A-Z].*s"].map(String::from))
        .collect();
    enclose(&alternatives.join("|"))
});

pub(crate) static PL_SB_POSTFIX_ADJ_STEMS: LazyLock<String> = LazyLock::new(|| {
    let definitions = [
        (
            "general",
            enclose(r"(?!major|lieutenant|brigadier|adjutant|.*star)\S+"),
        ),
        ("martial", enclose("court")),
        ("force", enclose("pound")),
    ];
    let alternatives: Vec<String> = definitions
        .iter()
        .map(|(key, val)| enclose(&format!(r"{val}(?=(?:-|\s+){key})")))
        .collect();
    format!("({})(.*)", alternatives.join("|"))
});

macro_rules! sets {
    ($($name:ident = $list:expr;)*) => {
        $(pub(crate) static $name: LazyLock<Set> = LazyLock::new(|| set(&[$list]));)*
    };
}

sets! {
    SI_SB_OIS_OI_CASE_SET = SI_SB_OIS_OI_CASE;
    SI_SB_USES_USE_CASE_SET = SI_SB_USES_USE_CASE;
    SI_SB_USES_USE_SET = SI_SB_USES_USE;
    SI_SB_IES_IE_CASE_SET = SI_SB_IES_IE_CASE;
    SI_SB_IES_IE_SET = SI_SB_IES_IE;
    SI_SB_OES_OE_CASE_SET = SI_SB_OES_OE_CASE;
    SI_SB_OES_OE_SET = SI_SB_OES_OE;
    SI_SB_Z_ZES_SET = SI_SB_Z_ZES;
    SI_SB_ZZES_ZZ_SET = SI_SB_ZZES_ZZ;
    SI_SB_CHES_CHE_CASE_SET = SI_SB_CHES_CHE_CASE;
    SI_SB_CHES_CHE_SET = SI_SB_CHES_CHE;
    SI_SB_XES_XE_SET = SI_SB_XES_XE;
    SI_SB_SSES_SSE_CASE_SET = SI_SB_SSES_SSE_CASE;
    SI_SB_SSES_SSE_SET = SI_SB_SSES_SSE;
    SI_SB_VES_VE_CASE_SET = SI_SB_VES_VE_CASE;
    SI_SB_VES_VE_SET = SI_SB_VES_VE;
    SI_SB_ES_IS_SET = SI_SB_ES_IS;
    PLVERB_IRREGULAR_NON_PRES_SET = PLVERB_IRREGULAR_NON_PRES;
    PL_V_OES_OE_SET = PL_V_OES_OE;
    PL_COUNT_ZERO_SET = PL_COUNT_ZERO;
    PL_COUNT_ONE_SET = PL_COUNT_ONE;
}

pub(crate) static PL_PREP_LIST_DA: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    PL_PREP_LIST
        .iter()
        .copied()
        .chain(["de", "du", "da"])
        .collect()
});
pub(crate) static PL_PREP_SET: LazyLock<Set> = LazyLock::new(|| set(&[&PL_PREP_LIST_DA]));
pub(crate) static PL_SB_PREP_DUAL_COMPOUND: LazyLock<String> = LazyLock::new(|| {
    format!(
        r"(.*?)((?:-|\s+)(?:{})(?:-|\s+))a(?:-|\s+)(.*)",
        enclose(&PL_PREP_LIST_DA.join("|"))
    )
});

macro_rules! maps {
    ($($name:ident = $pairs:expr;)*) => {
        $(pub(crate) static $name: LazyLock<Map> =
            LazyLock::new(|| $pairs.iter().copied().collect());)*
    };
}

maps! {
    PL_PRON_NOM_MAP = PL_PRON_NOM;
    PL_PRON_ACC_MAP = PL_PRON_ACC;
    PLVERB_IRREGULAR_PRES_MAP = PLVERB_IRREGULAR_PRES;
    PLVERB_AMBIGUOUS_PRES_MAP = PLVERB_AMBIGUOUS_PRES;
    PL_ADJ_SPECIAL_MAP = PL_ADJ_SPECIAL;
    PL_ADJ_POSS_MAP = PL_ADJ_POSS;
}

pub(crate) static PLVERB_AMBIGUOUS_PRES_KEYS: LazyLock<String> = LazyLock::new(|| {
    let keys: Vec<&str> = PLVERB_AMBIGUOUS_PRES.iter().map(|&(k, _)| k).collect();
    enclose(&keys.join("|"))
});

pub(crate) const NTH_SUFF: &[&str] = &["th", "st", "nd", "rd"];

pub(crate) fn nth(n: u32) -> Option<&'static str> {
    match n {
        1 => Some("st"),
        2 => Some("nd"),
        3 => Some("rd"),
        0 | 4..=9 | 11..=13 => Some("th"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stems() {
        assert_eq!(enclose("test"), "(?:test)");
        assert_eq!(
            joinstem(Some(2), &["ephemeris", "iris", ".*itis"]),
            "(?:ephemer|ir|.*it)"
        );
        assert_eq!(joinstem(None, &["ephemeris"]), "(?:ephemeris)");
        assert_eq!(joinstem::<&str>(Some(5), &[]), "(?:)");
    }

    #[test]
    fn derived_tables() {
        assert_eq!(SI_SB_IRREGULAR.get("brethren"), Some(&"brother"));
        assert_eq!(SI_SB_IRREGULAR.get("lore"), Some(&"lore"));
        assert_eq!(
            SI_SB_IRREGULAR_COMPOUND.get("prime donne"),
            Some(&"prima donna")
        );
        assert!(U_LOUSE_LICE.plurals.iter().any(|p| p == "woodlice"));
        assert!(PL_SB_UNINFLECTED_COMPLETE.contains("scissors"));
    }
}
