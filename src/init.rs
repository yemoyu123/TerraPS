use std::{
    fs::{write, DirBuilder},
    path::Path,
};

pub fn init() {
    dir_init();
    json_init();
}

// Create directories if they doesn't exist
fn dir_init() {
    if !Path::new("./config/").exists() {
        DirBuilder::new().recursive(true).create("./config/").expect("Failed to create config directory");
    }
    if !Path::new("./data/").exists() {
        DirBuilder::new().recursive(true).create("./data/").expect("Failed to create data directory");
    }
    if !Path::new("./data/announce/").exists() {
        DirBuilder::new().recursive(true).create("./data/announce/").expect("Failed to create announce directory");
    }
    if !Path::new("./data/crisis/").exists() {
        DirBuilder::new().recursive(true).create("./data/crisis/").expect("Failed to create crisis directory");
    }
    if !Path::new("./data/crisisV2/").exists() {
        DirBuilder::new().recursive(true).create("./data/crisisV2/").expect("Failed to create crisisV2 directory");
    }
    if !Path::new("./data/excel/").exists() {
        DirBuilder::new().recursive(true).create("./data/excel/").expect("Failed to create excel directory");
    }
    if !Path::new("./data/rlv2/").exists() {
        DirBuilder::new().recursive(true).create("./data/excel/").expect("Failed to create rlv2 directory");
    }
    if !Path::new("./data/tower/").exists() {
        DirBuilder::new().recursive(true).create("./data/tower/").expect("Failed to create tower directory");
    }
    if !Path::new("./data/user/").exists() {
        DirBuilder::new().recursive(true).create("./data/user/").expect("Failed to create raid directory");
    }
}

fn json_init() {
    if !Path::new("./config/config.json").exists() {
        let config = r#"{
            "server": {
                "host": "127.0.0.1",
                "port": 8443,
                "enableServer": true,
                "maintenanceMsg": "Server is currently under maintenance!",
                "mode": "cn",
                "adaptive": true,
                "noProxy": false,
                "timeout": 30
            },
            "assets": {
                "autoUpdate": false,
                "downloadLocally": true,
                "enableMods": false,
                "skipModCacheValidation": false
            },
            "version": {
                "android": {
                    "resVersion": "24-02-26-08-28-19-0f351f",
                    "clientVersion": "2.2.01"
                }
            },
            "remote": {
                "enableGameBI": false,
                "enableSDKNetSecure": true,
                "enableBestHttp": true
            },
            "networkConfig": {
                "cn": {
                    "sign": "sign",
                    "content": {
                        "configVer": "5",
                        "funcVer": "V047",
                        "configs": {
                            "V047": {
                                "override": true,
                                "network": {
                                    "gs": "{server}",
                                    "as": "{server}",
                                    "u8": "{server}/u8",
                                    "hu": "{server}/assetbundle/official",
                                    "hv": "{server}/config/prod/official/{0}/version",
                                    "rc": "{server}/config/prod/official/remote_config",
                                    "an": "{server}/config/prod/announce_meta/{0}/announcement.meta.json",
                                    "prean": "{server}/config/prod/announce_meta/{0}/preannouncement.meta.json",
                                    "sl": "https://ak.hypergryph.com/protocol/service",
                                    "of": "https://ak.hypergryph.com/index.html",
                                    "pkgAd": null,
                                    "pkgIOS": null,
                                    "secure": false
                                }
                            }
                        }
                    }
                },
                "global": {
                    "sign": "sign",
                    "content": {
                        "configVer": "1",
                        "funcVer": "V029",
                        "configs": {
                            "V029": {
                                "override": true,
                                "network": {
                                    "gs": "{server}",
                                    "as": "{server}",
                                    "u8": "{server}/u8",
                                    "hu": "{server}/assetbundle/official",
                                    "hv": "{server}/config/prod/official/{0}/version",
                                    "rc": "{server}/config/prod/official/remote_config",
                                    "an": "{server}/config/prod/announce_meta/{0}/announcement.meta.json",
                                    "prean": "{server}/config/prod/announce_meta/{0}/preannouncement.meta.json",
                                    "sl": "https://www.arknights.global/terms_of_service",
                                    "of": "https://www.arknights.global",
                                    "pkgAd": null,
                                    "pkgIOS": null,
                                    "secure": false
                                }
                            }
                        }
                    }
                }
            },
            "crisisConfig": {
                "selectedCrisis": "cc12"
            },
            "crisisV2Config": {
                "selectedCrisis": "cc1"
            },
            "towerConfig": {
                "season": "tower_season_4"
            },
            "rlv2Config": {
                "allChars": true
            },
            "battleReplayConfig": {
                "anonymous": false
            },
            "userConfig": {
                "restorePreviousStates": {
                    "is2": false,
                    "squadsAndFavs": false,
                    "ui": false
                },
                "activityMinStartTs": 1707984000,
                "activityMaxStartTs": 1710359999,
                "secretary": "char_245_cello",
                "secretarySkinId": "char_245_cello#2",
                "background": "bg_rogue_3",
                "theme": "tm_mainline_1",
                "fakeTime": -1
            },
            "charConfig": {
                "favorPoint": 25570,
                "potentialRank": 5,
                "mainSkillLvl": 7,
                "level": -1,
                "evolvePhase": -1,
                "skillsSpecializeLevel": 3,
                "customUnitInfo": {},
                "duplicateUnits": []
            },
            "gacha": {
                "5rarity": 1.0,
                "4rarity": 0,
                "3rarity": 0,
                "2rarity": 0
            },
            "assistUnits": [
                {
                    "charId": "char_479_sleach",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_sleach"
                },
                {
                    "charId": "char_293_thorns",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_4064_mlynar",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_264_f12yin",
                    "skillIndex": 1,
                    "currentEquip": null
                },
                {
                    "charId": "char_350_surtr",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_1013_chen2",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_003_kalts",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_kalts"
                },
                {
                    "charId": "char_179_cgbird",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_cgbird"
                },
                {
                    "charId": "char_358_lisa",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_lisa"
                },
                {
                    "charId": "char_1012_skadi2",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_1028_texas2",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_texas2"
                },
                {
                    "charId": "char_1026_gvial2",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_gvial2"
                },
                {
                    "charId": "char_1014_nearl2",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_nearl2"
                },
                {
                    "charId": "char_311_mudrok",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_mudrok"
                },
                {
                    "charId": "char_202_demkni",
                    "skillIndex": 0,
                    "currentEquip": "uniequip_003_demkni"
                },
                {
                    "charId": "char_4055_bgsnow",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_bgsnow"
                },
                {
                    "charId": "char_2012_typhon",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_typhon"
                },
                {
                    "charId": "char_180_amgoat",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_amgoat"
                },
                {
                    "charId": "char_134_ifrit",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_ifrit"
                },
                {
                    "charId": "char_4087_ines",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_010_chen",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_chen"
                },
                {
                    "charId": "char_017_huang",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_huang"
                },
                {
                    "charId": "char_263_skadi",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_skadi"
                },
                {
                    "charId": "char_1034_jesca2",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_2014_nian",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_nian"
                },
                {
                    "charId": "char_340_shwaz",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_shwaz"
                },
                {
                    "charId": "char_2013_cerber",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_cerber"
                },
                {
                    "charId": "char_4080_lin",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_lin"
                },
                {
                    "charId": "char_1016_agoat2",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_1029_yato2",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_yato2"
                },
                {
                    "charId": "char_136_hsguma",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_003_hsguma"
                },
                {
                    "charId": "char_2012_typhon",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_typhon"
                },
                {
                    "charId": "char_1026_gvial2",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_002_gvial2"
                },
                {
                    "charId": "char_245_cello",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_4088_hodrer",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_377_gdglow",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_151_myrtle",
                    "skillIndex": 0,
                    "currentEquip": "uniequip_002_myrtle"
                },
                {
                    "charId": "char_113_cqbw",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_cqbw"
                },
                {
                    "charId": "char_1034_jesca2",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_222_bpipe",
                    "skillIndex": 2,
                    "currentEquip": "uniequip_003_bpipe"
                },
                {
                    "charId": "char_4117_ray",
                    "skillIndex": 2,
                    "currentEquip": null
                },
                {
                    "charId": "char_4087_ines",
                    "skillIndex": 1,
                    "currentEquip": null
                },
                {
                    "charId": "char_1033_swire2",
                    "skillIndex": 1,
                    "currentEquip": "uniequip_002_swire2"
                }
            ]
        }"#;
        write("./config/config.json", config).expect("Unable to write JSON.");
    }
}
