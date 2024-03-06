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
    if !Path::new("./data/crisisv2/cc1.json").exists() {
        let contents = r#"{
            "info": {
                "seasonId": "crisis_v2_season_1_1",
                "mapStageDataMap": {
                    "crisis_v2_01-01": {
                        "stageId": "level_crisis_v2_01-01",
                        "mapId": "crisis_v2_01-01",
                        "levelId": "Obt/Crisis/V2/level_crisis_v2_01-01",
                        "stageType": "PERMANENT",
                        "code": "维多利亚",
                        "name": " 灰烬泽地",
                        "loadingPicId": "loading_VC_Reed",
                        "description": "这里被战火与自然摧毁了无数次，然而灾难过后，芦苇们仍能沾着灰烬重新昂起头颅。你能如芦苇般抵挡敌人至最后吗？还是像这满地的灰烬一般，接受自己的失败？\\n<@lv.item><芦苇丛></>置于其中的干员获得\"迷彩\"",
                        "picId": "map_fc_reed",
                        "logoPicId": "logo_victoria",
                        "startTime": -1,
                        "rewardEndTime": -1
                    },
                    "crisis_v2_01-02": {
                        "stageId": "level_crisis_v2_01-02",
                        "mapId": "crisis_v2_01-02",
                        "levelId": "Obt/Crisis/V2/level_crisis_v2_01-02",
                        "stageType": "TEMPORARY",
                        "code": " 大炎",
                        "name": "设防关隘",
                        "loadingPicId": "loading_WB_Wall",
                        "description": "此地自古以来都是官兵设防的 关卡，进可清剿贼寇，退可据险自保。近期一些山海众聚集到一起，想要攻下这处关隘作为他们的营地。官兵不答应，你也不答应。\\n<@lv.item><玉门天灾工事></>置于其中的单位对地面单位造成的伤害提升，受到来自地面单位的伤害降低",
                        "picId": "map_wb_wall",
                        "logoPicId": "logo_yan",
                        "startTime": 1700553600,
                        "rewardEndTime": 1700855999
                    },
                    "crisis_v2_01-03": {
                        "stageId": "level_crisis_v2_01-03",
                        "mapId": "crisis_v2_01-03",
                        "levelId": "Obt/Crisis/V2/level_crisis_v2_01-03",
                        "stageType": "TEMPORARY",
                        "code": "哥伦比亚",
                        "name": "联邦监狱",
                        "loadingPicId": "loading_Columbia",
                        "description": "这里罪犯众多，越狱也时有发生。典狱长常常为此事头疼。现在看来，除了日常的监督与惩戒，一只针对囚犯的快速反应部队也是必不可少。听，囚室那里响起了金属碰撞的声音，你该工作了。\\n<@lv.item><禁锢装置></>禁锢作用范围内已解放的敌人",
                        "picId": "map_columbia_center",
                        "logoPicId": "logo_columbia",
                        "startTime": 1700856000,
                        "rewardEndTime": 1701115199
                    },
                    "crisis_v2_01-05": {
                        "stageId": "level_crisis_v2_01-05",
                        "mapId": "crisis_v2_01-05",
                        "levelId": "Obt/Crisis/V2/level_crisis_v2_01-05",
                        "stageType": "TEMPORARY",
                        "code": "玻利瓦尔",
                        "name": "翻修中沙滩",
                        "loadingPicId": "loading_Bolivar_sea",
                        "description": "经营多年的人工沙滩到了不得不暂停营业进行翻修的时刻。怀念沙滩痛饮时光的酒精爱好者们决定假扮外国人闹事，闯入尚未完工的设施中大喝特喝。绝不能让他们的计划得逞。\\n<@lv.item><深水区></>无法部署，其中的敌方攻击速度和移动速度降低，持续流失生命\\n<@lv.item><特制水上平台></>可部署在深水区，之后该格可部署任意单位，部署的单位撤退时平台损坏",
                        "picId": "map_bolivar_sea",
                        "logoPicId": "logo_bolivar",
                        "startTime": 1701115200,
                        "rewardEndTime": 1701374399
                    },
                    "crisis_v2_01-07": {
                        "stageId": "level_crisis_v2_01-07",
                        "mapId": "crisis_v2_01-07",
                        "levelId": "Obt/Crisis/V2/level_crisis_v2_01-07",
                        "stageType": "TEMPORARY",
                        "code": "切尔诺伯格",
                        "name": "13区废墟",
                        "loadingPicId": "loading4",
                        "description": "切尔诺伯格核心城区之一，遭到了严重的源石污染。大量驻守在此的整合运动成员已失去理智，正在无差别地发动进攻。此等疯狂的场面必须被终结。\\n<@lv.item><“冰淇淋机”></>我方与敌方可夺取控制权，激活后回复周围友方单位技力",
                        "picId": "map_ursus_13ruins",
                        "logoPicId": "logo_ursus",
                        "startTime": 1701374400,
                        "rewardEndTime": 1701719999
                    }
                },
                "mapDetailDataMap": {
                    "crisis_v2_01-01": {
                        "commentDataMap": {
                            "comments_0_1": {
                                "condition": "KillWithSpecSlot",
                                "paramList": [
                                    "node_58",
                                    "1",
                                    "enemy_1269_nhfly",
                                    "30"
                                ],
                                "id": "comments_0_1",
                                "sortId": 1,
                                "desc": "眼疾手快"
                            },
                            "comments_0_2": {
                                "condition": "PassWithEventLess",
                                "paramList": [
                                    "not_build_at_reed",
                                    "0"
                                ],
                                "id": "comments_0_2",
                                "sortId": 2,
                                "desc": "不见人影"
                            },
                            "comments_0_3": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "0",
                                    "60"
                                ],
                                "id": "comments_0_3",
                                "sortId": 3,
                                "desc": "坚守战线"
                            },
                            "comments_0_4": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "0",
                                    "130"
                                ],
                                "id": "comments_0_4",
                                "sortId": 4,
                                "desc": "固守战线"
                            },
                            "comments_0_5": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "1",
                                    "110"
                                ],
                                "id": "comments_0_5",
                                "sortId": 5,
                                "desc": "精准打击"
                            },
                            "comments_0_6": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "1",
                                    "220"
                                ],
                                "id": "comments_0_6",
                                "sortId": 6,
                                "desc": "定点打击"
                            },
                            "comments_0_7": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "2;3",
                                    "110"
                                ],
                                "id": "comments_0_7",
                                "sortId": 7,
                                "desc": "运筹帷幄"
                            },
                            "comments_0_8": {
                                "condition": "PassWithDimensionScoreMore",
                                "paramList": [
                                    "4;5",
                                    "110"
                                ],
                                "id": "comments_0_8",
                                "sortId": 8,
                                "desc": "制宜应变"
                            },
                            "comments_0_9": {
                                "condition": "PassWithDeployLess",
                                "paramList": [
                                    "9"
                                ],
                                "id": "comments_0_9",
                                "sortId": 9,
                                "desc": "择能善任"
                            },
                            "comments_0_10": {
                                "condition": "PassWithDeployMore",
                                "paramList": [
                                    "20"
                                ],
                                "id": "comments_0_10",
                                "sortId": 10,
                                "desc": "多多益善"
                            },
                            "comments_0_11": {
                                "condition": "PassWithDeployMore",
                                "paramList": [
                                    "30"
                                ],
                                "id": "comments_0_11",
                                "sortId": 11,
                                "desc": "人海战术"
                            },
                            "comments_0_12": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "SNIPER",
                                    "4"
                                ],
                                "id": "comments_0_12",
                                "sortId": 12,
                                "desc": "狙击作战 专家"
                            },
                            "comments_0_13": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "CASTER",
                                    "4"
                                ],
                                "id": "comments_0_13",
                                "sortId": 13,
                                "desc": "法术作战专家"
                            },
                            "comments_0_14": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "WARRIOR",
                                    "4"
                                ],
                                "id": "comments_0_14",
                                "sortId": 14,
                                "desc": "近身作战专家"
                            },
                            "comments_0_15": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "SUPPORT",
                                    "4"
                                ],
                                "id": "comments_0_15",
                                "sortId": 15,
                                "desc": "支援作战专家"
                            },
                            "comments_0_16": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "PIONEER",
                                    "4"
                                ],
                                "id": "comments_0_16",
                                "sortId": 16,
                                "desc": "突击作战专家"
                            },
                            "comments_0_17": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "SPECIAL",
                                    "4"
                                ],
                                "id": "comments_0_17",
                                "sortId": 17,
                                "desc": "特种作战专家"
                            },
                            "comments_0_18": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "MEDIC",
                                    "4"
                                ],
                                "id": "comments_0_18",
                                "sortId": 18,
                                "desc": "医疗援助专家"
                            },
                            "comments_0_19": {
                                "condition": "PassWithSpecProf",
                                "paramList": [
                                    "TANK",
                                    "4"
                                ],
                                "id": "comments_0_19",
                                "sortId": 19,
                                "desc": "重装作战专家"
                            },
                            "comments_0_20": {
                                "condition": "PassWithEventLess",
                                "paramList": [
                                    "COST",
                                    "10"
                                ],
                                "id": "comments_0_20",
                                "sortId": 20,
                                "desc": "勤俭持家"
                            },
                            "comments_0_21": {
                                "condition": "PassWithAttrExistMore",
                                "paramList": [
                                    "COST",
                                    "90"
                                ],
                                "id": "comments_0_21",
                                "sortId": 21,
                                "desc": "功勋之人"
                            },
                            "comments_0_22": {
                                "condition": "PassWithAttrAllLess",
                                "paramList": [
                                    "HP_RATIO",
                                    "0.01"
                                ],
                                "id": "comments_0_22",
                                "sortId": 22,
                                "desc": "生死一线"
                            }
                        },
                        "challengeNodeDataMap": {
                            "keypoint_1": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "300"
                                ],
                                "slotId": "keypoint_1",
                                "previewTitle": "进阶作战",
                                "previewDesc": "完成一次作战，同时评分大于等于300分",
                                "missionSortId": 1,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 300,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_2": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "400"
                                ],
                                "slotId": "keypoint_2",
                                "previewTitle": "巅峰作战",
                                "previewDesc": "完成一次作战，同时评分大于等于400分",
                                "missionSortId": 2,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 300,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_3": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "540"
                                ],
                                "slotId": "keypoint_3",
                                "previewTitle": "极限作战",
                                "previewDesc": "完成一次作战，同时评分大于等于540分",
                                "missionSortId": 3,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 500,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            }
                        },
                        "groupDescDataMap": {
                            "char_atk": {
                                "desc": "所有我方单位的攻击力<@crisisv2.nag>{atk:0%}</>",
                                "sortId": 10
                            },
                            "skill_cost_slow_1": {
                                "desc": "近卫和狙击干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                "sortId": 23
                            },
                            "skill_cost_slow_2": {
                                "desc": "特种和术师干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                "sortId": 24
                            },
                            "skill_cost_block_1": {
                                "desc": "禁止使用 近卫和狙击干员",
                                "sortId": 20
                            },
                            "skill_cost_block_2": {
                                "desc": "禁止使用特种和术师干员",
                                "sortId": 21
                            },
                            "char_respawntime_1": {
                                "desc": "我方所有单位再部署时间延长<@crisisv2.nag>75%</>",
                                "sortId": 22
                            },
                            "group_char_hp": {
                                "desc": "所有我方单位的最大生命值<@crisisv2.nag>{max_hp:0%}</>",
                                "sortId": 11
                            },
                            "g_char_def": {
                                "desc": "所有我方单位的防御力<@crisisv2.nag>{def}</>",
                                "sortId": 80
                            },
                            "char_blockcnt_1": {
                                "desc": "所有我方单位的阻挡数<@crisisv2.nag>{value}</>",
                                "sortId": 85
                            },
                            "group_global_costrecovery_1": {
                                "desc": "部署费用的自然回复速度降低<@crisisv2.nag>25%</>",
                                "sortId": 34
                            },
                            "gourp_global_costrecovery_2": {
                                "desc": "部署费用的自然回复速度降低<@crisisv2.nag>50%</>",
                                "sortId": 31
                            },
                            "gourp_global_costrecovery_3": {
                                "desc": "部署费用的自然回复速度降低<@crisisv2.nag>75%</>",
                                "sortId": 30
                            },
                            "group_dusrch": {
                                "desc": "<重型沼泽探照车>的攻击力<@crisisv2.nag>+{atk:0%}</>，生命值<@crisisv2.nag>+{max_hp:0%}</>，进入目标点额外损失<@crisisv2.nag>{life_point}</>点目标生命",
                                "sortId": 60
                            },
                            "dusrch_def": {
                                "desc": "<重型沼泽探照车>的防御力<@crisisv2.nag>+{def}</>",
                                "sortId": 65
                            },
                            "group_dusrch_speed": {
                                "desc": "<重型沼泽探照车>的移动速度<@crisisv2.nag>+100%</>，并且不可阻挡",
                                "sortId": 62
                            },
                            "group_dusrch_invisible_1": {
                                "desc": "<重型沼泽探照车>获得隐匿，攻击速度<@crisisv2.nag>+{attack_speed}</>，法术抗性<@crisisv2.nag>+{magic_resistance}</>",
                                "sortId": 61
                            },
                            "dusrch_start": {
                                "desc": "地图中开始出现<重型沼泽探照车>",
                                "sortId": 64
                            },
                            "group_enemy_atk": {
                                "desc": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>",
                                "sortId": 1
                            },
                            "group_enemy_hp": {
                                "desc": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>",
                                "sortId": 2
                            },
                            "group_enemy_def": {
                                "desc": "所有敌人的防御力<@ba.grt>+{def}</>",
                                "sortId": 82
                            },
                            "group_lunmag_1": {
                                "desc": "场上初始存在<特战术士组长>，并且<特战 术士组长>，生命值提升至<@crisisv2.nag>{max_hp:0%}</>，攻击范围扩大",
                                "sortId": 81
                            },
                            "group_dusocr_1": {
                                "desc": "<深池塑能术士>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，法术抗性<@crisisv2.nag>+{magic_resistance}</>，获得隐匿且<净浊之焰>的伤害提 升至<@crisisv2.nag>130%</>",
                                "sortId": 84
                            },
                            "group_dusrch_left_1": {
                                "desc": "地图左侧出现额外的<深池塑能术士>",
                                "sortId": 52
                            },
                            "group_dusrch_right_2": {
                                "desc": "地图右侧初始出现<深池塑能术士>",
                                "sortId": 51
                            },
                            "group_global_lifepoint_1": {
                                "desc": "我方防御点可承受的敌方数量变为<@crisisv2.nag>1</>",
                                "sortId": 50
                            },
                            "group_melee_forbid_1": {
                                "desc": "地图中的<@crisisv2.nag>8</>个地 面位置无法部署",
                                "sortId": 54
                            },
                            "group_range_forbid_2": {
                                "desc": "地图中的<@crisisv2.nag>7</>个高台位置无法部署",
                                "sortId": 53
                            },
                            "group_global_squadnum_1": {
                                "desc": "最多可编入<@crisisv2.nag>10</>名干员进入作战",
                                "sortId": 36
                            },
                            "group_global_pcharnum_1": {
                                "desc": "可同时部署的单位数量减少至<@crisisv2.nag>7</>",
                                "sortId": 35
                            },
                            "group_global_squadnum_2": {
                                "desc": "最多可编入<@crisisv2.nag>7</>名干员进入作战",
                                "sortId": 32
                            },
                            "group_global_pcharnum_2": {
                                "desc": "可同时部署的单位数量减少至<@crisisv2.nag>5</>",
                                "sortId": 33
                            },
                            "group_reedf_skill_1": {
                                "desc": "部署在<芦苇丛>中的干员自然回复技力速度下降<@crisisv2.nag>50%</>",
                                "sortId": 71
                            },
                            "group_reedf_skill_2": {
                                "desc": "部署在<芦苇丛>中的干员自然回复技力速度下降<@crisisv2.nag>90%</>",
                                "sortId": 70
                            },
                            "group_reedf_duskls": {
                                "desc": "地图右侧的芦苇荡中的<特别行动队士兵>变成<深池逐火战士>",
                                "sortId": 74
                            },
                            "group_duskls_tal1": {
                                "desc": "<深池逐火战士>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，所有敌人攻击速度<@crisisv2.nag>+{attack_speed}</>，<怨恨的 余烬>需要<@crisisv2.nag>10</>次攻击才能击倒",
                                "sortId": 73
                            },
                            "group_tile_reed": {
                                "desc": "地图右侧的一处芦苇荡初始处于点燃状 态，并且地块燃烧造成的伤害提高<@crisisv2.nag>100%</>",
                                "sortId": 72
                            },
                            "group_wdarft_1": {
                                "desc": "<枯朽萃聚使徒>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，法术抗性提升<@crisisv2.nag>{magic_resistance}</>，防御力提升<@crisisv2.nag>{def}</>",
                                "sortId": 40
                            },
                            "group_wdarft_3": {
                                "desc": "<枯朽萃聚使徒>获得<@crisisv2.nag>隐匿</>",
                                "sortId": 41
                            },
                            "group_nhfly_1": {
                                "desc": "<枯朽 之种>攻击力提升至<@crisisv2.nag>{atk:0%}</>",
                                "sortId": 44
                            },
                            "g_wdarft_1": {
                                "desc": "地图中开始出现<枯朽萃聚使徒>",
                                "sortId": 45
                            }
                        },
                        "roadRelationDataMap": {
                            "road_2": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_1": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_0": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_2"
                                }
                            },
                            "road_3": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_5"
                                }
                            },
                            "road_4": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_1"
                                }
                            },
                            "road_8": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_9"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_10"
                                }
                            },
                            "road_6": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_11"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_12"
                                }
                            },
                            "road_7": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_5": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_9": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_14"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_10": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_11": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_16"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_12": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_35": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_20"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_45"
                                }
                            },
                            "road_17": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_7"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_58"
                                }
                            },
                            "road_18": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_58"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_22": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_23"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_59"
                                }
                            },
                            "road_24": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_24"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_26"
                                }
                            },
                            "road_23": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_24"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            },
                            "road_26": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_26"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_27"
                                }
                            },
                            "road_25": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_25"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_27"
                                }
                            },
                            "road_27": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_27"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_28"
                                }
                            },
                            "road_28": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_5"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_3"
                                }
                            },
                            "road_55": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_31"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_33"
                                }
                            },
                            "road_29": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_49"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_32"
                                }
                            },
                            "road_51": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_31"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_32"
                                }
                            },
                            "road_32": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_31"
                                }
                            },
                            "road_38": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_39"
                                }
                            },
                            "road_34": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_1"
                                }
                            },
                            "road_36": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_3"
                                }
                            },
                            "road_33": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_38"
                                }
                            },
                            "road_37": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_37"
                                }
                            },
                            "road_39": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_37"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                }
                            },
                            "road_40": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_38"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                }
                            },
                            "road_41": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_39"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                }
                            },
                            "road_45": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_43"
                                }
                            },
                            "road_44": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_40"
                                }
                            },
                            "road_43": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_41"
                                }
                            },
                            "road_42": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_42"
                                }
                            },
                            "road_46": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_34"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_2"
                                }
                            },
                            "road_52": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_7"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "reward_2"
                                }
                            },
                            "road_48": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_44"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_46"
                                }
                            },
                            "road_19": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_31": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_32"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_34"
                                }
                            },
                            "road_49": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_49"
                                }
                            },
                            "road_53": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_45"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_47"
                                }
                            },
                            "road_20": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_22"
                                }
                            },
                            "road_56": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_3"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_34"
                                }
                            },
                            "road_57": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_1"
                                }
                            },
                            "road_58": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_2"
                                }
                            },
                            "road_59": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_51"
                                }
                            },
                            "road_61": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_53"
                                }
                            },
                            "road_63": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_55"
                                }
                            },
                            "road_65": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_57"
                                }
                            },
                            "road_66": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_3"
                                }
                            },
                            "road_67": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_23"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_5"
                                }
                            },
                            "road_68": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_7"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_4"
                                }
                            },
                            "road_69": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_6"
                                }
                            },
                            "road_70": {
                                "start": {
                                    "type": "BAG",
                                    "id": "pack_4"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_7"
                                }
                            },
                            "road_71": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "BAG",
                                    "id": "pack_8"
                                }
                            },
                            "road_13": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_14": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_15": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_16"
                                }
                            },
                            "road_16": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_17"
                                }
                            },
                            "road_54": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_47"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_48"
                                }
                            },
                            "road_50": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_46"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_48"
                                }
                            },
                            "road_21": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_20"
                                }
                            },
                            "road_74": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_59"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_30": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_49"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_33"
                                }
                            },
                            "road_60": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_20"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_44"
                                }
                            },
                            "road_62": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_50"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_52"
                                }
                            },
                            "road_64": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_5"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                }
                            },
                            "road_72": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                }
                            },
                            "road_73": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_10"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                }
                            },
                            "road_75": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                }
                            },
                            "road_47": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_20"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_0"
                                }
                            }
                        },
                        "bagRoadDataMap": {
                            "road_70": {
                                "nodeRoadList": [
                                    "road_35",
                                    "road_60"
                                ]
                            },
                            "road_56": {
                                "nodeRoadList": [
                                    "road_38",
                                    "road_33",
                                    "road_37"
                                ]
                            }
                        },
                        "nodeViewData": {
                            "width": 885,
                            "height": 2470,
                            "bagPosMap": {
                                "pack_1": {
                                    "pos": {
                                        "x": 240,
                                        "y": -232.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 225
                                    }
                                },
                                "pack_2": {
                                    "pos": {
                                        "x": 240,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 225
                                    }
                                },
                                "pack_3": {
                                    "pos": {
                                        "x": 225,
                                        "y": -847.5
                                    },
                                    "size": {
                                        "x": 270,
                                        "y": 435
                                    }
                                },
                                "pack_4": {
                                    "pos": {
                                        "x": 225,
                                        "y": -1200
                                    },
                                    "size": {
                                        "x": 270,
                                        "y": 240
                                    }
                                },
                                "pack_5": {
                                    "pos": {
                                        "x": 390,
                                        "y": -1507.5
                                    },
                                    "size": {
                                        "x": 600,
                                        "y": 285
                                    }
                                },
                                "pack_6": {
                                    "pos": {
                                        "x": 390,
                                        "y": -1807.5
                                    },
                                    "size": {
                                        "x": 600,
                                        "y": 285
                                    }
                                },
                                "pack_34": {
                                    "pos": {
                                        "x": 540,
                                        "y": -847.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 435
                                    }
                                },
                                "pack_7": {
                                    "pos": {
                                        "x": 540,
                                        "y": -1200
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 240
                                    }
                                },
                                "pack_8": {
                                    "pos": {
                                        "x": 390,
                                        "y": -2122.5
                                    },
                                    "size": {
                                        "x": 600,
                                        "y": 255
                                    }
                                }
                            },
                            "roadPosMap": {
                                "road_2": {
                                    "id": "road_2",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_1": {
                                    "id": "road_1",
                                    "centerPos": {
                                        "x": 300,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_0": {
                                    "id": "road_0",
                                    "centerPos": {
                                        "x": 142.5,
                                        "y": -262.5
                                    },
                                    "size": {
                                        "x": 174,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -82.5,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 82.5,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -37.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -37.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_3": {
                                    "id": "road_3",
                                    "centerPos": {
                                        "x": 209.634247,
                                        "y": -298.9962
                                    },
                                    "size": {
                                        "x": 40.14691,
                                        "y": 9.430603
                                    },
                                    "startPos": {
                                        "x": -15.5734558,
                                        "y": 0.215301514
                                    },
                                    "endPos": {
                                        "x": 15.5734558,
                                        "y": -0.215301514
                                    },
                                    "inflectionList": []
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 354,
                                        "y": 128.999985
                                    },
                                    "startPos": {
                                        "x": -172.5,
                                        "y": 59.9999847
                                    },
                                    "endPos": {
                                        "x": 172.5,
                                        "y": -60
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": 60
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": -60
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_8": {
                                    "id": "road_8",
                                    "centerPos": {
                                        "x": 300,
                                        "y": -465
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_6": {
                                    "id": "road_6",
                                    "centerPos": {
                                        "x": 210,
                                        "y": -540
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_7": {
                                    "id": "road_7",
                                    "centerPos": {
                                        "x": 142.5,
                                        "y": -465
                                    },
                                    "size": {
                                        "x": 173.999985,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -82.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 82.4999847,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_5": {
                                    "id": "road_5",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -502.5
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_9": {
                                    "id": "road_9",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_10": {
                                    "id": "road_10",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_11": {
                                    "id": "road_11",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_12": {
                                    "id": "road_12",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": -0.0000610351563
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_35": {
                                    "id": "road_35",
                                    "centerPos": {
                                        "x": 382.5,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_17": {
                                    "id": "road_17",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 54.0000038,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5000038,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_18": {
                                    "id": "road_18",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_19": {
                                    "id": "road_19",
                                    "centerPos": {
                                        "x": 165,
                                        "y": -2145
                                    },
                                    "size": {
                                        "x": 219,
                                        "y": 129
                                    },
                                    "startPos": {
                                        "x": -105,
                                        "y": 60
                                    },
                                    "endPos": {
                                        "x": 105,
                                        "y": -60
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 75,
                                                "y": 60
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 75,
                                                "y": -60
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_22": {
                                    "id": "road_22",
                                    "centerPos": {
                                        "x": 82.63687,
                                        "y": -1545.1012
                                    },
                                    "size": {
                                        "x": 54.2737427,
                                        "y": 9.202393
                                    },
                                    "startPos": {
                                        "x": -22.6368713,
                                        "y": 0.101196289
                                    },
                                    "endPos": {
                                        "x": 22.6368713,
                                        "y": -0.101196289
                                    },
                                    "inflectionList": []
                                },
                                "road_24": {
                                    "id": "road_24",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_23": {
                                    "id": "road_23",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_26": {
                                    "id": "road_26",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_25": {
                                    "id": "road_25",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_27": {
                                    "id": "road_27",
                                    "centerPos": {
                                        "x": 480,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_28": {
                                    "id": "road_28",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -1485
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 159
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 75
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": -75
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 75
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -75
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_55": {
                                    "id": "road_55",
                                    "centerPos": {
                                        "x": 255,
                                        "y": -1867.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_29": {
                                    "id": "road_29",
                                    "centerPos": {
                                        "x": 255,
                                        "y": -1830
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -15
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 15
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -15
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 15
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_51": {
                                    "id": "road_51",
                                    "centerPos": {
                                        "x": 255,
                                        "y": -1830
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -15
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 15
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -15
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 15
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_32": {
                                    "id": "road_32",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -1845
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_38": {
                                    "id": "road_38",
                                    "centerPos": {
                                        "x": 360,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_34": {
                                    "id": "road_34",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -345.25
                                    },
                                    "size": {
                                        "x": 354,
                                        "y": 129.5
                                    },
                                    "startPos": {
                                        "x": -172.5,
                                        "y": -60.25
                                    },
                                    "endPos": {
                                        "x": 172.5,
                                        "y": 60.25
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": -59.75
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": 60.25
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_36": {
                                    "id": "road_36",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -1635
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 159
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": -75
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 75
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -75
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 75
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_33": {
                                    "id": "road_33",
                                    "centerPos": {
                                        "x": 360,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_37": {
                                    "id": "road_37",
                                    "centerPos": {
                                        "x": 360,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_39": {
                                    "id": "road_39",
                                    "centerPos": {
                                        "x": 517.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_40": {
                                    "id": "road_40",
                                    "centerPos": {
                                        "x": 517.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_41": {
                                    "id": "road_41",
                                    "centerPos": {
                                        "x": 517.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_45": {
                                    "id": "road_45",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_44": {
                                    "id": "road_44",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_43": {
                                    "id": "road_43",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_42": {
                                    "id": "road_42",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_46": {
                                    "id": "road_46",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -787.5
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 234
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 112.5
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": -112.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 112.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -112.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_52": {
                                    "id": "road_52",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -1012.5
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 234
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": -112.5
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 112.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -112.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 112.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_48": {
                                    "id": "road_48",
                                    "centerPos": {
                                        "x": 585,
                                        "y": -1185
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_31": {
                                    "id": "road_31",
                                    "centerPos": {
                                        "x": 360,
                                        "y": -1815
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_49": {
                                    "id": "road_49",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -1845
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_53": {
                                    "id": "road_53",
                                    "centerPos": {
                                        "x": 495,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_20": {
                                    "id": "road_20",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -1230
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": -15
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": 15
                                    },
                                    "inflectionList": []
                                },
                                "road_59": {
                                    "id": "road_59",
                                    "centerPos": {
                                        "x": 105,
                                        "y": -2107.5
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 15,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 15,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_61": {
                                    "id": "road_61",
                                    "centerPos": {
                                        "x": 165,
                                        "y": -2107.5
                                    },
                                    "size": {
                                        "x": 219,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -105,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 105,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 75,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 75,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_63": {
                                    "id": "road_63",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -2107.5
                                    },
                                    "size": {
                                        "x": 339,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -165,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 165,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 135,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 135,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_65": {
                                    "id": "road_65",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -2107.5
                                    },
                                    "size": {
                                        "x": 459,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -225,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 225,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 195,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 195,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_13": {
                                    "id": "road_13",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_14": {
                                    "id": "road_14",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_15": {
                                    "id": "road_15",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_16": {
                                    "id": "road_16",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": -0.0000610351563
                                    },
                                    "inflectionList": []
                                },
                                "road_54": {
                                    "id": "road_54",
                                    "centerPos": {
                                        "x": 585,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_50": {
                                    "id": "road_50",
                                    "centerPos": {
                                        "x": 630,
                                        "y": -1230
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 15
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -15
                                    },
                                    "inflectionList": []
                                },
                                "road_21": {
                                    "id": "road_21",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -1275
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_74": {
                                    "id": "road_74",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_30": {
                                    "id": "road_30",
                                    "centerPos": {
                                        "x": 255,
                                        "y": -1867.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_60": {
                                    "id": "road_60",
                                    "centerPos": {
                                        "x": 427.5,
                                        "y": -1230
                                    },
                                    "size": {
                                        "x": 174,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -82.5,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 82.5,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -52.5,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -52.5,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_62": {
                                    "id": "road_62",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -2145
                                    },
                                    "size": {
                                        "x": 459,
                                        "y": 129
                                    },
                                    "startPos": {
                                        "x": -225,
                                        "y": 60
                                    },
                                    "endPos": {
                                        "x": 225,
                                        "y": -60
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 195,
                                                "y": 60
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 195,
                                                "y": -60
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_64": {
                                    "id": "road_64",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_72": {
                                    "id": "road_72",
                                    "centerPos": {
                                        "x": 345,
                                        "y": -277.5
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -22.5
                                    },
                                    "inflectionList": []
                                },
                                "road_73": {
                                    "id": "road_73",
                                    "centerPos": {
                                        "x": 345,
                                        "y": -517.5
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -22.5
                                    },
                                    "inflectionList": []
                                },
                                "road_75": {
                                    "id": "road_75",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -540
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_47": {
                                    "id": "road_47",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -1230
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": -15
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": 15
                                    },
                                    "inflectionList": []
                                }
                            },
                            "nodePosMap": {
                                "node_1": {
                                    "position": {
                                        "x": 60,
                                        "y": -300
                                    }
                                },
                                "node_2": {
                                    "position": {
                                        "x": 255,
                                        "y": -225
                                    }
                                },
                                "node_3": {
                                    "position": {
                                        "x": 345,
                                        "y": -225
                                    }
                                },
                                "node_4": {
                                    "position": {
                                        "x": 165,
                                        "y": -300
                                    }
                                },
                                "node_5": {
                                    "position": {
                                        "x": 255,
                                        "y": -300
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 60,
                                        "y": -465
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 59.9999962,
                                        "y": -1275
                                    }
                                },
                                "reward_1": {
                                    "position": {
                                        "x": 795,
                                        "y": -285
                                    }
                                },
                                "node_9": {
                                    "position": {
                                        "x": 254.999985,
                                        "y": -465
                                    }
                                },
                                "node_10": {
                                    "position": {
                                        "x": 345,
                                        "y": -465
                                    }
                                },
                                "node_11": {
                                    "position": {
                                        "x": 165,
                                        "y": -540
                                    }
                                },
                                "node_12": {
                                    "position": {
                                        "x": 255,
                                        "y": -540
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 60,
                                        "y": -885
                                    }
                                },
                                "node_14": {
                                    "position": {
                                        "x": 180,
                                        "y": -780
                                    }
                                },
                                "node_15": {
                                    "position": {
                                        "x": 180,
                                        "y": -855
                                    }
                                },
                                "node_16": {
                                    "position": {
                                        "x": 180,
                                        "y": -930
                                    }
                                },
                                "node_17": {
                                    "position": {
                                        "x": 180,
                                        "y": -1005
                                    }
                                },
                                "node_18": {
                                    "position": {
                                        "x": 285,
                                        "y": -885
                                    }
                                },
                                "node_19": {
                                    "position": {
                                        "x": 225,
                                        "y": -1275
                                    }
                                },
                                "node_20": {
                                    "position": {
                                        "x": 315,
                                        "y": -1275
                                    }
                                },
                                "node_21": {
                                    "position": {
                                        "x": 300,
                                        "y": -2205
                                    }
                                },
                                "node_22": {
                                    "position": {
                                        "x": 225,
                                        "y": -1185
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 60,
                                        "y": -1545
                                    }
                                },
                                "node_24": {
                                    "position": {
                                        "x": 225,
                                        "y": -1545
                                    }
                                },
                                "node_25": {
                                    "position": {
                                        "x": 330,
                                        "y": -1515
                                    }
                                },
                                "node_26": {
                                    "position": {
                                        "x": 330,
                                        "y": -1590
                                    }
                                },
                                "node_27": {
                                    "position": {
                                        "x": 435,
                                        "y": -1545
                                    }
                                },
                                "node_28": {
                                    "position": {
                                        "x": 525,
                                        "y": -1545
                                    }
                                },
                                "reward_3": {
                                    "position": {
                                        "x": 795,
                                        "y": -1560
                                    }
                                },
                                "node_30": {
                                    "position": {
                                        "x": 60,
                                        "y": -1845
                                    }
                                },
                                "node_31": {
                                    "position": {
                                        "x": 180,
                                        "y": -1890
                                    }
                                },
                                "node_32": {
                                    "position": {
                                        "x": 315,
                                        "y": -1815
                                    }
                                },
                                "node_33": {
                                    "position": {
                                        "x": 315,
                                        "y": -1890
                                    }
                                },
                                "node_34": {
                                    "position": {
                                        "x": 405,
                                        "y": -1815
                                    }
                                },
                                "reward_2": {
                                    "position": {
                                        "x": 795,
                                        "y": -900
                                    }
                                },
                                "keypoint_1": {
                                    "position": {
                                        "x": 540,
                                        "y": -885
                                    }
                                },
                                "node_37": {
                                    "position": {
                                        "x": 450,
                                        "y": -810
                                    }
                                },
                                "node_38": {
                                    "position": {
                                        "x": 450,
                                        "y": -885
                                    }
                                },
                                "node_39": {
                                    "position": {
                                        "x": 450,
                                        "y": -960
                                    }
                                },
                                "node_40": {
                                    "position": {
                                        "x": 630,
                                        "y": -780
                                    }
                                },
                                "node_41": {
                                    "position": {
                                        "x": 630,
                                        "y": -855
                                    }
                                },
                                "node_42": {
                                    "position": {
                                        "x": 630,
                                        "y": -930
                                    }
                                },
                                "node_43": {
                                    "position": {
                                        "x": 630,
                                        "y": -1004.99994
                                    }
                                },
                                "node_44": {
                                    "position": {
                                        "x": 540,
                                        "y": -1185
                                    }
                                },
                                "node_45": {
                                    "position": {
                                        "x": 450,
                                        "y": -1275
                                    }
                                },
                                "node_46": {
                                    "position": {
                                        "x": 630,
                                        "y": -1185
                                    }
                                },
                                "node_47": {
                                    "position": {
                                        "x": 540,
                                        "y": -1275
                                    }
                                },
                                "node_48": {
                                    "position": {
                                        "x": 630,
                                        "y": -1275
                                    }
                                },
                                "node_49": {
                                    "position": {
                                        "x": 180,
                                        "y": -1815
                                    }
                                },
                                "node_50": {
                                    "position": {
                                        "x": 60,
                                        "y": -2085
                                    }
                                },
                                "node_53": {
                                    "position": {
                                        "x": 300,
                                        "y": -2130
                                    }
                                },
                                "node_55": {
                                    "position": {
                                        "x": 420,
                                        "y": -2130
                                    }
                                },
                                "node_57": {
                                    "position": {
                                        "x": 540,
                                        "y": -2130
                                    }
                                },
                                "node_58": {
                                    "position": {
                                        "x": 135,
                                        "y": -1275
                                    }
                                },
                                "node_59": {
                                    "position": {
                                        "x": 135,
                                        "y": -1545
                                    }
                                },
                                "node_51": {
                                    "position": {
                                        "x": 180,
                                        "y": -2130
                                    }
                                },
                                "node_52": {
                                    "position": {
                                        "x": 540,
                                        "y": -2205
                                    }
                                },
                                "keypoint_2": {
                                    "position": {
                                        "x": 345,
                                        "y": -300
                                    }
                                },
                                "keypoint_3": {
                                    "position": {
                                        "x": 345,
                                        "y": -540
                                    }
                                },
                                "node_0": {
                                    "position": {
                                        "x": 315,
                                        "y": -1185
                                    }
                                }
                            },
                            "exclusionDataMap": {
                                "exclude_1": {
                                    "id": "exclude_1",
                                    "pos": {
                                        "x": 180,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 330
                                    }
                                },
                                "exclude_2": {
                                    "id": "exclude_2",
                                    "pos": {
                                        "x": 330,
                                        "y": -1545
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                },
                                "exclude_5": {
                                    "id": "exclude_5",
                                    "pos": {
                                        "x": 180,
                                        "y": -1845
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                },
                                "exclude_3": {
                                    "id": "exclude_3",
                                    "pos": {
                                        "x": 450,
                                        "y": -877.5
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 255
                                    }
                                },
                                "exclude_4": {
                                    "id": "exclude_4",
                                    "pos": {
                                        "x": 630,
                                        "y": -885
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 330
                                    }
                                }
                            }
                        },
                        "bagViewData": {
                            "width": 885,
                            "height": 1255,
                            "treasurePosMap": {
                                "node_1": {
                                    "position": {
                                        "x": 60,
                                        "y": -180
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 60.0000038,
                                        "y": -300
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 60,
                                        "y": -570
                                    }
                                },
                                "reward_1": {
                                    "position": {
                                        "x": 795,
                                        "y": -240
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 60,
                                        "y": -720
                                    }
                                },
                                "reward_3": {
                                    "position": {
                                        "x": 795,
                                        "y": -780
                                    }
                                },
                                "node_30": {
                                    "position": {
                                        "x": 60,
                                        "y": -840
                                    }
                                },
                                "reward_2": {
                                    "position": {
                                        "x": 795,
                                        "y": -510
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 60,
                                        "y": -450
                                    }
                                },
                                "node_50": {
                                    "position": {
                                        "x": 60,
                                        "y": -990
                                    }
                                }
                            },
                            "bagPosMap": {
                                "pack_1": {
                                    "pos": {
                                        "x": 240,
                                        "y": -172.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 105
                                    }
                                },
                                "pack_2": {
                                    "pos": {
                                        "x": 240,
                                        "y": -292.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 105
                                    }
                                },
                                "pack_3": {
                                    "pos": {
                                        "x": 225,
                                        "y": -442.5
                                    },
                                    "size": {
                                        "x": 270,
                                        "y": 105
                                    }
                                },
                                "pack_4": {
                                    "pos": {
                                        "x": 225,
                                        "y": -562.5
                                    },
                                    "size": {
                                        "x": 270,
                                        "y": 105
                                    }
                                },
                                "pack_5": {
                                    "pos": {
                                        "x": 390,
                                        "y": -712.5
                                    },
                                    "size": {
                                        "x": 600,
                                        "y": 105
                                    }
                                },
                                "pack_6": {
                                    "pos": {
                                        "x": 390.000153,
                                        "y": -832.5
                                    },
                                    "size": {
                                        "x": 600.0003,
                                        "y": 105
                                    }
                                },
                                "pack_34": {
                                    "pos": {
                                        "x": 540,
                                        "y": -442.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 105
                                    }
                                },
                                "pack_7": {
                                    "pos": {
                                        "x": 540,
                                        "y": -562.5
                                    },
                                    "size": {
                                        "x": 300,
                                        "y": 105
                                    }
                                },
                                "pack_8": {
                                    "pos": {
                                        "x": 390,
                                        "y": -982.5
                                    },
                                    "size": {
                                        "x": 600,
                                        "y": 105
                                    }
                                }
                            },
                            "roadPosMap": {
                                "road_56": {
                                    "id": "road_56",
                                    "centerPos": {
                                        "x": 375,
                                        "y": -450
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_57": {
                                    "id": "road_57",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -180
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_58": {
                                    "id": "road_58",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 38.9999962,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -14.9999962,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_66": {
                                    "id": "road_66",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -450
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_67": {
                                    "id": "road_67",
                                    "centerPos": {
                                        "x": 75.0000153,
                                        "y": -720
                                    },
                                    "size": {
                                        "x": 39.00003,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15.0000153,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15.0000153,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_68": {
                                    "id": "road_68",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -570
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_69": {
                                    "id": "road_69",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -840
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_71": {
                                    "id": "road_71",
                                    "centerPos": {
                                        "x": 75,
                                        "y": -990
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_28": {
                                    "id": "road_28",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -750
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_36": {
                                    "id": "road_36",
                                    "centerPos": {
                                        "x": 712.5001,
                                        "y": -810
                                    },
                                    "size": {
                                        "x": 53.9996948,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -22.4998169,
                                        "y": -30
                                    },
                                    "endPos": {
                                        "x": 22.4998779,
                                        "y": 30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.500122,
                                                "y": -30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.500122,
                                                "y": 30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_52": {
                                    "id": "road_52",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -540
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": -30
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_46": {
                                    "id": "road_46",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -480
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_70": {
                                    "id": "road_70",
                                    "centerPos": {
                                        "x": 375,
                                        "y": -570
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -210
                                    },
                                    "size": {
                                        "x": 354,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -172.5,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 172.5,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_34": {
                                    "id": "road_34",
                                    "centerPos": {
                                        "x": 562.5,
                                        "y": -270
                                    },
                                    "size": {
                                        "x": 354,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -172.5,
                                        "y": -30
                                    },
                                    "endPos": {
                                        "x": 172.5,
                                        "y": 30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": -30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": 30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                }
                            }
                        },
                        "bagDataMap": {
                            "pack_1": {
                                "slotPackId": "pack_1",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "恶敌",
                                "slotPackFullName": "指标集：恶敌",
                                "isDaily": false,
                                "dimension": 0,
                                "previewTitle": "恶敌阻击作战",
                                "previewDesc": "完成【指标集：恶敌】",
                                "rewardScore": 10,
                                "sortId": 1,
                                "missionSortId": 1,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_2": {
                                "slotPackId": "pack_2",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "衰退",
                                "slotPackFullName": "指标集：衰退",
                                "isDaily": false,
                                "dimension": 3,
                                "previewTitle": "衰退适应作战",
                                "previewDesc": "完成【指标集：衰退】",
                                "rewardScore": 10,
                                "sortId": 2,
                                "missionSortId": 2,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_3": {
                                "slotPackId": "pack_3",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "缺失",
                                "slotPackFullName": "指标集：缺失",
                                "isDaily": false,
                                "dimension": 2,
                                "previewTitle": "缺失适应作战",
                                "previewDesc": "完成【指标集：缺失】",
                                "rewardScore": 5,
                                "sortId": 3,
                                "missionSortId": 3,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_4": {
                                "slotPackId": "pack_4",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "枯朽之影",
                                "slotPackFullName": "指标集：枯朽之影",
                                "isDaily": false,
                                "dimension": 1,
                                "previewTitle": "枯朽夷灭作战",
                                "previewDesc": "完成【指标集：枯朽之影】",
                                "rewardScore": 10,
                                "sortId": 5,
                                "missionSortId": 4,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 200,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_5": {
                                "slotPackId": "pack_5",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "沼泽行军",
                                "slotPackFullName": "指标集：沼泽行军",
                                "isDaily": false,
                                "dimension": 1,
                                "previewTitle": "沼泽抵御作战",
                                "previewDesc": "完成【指标集：沼泽行军】",
                                "rewardScore": 10,
                                "sortId": 7,
                                "missionSortId": 5,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_6": {
                                "slotPackId": "pack_6",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "野草",
                                "slotPackFullName": "指标集：野草",
                                "isDaily": false,
                                "dimension": 5,
                                "previewTitle": "野草跋涉作战",
                                "previewDesc": "完成【指标集：野草 】",
                                "rewardScore": 10,
                                "sortId": 8,
                                "missionSortId": 6,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_7": {
                                "slotPackId": "pack_7",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "险地",
                                "slotPackFullName": "指标集：险地",
                                "isDaily": false,
                                "dimension": 4,
                                "previewTitle": "险地安身作战",
                                "previewDesc": "完成【指标集：险地】",
                                "rewardScore": 10,
                                "sortId": 6,
                                "missionSortId": 7,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 200,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_34": {
                                "slotPackId": "pack_34",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "约束",
                                "slotPackFullName": "指标集：约束",
                                "isDaily": false,
                                "dimension": 2,
                                "previewTitle": "约束适应作战",
                                "previewDesc": "完成【指标集：约束】",
                                "rewardScore": 5,
                                "sortId": 4,
                                "missionSortId": 8,
                                "slotPackRewards": [
                                    {
                                        "id": "CRISIS_SHOP_COIN_V2",
                                        "count": 150,
                                        "type": "CRS_SHOP_COIN_V2"
                                    }
                                ]
                            },
                            "pack_8": {
                                "slotPackId": "pack_8",
                                "slotPackType": "HIGHEST_TOTAL_SCORE",
                                "mapId": "crisis_v2_01-01",
                                "mapSlotId": null,
                                "slotPackName": "应变",
                                "slotPackFullName": "指标集：应变",
                                "isDaily": true,
                                "dimension": 0,
                                "previewTitle": "应变淬炼作战",
                                "previewDesc": "完成【指标集：应变】",
                                "rewardScore": 0,
                                "sortId": 9,
                                "missionSortId": 9,
                                "slotPackRewards": []
                            }
                        },
                        "exclusionDataMap": {
                            "exclude_1": {
                                "defaultSlotId": "node_17"
                            },
                            "exclude_2": {
                                "defaultSlotId": "node_26"
                            },
                            "exclude_5": {
                                "defaultSlotId": "node_31"
                            },
                            "exclude_3": {
                                "defaultSlotId": "node_39"
                            },
                            "exclude_4": {
                                "defaultSlotId": "node_42"
                            }
                        },
                        "dimensionItemList": [
                            {
                                "desc": "战略",
                                "maxScore": 120
                            },
                            {
                                "desc": "技巧",
                                "maxScore": 220
                            },
                            {
                                "desc": "调遣",
                                "maxScore": 110
                            },
                            {
                                "desc": "应变",
                                "maxScore": 120
                            },
                            {
                                "desc": "洞察",
                                "maxScore": 100
                            },
                            {
                                "desc": "筹划",
                                "maxScore": 90
                            }
                        ],
                        "rewardNodeDataMap": {
                            "reward_1": {
                                "slotId": "reward_1",
                                "requestBagCnt": 2,
                                "goodId": "good_cs_1_1_skin1",
                                "previewTitle": "不对等作战",
                                "previewDesc": "完成一次作战，并同时携带【指标集：恶敌】和【指标集：衰退】",
                                "missionListTitle": null,
                                "missionListDesc": null,
                                "missionSortId": 1,
                                "reward": {
                                    "id": "char_131_flameb@whirlwind#6",
                                    "count": 1,
                                    "type": "CHAR_SKIN"
                                },
                                "requestBagIdList": [
                                    "pack_1",
                                    "pack_2"
                                ]
                            },
                            "reward_2": {
                                "slotId": "reward_2",
                                "requestBagCnt": 2,
                                "goodId": "good_cs_1_1_furniture1",
                                "previewTitle": "不良状况作战",
                                "previewDesc": "完成一次作战，并同时携带【指标集：约束】和【指标集：险地】",
                                "missionListTitle": null,
                                "missionListDesc": null,
                                "missionSortId": 2,
                                "reward": {
                                    "id": "furni_contract2_s1_01",
                                    "count": 1,
                                    "type": "FURN"
                                },
                                "requestBagIdList": [
                                    "pack_34",
                                    "pack_7"
                                ]
                            },
                            "reward_3": {
                                "slotId": "reward_3",
                                "requestBagCnt": 2,
                                "goodId": null,
                                "previewTitle": " 不利环境作战",
                                "previewDesc": "完成一次作战，并同时携带【指标集：沼泽行军】和【指标集：野草】",
                                "missionListTitle": null,
                                "missionListDesc": null,
                                "missionSortId": 3,
                                "reward": {
                                    "id": "itempack_mod_4",
                                    "count": 1,
                                    "type": "ITEM_PACK"
                                },
                                "requestBagIdList": [
                                    "pack_5",
                                    "pack_6"
                                ]
                            }
                        },
                        "dailyRuneUnlcokDataMap": {
                            "node_51": {
                                "startTs": 1700553600,
                                "mapAttachOn": "crisis_v2_01-02"
                            },
                            "node_21": {
                                "startTs": 1700856000,
                                "mapAttachOn": "crisis_v2_01-03"
                            },
                            "node_53": {
                                "startTs": 1700856000,
                                "mapAttachOn": "crisis_v2_01-03"
                            },
                            "node_55": {
                                "startTs": 1701115200,
                                "mapAttachOn": "crisis_v2_01-05"
                            },
                            "node_52": {
                                "startTs": 1701374400,
                                "mapAttachOn": "crisis_v2_01-07"
                            },
                            "node_57": {
                                "startTs": 1701374400,
                                "mapAttachOn": "crisis_v2_01-07"
                            }
                        },
                        "nodeDataMap": {
                            "node_1": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_2"
                                ]
                            },
                            "node_2": {
                                "runeId": "enemy_atk_1",
                                "slotPackId": "pack_1",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_1"
                                ]
                            },
                            "node_3": {
                                "runeId": "enemy_atk_2",
                                "slotPackId": "pack_1",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_2",
                                    "keypoint_2"
                                ]
                            },
                            "node_4": {
                                "runeId": "enemy_hp_1",
                                "slotPackId": "pack_1",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1",
                                    "node_5"
                                ]
                            },
                            "node_5": {
                                "runeId": "enemy_hp_2",
                                "slotPackId": "pack_1",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4",
                                    "keypoint_2"
                                ]
                            },
                            "node_6": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_9",
                                    "node_11"
                                ]
                            },
                            "node_7": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_58"
                                ]
                            },
                            "reward_3": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "TREASURE",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": []
                            },
                            "node_9": {
                                "runeId": "char_atk_1",
                                "slotPackId": "pack_2",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_10",
                                    "node_6"
                                ]
                            },
                            "node_10": {
                                "runeId": "char_atk_2",
                                "slotPackId": "pack_2",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_9",
                                    "keypoint_3"
                                ]
                            },
                            "node_11": {
                                "runeId": "char_hp_1",
                                "slotPackId": "pack_2",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_6"
                                ]
                            },
                            "node_12": {
                                "runeId": "char_hp_2",
                                "slotPackId": "pack_2",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_11",
                                    "keypoint_3"
                                ]
                            },
                            "node_13": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_14",
                                    "node_15",
                                    "node_16",
                                    "node_17"
                                ]
                            },
                            "node_14": {
                                "runeId": "skill_cost_slow_1",
                                "slotPackId": "pack_3",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_1",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_13"
                                ]
                            },
                            "node_15": {
                                "runeId": "skill_cost_slow_2",
                                "slotPackId": "pack_3",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_1",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_13"
                                ]
                            },
                            "node_16": {
                                "runeId": "skill_cost_block_1",
                                "slotPackId": "pack_3",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_1",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_13"
                                ]
                            },
                            "node_17": {
                                "runeId": "skill_cost_block_2",
                                "slotPackId": "pack_3",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_1",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_13"
                                ]
                            },
                            "node_18": {
                                "runeId": "char_respawntime_1",
                                "slotPackId": "pack_3",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_14",
                                    "node_15",
                                    "node_16",
                                    "node_17",
                                    "node_39",
                                    "node_38",
                                    "node_37"
                                ]
                            },
                            "node_19": {
                                "runeId": "wdarft_1",
                                "slotPackId": "pack_4",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_58",
                                    "node_22",
                                    "node_20"
                                ]
                            },
                            "node_20": {
                                "runeId": "nhfly_1",
                                "slotPackId": "pack_4",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_45",
                                    "node_19",
                                    "node_44",
                                    "node_0"
                                ]
                            },
                            "node_21": {
                                "runeId": "enemy_def",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            },
                            "node_22": {
                                "runeId": "wdarft_2",
                                "slotPackId": "pack_4",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_19"
                                ]
                            },
                            "node_23": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_59"
                                ]
                            },
                            "node_24": {
                                "runeId": "dusrch_1",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_26",
                                    "node_25",
                                    "node_59"
                                ]
                            },
                            "node_25": {
                                "runeId": "dusrch_speed_1",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_2",
                                "adjacentNodeList": [
                                    "node_24",
                                    "node_27"
                                ]
                            },
                            "node_26": {
                                "runeId": "dusrch_invisible_1",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_2",
                                "adjacentNodeList": [
                                    "node_24",
                                    "node_27"
                                ]
                            },
                            "node_27": {
                                "runeId": "dusrch_2",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_26",
                                    "node_25",
                                    "node_28"
                                ]
                            },
                            "node_28": {
                                "runeId": "dusrch_3",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_27"
                                ]
                            },
                            "node_30": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_31",
                                    "node_49"
                                ]
                            },
                            "node_31": {
                                "runeId": "reedf_skill_2",
                                "slotPackId": "pack_6",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_5",
                                "adjacentNodeList": [
                                    "node_33",
                                    "node_32",
                                    "node_30"
                                ]
                            },
                            "node_32": {
                                "runeId": "reedf_duskls",
                                "slotPackId": "pack_6",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_49",
                                    "node_31",
                                    "node_34"
                                ]
                            },
                            "node_33": {
                                "runeId": "map_tile_reed1",
                                "slotPackId": "pack_6",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_31",
                                    "node_49"
                                ]
                            },
                            "node_34": {
                                "runeId": "duskls_tal1",
                                "slotPackId": "pack_6",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "reward_2": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "TREASURE",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": []
                            },
                            "keypoint_1": {
                                "runeId": null,
                                "slotPackId": "pack_34",
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_37",
                                    "node_38",
                                    "node_39",
                                    "node_43",
                                    "node_40",
                                    "node_41",
                                    "node_42"
                                ]
                            },
                            "node_37": {
                                "runeId": "global_costrecovery_1",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_3",
                                "adjacentNodeList": [
                                    "node_18",
                                    "keypoint_1"
                                ]
                            },
                            "node_38": {
                                "runeId": "global_costrecovery_2",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_3",
                                "adjacentNodeList": [
                                    "node_18",
                                    "keypoint_1"
                                ]
                            },
                            "node_39": {
                                "runeId": "global_costrecovery_3",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_3",
                                "adjacentNodeList": [
                                    "node_18",
                                    "keypoint_1"
                                ]
                            },
                            "node_40": {
                                "runeId": "global_squadnum_1",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_4",
                                "adjacentNodeList": [
                                    "keypoint_1"
                                ]
                            },
                            "node_41": {
                                "runeId": "global_pcharnum_1",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_4",
                                "adjacentNodeList": [
                                    "keypoint_1"
                                ]
                            },
                            "node_42": {
                                "runeId": "global_squadnum_2",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_4",
                                "adjacentNodeList": [
                                    "keypoint_1"
                                ]
                            },
                            "node_43": {
                                "runeId": "global_pcharnum_2",
                                "slotPackId": "pack_34",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_4",
                                "adjacentNodeList": [
                                    "keypoint_1"
                                ]
                            },
                            "node_44": {
                                "runeId": "melee_forbid_1",
                                "slotPackId": "pack_7",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_46",
                                    "node_20"
                                ]
                            },
                            "node_45": {
                                "runeId": "range_forbid_2",
                                "slotPackId": "pack_7",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_20",
                                    "node_47"
                                ]
                            },
                            "node_46": {
                                "runeId": "dusrch_left_1",
                                "slotPackId": "pack_7",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_44",
                                    "node_48"
                                ]
                            },
                            "node_47": {
                                "runeId": "dusrch_right_2",
                                "slotPackId": "pack_7",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_45",
                                    "node_48"
                                ]
                            },
                            "node_48": {
                                "runeId": "global_lifepoint_1",
                                "slotPackId": "pack_7",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_47",
                                    "node_46"
                                ]
                            },
                            "node_49": {
                                "runeId": "reedf_skill_1",
                                "slotPackId": "pack_6",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclude_5",
                                "adjacentNodeList": [
                                    "node_32",
                                    "node_30",
                                    "node_33"
                                ]
                            },
                            "node_50": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_21",
                                    "node_51",
                                    "node_53",
                                    "node_55",
                                    "node_57",
                                    "node_52"
                                ]
                            },
                            "node_51": {
                                "runeId": "char_def_1",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            },
                            "node_53": {
                                "runeId": "lunmag_1",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            },
                            "node_55": {
                                "runeId": "enemy_hp_3",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            },
                            "node_57": {
                                "runeId": "dusocr_1",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            },
                            "node_58": {
                                "runeId": "wdarft_start",
                                "slotPackId": "pack_4",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_7",
                                    "node_19"
                                ]
                            },
                            "node_59": {
                                "runeId": "dusrch_start",
                                "slotPackId": "pack_5",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_23",
                                    "node_24"
                                ]
                            },
                            "reward_1": {
                                "runeId": " ",
                                "slotPackId": null,
                                "nodeType": "TREASURE",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": []
                            },
                            "keypoint_2": {
                                "runeId": null,
                                "slotPackId": "pack_1",
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_5",
                                    "node_3"
                                ]
                            },
                            "keypoint_3": {
                                "runeId": null,
                                "slotPackId": "pack_2",
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_10",
                                    "node_12"
                                ]
                            },
                            "node_0": {
                                "runeId": "wdarft_3",
                                "slotPackId": "pack_4",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_20"
                                ]
                            },
                            "node_52": {
                                "runeId": "char_blockcnt_1",
                                "slotPackId": "pack_8",
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_50"
                                ]
                            }
                        },
                        "runeDataMap": {
                            "enemy_atk_1": {
                                "runeId": "enemy_atk_1",
                                "runeGroupId": "group_enemy_atk",
                                "runeIcon": "g_enemy_atk_1",
                                "runeName": "源石环境：刺激I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 3
                            },
                            "enemy_atk_2": {
                                "runeId": "enemy_atk_2",
                                "runeGroupId": "group_enemy_atk",
                                "runeIcon": "g_enemy_atk_2",
                                "runeName": "源石环境：刺激II",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.6
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 1
                            },
                            "enemy_hp_1": {
                                "runeId": "enemy_hp_1",
                                "runeGroupId": "group_enemy_hp",
                                "runeIcon": "g_enemy_hp_1",
                                "runeName": "源石环境：活性I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hpdef",
                                    "description": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 4
                            },
                            "enemy_hp_2": {
                                "runeId": "enemy_hp_2",
                                "runeGroupId": "group_enemy_hp",
                                "runeIcon": "g_enemy_hp_2",
                                "runeName": "源石环境：活性II",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hpdef",
                                    "description": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.9
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 2
                            },
                            "char_atk_1": {
                                "runeId": "char_atk_1",
                                "runeGroupId": "char_atk",
                                "runeIcon": "g_char_atk_1",
                                "runeName": "环境：高价值目标I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_atk",
                                    "description": "所有我方单位的攻击力<@crisisv2.nag>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": -0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 12
                            },
                            "char_atk_2": {
                                "runeId": "char_atk_2",
                                "runeGroupId": "char_atk",
                                "runeIcon": "g_char_atk_2",
                                "runeName": "环境：高价值目标II",
                                "score": 30,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_atk_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_atk",
                                    "description": "所有我方单位的攻击力<@crisisv2.nag>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": -0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 10
                            },
                            "char_hp_1": {
                                "runeId": "char_hp_1",
                                "runeGroupId": "group_char_hp",
                                "runeIcon": "g_char_hp_1",
                                "runeName": "源石环境：侵蚀I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_maxhpdef",
                                    "description": "所有我方单位的最大生命值<@crisisv2.nag>{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 13
                            },
                            "char_hp_2": {
                                "runeId": "char_hp_2",
                                "runeGroupId": "group_char_hp",
                                "runeIcon": "g_char_hp_2",
                                "runeName": "源石环境：侵蚀II",
                                "score": 30,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_maxhpdef",
                                    "description": "所有我方单位的最大生命值<@crisisv2.nag>{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 11
                            },
                            "skill_cost_slow_1": {
                                "runeId": "skill_cost_slow_1",
                                "runeGroupId": "skill_cost_slow_1",
                                "runeIcon": "g_skill_cost_slow_1",
                                "runeName": "目标：应急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "skill_cost_slow_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "近卫和狙击干员的部 署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 3,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 23
                            },
                            "skill_cost_slow_2": {
                                "runeId": "skill_cost_slow_2",
                                "runeGroupId": "skill_cost_slow_2",
                                "runeIcon": "g_skill_cost_slow_2",
                                "runeName": "目标：应急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "skill_cost_slow_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "特种和术师干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 96,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 24
                            },
                            "skill_cost_block_1": {
                                "runeId": "skill_cost_block_1",
                                "runeGroupId": "skill_cost_block_1",
                                "runeIcon": "g_skill_cost_block_1",
                                "runeName": "目标：战略调度",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "skill_cost_block_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "禁止使用近卫和狙击干员",
                                    "runes": [
                                        {
                                            "key": "char_exclude",
                                            "selector": {
                                                "professionMask": 3,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": []
                                        }
                                    ]
                                },
                                "sortId": 20
                            },
                            "skill_cost_block_2": {
                                "runeId": "skill_cost_block_2",
                                "runeGroupId": "skill_cost_block_2",
                                "runeIcon": "g_skill_cost_block_2",
                                "runeName": "目标：战略调度",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "skill_cost_block_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "禁止使用特种和术师干员",
                                    "runes": [
                                        {
                                            "key": "char_exclude",
                                            "selector": {
                                                "professionMask": 96,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": []
                                        }
                                    ]
                                },
                                "sortId": 21
                            },
                            "char_respawntime_1": {
                                "runeId": "char_respawntime_1",
                                "runeGroupId": "char_respawntime_1",
                                "runeIcon": "g_char_respawntime_1",
                                "runeName": "目标：效率低下I",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_respawntime_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "我方所有单位再部署时间延长<@crisisv2.nag>75%</>",
                                    "runes": [
                                        {
                                            "key": "char_respawntime_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 1.75
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 22
                            },
                            "wdarft_1": {
                                "runeId": "wdarft_1",
                                "runeGroupId": "group_wdarft_1",
                                "runeIcon": "g_wdarft_1",
                                "runeName": "枯朽萃聚使徒：尸痕累累",
                                "score": 30,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "wdarft_1",
                                    "points": 0,
                                    "mutexGroupKey": "wdarft",
                                    "description": "<枯朽 萃聚使徒>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，法术抗性提升<@crisisv2.nag>{magic_resistance}</>，防御力提升<@crisisv2.nag>{def}</>（同类效果叠加）",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1321_wdarft"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 2
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1321_wdarft"
                                                },
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 50
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 100
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 40
                            },
                            "nhfly_1": {
                                "runeId": "nhfly_1",
                                "runeGroupId": "group_nhfly_1",
                                "runeIcon": "g_nhfly_1",
                                "runeName": "枯朽萃聚使徒：彻底毁灭",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "nhfly_1",
                                    "points": 0,
                                    "mutexGroupKey": "wdarft",
                                    "description": "<枯朽之种>攻击力提升至<@crisisv2.nag>{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1269_nhfly"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 1.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 44
                            },
                            "enemy_def": {
                                "runeId": "enemy_def",
                                "runeGroupId": "group_enemy_def",
                                "runeIcon": "g_enemy_def_1",
                                "runeName": "反装甲I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_def",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hpdef",
                                    "description": "所有敌人的防御力<@ba.grt>+{def}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 300
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 82
                            },
                            "wdarft_2": {
                                "runeId": "wdarft_2",
                                "runeGroupId": "group_wdarft_1",
                                "runeIcon": "g_wdarft_2",
                                "runeName": "枯朽萃聚使徒：腐烂堆积",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "wdarft_2",
                                    "points": 0,
                                    "mutexGroupKey": "wdarft",
                                    "description": "<枯朽萃聚使徒>防御力提升<@crisisv2.nag>{def}</>（同类效果叠加）",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1321_wdarft"
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 850
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 43
                            },
                            "dusrch_1": {
                                "runeId": "dusrch_1",
                                "runeGroupId": "group_dusrch",
                                "runeIcon": "g_dusrch_1",
                                "runeName": "重型沼泽探照车：装甲改装",
                                "score": 30,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_1",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "<重型沼泽探照车>的攻击力<@crisisv2.nag>+{atk:0%}</>，生命值<@crisisv2.nag>+{max_hp:0%}</>（同类效果叠加）,进入目标点额外损失<@crisisv2.nag>{life_point}</>点目标生命",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.5
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "dusrch_2"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_life_reduce_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "life_point",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 60
                            },
                            "dusrch_speed_1": {
                                "runeId": "dusrch_speed_1",
                                "runeGroupId": "group_dusrch_speed",
                                "runeIcon": "g_dusrch_speed_1",
                                "runeName": "重型沼泽探照车：急速开飚",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_speed_1",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "<重型沼泽探照车>的移动速度<@crisisv2.nag>+100%</>，并且不可阻挡",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new_with_verify",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "enemy_block_free"
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "move_speed",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 62
                            },
                            "dusrch_invisible_1": {
                                "runeId": "dusrch_invisible_1",
                                "runeGroupId": "group_dusrch_invisible_1",
                                "runeIcon": "g_dusrch_invisible_1",
                                "runeName": "重型沼泽探照车：环境迷彩",
                                "score": 30,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_invisible_1",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "<重型沼泽探照车>获得隐匿，攻击速度<@crisisv2.nag>+{attack_speed}</>，法术抗性<@crisisv2.nag>+{magic_resistance}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_dynamic_ability_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "key",
                                                    "valueStr": "invisible"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "attack_speed",
                                                    "value": 50
                                                },
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 50
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 61
                            },
                            "dusrch_2": {
                                "runeId": "dusrch_2",
                                "runeGroupId": "group_dusrch",
                                "runeIcon": "g_dusrch_2",
                                "runeName": "重型沼泽探照车：尖刺改装",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_2",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "<重型沼泽探照车>的攻击力<@crisisv2.nag>+{atk:0%}</>，生命值<@crisisv2.nag>+{max_hp:0%}</>（同类效果叠加）",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.8
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.2
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "dusrch_2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 63
                            },
                            "dusrch_3": {
                                "runeId": "dusrch_3",
                                "runeGroupId": "dusrch_def",
                                "runeIcon": "g_dusrch_3",
                                "runeName": "重型沼泽探照车：车身加固",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_3",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "<重型沼泽探照车>的防御力<@crisisv2.nag>+{def}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1291_dusrch_2"
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 800
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 65
                            },
                            "reedf_skill_2": {
                                "runeId": "reedf_skill_2",
                                "runeGroupId": "group_reedf_skill_2",
                                "runeIcon": "g_reedf_skill_2",
                                "runeName": "环境：纠缠芦苇",
                                "score": 50,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "reedf_skill_2",
                                    "points": 0,
                                    "mutexGroupKey": "tile",
                                    "description": "部署在<芦苇丛>中的干员自然回复技力速度下降<@crisisv2.nag>90%</>",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reed"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.1
                                                }
                                            ]
                                        },
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reedw"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.1
                                                }
                                            ]
                                        },
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reedf"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 70
                            },
                            "reedf_duskls": {
                                "runeId": "reedf_duskls",
                                "runeGroupId": "group_reedf_duskls",
                                "runeIcon": "g_reedf_duskls",
                                "runeName": "目标：调度",
                                "score": 10,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "reedf_duskls",
                                    "points": 0,
                                    "mutexGroupKey": "tile",
                                    "description": "地图右侧的芦苇丛中的<特别行动队士兵>变成<深池逐火战士>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "duskls"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "level_hidden_group_disable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "noduskls"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 74
                            },
                            "map_tile_reed1": {
                                "runeId": "map_tile_reed1",
                                "runeGroupId": "group_tile_reed",
                                "runeIcon": "g_map_tile_reed3",
                                "runeName": "环境：旺盛火势I",
                                "score": 20,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "map_tile_reed1",
                                    "points": 0,
                                    "mutexGroupKey": "tile",
                                    "description": "地图右侧的一处芦苇丛初始处于点 燃状态，并且地块燃烧造成的伤害提高<@crisisv2.nag>100%</>",
                                    "runes": [
                                        {
                                            "key": "map_tile_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(1,7)"
                                                },
                                                {
                                                    "key": "mode",
                                                    "value": 1
                                                }
                                            ]
                                        },
                                        {
                                            "key": "map_tile_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "damage",
                                                    "value": 40
                                                },
                                                {
                                                    "key": "key",
                                                    "valueStr": "tile_reed|tile_reedf|tile_reedw"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 72
                            },
                            "duskls_tal1": {
                                "runeId": "duskls_tal1",
                                "runeGroupId": "group_duskls_tal1",
                                "runeIcon": "g_duskls_tal1",
                                "runeName": "目标：灼热灰烬I",
                                "score": 20,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "duskls_tal1",
                                    "points": 0,
                                    "mutexGroupKey": "tile",
                                    "description": "<深池逐火战士>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，所有敌人攻击速度<@crisisv2.nag>+{attack_speed}</>，<怨恨的余烬>需要<@crisisv2.nag>10</>次攻击才能击倒",
                                    "runes": [
                                        {
                                            "key": "enemy_talent_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1288_duskls"
                                                },
                                                {
                                                    "key": "Revive[Trigger].prop_max_hp",
                                                    "value": 5
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1288_duskls"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.6
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "enemy_1288_duskls"
                                                },
                                                {
                                                    "key": "attack_speed",
                                                    "value": 50
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 73
                            },
                            "global_costrecovery_1": {
                                "runeId": "global_costrecovery_1",
                                "runeGroupId": "group_global_costrecovery_1",
                                "runeIcon": "g_global_costrecovery_1",
                                "runeName": "目标：深度渗透I",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_costrecovery_1",
                                    "points": 0,
                                    "mutexGroupKey": "cost_recovery",
                                    "description": "部署费用的自然回复速度降低<@crisisv2.nag>25%</>",
                                    "runes": [
                                        {
                                            "key": "global_cost_recovery_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 1.3333
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 34
                            },
                            "global_costrecovery_2": {
                                "runeId": "global_costrecovery_2",
                                "runeGroupId": "gourp_global_costrecovery_2",
                                "runeIcon": "g_global_costrecovery_2",
                                "runeName": "目标：深度渗透II",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_costrecovery_2",
                                    "points": 0,
                                    "mutexGroupKey": "cost_recovery",
                                    "description": "部署费用的自然回复速度降低<@crisisv2.nag>50%</>",
                                    "runes": [
                                        {
                                            "key": "global_cost_recovery_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "global_costrecovery_3": {
                                "runeId": "global_costrecovery_3",
                                "runeGroupId": "gourp_global_costrecovery_3",
                                "runeIcon": "g_global_costrecovery_3",
                                "runeName": "目标：深度渗透III",
                                "score": 40,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_costrecovery_3",
                                    "points": 0,
                                    "mutexGroupKey": "cost_recovery",
                                    "description": "部署费用的自然回复速度降低<@crisisv2.nag>75%</>",
                                    "runes": [
                                        {
                                            "key": "global_cost_recovery_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 30
                            },
                            "global_squadnum_1": {
                                "runeId": "global_squadnum_1",
                                "runeGroupId": "group_global_squadnum_1",
                                "runeIcon": "g_global_squadnum_1",
                                "runeName": "目标：隐秘行动I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_squadnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "最多可编入<@crisisv2.nag>10</>名干员进入作战",
                                    "runes": [
                                        {
                                            "key": "global_squad_num_limit",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 10
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 36
                            },
                            "global_pcharnum_1": {
                                "runeId": "global_pcharnum_1",
                                "runeGroupId": "group_global_pcharnum_1",
                                "runeIcon": "g_global_pcharnum_1",
                                "runeName": "环境：交战区I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "可同时部署的单位数量减少至<@crisisv2.nag>7</>",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 35
                            },
                            "global_squadnum_2": {
                                "runeId": "global_squadnum_2",
                                "runeGroupId": "group_global_squadnum_2",
                                "runeIcon": "g_global_squadnum_2",
                                "runeName": "目标：隐秘行动II",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_squadnum_2",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "最多可编入<@crisisv2.nag>7</>名干员进入作战",
                                    "runes": [
                                        {
                                            "key": "global_squad_num_limit",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 7
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 32
                            },
                            "global_pcharnum_2": {
                                "runeId": "global_pcharnum_2",
                                "runeGroupId": "group_global_pcharnum_2",
                                "runeIcon": "g_global_pcharnum_2",
                                "runeName": "环境：交战区II",
                                "score": 30,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_2",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "可同时部署的单位数量减少至<@crisisv2.nag>5</>",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 33
                            },
                            "melee_forbid_1": {
                                "runeId": "melee_forbid_1",
                                "runeGroupId": "group_melee_forbid_1",
                                "runeIcon": "g_melee_forbid_1",
                                "runeName": "环境：泥沼",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "melee_forbid_1",
                                    "points": 0,
                                    "mutexGroupKey": "maptile",
                                    "description": "地图中的<@crisisv2.nag>8</>个地面位置无法部署",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(3,6)|(3,7)|(4,6)|(4,7)|(5,6)|(5,7)|(2,5)|(2,7)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 54
                            },
                            "range_forbid_2": {
                                "runeId": "range_forbid_2",
                                "runeGroupId": "group_range_forbid_2",
                                "runeIcon": "g_range_forbid_2",
                                "runeName": "环境：洼地",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "range_forbid_2",
                                    "points": 0,
                                    "mutexGroupKey": "maptile",
                                    "description": "地图中的<@crisisv2.nag>7</>个高台位置无法部署",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(5,3)|(3,3)|(1,4)|(3,4)|(4,5)|(3,8)|(5,8)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 53
                            },
                            "dusrch_left_1": {
                                "runeId": "dusrch_left_1",
                                "runeGroupId": "group_dusrch_left_1",
                                "runeIcon": "g_dusrch_left_1",
                                "runeName": "环境：左侧告急I",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "dusrch_left_1",
                                    "points": 0,
                                    "mutexGroupKey": "maptile",
                                    "description": "地图左侧出现额外的<深池塑能术士>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "dusrch1"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 52
                            },
                            "dusrch_right_2": {
                                "runeId": "dusrch_right_2",
                                "runeGroupId": "group_dusrch_right_2",
                                "runeIcon": "g_dusrch_right_1",
                                "runeName": "环境：右侧告急",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "dusrch_right_2",
                                    "points": 0,
                                    "mutexGroupKey": "maptile",
                                    "description": "地图右侧初始出现<深池塑能术士>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "dusrch2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 51
                            },
                            "global_lifepoint_1": {
                                "runeId": "global_lifepoint_1",
                                "runeGroupId": "group_global_lifepoint_1",
                                "runeIcon": "g_global_lifepoint_1",
                                "runeName": "目标：最后防线",
                                "score": 30,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_lifepoint_1",
                                    "points": 0,
                                    "mutexGroupKey": "maptile",
                                    "description": "我方防御点可承受的敌方数量变 为<@crisisv2.nag>1</>",
                                    "runes": [
                                        {
                                            "key": "global_lifepoint",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 50
                            },
                            "reedf_skill_1": {
                                "runeId": "reedf_skill_1",
                                "runeGroupId": "group_reedf_skill_1",
                                "runeIcon": "g_reedf_skill",
                                "runeName": "环境：深池",
                                "score": 20,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "reedf_skill_1",
                                    "points": 0,
                                    "mutexGroupKey": "tile",
                                    "description": "部署在<芦苇丛>中的干员自然回复技力速度下降<@crisisv2.nag>50%</>",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reed"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.5
                                                }
                                            ]
                                        },
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reedw"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.5
                                                }
                                            ]
                                        },
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "sp_recovery_reduction_certain_tile"
                                                },
                                                {
                                                    "key": "tile",
                                                    "valueStr": "tile_reedf"
                                                },
                                                {
                                                    "key": "sp_recovery_per_sec",
                                                    "value": 0.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 71
                            },
                            "char_def_1": {
                                "runeId": "char_def_1",
                                "runeGroupId": "g_char_def",
                                "runeIcon": "g_char_def_1",
                                "runeName": "目标：绝密作战I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_def_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_maxhpdef",
                                    "description": "所有我方单位的防御力<@crisisv2.nag>{def}</>",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": -200
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 80
                            },
                            "lunmag_1": {
                                "runeId": "lunmag_1",
                                "runeGroupId": "group_lunmag_1",
                                "runeIcon": "g_lunmag_1",
                                "runeName": "目标：术师领队",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "lunmag_1",
                                    "points": 0,
                                    "mutexGroupKey": "enti",
                                    "description": "场上初始存在<特战术士组长>，并且<特战术士组长>，生命值提升至<@crisisv2.nag>{max_hp:0%}</>，攻击范 围扩大",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1038_lunmag_2"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.6
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attackradius_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1038_lunmag_2"
                                                },
                                                {
                                                    "key": "scale",
                                                    "value": 1.5
                                                }
                                            ]
                                        },
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "lunmag_2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 81
                            },
                            "enemy_hp_3": {
                                "runeId": "enemy_hp_3",
                                "runeGroupId": "group_enemy_hp",
                                "runeIcon": "g_enemy_hp_1",
                                "runeName": "源石环境：活性I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_3",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hpdef",
                                    "description": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 83
                            },
                            "dusocr_1": {
                                "runeId": "dusocr_1",
                                "runeGroupId": "group_dusocr_1",
                                "runeIcon": "g_dusocr_1",
                                "runeName": "目标：伏击火球",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusocr_1",
                                    "points": 0,
                                    "mutexGroupKey": "enti",
                                    "description": "<深池塑能术士>生命值提升至<@crisisv2.nag>{max_hp:0%}</>，法术抗性<@crisisv2.nag>+{magic_resistance}</>，获得隐匿且<净浊之焰>的伤害提升至<@crisisv2.nag>130%</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1176_dusocr"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.6
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1176_dusocr"
                                                },
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 40
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_dynamic_ability_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1176_dusocr"
                                                },
                                                {
                                                    "key": "key",
                                                    "valueStr": "invisible"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1177_dufrbl"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 1.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 84
                            },
                            "wdarft_start": {
                                "runeId": "wdarft_start",
                                "runeGroupId": "g_wdarft_1",
                                "runeIcon": "g_wdarft_start",
                                "runeName": "环境：散播 朽败",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "wdarft_start",
                                    "points": 0,
                                    "mutexGroupKey": "wdarft",
                                    "description": "地图中开始出现<枯朽萃聚使徒>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "fly"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "env_gbuff_new_with_verify",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "crisisv2_1_comment_buff_1"
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1269_nhfly"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 45
                            },
                            "dusrch_start": {
                                "runeId": "dusrch_start",
                                "runeGroupId": "dusrch_start",
                                "runeIcon": "g_dusrch_start",
                                "runeName": "环境：重械开道",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "dusrch_start",
                                    "points": 0,
                                    "mutexGroupKey": "Dusrch",
                                    "description": "地图中开始出现<重型沼泽探照车>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "car"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 64
                            },
                            "wdarft_3": {
                                "runeId": "wdarft_3",
                                "runeGroupId": "group_wdarft_3",
                                "runeIcon": "g_wdarft_3",
                                "runeName": "枯朽萃聚使徒：阴影纠缠",
                                "score": 30,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "wdarft_3",
                                    "points": 0,
                                    "mutexGroupKey": "wdarft",
                                    "description": "<枯朽萃聚使徒>获得<@crisisv2.nag>隐匿</>",
                                    "runes": [
                                        {
                                            "key": "enemy_dynamic_ability_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1321_wdarft"
                                                },
                                                {
                                                    "key": "key",
                                                    "valueStr": "invisible"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 41
                            },
                            "char_blockcnt_1": {
                                "runeId": "char_blockcnt_1",
                                "runeGroupId": "char_blockcnt_1",
                                "runeIcon": "g_char_blockcnt_1",
                                "runeName": "环境：精神溃散",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_blockcnt_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_maxhpdef",
                                    "description": "所有我方单位的阻挡数<@crisisv2.nag>{value}</>",
                                    "runes": [
                                        {
                                            "key": "char_blockcnt_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 85
                            }
                        }
                    },
                    "crisis_v2_01-02": {
                        "commentDataMap": {},
                        "challengeNodeDataMap": {
                            "keypoint_1": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "200"
                                ],
                                "slotId": "keypoint_1",
                                "previewTitle": "标 准能力测试",
                                "previewDesc": "完成一次作战，同时评分大于等于200分",
                                "missionSortId": 4,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "4001",
                                            "count": 5000,
                                            "type": "GOLD"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_2": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_7;node_8",
                                    "2"
                                ],
                                "slotId": "keypoint_2",
                                "previewTitle": "协防训练",
                                "previewDesc": "完成一次作战，并同时携带【 目标：装备精良】和【目标：反击训练】",
                                "missionSortId": 5,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "2002",
                                            "count": 10,
                                            "type": "CARD_EXP"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 2,
                                "slotIdList": [
                                    "node_7",
                                    "node_8"
                                ]
                            },
                            "keypoint_3": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_12;node_14;node_18",
                                    "3"
                                ],
                                "slotId": "keypoint_3",
                                "previewTitle": "火力维续训练",
                                "previewDesc": "完成一次作战，并同时携带【山海众游勇：漫山遍野】、【山海众秘使：漫山遍野】、【源石环境：刺激I】",
                                "missionSortId": 6,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "3302",
                                            "count": 5,
                                            "type": "MATERIAL"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 3,
                                "slotIdList": [
                                    "node_12",
                                    "node_14",
                                    "node_18"
                                ]
                            }
                        },
                        "groupDescDataMap": {},
                        "roadRelationDataMap": {
                            "road_0": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_5"
                                }
                            },
                            "road_1": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_6"
                                }
                            },
                            "road_2": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_3": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_8"
                                }
                            },
                            "road_4": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_7"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_5": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_8"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_6": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_7": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_2"
                                }
                            },
                            "road_8": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_9": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_13"
                                }
                            },
                            "road_10": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_11": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_12": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_20"
                                }
                            },
                            "road_13": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_22"
                                }
                            },
                            "road_14": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            },
                            "road_30": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_32"
                                }
                            },
                            "road_16": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_22"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            },
                            "road_17": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_23"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_18": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_16"
                                }
                            },
                            "road_19": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_17"
                                }
                            },
                            "road_20": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_16"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_21": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_22": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_23": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_25"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_26"
                                }
                            },
                            "road_24": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_25"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_27"
                                }
                            },
                            "road_25": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_26"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                }
                            },
                            "road_26": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_27"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                }
                            },
                            "road_27": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_29"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_0"
                                }
                            },
                            "road_28": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_10"
                                }
                            },
                            "road_29": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_31"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_12"
                                }
                            },
                            "road_15": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_20"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_32"
                                }
                            },
                            "road_31": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_33"
                                }
                            },
                            "road_32": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_33": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_29"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_1"
                                }
                            },
                            "road_34": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_2"
                                }
                            },
                            "road_35": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_36": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_37": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            }
                        },
                        "bagRoadDataMap": {},
                        "nodeViewData": {
                            "width": 885,
                            "height": 1180,
                            "bagPosMap": {},
                            "roadPosMap": {
                                "road_0": {
                                    "id": "road_0",
                                    "centerPos": {
                                        "x": 435,
                                        "y": -150
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_1": {
                                    "id": "road_1",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_2": {
                                    "id": "road_2",
                                    "centerPos": {
                                        "x": 435,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_3": {
                                    "id": "road_3",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 652.5,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 174,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -82.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 82.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_5": {
                                    "id": "road_5",
                                    "centerPos": {
                                        "x": 622.5,
                                        "y": -345
                                    },
                                    "size": {
                                        "x": 294,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": -142.5,
                                        "y": -15
                                    },
                                    "endPos": {
                                        "x": 142.5,
                                        "y": 15
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 142.5,
                                                "y": -15
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_6": {
                                    "id": "road_6",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -262.5
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_7": {
                                    "id": "road_7",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -187.5
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_8": {
                                    "id": "road_8",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_9": {
                                    "id": "road_9",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -592.5
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 234
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": -112.5
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 112.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": -112.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": 112.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_10": {
                                    "id": "road_10",
                                    "centerPos": {
                                        "x": 435,
                                        "y": -480
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_11": {
                                    "id": "road_11",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 159
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": -75
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 75
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": -75
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": 75
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_12": {
                                    "id": "road_12",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -705
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_13": {
                                    "id": "road_13",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -742.5
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_14": {
                                    "id": "road_14",
                                    "centerPos": {
                                        "x": 240,
                                        "y": -787.5
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 174
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 82.5
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": -82.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": 82.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": -82.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_30": {
                                    "id": "road_30",
                                    "centerPos": {
                                        "x": 510,
                                        "y": -705
                                    },
                                    "size": {
                                        "x": 68.99997,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": -29.99997,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_16": {
                                    "id": "road_16",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_17": {
                                    "id": "road_17",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 158.999969,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -74.99997,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_18": {
                                    "id": "road_18",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -555
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_19": {
                                    "id": "road_19",
                                    "centerPos": {
                                        "x": 435,
                                        "y": -585
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -45,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -45,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_20": {
                                    "id": "road_20",
                                    "centerPos": {
                                        "x": 570,
                                        "y": -570
                                    },
                                    "size": {
                                        "x": 189,
                                        "y": 39
                                    },
                                    "startPos": {
                                        "x": -90,
                                        "y": 15
                                    },
                                    "endPos": {
                                        "x": 90,
                                        "y": -15
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 90,
                                                "y": 15
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_21": {
                                    "id": "road_21",
                                    "centerPos": {
                                        "x": 600,
                                        "y": -615
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_22": {
                                    "id": "road_22",
                                    "centerPos": {
                                        "x": 712.5,
                                        "y": -615
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_23": {
                                    "id": "road_23",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -870
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_24": {
                                    "id": "road_24",
                                    "centerPos": {
                                        "x": 435,
                                        "y": -900
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": -30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -45,
                                                "y": 30
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -45,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_25": {
                                    "id": "road_25",
                                    "centerPos": {
                                        "x": 570,
                                        "y": -870
                                    },
                                    "size": {
                                        "x": 189,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -90,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 90,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_26": {
                                    "id": "road_26",
                                    "centerPos": {
                                        "x": 615,
                                        "y": -900
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": -30
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": 30
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 45,
                                                "y": -30
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_27": {
                                    "id": "road_27",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_28": {
                                    "id": "road_28",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -450
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9.00003052
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": -0.0000305175781
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_29": {
                                    "id": "road_29",
                                    "centerPos": {
                                        "x": 90,
                                        "y": -705
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30.0000038,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_15": {
                                    "id": "road_15",
                                    "centerPos": {
                                        "x": 390,
                                        "y": -705
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_31": {
                                    "id": "road_31",
                                    "centerPos": {
                                        "x": 697.5,
                                        "y": -870
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_32": {
                                    "id": "road_32",
                                    "centerPos": {
                                        "x": 727.5,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_33": {
                                    "id": "road_33",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_34": {
                                    "id": "road_34",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -187.5
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_35": {
                                    "id": "road_35",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -262.5
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 22.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_36": {
                                    "id": "road_36",
                                    "centerPos": {
                                        "x": 247.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_37": {
                                    "id": "road_37",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -450
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": -0.0000305175781
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0.0000305175781
                                    },
                                    "inflectionList": []
                                }
                            },
                            "nodePosMap": {
                                "node_0": {
                                    "position": {
                                        "x": 150,
                                        "y": -195
                                    }
                                },
                                "node_1": {
                                    "position": {
                                        "x": 150,
                                        "y": -270
                                    }
                                },
                                "node_2": {
                                    "position": {
                                        "x": 330,
                                        "y": -150
                                    }
                                },
                                "node_3": {
                                    "position": {
                                        "x": 330,
                                        "y": -225
                                    }
                                },
                                "node_4": {
                                    "position": {
                                        "x": 330,
                                        "y": -300
                                    }
                                },
                                "node_5": {
                                    "position": {
                                        "x": 540,
                                        "y": -150
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 450,
                                        "y": -225
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 540,
                                        "y": -300
                                    }
                                },
                                "node_8": {
                                    "position": {
                                        "x": 450,
                                        "y": -360
                                    }
                                },
                                "node_9": {
                                    "position": {
                                        "x": 765,
                                        "y": -300
                                    }
                                },
                                "node_10": {
                                    "position": {
                                        "x": 150,
                                        "y": -420
                                    }
                                },
                                "node_11": {
                                    "position": {
                                        "x": 150,
                                        "y": -494.999969
                                    }
                                },
                                "node_12": {
                                    "position": {
                                        "x": 150,
                                        "y": -705
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 330,
                                        "y": -480
                                    }
                                },
                                "node_14": {
                                    "position": {
                                        "x": 540,
                                        "y": -480
                                    }
                                },
                                "node_15": {
                                    "position": {
                                        "x": 330,
                                        "y": -555
                                    }
                                },
                                "node_16": {
                                    "position": {
                                        "x": 450,
                                        "y": -555
                                    }
                                },
                                "node_17": {
                                    "position": {
                                        "x": 540,
                                        "y": -615
                                    }
                                },
                                "node_18": {
                                    "position": {
                                        "x": 660,
                                        "y": -615
                                    }
                                },
                                "node_19": {
                                    "position": {
                                        "x": 765,
                                        "y": -615
                                    }
                                },
                                "node_20": {
                                    "position": {
                                        "x": 330,
                                        "y": -705
                                    }
                                },
                                "keypoint_1": {
                                    "position": {
                                        "x": 540,
                                        "y": -705
                                    }
                                },
                                "node_22": {
                                    "position": {
                                        "x": 330,
                                        "y": -780
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 450,
                                        "y": -780
                                    }
                                },
                                "node_24": {
                                    "position": {
                                        "x": 660,
                                        "y": -780
                                    }
                                },
                                "node_25": {
                                    "position": {
                                        "x": 330,
                                        "y": -870
                                    }
                                },
                                "node_26": {
                                    "position": {
                                        "x": 450,
                                        "y": -870
                                    }
                                },
                                "node_27": {
                                    "position": {
                                        "x": 540,
                                        "y": -930
                                    }
                                },
                                "keypoint_3": {
                                    "position": {
                                        "x": 660,
                                        "y": -870
                                    }
                                },
                                "node_29": {
                                    "position": {
                                        "x": 60,
                                        "y": -225
                                    }
                                },
                                "node_30": {
                                    "position": {
                                        "x": 60,
                                        "y": -450
                                    }
                                },
                                "node_31": {
                                    "position": {
                                        "x": 59.9999962,
                                        "y": -705
                                    }
                                },
                                "node_32": {
                                    "position": {
                                        "x": 450,
                                        "y": -705
                                    }
                                },
                                "node_33": {
                                    "position": {
                                        "x": 765,
                                        "y": -870
                                    }
                                },
                                "keypoint_2": {
                                    "position": {
                                        "x": 765,
                                        "y": -780
                                    }
                                }
                            },
                            "exclusionDataMap": {
                                "exclusive_1": {
                                    "id": "exclusive_1",
                                    "pos": {
                                        "x": 150,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                },
                                "exclusive_2": {
                                    "id": "exclusive_2",
                                    "pos": {
                                        "x": 150,
                                        "y": -450
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                }
                            }
                        },
                        "bagViewData": null,
                        "bagDataMap": {},
                        "exclusionDataMap": {
                            "exclusive_1": {
                                "defaultSlotId": null
                            },
                            "exclusive_2": {
                                "defaultSlotId": null
                            }
                        },
                        "dimensionItemList": [
                            {
                                "desc": "战略",
                                "maxScore": 90
                            },
                            {
                                "desc": "技巧",
                                "maxScore": 110
                            },
                            {
                                "desc": " 调遣",
                                "maxScore": 10
                            },
                            {
                                "desc": "应变",
                                "maxScore": 90
                            },
                            {
                                "desc": "洞察",
                                "maxScore": 40
                            },
                            {
                                "desc": "筹划",
                                "maxScore": 10
                            }
                        ],
                        "rewardNodeDataMap": {},
                        "dailyRuneUnlcokDataMap": {},
                        "nodeDataMap": {
                            "node_0": {
                                "runeId": "npcsld_enable_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusive_1",
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_29",
                                    "node_2",
                                    "node_4"
                                ]
                            },
                            "node_1": {
                                "runeId": "npcsld_enable_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusive_1",
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_2",
                                    "node_29",
                                    "node_3"
                                ]
                            },
                            "node_2": {
                                "runeId": "global_forbidloc_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_5",
                                    "node_1",
                                    "node_0"
                                ]
                            },
                            "node_3": {
                                "runeId": "char_def_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_6",
                                    "node_0",
                                    "node_1"
                                ]
                            },
                            "node_4": {
                                "runeId": "char_blockcnt_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_7",
                                    "node_8",
                                    "node_1",
                                    "node_0"
                                ]
                            },
                            "node_5": {
                                "runeId": "npcsld_rec_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_2"
                                ]
                            },
                            "node_6": {
                                "runeId": "npcsld_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3"
                                ]
                            },
                            "node_7": {
                                "runeId": "npcsld_def_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_8": {
                                "runeId": "npcsld_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_9": {
                                "runeId": "global_lifepoint_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_7",
                                    "node_8"
                                ]
                            },
                            "node_10": {
                                "runeId": "char_guardspecial_cost_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusive_2",
                                "adjacentNodeList": [
                                    "node_30"
                                ]
                            },
                            "node_11": {
                                "runeId": "char_cost_healerdefender_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusive_2",
                                "adjacentNodeList": [
                                    "node_30"
                                ]
                            },
                            "node_12": {
                                "runeId": "enemy_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13",
                                    "node_15",
                                    "node_20",
                                    "node_22",
                                    "node_25",
                                    "node_31"
                                ]
                            },
                            "node_13": {
                                "runeId": "ymknif_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_14"
                                ]
                            },
                            "node_14": {
                                "runeId": "level_hidden_ymknif_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13"
                                ]
                            },
                            "node_15": {
                                "runeId": "ymkilr_skill_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_16",
                                    "node_17"
                                ]
                            },
                            "node_16": {
                                "runeId": "ymkilr_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_15",
                                    "node_18"
                                ]
                            },
                            "node_17": {
                                "runeId": "ymkilr_hp_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_15",
                                    "node_18"
                                ]
                            },
                            "node_18": {
                                "runeId": "level_hidden_ymkilr_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_16",
                                    "node_17",
                                    "node_19"
                                ]
                            },
                            "node_19": {
                                "runeId": "ymkilr_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_18"
                                ]
                            },
                            "node_20": {
                                "runeId": "enemy_atk_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_32"
                                ]
                            },
                            "keypoint_1": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "node_22": {
                                "runeId": "char_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_23"
                                ]
                            },
                            "node_23": {
                                "runeId": "char_hp_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_22",
                                    "node_24"
                                ]
                            },
                            "node_24": {
                                "runeId": "char_hp_3",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_23",
                                    "keypoint_2"
                                ]
                            },
                            "node_25": {
                                "runeId": "ymmir_skill_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_26",
                                    "node_27"
                                ]
                            },
                            "node_26": {
                                "runeId": "ymmir_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_25",
                                    "keypoint_3"
                                ]
                            },
                            "node_27": {
                                "runeId": "ymmir_hp_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_25",
                                    "keypoint_3"
                                ]
                            },
                            "node_29": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_0",
                                    "node_1"
                                ]
                            },
                            "node_30": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_10",
                                    "node_11"
                                ]
                            },
                            "node_31": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12"
                                ]
                            },
                            "node_32": {
                                "runeId": "enemy_atk_3",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "keypoint_1",
                                    "node_20"
                                ]
                            },
                            "node_33": {
                                "runeId": "ymmir_invisible_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "keypoint_3"
                                ]
                            },
                            "keypoint_3": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_26",
                                    "node_27",
                                    "node_33"
                                ]
                            },
                            "keypoint_2": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24"
                                ]
                            }
                        },
                        "runeDataMap": {
                            "npcsld_enable_1": {
                                "runeId": "npcsld_enable_1",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_enable_1",
                                "runeName": "环境：人手短缺 （北）",
                                "score": 10,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "npcsld_enable_1",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "场地上方不再出现<盾卫>",
                                    "runes": [
                                        {
                                            "key": "level_predefines_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "trap_024_npcsld#1",
                                                    "value": 0
                                                },
                                                {
                                                    "key": "trap_024_npcsld#2",
                                                    "value": 0
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 41
                            },
                            "npcsld_enable_2": {
                                "runeId": "npcsld_enable_2",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_enable_2",
                                "runeName": "环境：人手短缺（南）",
                                "score": 10,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "npcsld_enable_2",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "场地下方不再出现<盾卫>",
                                    "runes": [
                                        {
                                            "key": "level_predefines_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "trap_024_npcsld#3",
                                                    "value": 0
                                                },
                                                {
                                                    "key": "trap_024_npcsld#4",
                                                    "value": 0
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 42
                            },
                            "global_forbidloc_1": {
                                "runeId": "global_forbidloc_1",
                                "runeGroupId": null,
                                "runeIcon": "g_melee_forbid_1",
                                "runeName": "环境：坑坑洼洼",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_forbidloc_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场中<@crisisv2.nag>6</>个位置无法部署我方单位",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(4,2)|(4,3)|(5,7)|(5,8)|(3,7)|(2,7)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 43
                            },
                            "char_def_1": {
                                "runeId": "char_def_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_def_1",
                                "runeName": "目标：绝密作战I",
                                "score": 10,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_def_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "所有我方单位的防御力降低<@crisisv2.nag>{-def}</>",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": -100
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 44
                            },
                            "char_blockcnt_1": {
                                "runeId": "char_blockcnt_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_blockcnt_1",
                                "runeName": "环境：精神溃散",
                                "score": 10,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_blockcnt_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "所有角色阻挡数<@crisisv2.nag>{value}</>",
                                    "runes": [
                                        {
                                            "key": "char_blockcnt_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 45
                            },
                            "npcsld_rec_1": {
                                "runeId": "npcsld_rec_1",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_rec_1",
                                "runeName": "目标：斗志昂扬",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "npcsld_rec_1",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "<盾卫>每秒恢复<@crisisv2.pos>{hp_recovery_per_sec}</>点生命值",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 256,
                                                "buildableMask": 1,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "hp_recovery_per_sec",
                                                    "value": 100
                                                },
                                                {
                                                    "key": "char",
                                                    "valueStr": "trap_024_npcsld"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 56
                            },
                            "npcsld_hp_1": {
                                "runeId": "npcsld_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_hp_1",
                                "runeName": "目标：补给充足",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "npcsld_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "<盾卫>的生命值提升<@crisisv2.pos>{max_hp:0%}</>",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 256,
                                                "buildableMask": 1,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 1
                                                },
                                                {
                                                    "key": "char",
                                                    "valueStr": "trap_024_npcsld"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 57
                            },
                            "npcsld_def_1": {
                                "runeId": "npcsld_def_1",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_def_1",
                                "runeName": "目标：装备精良",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "npcsld_def_1",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "<盾卫>防御力增加<@crisisv2.pos>{def}</>",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 256,
                                                "buildableMask": 1,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 1000
                                                },
                                                {
                                                    "key": "char",
                                                    "valueStr": "trap_024_npcsld"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 53
                            },
                            "npcsld_atk_1": {
                                "runeId": "npcsld_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_npcsld_atk_1",
                                "runeName": "目标：反击训练",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "npcsld_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "npcsld",
                                    "description": "<盾卫>每次受到攻击时对目标造成相当于盾卫攻击力<@crisisv2.pos>{atk_scale:0%}</>的物理伤害",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new_with_verify",
                                            "selector": {
                                                "professionMask": 384,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "inverse_damage[physic]"
                                                },
                                                {
                                                    "key": "atk_scale",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "char",
                                                    "valueStr": "trap_024_npcsld"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 54
                            },
                            "global_lifepoint_1": {
                                "runeId": "global_lifepoint_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_lifepoint_1",
                                "runeName": "目标：最后防线",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_lifepoint_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "我方防御点可承受的敌方数量变为<@crisisv2.nag>{value}</>",
                                    "runes": [
                                        {
                                            "key": "global_lifepoint",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 46
                            },
                            "char_guardspecial_cost_1": {
                                "runeId": "char_guardspecial_cost_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_guardspecial_cost_1",
                                "runeName": "目标：应 急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_guardspecial_cost_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "近卫和特种干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 65,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 47
                            },
                            "char_cost_healerdefender_1": {
                                "runeId": "char_cost_healerdefender_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_healerdefender_1",
                                "runeName": "目标：应急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_healerdefender_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "重装和医疗干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 12,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 1.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 48
                            },
                            "enemy_atk_1": {
                                "runeId": "enemy_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_atk_1",
                                "runeName": "源石环境：刺激I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人攻击力提升<@crisisv2.nag>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "ymknif_hp_1": {
                                "runeId": "ymknif_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymknif_hp_1",
                                "runeName": "山海众游勇：健体",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymknif_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymknif",
                                    "description": "<山海众游勇>的生命值提升<@crisisv2.nag>{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.8
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1296_ymknif_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymknif"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 49
                            },
                            "level_hidden_ymknif_1": {
                                "runeId": "level_hidden_ymknif_1",
                                "runeGroupId": null,
                                "runeIcon": "g_level_hidden_ymknif_1",
                                "runeName": "山海众游勇：漫山遍野",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "level_hidden_ymknif_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymknif",
                                    "description": "场上初始出现4名<山海众游勇>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "ymknif1"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "ymknif2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 50
                            },
                            "ymkilr_skill_1": {
                                "runeId": "ymkilr_skill_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymkilr_skill_1",
                                "runeName": "山海众秘使：出鞘",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymkilr_skill_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymkilr",
                                    "description": "<山海众秘使>的技能伤害提升<@crisisv2.nag>100%</>",
                                    "runes": [
                                        {
                                            "key": "enemy_skill_blackb_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk_scale",
                                                    "value": 2
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 51
                            },
                            "ymkilr_hp_1": {
                                "runeId": "ymkilr_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymkilr_hp_1",
                                "runeName": "山海众秘使：自砺",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymkilr_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymkilr",
                                    "description": "<山海众秘使>的生命值提升<@crisisv2.nag>{max_hp:0%}且防御力提升<@crisisv2.nag>{def}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymkilr"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 400
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymkilr"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 52
                            },
                            "ymkilr_hp_2": {
                                "runeId": "ymkilr_hp_2",
                                "runeGroupId": null,
                                "runeIcon": "g_ymkilr_hp_1",
                                "runeName": "山海众秘使：自砺",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymkilr_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymkilr",
                                    "description": "<山海众秘使>的生命值提升<@crisisv2.nag>{max_hp:0%}且防御力提升<@crisisv2.nag>{def}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymkilr"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 400
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymkilr"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 53
                            },
                            "level_hidden_ymkilr_1": {
                                "runeId": "level_hidden_ymkilr_1",
                                "runeGroupId": null,
                                "runeIcon": "g_level_hidden_ymkilr_1",
                                "runeName": "山海众秘使：漫山遍野",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "level_hidden_ymkilr_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymkilr",
                                    "description": "出现额外的<山海众秘使>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "ymkilr1"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 54
                            },
                            "ymkilr_atk_1": {
                                "runeId": "ymkilr_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymkilr_atk_1",
                                "runeName": "山海众秘使：锐锋",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymkilr_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymkilr",
                                    "description": "<山海众秘使>的攻击力提升至<@crisisv2.nag>{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 2
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1299_ymkilr_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymkilr"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "enemy_atk_2": {
                                "runeId": "enemy_atk_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_atk_2",
                                "runeName": "源石环境：刺激II",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力提升<@crisisv2.nag>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 21
                            },
                            "char_hp_1": {
                                "runeId": "char_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_hp_1",
                                "runeName": "源石环境：侵蚀I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_hp",
                                    "description": "所有我方单位的生命值降低<@crisisv2.nag>{-max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 32
                            },
                            "char_hp_2": {
                                "runeId": "char_hp_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_hp_1",
                                "runeName": "源石环境：侵蚀I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_hp",
                                    "description": "所有我方单位的生命值降低<@crisisv2.nag>{-max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 33
                            },
                            "char_hp_3": {
                                "runeId": "char_hp_3",
                                "runeGroupId": null,
                                "runeIcon": "g_char_hp_2",
                                "runeName": "源石环境：侵蚀II",
                                "score": 30,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_3",
                                    "points": 0,
                                    "mutexGroupKey": "char_hp",
                                    "description": "所有我方单位的生命值降低<@crisisv2.nag>{-max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 22
                            },
                            "ymmir_skill_1": {
                                "runeId": "ymmir_skill_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymmir_skill_1",
                                "runeName": "山海众司魅人：奇术",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymmir_skill_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymmir",
                                    "description": "<山海众司魅人>的技能释放间隔和蓄力时间缩短<@crisisv2.nag>60%</>",
                                    "runes": [
                                        {
                                            "key": "enemy_skill_init_cd_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "skill",
                                                    "valueStr": "Guide"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_skill_cd_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "scale",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "skill",
                                                    "valueStr": "Guide"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_skill_blackb_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "anim_duration",
                                                    "value": 0.4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 34
                            },
                            "ymmir_hp_1": {
                                "runeId": "ymmir_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymmir_hp_1",
                                "runeName": "山海众司魅人：定神",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymmir_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymmir",
                                    "description": "<山海众司魅人>的生命值提升<@crisisv2.nag>{max_hp:0%}</>且法术抗性提升<@crisisv2.nag>{magic_resistance}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymmir"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 20
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymmir"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 55
                            },
                            "ymmir_hp_2": {
                                "runeId": "ymmir_hp_2",
                                "runeGroupId": null,
                                "runeIcon": "g_ymmir_hp_1",
                                "runeName": "山海众司魅人：定神",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymmir_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymmir",
                                    "description": "<山海众司魅人>的生命值提升<@crisisv2.nag>{max_hp:0%}</>且法术抗性提升<@crisisv2.nag>{magic_resistance}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymmir"
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 20
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "ymmir"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 56
                            },
                            "enemy_atk_3": {
                                "runeId": "enemy_atk_3",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_atk_3",
                                "runeName": "源石环境：刺 激III",
                                "score": 40,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_3",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": " 所有敌人的攻击力提升<@crisisv2.nag>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 11
                            },
                            "ymmir_invisible_1": {
                                "runeId": "ymmir_invisible_1",
                                "runeGroupId": null,
                                "runeIcon": "g_ymmir_invisible_1",
                                "runeName": "山海众司魅人：避世",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "ymmir_invisible_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_ymmir",
                                    "description": "<山海众司魅人>获得<@crisisv2.nag>隐匿</>",
                                    "runes": [
                                        {
                                            "key": "enemy_dynamic_ability_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1300_ymmir_2"
                                                },
                                                {
                                                    "key": "key",
                                                    "valueStr": "invisible"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 57
                            }
                        }
                    },
                    "crisis_v2_01-03": {
                        "commentDataMap": {},
                        "challengeNodeDataMap": {
                            "keypoint_1": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "200"
                                ],
                                "slotId": "keypoint_1",
                                "previewTitle": "标准能力测试",
                                "previewDesc": "完成一次作战，同时评分大于等于200分",
                                "missionSortId": 7,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "4001",
                                            "count": 5000,
                                            "type": "GOLD"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_2": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_0;node_3",
                                    "2"
                                ],
                                "slotId": "keypoint_2",
                                "previewTitle": "白刃战训练",
                                "previewDesc": "完成一次作战，并同时携带携带【环境：专心致志III】 与【环境：狭路相逢】",
                                "missionSortId": 8,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "2002",
                                            "count": 10,
                                            "type": "CARD_EXP"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 2,
                                "slotIdList": [
                                    "node_0",
                                    "node_3"
                                ]
                            },
                            "keypoint_3": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_4;node_9;node_10",
                                    "3"
                                ],
                                "slotId": "keypoint_3",
                                "previewTitle": "杀手应对训练",
                                "previewDesc": "完成一次作战，并同时携带【环境：禁闭区】、【环境：越狱】、【杰斯顿：憎恨】",
                                "missionSortId": 9,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "3302",
                                            "count": 5,
                                            "type": "MATERIAL"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 3,
                                "slotIdList": [
                                    "node_4",
                                    "node_9",
                                    "node_10"
                                ]
                            }
                        },
                        "groupDescDataMap": {},
                        "roadRelationDataMap": {
                            "road_0": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_2"
                                }
                            },
                            "road_1": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_2": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_6"
                                }
                            },
                            "road_3": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_4": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_5": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_9"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_10"
                                }
                            },
                            "road_6": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_11"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_12"
                                }
                            },
                            "road_7": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_11"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_13"
                                }
                            },
                            "road_8": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_9": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_10": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_14"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_16"
                                }
                            },
                            "road_12": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_18"
                                }
                            },
                            "road_13": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_35": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            },
                            "road_14": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_20"
                                }
                            },
                            "road_16": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_17": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_18": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_19": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_29"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_0"
                                }
                            },
                            "road_20": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_30"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_21": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_31"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_17"
                                }
                            },
                            "road_22": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_32"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_23": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_1"
                                }
                            },
                            "road_24": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_25": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_5"
                                }
                            },
                            "road_26": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_8"
                                }
                            },
                            "road_27": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_28": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_29": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_5"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_30": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_7"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_31": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_8"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_32": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_14"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_36": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_20"
                                }
                            },
                            "road_38": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_22"
                                }
                            },
                            "road_39": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            },
                            "road_37": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_15": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_34": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_22"
                                }
                            },
                            "road_40": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_32"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            },
                            "road_41": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_32"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_26"
                                }
                            },
                            "road_42": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_32"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_27"
                                }
                            }
                        },
                        "bagRoadDataMap": {},
                        "nodeViewData": {
                            "width": 885,
                            "height": 1270,
                            "bagPosMap": null,
                            "roadPosMap": {
                                "road_0": {
                                    "id": "road_0",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_1": {
                                    "id": "road_1",
                                    "centerPos": {
                                        "x": 300,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_2": {
                                    "id": "road_2",
                                    "centerPos": {
                                        "x": 405,
                                        "y": -277.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 114
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -52.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 52.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -52.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 52.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_3": {
                                    "id": "road_3",
                                    "centerPos": {
                                        "x": 405,
                                        "y": -382.5
                                    },
                                    "size": {
                                        "x": 69.00003,
                                        "y": 114
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 52.5
                                    },
                                    "endPos": {
                                        "x": 30.00003,
                                        "y": -52.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 52.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -52.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 219
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 105
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -105
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 105
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -105
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_5": {
                                    "id": "road_5",
                                    "centerPos": {
                                        "x": 690,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_6": {
                                    "id": "road_6",
                                    "centerPos": {
                                        "x": 195,
                                        "y": -585
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_7": {
                                    "id": "road_7",
                                    "centerPos": {
                                        "x": 195,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_8": {
                                    "id": "road_8",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -585
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_9": {
                                    "id": "road_9",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_10": {
                                    "id": "road_10",
                                    "centerPos": {
                                        "x": 420,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_12": {
                                    "id": "road_12",
                                    "centerPos": {
                                        "x": 195,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_13": {
                                    "id": "road_13",
                                    "centerPos": {
                                        "x": 195,
                                        "y": -825
                                    },
                                    "size": {
                                        "x": 68.9999847,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 45
                                    },
                                    "endPos": {
                                        "x": 29.9999847,
                                        "y": -45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_35": {
                                    "id": "road_35",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_14": {
                                    "id": "road_14",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_16": {
                                    "id": "road_16",
                                    "centerPos": {
                                        "x": 345,
                                        "y": -285
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 45
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -45
                                    },
                                    "inflectionList": []
                                },
                                "road_17": {
                                    "id": "road_17",
                                    "centerPos": {
                                        "x": 375,
                                        "y": -570
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -30
                                    },
                                    "inflectionList": []
                                },
                                "road_18": {
                                    "id": "road_18",
                                    "centerPos": {
                                        "x": 615,
                                        "y": -375
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 69
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 30
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -30
                                    },
                                    "inflectionList": []
                                },
                                "road_19": {
                                    "id": "road_19",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 54.0000076,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5000076,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_20": {
                                    "id": "road_20",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_21": {
                                    "id": "road_21",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 54.0000076,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5000076,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_22": {
                                    "id": "road_22",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -1005
                                    },
                                    "size": {
                                        "x": 53.9999962,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -22.4999962,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": -0.0000610351563
                                    },
                                    "inflectionList": []
                                },
                                "road_23": {
                                    "id": "road_23",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_24": {
                                    "id": "road_24",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_25": {
                                    "id": "road_25",
                                    "centerPos": {
                                        "x": 405,
                                        "y": -277.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 114
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -52.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 52.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -52.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 52.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_26": {
                                    "id": "road_26",
                                    "centerPos": {
                                        "x": 405,
                                        "y": -382.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 114
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 52.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -52.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 52.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -52.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_27": {
                                    "id": "road_27",
                                    "centerPos": {
                                        "x": 300,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_28": {
                                    "id": "road_28",
                                    "centerPos": {
                                        "x": 300,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_29": {
                                    "id": "road_29",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 219
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 105
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -105
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 105
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -105
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_30": {
                                    "id": "road_30",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_31": {
                                    "id": "road_31",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_32": {
                                    "id": "road_32",
                                    "centerPos": {
                                        "x": 420,
                                        "y": -630
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_36": {
                                    "id": "road_36",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -825
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_38": {
                                    "id": "road_38",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -825
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_39": {
                                    "id": "road_39",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -825
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_37": {
                                    "id": "road_37",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -825
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 99
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -45
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 45
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -45
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 45
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_15": {
                                    "id": "road_15",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_34": {
                                    "id": "road_34",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_40": {
                                    "id": "road_40",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -1005
                                    },
                                    "size": {
                                        "x": 53.9999962,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -22.4999962,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0.0000610351563
                                    },
                                    "inflectionList": []
                                },
                                "road_41": {
                                    "id": "road_41",
                                    "centerPos": {
                                        "x": 82.49999,
                                        "y": -1005
                                    },
                                    "size": {
                                        "x": 53.99998,
                                        "y": 9.000061
                                    },
                                    "startPos": {
                                        "x": -22.4999886,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.4999924,
                                        "y": -0.0000610351563
                                    },
                                    "inflectionList": []
                                },
                                "road_42": {
                                    "id": "road_42",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -1005
                                    },
                                    "size": {
                                        "x": 53.9999962,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.4999962,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                }
                            },
                            "nodePosMap": {
                                "node_0": {
                                    "position": {
                                        "x": 135,
                                        "y": -330
                                    }
                                },
                                "node_1": {
                                    "position": {
                                        "x": 240,
                                        "y": -255
                                    }
                                },
                                "node_2": {
                                    "position": {
                                        "x": 240,
                                        "y": -330
                                    }
                                },
                                "node_3": {
                                    "position": {
                                        "x": 240,
                                        "y": -405
                                    }
                                },
                                "node_4": {
                                    "position": {
                                        "x": 345,
                                        "y": -330
                                    }
                                },
                                "node_5": {
                                    "position": {
                                        "x": 480,
                                        "y": -195
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 480,
                                        "y": -270
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 480.000031,
                                        "y": -405
                                    }
                                },
                                "node_8": {
                                    "position": {
                                        "x": 480,
                                        "y": -480
                                    }
                                },
                                "node_9": {
                                    "position": {
                                        "x": 615,
                                        "y": -435
                                    }
                                },
                                "node_10": {
                                    "position": {
                                        "x": 765,
                                        "y": -435
                                    }
                                },
                                "node_11": {
                                    "position": {
                                        "x": 135,
                                        "y": -630
                                    }
                                },
                                "node_12": {
                                    "position": {
                                        "x": 255,
                                        "y": -540
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 255,
                                        "y": -630
                                    }
                                },
                                "node_14": {
                                    "position": {
                                        "x": 375,
                                        "y": -630
                                    }
                                },
                                "node_15": {
                                    "position": {
                                        "x": 555,
                                        "y": -630
                                    }
                                },
                                "node_16": {
                                    "position": {
                                        "x": 480,
                                        "y": -630
                                    }
                                },
                                "node_17": {
                                    "position": {
                                        "x": 135,
                                        "y": -780
                                    }
                                },
                                "node_18": {
                                    "position": {
                                        "x": 255,
                                        "y": -780
                                    }
                                },
                                "node_19": {
                                    "position": {
                                        "x": 254.999985,
                                        "y": -870
                                    }
                                },
                                "node_20": {
                                    "position": {
                                        "x": 390,
                                        "y": -780
                                    }
                                },
                                "node_21": {
                                    "position": {
                                        "x": 465,
                                        "y": -780
                                    }
                                },
                                "node_22": {
                                    "position": {
                                        "x": 540,
                                        "y": -780
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 615,
                                        "y": -780
                                    }
                                },
                                "node_24": {
                                    "position": {
                                        "x": 375,
                                        "y": -1005.00006
                                    }
                                },
                                "node_25": {
                                    "position": {
                                        "x": 300,
                                        "y": -1005
                                    }
                                },
                                "node_26": {
                                    "position": {
                                        "x": 225,
                                        "y": -1005.00006
                                    }
                                },
                                "node_27": {
                                    "position": {
                                        "x": 150,
                                        "y": -1005
                                    }
                                },
                                "keypoint_1": {
                                    "position": {
                                        "x": 345,
                                        "y": -240
                                    }
                                },
                                "keypoint_2": {
                                    "position": {
                                        "x": 615,
                                        "y": -345
                                    }
                                },
                                "keypoint_3": {
                                    "position": {
                                        "x": 375,
                                        "y": -540
                                    }
                                },
                                "node_29": {
                                    "position": {
                                        "x": 60,
                                        "y": -330
                                    }
                                },
                                "node_30": {
                                    "position": {
                                        "x": 60,
                                        "y": -630
                                    }
                                },
                                "node_31": {
                                    "position": {
                                        "x": 60.0000038,
                                        "y": -780
                                    }
                                },
                                "node_32": {
                                    "position": {
                                        "x": 60.0000038,
                                        "y": -1005
                                    }
                                }
                            },
                            "exclusionDataMap": {
                                "block_dmgreduce": {
                                    "id": "block_dmgreduce",
                                    "pos": {
                                        "x": 240,
                                        "y": -322.5
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 255
                                    }
                                },
                                "lifbos": {
                                    "id": "lifbos",
                                    "pos": {
                                        "x": 480,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                },
                                "liberty": {
                                    "id": "liberty",
                                    "pos": {
                                        "x": 480,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                },
                                "vofwiz": {
                                    "id": "vofwiz",
                                    "pos": {
                                        "x": 517.5,
                                        "y": -622.5
                                    },
                                    "size": {
                                        "x": 165,
                                        "y": 105
                                    }
                                },
                                "cost": {
                                    "id": "cost",
                                    "pos": {
                                        "x": 502.5,
                                        "y": -772.5
                                    },
                                    "size": {
                                        "x": 315,
                                        "y": 105
                                    }
                                },
                                "squad": {
                                    "id": "squad",
                                    "pos": {
                                        "x": 262.5,
                                        "y": -997.5
                                    },
                                    "size": {
                                        "x": 315,
                                        "y": 105
                                    }
                                }
                            }
                        },
                        "bagViewData": null,
                        "bagDataMap": {},
                        "exclusionDataMap": {
                            "block_dmgreduce": {
                                "defaultSlotId": null
                            },
                            "lifbos": {
                                "defaultSlotId": null
                            },
                            "liberty": {
                                "defaultSlotId": null
                            },
                            "vofwiz": {
                                "defaultSlotId": null
                            },
                            "cost": {
                                "defaultSlotId": null
                            },
                            "squad": {
                                "defaultSlotId": null
                            }
                        },
                        "dimensionItemList": [
                            {
                                "desc": "战略",
                                "maxScore": 90
                            },
                            {
                                "desc": "技巧",
                                "maxScore": 130
                            },
                            {
                                "desc": "调遣",
                                "maxScore": 40
                            },
                            {
                                "desc": "应变",
                                "maxScore": 30
                            },
                            {
                                "desc": "洞察",
                                "maxScore": 30
                            },
                            {
                                "desc": "筹划",
                                "maxScore": 10
                            }
                        ],
                        "rewardNodeDataMap": {},
                        "dailyRuneUnlcokDataMap": {},
                        "nodeDataMap": {
                            "node_0": {
                                "runeId": "char_blockcnt_max_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_29",
                                    "node_1",
                                    "node_3"
                                ]
                            },
                            "node_1": {
                                "runeId": "enemy_block_dmgreduce_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "block_dmgreduce",
                                "adjacentNodeList": [
                                    "node_0",
                                    "node_4"
                                ]
                            },
                            "node_2": {
                                "runeId": "enemy_block_dmgreduce_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "block_dmgreduce",
                                "adjacentNodeList": [
                                    "node_0",
                                    "node_4"
                                ]
                            },
                            "node_3": {
                                "runeId": "enemy_block_dmgreduce_3",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "block_dmgreduce",
                                "adjacentNodeList": [
                                    "node_0",
                                    "node_4"
                                ]
                            },
                            "node_4": {
                                "runeId": "level_hidden_prison",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_6",
                                    "node_7",
                                    "keypoint_1",
                                    "node_5",
                                    "node_8",
                                    "node_1",
                                    "node_3"
                                ]
                            },
                            "node_5": {
                                "runeId": "enemy_lifbos_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "lifbos",
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_6": {
                                "runeId": "enemy_lifbos_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "lifbos",
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_7": {
                                "runeId": "enemy_lifbos_liberty_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "liberty",
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_8": {
                                "runeId": "enemy_lifbos_liberty_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "liberty",
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_9"
                                ]
                            },
                            "node_9": {
                                "runeId": "level_hidden_jw",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_6",
                                    "node_10",
                                    "keypoint_2",
                                    "node_5",
                                    "node_7",
                                    "node_8"
                                ]
                            },
                            "node_10": {
                                "runeId": "enemy_jakill_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_9"
                                ]
                            },
                            "node_11": {
                                "runeId": "enemy_attackspeed_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_13",
                                    "node_30"
                                ]
                            },
                            "node_12": {
                                "runeId": "enemy_hp_3",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_11",
                                    "node_14"
                                ]
                            },
                            "node_13": {
                                "runeId": "enemy_lidbox_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_11",
                                    "node_14"
                                ]
                            },
                            "node_14": {
                                "runeId": "enemy_vofwiz_tal_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_13",
                                    "node_16",
                                    "keypoint_3",
                                    "node_15"
                                ]
                            },
                            "node_15": {
                                "runeId": "enemy_vofwiz_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "vofwiz",
                                "adjacentNodeList": [
                                    "node_14"
                                ]
                            },
                            "node_16": {
                                "runeId": "enemy_vofwiz_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "vofwiz",
                                "adjacentNodeList": [
                                    "node_14"
                                ]
                            },
                            "node_17": {
                                "runeId": "global_lifepoint_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_19",
                                    "node_31"
                                ]
                            },
                            "node_18": {
                                "runeId": "global_forbidloc_right",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_17",
                                    "node_23",
                                    "node_20",
                                    "node_21",
                                    "node_22"
                                ]
                            },
                            "node_19": {
                                "runeId": "global_forbidloc_high",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_17",
                                    "node_20",
                                    "node_22",
                                    "node_23",
                                    "node_21"
                                ]
                            },
                            "node_20": {
                                "runeId": "char_guardspecial_cost_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "cost",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_19"
                                ]
                            },
                            "node_21": {
                                "runeId": "char_guardspecial_cost_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "cost",
                                "adjacentNodeList": [
                                    "node_19",
                                    "node_18"
                                ]
                            },
                            "node_22": {
                                "runeId": "char_cost_healerdefender_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "cost",
                                "adjacentNodeList": [
                                    "node_19",
                                    "node_18"
                                ]
                            },
                            "node_23": {
                                "runeId": "char_cost_healerdefender_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "cost",
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_19"
                                ]
                            },
                            "node_24": {
                                "runeId": "global_squadnum_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "squad",
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "node_25": {
                                "runeId": "global_squadnum_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "squad",
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "node_26": {
                                "runeId": "global_pcharnum_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "squad",
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "node_27": {
                                "runeId": "global_pcharnum_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "squad",
                                "adjacentNodeList": [
                                    "node_32"
                                ]
                            },
                            "keypoint_2": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_9"
                                ]
                            },
                            "keypoint_3": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_14"
                                ]
                            },
                            "node_29": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_0"
                                ]
                            },
                            "node_30": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_11"
                                ]
                            },
                            "node_31": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_17"
                                ]
                            },
                            "node_32": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24",
                                    "node_25",
                                    "node_26",
                                    "node_27"
                                ]
                            },
                            "keypoint_1": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4"
                                ]
                            }
                        },
                        "runeDataMap": {
                            "char_blockcnt_max_1": {
                                "runeId": "char_blockcnt_max_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_blockcnt_max_1",
                                "runeName": "环境：狭路相逢",
                                "score": 30,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_blockcnt_max_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "所有干员的阻挡数变为<@crisisv2.nag>{value}</>",
                                    "runes": [
                                        {
                                            "key": "char_blockcnt_max",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 20
                            },
                            "enemy_block_dmgreduce_1": {
                                "runeId": "enemy_block_dmgreduce_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_block_dmgreduce_1",
                                "runeName": "目标：专心一致I",
                                "score": 10,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_block_dmgreduce_1",
                                    "points": 0,
                                    "mutexGroupKey": "duel",
                                    "description": "所有敌人受到的来自阻挡者之外的伤害降低<@crisisv2.nag>25%</>",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "enemy_block_damage_reduce"
                                                },
                                                {
                                                    "key": "damage_scale",
                                                    "value": 0.75
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 41
                            },
                            "enemy_block_dmgreduce_2": {
                                "runeId": "enemy_block_dmgreduce_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_block_dmgreduce_2",
                                "runeName": "目标：专心一致II",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_block_dmgreduce_2",
                                    "points": 0,
                                    "mutexGroupKey": "duel",
                                    "description": "所有敌人受到的来自阻挡者之外的伤害降低<@crisisv2.nag>50%</>",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "enemy_block_damage_reduce"
                                                },
                                                {
                                                    "key": "damage_scale",
                                                    "value": 0.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 29
                            },
                            "enemy_block_dmgreduce_3": {
                                "runeId": "enemy_block_dmgreduce_3",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_block_dmgreduce_3",
                                "runeName": "目标：专心一致III",
                                "score": 40,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_block_dmgreduce_3",
                                    "points": 0,
                                    "mutexGroupKey": "duel",
                                    "description": "所有敌人受到的来自阻挡者之外的伤害降低<@crisisv2.nag>100%</>",
                                    "runes": [
                                        {
                                            "key": "env_gbuff_new",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "enemy_block_damage_reduce"
                                                },
                                                {
                                                    "key": "damage_scale",
                                                    "value": 0
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 1
                            },
                            "level_hidden_prison": {
                                "runeId": "level_hidden_prison",
                                "runeGroupId": null,
                                "runeIcon": "g_level_hidden_prison",
                                "runeName": "环境：禁闭区",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "level_hidden_prison",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场中出现<禁锢装置>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "trap1"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 99
                            },
                            "enemy_lifbos_1": {
                                "runeId": "enemy_lifbos_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_lifbos_liberty_1",
                                "runeName": "重犯：自律I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_lifbos_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_lifbos",
                                    "description": "<重犯>的生命值提高<@crisisv2.nag>{max_hp:0%}</>，攻击力提升<@crisisv2.nag>{atk:0%}</>，防御力提升<@crisisv2.nag>{def}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.4
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.9
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 150
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 42
                            },
                            "enemy_lifbos_2": {
                                "runeId": "enemy_lifbos_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_lifbos_liberty_2",
                                "runeName": "重犯：自律II",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_lifbos_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_lifbos",
                                    "description": "<重犯>的生命值提高<@crisisv2.nag>{max_hp:0%}</>，攻击力提升<@crisisv2.nag>{atk:0%}</>，防御力提升<@crisisv2.nag>{def}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.8
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.8
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 300
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 30
                            },
                            "enemy_lifbos_liberty_1": {
                                "runeId": "enemy_lifbos_liberty_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_lifbos_liberty_1",
                                "runeName": " 重犯：谋划已久I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_lifbos_liberty_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_lifbos",
                                    "description": "<重犯>解放后攻击力额外提升<@crisisv2.nag>{liberty.atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_talent_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "liberty.atk",
                                                    "value": 0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 43
                            },
                            "enemy_lifbos_liberty_2": {
                                "runeId": "enemy_lifbos_liberty_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_lifbos_liberty_2",
                                "runeName": "重犯：谋划已久II",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_lifbos_liberty_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_lifbos",
                                    "description": "<重犯>解放后攻击力额外提升<@crisisv2.nag>{liberty.atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_talent_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1121_lifbos"
                                                },
                                                {
                                                    "key": "liberty.atk",
                                                    "value": 0.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "level_hidden_jw": {
                                "runeId": "level_hidden_jw",
                                "runeGroupId": null,
                                "runeIcon": "g_level_hidden_jw",
                                "runeName": "环 境：越狱",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "level_hidden_jw",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_champion",
                                    "description": "战场中出现<杰斯顿·威廉姆斯>和额外的<普通囚犯>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "jw"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 44
                            },
                            "enemy_jakill_1": {
                                "runeId": "enemy_jakill_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_jakill_1",
                                "runeName": "杰斯顿：憎恨",
                                "score": 30,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_jakill_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "<杰斯顿·威廉姆斯>最大生命值提升<@crisisv2.nag>{max_hp:0%}</>，攻击力提升<@crisisv2.nag>{atk:0%}</>，狱警形态时法术抗性提升<@crisisv2.nag>{enhance.magic_resistance}</>，杀手形态时，“装甲穿刺”防御力无视量提升至<@crisisv2.nag>80%</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1516_jakill"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 2.3
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 1.1
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_talent_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1516_jakill"
                                                },
                                                {
                                                    "key": "enhance.magic_resistance",
                                                    "value": 20
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_skill_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1516_jakill"
                                                },
                                                {
                                                    "key": "def_penetrate",
                                                    "value": 0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 23
                            },
                            "enemy_attackspeed_1": {
                                "runeId": "enemy_attackspeed_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_attackspeed_1",
                                "runeName": "反列队",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_attackspeed_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_attackspeed",
                                    "description": "所有敌人的攻击速度提升<@crisisv2.nag>{attack_speed}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "attack_speed",
                                                    "value": 30
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 32
                            },
                            "enemy_hp_3": {
                                "runeId": "enemy_hp_3",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_hp_3",
                                "runeName": "源石环境：活性III",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_3",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hp",
                                    "description": "所有敌人的生命值提升<@crisisv2.nag>{max_hp:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 22
                            },
                            "enemy_lidbox_2": {
                                "runeId": "enemy_lidbox_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_lidbox_2",
                                "runeName": "拳手囚犯：健身II",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_lidbox_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_lidbox",
                                    "description": "<拳手囚犯>的最大生命值提升<@crisisv2.nag>{max_hp:0%}</>，防御力提升<@crisisv2.nag>{def}</>，法抗提升<@crisisv2.nag>{magic_resistance}</>，未解放时不再降低攻击速度",
                                    "runes": [
                                        {
                                            "key": "enemy_talent_blackb_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1118_lidbox"
                                                },
                                                {
                                                    "key": "confinement.attack_speed",
                                                    "value": 50
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1118_lidbox"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.8
                                                }
                                            ]
                                        },
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1118_lidbox"
                                                },
                                                {
                                                    "key": "def",
                                                    "value": 160
                                                },
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 35
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 34
                            },
                            "enemy_vofwiz_tal_1": {
                                "runeId": "enemy_vofwiz_tal_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_vofwiz_tal_1",
                                "runeName": "术师囚犯：拉帮结派",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_vofwiz_tal_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_vofwiz",
                                    "description": "<术师囚犯>在场的敌人攻击速度提升量变为<@crisisv2.nag>{atkspeedup.attack_speed}</>倍。",
                                    "runes": [
                                        {
                                            "key": "enemy_talent_blackb_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1120_vofwiz"
                                                },
                                                {
                                                    "key": "atkspeedup.attack_speed",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 35
                            },
                            "enemy_vofwiz_2": {
                                "runeId": "enemy_vofwiz_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_vofwiz_2",
                                "runeName": "术师囚犯：隐忍II",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_vofwiz_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_vofwiz",
                                    "description": "<术师囚犯>的最大生命值提升<@crisisv2.nag>{max_hp:0%}</>，攻击力提升<@crisisv2.nag>{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1120_vofwiz"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 1.6
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.6
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 36
                            },
                            "enemy_vofwiz_1": {
                                "runeId": "enemy_vofwiz_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_vofwiz_1",
                                "runeName": "术师囚犯：隐忍I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "enemy_vofwiz_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_vofwiz",
                                    "description": "<术师囚犯>的最大生命值提升<@crisisv2.nag>{max_hp:0%}</>，攻击力提升<@crisisv2.nag>{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1120_vofwiz"
                                                },
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.8
                                                },
                                                {
                                                    "key": "atk",
                                                    "value": 0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 45
                            },
                            "global_lifepoint_1": {
                                "runeId": "global_lifepoint_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_lifepoint_1",
                                "runeName": "目标：最后防线",
                                "score": 10,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "global_lifepoint_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "我方防御点可承受的敌方数量变为<@crisisv2.nag>{value}</>",
                                    "runes": [
                                        {
                                            "key": "global_lifepoint",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 46
                            },
                            "global_forbidloc_right": {
                                "runeId": "global_forbidloc_right",
                                "runeGroupId": null,
                                "runeIcon": "g_melee_forbid_1",
                                "runeName": "环境：地表施 工",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_forbidloc_right",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场<@crisisv2.nag>4</>个地面位置不可部署",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(6,11)|(5,11)|(4,11)|(3,11)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 47
                            },
                            "global_forbidloc_high": {
                                "runeId": "global_forbidloc_high",
                                "runeGroupId": null,
                                "runeIcon": "g_range_forbid_2",
                                "runeName": "环境：高台施工",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_forbidloc_high",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场<@crisisv2.nag>5</>个高台位置不可部署",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(1,5)|(2,5)|(3,10)|(4,10)|(3,9)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 48
                            },
                            "char_guardspecial_cost_1": {
                                "runeId": "char_guardspecial_cost_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_guardspecial_cost_1",
                                "runeName": "目标：应急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_guardspecial_cost_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "近卫和特种干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 65,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 49
                            },
                            "char_guardspecial_cost_2": {
                                "runeId": "char_guardspecial_cost_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_guardspecial_cost_2",
                                "runeName": "目标：应急合约",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_guardspecial_cost_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "近卫和特 种干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 65,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 37
                            },
                            "char_cost_healerdefender_1": {
                                "runeId": "char_cost_healerdefender_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_healerdefender_1",
                                "runeName": "目标：应急合约",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_healerdefender_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "重装和医疗干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 12,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 50
                            },
                            "char_cost_healerdefender_2": {
                                "runeId": "char_cost_healerdefender_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_healerdefender_2",
                                "runeName": "目标：应急合约",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_healerdefender_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "重装和医疗干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 12,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 38
                            },
                            "global_squadnum_1": {
                                "runeId": "global_squadnum_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_squadnum_1",
                                "runeName": "目标：隐秘行动I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_squadnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "最多可编入<@crisisv2.nag>10</>名干员进入作战",
                                    "runes": [
                                        {
                                            "key": "global_squad_num_limit",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 10
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 52
                            },
                            "global_squadnum_2": {
                                "runeId": "global_squadnum_2",
                                "runeGroupId": null,
                                "runeIcon": "g_global_squadnum_2",
                                "runeName": "目标：隐秘行动II",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_squadnum_2",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "最多可编入<@crisisv2.nag>7</>名干员进入作战",
                                    "runes": [
                                        {
                                            "key": "global_squad_num_limit",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 7
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 40
                            },
                            "global_pcharnum_1": {
                                "runeId": "global_pcharnum_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_pcharnum_1",
                                "runeName": "环境：交战区I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "可同时部署的单位数量减少至<@crisisv2.nag>7</>",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 51
                            },
                            "global_pcharnum_2": {
                                "runeId": "global_pcharnum_2",
                                "runeGroupId": null,
                                "runeIcon": "g_global_pcharnum_2",
                                "runeName": "环境：交战区II",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_2",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "可同时部署的单位数量减少至<@crisisv2.nag>5</>",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 39
                            }
                        }
                    },
                    "crisis_v2_01-05": {
                        "commentDataMap": {},
                        "challengeNodeDataMap": {
                            "keypoint_1": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "200"
                                ],
                                "slotId": "keypoint_1",
                                "previewTitle": "标准能力测试",
                                "previewDesc": "完成一次作战，同时评分大于等于200分",
                                "missionSortId": 10,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "4001",
                                            "count": 5000,
                                            "type": "GOLD"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_2": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_7;node_11",
                                    "1"
                                ],
                                "slotId": "keypoint_2",
                                "previewTitle": "环境适应训练",
                                "previewDesc": "完成一次作战，并携带【环境：平台拆除】或【目标：深度渗透I】中的任意一个",
                                "missionSortId": 11,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "2002",
                                            "count": 10,
                                            "type": "CARD_EXP"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 1,
                                "slotIdList": [
                                    "node_7",
                                    "node_11"
                                ]
                            },
                            "keypoint_3": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_20;node_21",
                                    "2"
                                ],
                                "slotId": "keypoint_3",
                                "previewTitle": " 装甲击破训练",
                                "previewDesc": "完成一次作战，并同时携带【游击队盾卫：壁垒之志I】、【游击队盾卫：坚实之躯I】",
                                "missionSortId": 12,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "3302",
                                            "count": 5,
                                            "type": "MATERIAL"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 2,
                                "slotIdList": [
                                    "node_20",
                                    "node_21"
                                ]
                            }
                        },
                        "groupDescDataMap": {},
                        "roadRelationDataMap": {
                            "road_0": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_2"
                                }
                            },
                            "road_1": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_6"
                                }
                            },
                            "road_2": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_3": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_4": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_5"
                                }
                            },
                            "road_5": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_6": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_7": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_5"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_8": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_8"
                                }
                            },
                            "road_9": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_10": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_6"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_10"
                                }
                            },
                            "road_11": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_8"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_12": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_9"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_13": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_10"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_14": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_1"
                                }
                            },
                            "road_20": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_16": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_17": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_18": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_16"
                                }
                            },
                            "road_19": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_16"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_17"
                                }
                            },
                            "road_15": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_14"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                }
                            },
                            "road_21": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_22": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_23": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_13"
                                }
                            },
                            "road_24": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_20"
                                }
                            },
                            "road_25": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_26": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_22"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            },
                            "road_27": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_23"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_28": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_24"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            },
                            "road_29": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            }
                        },
                        "bagRoadDataMap": {},
                        "nodeViewData": {
                            "width": 885,
                            "height": 985,
                            "bagPosMap": null,
                            "roadPosMap": {
                                "road_0": {
                                    "id": "road_0",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_1": {
                                    "id": "road_1",
                                    "centerPos": {
                                        "x": 135,
                                        "y": -262.5
                                    },
                                    "size": {
                                        "x": 9,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": 0,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 0,
                                        "y": -37.5
                                    },
                                    "inflectionList": []
                                },
                                "road_2": {
                                    "id": "road_2",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_3": {
                                    "id": "road_3",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9.000015
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": -0.0000152587891
                                    },
                                    "inflectionList": []
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9.000015
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": -0.0000152587891
                                    },
                                    "inflectionList": []
                                },
                                "road_5": {
                                    "id": "road_5",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_6": {
                                    "id": "road_6",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9.000015
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": -0.0000152587891
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_7": {
                                    "id": "road_7",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9.000015
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": -0.0000152587891
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_8": {
                                    "id": "road_8",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_9": {
                                    "id": "road_9",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_10": {
                                    "id": "road_10",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_11": {
                                    "id": "road_11",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_12": {
                                    "id": "road_12",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_13": {
                                    "id": "road_13",
                                    "centerPos": {
                                        "x": 577.5,
                                        "y": -330
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_14": {
                                    "id": "road_14",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -195
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_20": {
                                    "id": "road_20",
                                    "centerPos": {
                                        "x": 540,
                                        "y": -457.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 30,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_16": {
                                    "id": "road_16",
                                    "centerPos": {
                                        "x": 127.5,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 144,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -67.5,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 67.5,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_17": {
                                    "id": "road_17",
                                    "centerPos": {
                                        "x": 382.5,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 144,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -67.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 67.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_18": {
                                    "id": "road_18",
                                    "centerPos": {
                                        "x": 127.5,
                                        "y": -510
                                    },
                                    "size": {
                                        "x": 144,
                                        "y": 9.00003052
                                    },
                                    "startPos": {
                                        "x": -67.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 67.5,
                                        "y": -0.0000305175781
                                    },
                                    "inflectionList": []
                                },
                                "road_19": {
                                    "id": "road_19",
                                    "centerPos": {
                                        "x": 315,
                                        "y": -510
                                    },
                                    "size": {
                                        "x": 129,
                                        "y": 9.00003052
                                    },
                                    "startPos": {
                                        "x": -60,
                                        "y": -0.0000305175781
                                    },
                                    "endPos": {
                                        "x": 60,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_15": {
                                    "id": "road_15",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_21": {
                                    "id": "road_21",
                                    "centerPos": {
                                        "x": 487.5,
                                        "y": -510
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_22": {
                                    "id": "road_22",
                                    "centerPos": {
                                        "x": 630,
                                        "y": -510
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9.00003052
                                    },
                                    "startPos": {
                                        "x": 30,
                                        "y": -0.0000305175781
                                    },
                                    "endPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_23": {
                                    "id": "road_23",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -690
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_24": {
                                    "id": "road_24",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -690
                                    },
                                    "size": {
                                        "x": 219,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -105,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 105,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_25": {
                                    "id": "road_25",
                                    "centerPos": {
                                        "x": 210,
                                        "y": -712.5
                                    },
                                    "size": {
                                        "x": 99,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -45,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 45,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -15,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -15,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_26": {
                                    "id": "road_26",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -600
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_27": {
                                    "id": "road_27",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -600
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_28": {
                                    "id": "road_28",
                                    "centerPos": {
                                        "x": 270,
                                        "y": -600
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_29": {
                                    "id": "road_29",
                                    "centerPos": {
                                        "x": 375,
                                        "y": -600
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                }
                            },
                            "nodePosMap": {
                                "node_0": {
                                    "position": {
                                        "x": 60,
                                        "y": -195
                                    }
                                },
                                "node_1": {
                                    "position": {
                                        "x": 135,
                                        "y": -195
                                    }
                                },
                                "node_2": {
                                    "position": {
                                        "x": 225,
                                        "y": -195
                                    }
                                },
                                "node_3": {
                                    "position": {
                                        "x": 330,
                                        "y": -195
                                    }
                                },
                                "node_4": {
                                    "position": {
                                        "x": 405,
                                        "y": -195
                                    }
                                },
                                "node_5": {
                                    "position": {
                                        "x": 480,
                                        "y": -195
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 135,
                                        "y": -330
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 660,
                                        "y": -195
                                    }
                                },
                                "node_8": {
                                    "position": {
                                        "x": 330,
                                        "y": -330
                                    }
                                },
                                "node_9": {
                                    "position": {
                                        "x": 405,
                                        "y": -330
                                    }
                                },
                                "node_10": {
                                    "position": {
                                        "x": 480,
                                        "y": -330
                                    }
                                },
                                "node_11": {
                                    "position": {
                                        "x": 660,
                                        "y": -330
                                    }
                                },
                                "node_12": {
                                    "position": {
                                        "x": 60,
                                        "y": -510
                                    }
                                },
                                "keypoint_1": {
                                    "position": {
                                        "x": 315,
                                        "y": -435
                                    }
                                },
                                "node_14": {
                                    "position": {
                                        "x": 225,
                                        "y": -435
                                    }
                                },
                                "node_15": {
                                    "position": {
                                        "x": 480,
                                        "y": -435
                                    }
                                },
                                "node_16": {
                                    "position": {
                                        "x": 225,
                                        "y": -510.000031
                                    }
                                },
                                "node_17": {
                                    "position": {
                                        "x": 405,
                                        "y": -510
                                    }
                                },
                                "node_18": {
                                    "position": {
                                        "x": 60,
                                        "y": -690
                                    }
                                },
                                "node_19": {
                                    "position": {
                                        "x": 570,
                                        "y": -510
                                    }
                                },
                                "keypoint_2": {
                                    "position": {
                                        "x": 660,
                                        "y": -510
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 135,
                                        "y": -690
                                    }
                                },
                                "node_20": {
                                    "position": {
                                        "x": 405,
                                        "y": -690
                                    }
                                },
                                "node_21": {
                                    "position": {
                                        "x": 285,
                                        "y": -735
                                    }
                                },
                                "node_22": {
                                    "position": {
                                        "x": 60,
                                        "y": -600
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 135,
                                        "y": -600
                                    }
                                },
                                "node_24": {
                                    "position": {
                                        "x": 225,
                                        "y": -600
                                    }
                                },
                                "node_25": {
                                    "position": {
                                        "x": 315,
                                        "y": -600
                                    }
                                },
                                "keypoint_3": {
                                    "position": {
                                        "x": 405,
                                        "y": -600
                                    }
                                }
                            },
                            "exclusionDataMap": {
                                "exclusion_1": {
                                    "id": "exclusion_1",
                                    "pos": {
                                        "x": 405,
                                        "y": -187.5
                                    },
                                    "size": {
                                        "x": 240,
                                        "y": 105
                                    }
                                },
                                "exclusion_2": {
                                    "id": "exclusion_2",
                                    "pos": {
                                        "x": 405,
                                        "y": -322.5
                                    },
                                    "size": {
                                        "x": 240,
                                        "y": 105
                                    }
                                }
                            }
                        },
                        "bagViewData": null,
                        "bagDataMap": {},
                        "exclusionDataMap": {
                            "exclusion_1": {
                                "defaultSlotId": null
                            },
                            "exclusion_2": {
                                "defaultSlotId": null
                            }
                        },
                        "dimensionItemList": [
                            {
                                "desc": "战略",
                                "maxScore": 90
                            },
                            {
                                "desc": "技巧",
                                "maxScore": 60
                            },
                            {
                                "desc": "调遣",
                                "maxScore": 90
                            },
                            {
                                "desc": "应变",
                                "maxScore": 50
                            },
                            {
                                "desc": "洞察",
                                "maxScore": 30
                            },
                            {
                                "desc": "筹划",
                                "maxScore": 30
                            }
                        ],
                        "rewardNodeDataMap": {},
                        "dailyRuneUnlcokDataMap": {},
                        "nodeDataMap": {
                            "node_0": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1"
                                ]
                            },
                            "node_1": {
                                "runeId": "global_pcharnum_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_6",
                                    "node_0"
                                ]
                            },
                            "node_2": {
                                "runeId": "global_canoe2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1",
                                    "node_3",
                                    "node_4",
                                    "node_5"
                                ]
                            },
                            "node_3": {
                                "runeId": "char_atkspeed_p1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_1",
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_7"
                                ]
                            },
                            "node_4": {
                                "runeId": "char_atk_p1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_1",
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_7"
                                ]
                            },
                            "node_5": {
                                "runeId": "char_skill_cd_p1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_1",
                                "adjacentNodeList": [
                                    "node_2",
                                    "node_7"
                                ]
                            },
                            "node_6": {
                                "runeId": "char_cost_sniperguard_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1",
                                    "node_8",
                                    "node_9",
                                    "node_10"
                                ]
                            },
                            "node_7": {
                                "runeId": "global_canoe1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_4",
                                    "node_5"
                                ]
                            },
                            "node_8": {
                                "runeId": "char_atkspeed_p2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_2",
                                "adjacentNodeList": [
                                    "node_6",
                                    "node_11"
                                ]
                            },
                            "node_9": {
                                "runeId": "char_atk_p2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_2",
                                "adjacentNodeList": [
                                    "node_6",
                                    "node_11"
                                ]
                            },
                            "node_10": {
                                "runeId": "char_respawntime_p1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_2",
                                "adjacentNodeList": [
                                    "node_6",
                                    "node_11"
                                ]
                            },
                            "node_11": {
                                "runeId": "global_costrecovery_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_8",
                                    "node_9",
                                    "node_10"
                                ]
                            },
                            "node_12": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_14",
                                    "node_16"
                                ]
                            },
                            "node_13": {
                                "runeId": "level_hidden_sotisd_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_20",
                                    "node_21"
                                ]
                            },
                            "node_14": {
                                "runeId": "char_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "keypoint_1"
                                ]
                            },
                            "node_15": {
                                "runeId": "char_hp_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_19",
                                    "keypoint_1"
                                ]
                            },
                            "node_16": {
                                "runeId": "enemy_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_17"
                                ]
                            },
                            "node_17": {
                                "runeId": "enemy_atk_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_16",
                                    "node_19"
                                ]
                            },
                            "node_18": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13"
                                ]
                            },
                            "node_19": {
                                "runeId": "enemy_mag1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_15",
                                    "node_17",
                                    "keypoint_2"
                                ]
                            },
                            "keypoint_1": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_15",
                                    "node_14"
                                ]
                            },
                            "keypoint_2": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_19"
                                ]
                            },
                            "node_20": {
                                "runeId": "sotisd_def1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13"
                                ]
                            },
                            "node_21": {
                                "runeId": "sotisd_hp1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13"
                                ]
                            },
                            "node_22": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_23"
                                ]
                            },
                            "node_23": {
                                "runeId": "sotisp_hp1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_22",
                                    "node_24"
                                ]
                            },
                            "node_24": {
                                "runeId": "sotisp_atk1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_23",
                                    "node_25"
                                ]
                            },
                            "node_25": {
                                "runeId": "sotisp_def1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24",
                                    "keypoint_3"
                                ]
                            },
                            "keypoint_3": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_25"
                                ]
                            }
                        },
                        "runeDataMap": {
                            "global_pcharnum_1": {
                                "runeId": "global_pcharnum_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_pcharnum_3",
                                "runeName": "环境：单一性测试",
                                "score": 50,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "self",
                                    "description": "可同时部署的单位数量减少至1",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -7
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 1
                            },
                            "global_canoe2": {
                                "runeId": "global_canoe2",
                                "runeGroupId": null,
                                "runeIcon": "g_global_canoe2",
                                "runeName": "环境：平台翻修",
                                "score": 20,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "global_canoe2",
                                    "points": 0,
                                    "mutexGroupKey": "normal",
                                    "description": "可部署的<特制水上平台>减少至<@crisisv2.nag>1</>",
                                    "runes": [
                                        {
                                            "key": "global_token_cnt_add",
                                            "selector": {
                                                "professionMask": 256,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "trap",
                                                    "valueStr": "trap_040_canoe"
                                                },
                                                {
                                                    "key": "value",
                                                    "value": -4
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 30
                            },
                            "char_atkspeed_p1": {
                                "runeId": "char_atkspeed_p1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atkspeed_p1",
                                "runeName": "协同战术：突击",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_atkspeed_p1",
                                    "points": 0,
                                    "mutexGroupKey": "self1",
                                    "description": "所有我方单位的攻击速度<@crisisv2.pos>+{attack_speed}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "attack_speed",
                                                    "value": 30
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 50
                            },
                            "char_atk_p1": {
                                "runeId": "char_atk_p1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_p1",
                                "runeName": " 协同战术：军械",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_atk_p1",
                                    "points": 0,
                                    "mutexGroupKey": "self1",
                                    "description": "所有我方单位的攻击力提升<@crisisv2.pos>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 51
                            },
                            "char_skill_cd_p1": {
                                "runeId": "char_skill_cd_p1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_skillcd_p1",
                                "runeName": "协同战术：技巧",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_skill_cd_p1",
                                    "points": 0,
                                    "mutexGroupKey": "self2",
                                    "description": "所有我方技能的技力消耗减少<@crisisv2.pos>25%</>",
                                    "runes": [
                                        {
                                            "key": "char_skill_cd_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 0.75
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 52
                            },
                            "char_cost_sniperguard_2": {
                                "runeId": "char_cost_sniperguard_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_sniperguard_2",
                                "runeName": "目标：应急合约",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_sniperguard_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": " 近卫、重装和特种干员的部署费用提升至<@crisisv2.nag>{scale}</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 69,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "global_canoe1": {
                                "runeId": "global_canoe1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_canoe1",
                                "runeName": "环境：平台拆除",
                                "score": 10,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "global_canoe1",
                                    "points": 0,
                                    "mutexGroupKey": "normal",
                                    "description": "场上初始存在的<特制水上平台>减少",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_disable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "g2"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 40
                            },
                            "char_atkspeed_p2": {
                                "runeId": "char_atkspeed_p2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atkspeed_p1",
                                "runeName": "协同战术：突击",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_atkspeed_p2",
                                    "points": 0,
                                    "mutexGroupKey": "self2",
                                    "description": "所有我方单位的攻击速度<@crisisv2.pos>+{attack_speed}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "attack_speed",
                                                    "value": 30
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 53
                            },
                            "char_atk_p2": {
                                "runeId": "char_atk_p2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_p1",
                                "runeName": "协同战术：军械",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_atk_p2",
                                    "points": 0,
                                    "mutexGroupKey": "self2",
                                    "description": "所有我方单位的攻击力提升<@crisisv2.pos>{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 54
                            },
                            "char_respawntime_p1": {
                                "runeId": "char_respawntime_p1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_respawntime_p1",
                                "runeName": "协同战术：重部署",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "char_respawntime_p1",
                                    "points": 0,
                                    "mutexGroupKey": "self1",
                                    "description": "所有我方单位的再部署时间减少<@crisisv2.pos>25%</>",
                                    "runes": [
                                        {
                                            "key": "char_respawntime_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 0.75
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 55
                            },
                            "global_costrecovery_1": {
                                "runeId": "global_costrecovery_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_costrecovery_1",
                                "runeName": "目标：深度渗透I",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_costrecovery_1",
                                    "points": 0,
                                    "mutexGroupKey": "cost_recovery",
                                    "description": "部署费用的自然回复速度降低<@crisisv2.nag>25%</>",
                                    "runes": [
                                        {
                                            "key": "global_cost_recovery_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 1.3333
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 32
                            },
                            "level_hidden_sotisd_1": {
                                "runeId": "level_hidden_sotisd_1",
                                "runeGroupId": null,
                                "runeIcon": "g_level_hidden_sotisd_1",
                                "runeName": "环境：无畏铁壁",
                                "score": 30,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "level_hidden_sotisd_1",
                                    "points": 0,
                                    "mutexGroupKey": "normal",
                                    "description": "场上初始存在额外的<游击队盾卫>",
                                    "runes": [
                                        {
                                            "key": "level_hidden_group_enable",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "key",
                                                    "valueStr": "g1"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 22
                            },
                            "char_hp_1": {
                                "runeId": "char_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_hp_1",
                                "runeName": "源石环境：侵蚀I",
                                "score": 20,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_atk",
                                    "description": "所有我方单位的最大生命值<@crisisv2.nag>{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.15
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 33
                            },
                            "char_hp_2": {
                                "runeId": "char_hp_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_hp_2",
                                "runeName": "源石环境：侵蚀II",
                                "score": 30,
                                "dimension": 3,
                                "packedRune": {
                                    "id": "char_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "char_atk",
                                    "description": "所有我方单位的最大生命值<@crisisv2.nag>{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "char_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": -0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 20
                            },
                            "enemy_atk_1": {
                                "runeId": "enemy_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_1",
                                "runeName": "环境：高价值目标I",
                                "score": 20,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 34
                            },
                            "enemy_atk_2": {
                                "runeId": "enemy_atk_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_2",
                                "runeName": "环境：高价值目标II",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 1023,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 21
                            },
                            "enemy_mag1": {
                                "runeId": "enemy_mag1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_mag1",
                                "runeName": "目标：法术阻滞",
                                "score": 40,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_mag1",
                                    "points": 0,
                                    "mutexGroupKey": "normal",
                                    "description": "所有敌人的法术抗性<@crisisv2.nag>+{magic_resistance}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "magic_resistance",
                                                    "value": 30
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 11
                            },
                            "sotisd_def1": {
                                "runeId": "sotisd_def1",
                                "runeGroupId": null,
                                "runeIcon": "g_sotisd_def1",
                                "runeName": "游击队盾卫：壁垒之志I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "sotisd_def1",
                                    "points": 0,
                                    "mutexGroupKey": "sotisd",
                                    "description": "<游击队盾卫>的防御力<@crisisv2.nag>+{def}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 2000
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1081_sotisd"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "sotisd"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 44
                            },
                            "sotisd_hp1": {
                                "runeId": "sotisd_hp1",
                                "runeGroupId": null,
                                "runeIcon": "g_sotisd_hp1",
                                "runeName": "游击队盾卫：坚实之躯I",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "sotisd_hp1",
                                    "points": 0,
                                    "mutexGroupKey": "sotisd",
                                    "description": "<游击队盾卫>的生命值<@crisisv2.nag>+{max_hp:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.4
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1081_sotisd"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "sotisd"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 35
                            },
                            "sotisp_hp1": {
                                "runeId": "sotisp_hp1",
                                "runeGroupId": null,
                                "runeIcon": "g_sotisp_hp1",
                                "runeName": "游击队狙击手：营养补剂",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "sotisp_hp1",
                                    "points": 0,
                                    "mutexGroupKey": "sotisp",
                                    "description": "<游击队狙击手>的生命值<@crisisv2.nag>+{max_hp:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 1
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1079_sotisp"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "sotisp"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 41
                            },
                            "sotisp_atk1": {
                                "runeId": "sotisp_atk1",
                                "runeGroupId": null,
                                "runeIcon": "g_sotisp_atk1",
                                "runeName": "游击队狙击手：穿甲弩箭",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "sotisp_atk1",
                                    "points": 0,
                                    "mutexGroupKey": "sotisp",
                                    "description": "<游击队狙击手>的攻击力<@crisisv2.nag>+{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.5
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1079_sotisp"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "sotisp"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 42
                            },
                            "sotisp_def1": {
                                "runeId": "sotisp_def1",
                                "runeGroupId": null,
                                "runeIcon": "g_sotisp_def1",
                                "runeName": "游击队狙击手 ：防冲击插板",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "sotisp_def1",
                                    "points": 0,
                                    "mutexGroupKey": "sotisp",
                                    "description": "<游击队狙击手>的防御力<@crisisv2.nag>+{def}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "def",
                                                    "value": 300
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1079_sotisp"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "sotisp"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 43
                            }
                        }
                    },
                    "crisis_v2_01-07": {
                        "commentDataMap": {},
                        "challengeNodeDataMap": {
                            "keypoint_1": {
                                "missionType": "PassWithDimScore",
                                "missionParamList": [
                                    "0;1;2;3;4;5",
                                    "200"
                                ],
                                "slotId": "keypoint_1",
                                "previewTitle": "标准能力测试",
                                "previewDesc": "完成一次作战，同时评分大于等于200分",
                                "missionSortId": 13,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "4001",
                                            "count": 5000,
                                            "type": "GOLD"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 0,
                                "slotIdList": []
                            },
                            "keypoint_2": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_8;node_11",
                                    "1"
                                ],
                                "slotId": "keypoint_2",
                                "previewTitle": "战力规划训练",
                                "previewDesc": "完成一次作战，并携带一个【目标：经费充足I】",
                                "missionSortId": 14,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "2002",
                                            "count": 10,
                                            "type": "CARD_EXP"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 1,
                                "slotIdList": [
                                    "node_8",
                                    "node_11"
                                ]
                            },
                            "keypoint_3": {
                                "missionType": "PassWithRunes",
                                "missionParamList": [
                                    "node_24;node_25;node_26",
                                    "3"
                                ],
                                "slotId": "keypoint_3",
                                "previewTitle": "狂乱平息训练",
                                "previewDesc": "完成一次作战，并同时携带【狂暴宿主投掷手：极度疯狂I】、【狂暴宿主投掷手：异变躯体I】、【狂暴宿主投掷手：骇人怪力I】",
                                "missionSortId": 15,
                                "rewardList": [
                                    {
                                        "reward": {
                                            "id": "CRISIS_SHOP_COIN_V2",
                                            "count": 100,
                                            "type": "CRS_SHOP_COIN_V2"
                                        },
                                        "isTimeLimit": true
                                    },
                                    {
                                        "reward": {
                                            "id": "3302",
                                            "count": 5,
                                            "type": "MATERIAL"
                                        },
                                        "isTimeLimit": false
                                    }
                                ],
                                "requiredSlotCount": 3,
                                "slotIdList": [
                                    "node_24",
                                    "node_25",
                                    "node_26"
                                ]
                            }
                        },
                        "groupDescDataMap": {},
                        "roadRelationDataMap": {
                            "road_0": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_0"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_1"
                                }
                            },
                            "road_2": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_3"
                                }
                            },
                            "road_3": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_4"
                                }
                            },
                            "road_4": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_4"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_5"
                                }
                            },
                            "road_5": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_5"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_6"
                                }
                            },
                            "road_6": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_7"
                                }
                            },
                            "road_7": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_7"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_8"
                                }
                            },
                            "road_8": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_9"
                                }
                            },
                            "road_9": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_9"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_10"
                                }
                            },
                            "road_10": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_10"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_11"
                                }
                            },
                            "road_11": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_13"
                                }
                            },
                            "road_12": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_12"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_14"
                                }
                            },
                            "road_13": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_13"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_14": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_14"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_15"
                                }
                            },
                            "road_15": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_16"
                                }
                            },
                            "road_16": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_15"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_17"
                                }
                            },
                            "road_17": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_19"
                                }
                            },
                            "road_18": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_20"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_24"
                                }
                            },
                            "road_19": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_19"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_20": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_18"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_22"
                                }
                            },
                            "road_21": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_22"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            },
                            "road_22": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_24"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_25"
                                }
                            },
                            "road_23": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_24"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_26"
                                }
                            },
                            "road_1": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_1"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_21"
                                }
                            },
                            "road_25": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_16"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                }
                            },
                            "road_26": {
                                "start": {
                                    "type": "NODE",
                                    "id": "node_17"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "keypoint_2"
                                }
                            },
                            "road_24": {
                                "start": {
                                    "type": "NODE",
                                    "id": "keypoint_3"
                                },
                                "end": {
                                    "type": "NODE",
                                    "id": "node_23"
                                }
                            }
                        },
                        "bagRoadDataMap": {},
                        "nodeViewData": {
                            "width": 885,
                            "height": 1075,
                            "bagPosMap": null,
                            "roadPosMap": {
                                "road_0": {
                                    "id": "road_0",
                                    "centerPos": {
                                        "x": 82.5,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_2": {
                                    "id": "road_2",
                                    "centerPos": {
                                        "x": 180,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 39,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -15,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 15,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_3": {
                                    "id": "road_3",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -187.5
                                    },
                                    "size": {
                                        "x": 68.9999847,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -29.9999847,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_4": {
                                    "id": "road_4",
                                    "centerPos": {
                                        "x": 397.5,
                                        "y": -150
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_5": {
                                    "id": "road_5",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -150
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_6": {
                                    "id": "road_6",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 68.9999847,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -29.9999847,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_7": {
                                    "id": "road_7",
                                    "centerPos": {
                                        "x": 450,
                                        "y": -225
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_8": {
                                    "id": "road_8",
                                    "centerPos": {
                                        "x": 285,
                                        "y": -262.5
                                    },
                                    "size": {
                                        "x": 68.9999847,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -29.9999847,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_9": {
                                    "id": "road_9",
                                    "centerPos": {
                                        "x": 397.5,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 54,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -22.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 22.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_10": {
                                    "id": "road_10",
                                    "centerPos": {
                                        "x": 555,
                                        "y": -300
                                    },
                                    "size": {
                                        "x": 159,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -75,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 75,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_11": {
                                    "id": "road_11",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_12": {
                                    "id": "road_12",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_13": {
                                    "id": "road_13",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_14": {
                                    "id": "road_14",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_15": {
                                    "id": "road_15",
                                    "centerPos": {
                                        "x": 352.5,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_16": {
                                    "id": "road_16",
                                    "centerPos": {
                                        "x": 352.5,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_17": {
                                    "id": "road_17",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -615
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_18": {
                                    "id": "road_18",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_19": {
                                    "id": "road_19",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -615
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_20": {
                                    "id": "road_20",
                                    "centerPos": {
                                        "x": 97.5,
                                        "y": -652.5
                                    },
                                    "size": {
                                        "x": 84,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -37.5,
                                        "y": 37.5
                                    },
                                    "endPos": {
                                        "x": 37.5,
                                        "y": -37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_21": {
                                    "id": "road_21",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -690
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_22": {
                                    "id": "road_22",
                                    "centerPos": {
                                        "x": 292.5,
                                        "y": -780
                                    },
                                    "size": {
                                        "x": 204,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -97.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 97.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_23": {
                                    "id": "road_23",
                                    "centerPos": {
                                        "x": 225,
                                        "y": -802.5
                                    },
                                    "size": {
                                        "x": 69,
                                        "y": 54
                                    },
                                    "startPos": {
                                        "x": -30,
                                        "y": 22.5
                                    },
                                    "endPos": {
                                        "x": 30,
                                        "y": -22.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": 22.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": 0,
                                                "y": -22.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_1": {
                                    "id": "road_1",
                                    "centerPos": {
                                        "x": 367.5,
                                        "y": -615
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_25": {
                                    "id": "road_25",
                                    "centerPos": {
                                        "x": 502.5,
                                        "y": -435
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                },
                                "road_26": {
                                    "id": "road_26",
                                    "centerPos": {
                                        "x": 502.5,
                                        "y": -472.5
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 84
                                    },
                                    "startPos": {
                                        "x": -52.5,
                                        "y": -37.5
                                    },
                                    "endPos": {
                                        "x": 52.5,
                                        "y": 37.5
                                    },
                                    "inflectionList": [
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": -37.5
                                            },
                                            "radius": 10
                                        },
                                        {
                                            "cornerPos": {
                                                "x": -7.5,
                                                "y": 37.5
                                            },
                                            "radius": 10
                                        }
                                    ]
                                },
                                "road_24": {
                                    "id": "road_24",
                                    "centerPos": {
                                        "x": 367.5,
                                        "y": -690
                                    },
                                    "size": {
                                        "x": 114,
                                        "y": 9
                                    },
                                    "startPos": {
                                        "x": 52.5,
                                        "y": 0
                                    },
                                    "endPos": {
                                        "x": -52.5,
                                        "y": 0
                                    },
                                    "inflectionList": []
                                }
                            },
                            "nodePosMap": {
                                "node_0": {
                                    "position": {
                                        "x": 59.9999962,
                                        "y": -225
                                    }
                                },
                                "node_1": {
                                    "position": {
                                        "x": 135,
                                        "y": -225
                                    }
                                },
                                "keypoint_1": {
                                    "position": {
                                        "x": 420,
                                        "y": -615
                                    }
                                },
                                "node_3": {
                                    "position": {
                                        "x": 225,
                                        "y": -225
                                    }
                                },
                                "node_4": {
                                    "position": {
                                        "x": 345,
                                        "y": -150
                                    }
                                },
                                "node_5": {
                                    "position": {
                                        "x": 450,
                                        "y": -150
                                    }
                                },
                                "node_6": {
                                    "position": {
                                        "x": 660,
                                        "y": -150
                                    }
                                },
                                "node_7": {
                                    "position": {
                                        "x": 345,
                                        "y": -225
                                    }
                                },
                                "node_8": {
                                    "position": {
                                        "x": 555,
                                        "y": -225
                                    }
                                },
                                "node_9": {
                                    "position": {
                                        "x": 345,
                                        "y": -300
                                    }
                                },
                                "node_10": {
                                    "position": {
                                        "x": 450,
                                        "y": -300
                                    }
                                },
                                "node_11": {
                                    "position": {
                                        "x": 660,
                                        "y": -300
                                    }
                                },
                                "node_12": {
                                    "position": {
                                        "x": 60,
                                        "y": -435
                                    }
                                },
                                "node_13": {
                                    "position": {
                                        "x": 165,
                                        "y": -435
                                    }
                                },
                                "node_14": {
                                    "position": {
                                        "x": 165,
                                        "y": -510
                                    }
                                },
                                "node_15": {
                                    "position": {
                                        "x": 285,
                                        "y": -435
                                    }
                                },
                                "node_16": {
                                    "position": {
                                        "x": 420,
                                        "y": -435
                                    }
                                },
                                "node_17": {
                                    "position": {
                                        "x": 420,
                                        "y": -510
                                    }
                                },
                                "node_18": {
                                    "position": {
                                        "x": 60,
                                        "y": -615
                                    }
                                },
                                "node_19": {
                                    "position": {
                                        "x": 165,
                                        "y": -615
                                    }
                                },
                                "node_20": {
                                    "position": {
                                        "x": 60,
                                        "y": -780
                                    }
                                },
                                "node_21": {
                                    "position": {
                                        "x": 285,
                                        "y": -615
                                    }
                                },
                                "node_22": {
                                    "position": {
                                        "x": 165,
                                        "y": -690
                                    }
                                },
                                "node_23": {
                                    "position": {
                                        "x": 285,
                                        "y": -690
                                    }
                                },
                                "node_24": {
                                    "position": {
                                        "x": 165,
                                        "y": -780
                                    }
                                },
                                "node_25": {
                                    "position": {
                                        "x": 420,
                                        "y": -780
                                    }
                                },
                                "node_26": {
                                    "position": {
                                        "x": 285,
                                        "y": -825
                                    }
                                },
                                "keypoint_2": {
                                    "position": {
                                        "x": 555,
                                        "y": -435
                                    }
                                },
                                "keypoint_3": {
                                    "position": {
                                        "x": 420,
                                        "y": -690
                                    }
                                }
                            },
                            "exclusionDataMap": {
                                "exclusion_1": {
                                    "id": "exclusion_1",
                                    "pos": {
                                        "x": 420,
                                        "y": -465
                                    },
                                    "size": {
                                        "x": 90,
                                        "y": 180
                                    }
                                }
                            }
                        },
                        "bagViewData": null,
                        "bagDataMap": {},
                        "exclusionDataMap": {
                            "exclusion_1": {
                                "defaultSlotId": null
                            }
                        },
                        "dimensionItemList": [
                            {
                                "desc": "战略",
                                "maxScore": 140
                            },
                            {
                                "desc": "技巧",
                                "maxScore": 50
                            },
                            {
                                "desc": "调遣",
                                "maxScore": 100
                            },
                            {
                                "desc": "应变",
                                "maxScore": 0
                            },
                            {
                                "desc": "洞察",
                                "maxScore": 50
                            },
                            {
                                "desc": "筹划",
                                "maxScore": 20
                            }
                        ],
                        "rewardNodeDataMap": {},
                        "dailyRuneUnlcokDataMap": {},
                        "nodeDataMap": {
                            "node_0": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1"
                                ]
                            },
                            "node_1": {
                                "runeId": "global_nocostrecovery_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_0",
                                    "node_3"
                                ]
                            },
                            "keypoint_2": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_16",
                                    "node_17"
                                ]
                            },
                            "node_3": {
                                "runeId": "cost_battlebegin_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_1",
                                    "node_4",
                                    "node_7",
                                    "node_9"
                                ]
                            },
                            "node_4": {
                                "runeId": "pioneer_cdtime_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_5"
                                ]
                            },
                            "node_5": {
                                "runeId": "char_cost_pioneer_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_4",
                                    "node_6"
                                ]
                            },
                            "node_6": {
                                "runeId": "cost_battlebegin_5",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_5"
                                ]
                            },
                            "node_7": {
                                "runeId": "trap_wpnsts_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_8"
                                ]
                            },
                            "node_8": {
                                "runeId": "cost_battlebegin_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_7"
                                ]
                            },
                            "node_9": {
                                "runeId": "char_cost_special_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_3",
                                    "node_10"
                                ]
                            },
                            "node_10": {
                                "runeId": "char_cost_warrior_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_9",
                                    "node_11"
                                ]
                            },
                            "node_11": {
                                "runeId": "cost_battlebegin_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_10"
                                ]
                            },
                            "node_12": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13",
                                    "node_14"
                                ]
                            },
                            "node_13": {
                                "runeId": "global_forbidloc_low_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_15"
                                ]
                            },
                            "node_14": {
                                "runeId": "global_forbidloc_high_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_12",
                                    "node_15"
                                ]
                            },
                            "node_15": {
                                "runeId": "global_lifepoint_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_13",
                                    "node_14",
                                    "node_16",
                                    "node_17"
                                ]
                            },
                            "node_16": {
                                "runeId": "global_squadnum_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_1",
                                "adjacentNodeList": [
                                    "node_15",
                                    "keypoint_2"
                                ]
                            },
                            "node_17": {
                                "runeId": "global_pcharnum_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": "exclusion_1",
                                "adjacentNodeList": [
                                    "node_15",
                                    "keypoint_2"
                                ]
                            },
                            "node_18": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_19",
                                    "node_22"
                                ]
                            },
                            "node_19": {
                                "runeId": "enemy_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_21"
                                ]
                            },
                            "node_20": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "START",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24"
                                ]
                            },
                            "node_21": {
                                "runeId": "enemy_atk_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_19",
                                    "keypoint_1"
                                ]
                            },
                            "node_22": {
                                "runeId": "enemy_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_18",
                                    "node_23"
                                ]
                            },
                            "node_23": {
                                "runeId": "enemy_hp_2",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_22",
                                    "keypoint_3"
                                ]
                            },
                            "node_24": {
                                "runeId": "rageth_atkspeed_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_20",
                                    "node_25",
                                    "node_26"
                                ]
                            },
                            "node_25": {
                                "runeId": "rageth_hp_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24"
                                ]
                            },
                            "node_26": {
                                "runeId": "rageth_atk_1",
                                "slotPackId": null,
                                "nodeType": "NORMAL",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_24"
                                ]
                            },
                            "keypoint_3": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_23"
                                ]
                            },
                            "keypoint_1": {
                                "runeId": null,
                                "slotPackId": null,
                                "nodeType": "KEYPOINT",
                                "mutualExclusionGroup": null,
                                "adjacentNodeList": [
                                    "node_21"
                                ]
                            }
                        },
                        "runeDataMap": {
                            "global_nocostrecovery_1": {
                                "runeId": "global_nocostrecovery_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_nocostrecovery_1",
                                "runeName": "目标：全面渗 透",
                                "score": 50,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_nocostrecovery_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "部署费用不再自然回复",
                                    "runes": [
                                        {
                                            "key": "global_cost_recovery_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 9999
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 0
                            },
                            "cost_battlebegin_1": {
                                "runeId": "cost_battlebegin_1",
                                "runeGroupId": null,
                                "runeIcon": "g_cost_battlebegin_1",
                                "runeName": "目标：经费充足I",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "cost_battlebegin_1",
                                    "points": 0,
                                    "mutexGroupKey": "cost_battlebegin",
                                    "description": "初始费用增加<@crisisv2.pos>{value}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "global_initial_cost_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 50
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 52
                            },
                            "pioneer_cdtime_1": {
                                "runeId": "pioneer_cdtime_1",
                                "runeGroupId": null,
                                "runeIcon": "g_pioneer_cdtime_1",
                                "runeName": "源石环境：目的性侵蚀I",
                                "score": 20,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "pioneer_cdtime_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "所有我方先锋干员的技能技力消耗增加<@crisisv2.nag>100%</>，生命值降低<@crisisv2.nag>95%</>",
                                    "runes": [
                                        {
                                            "key": "char_skill_cd_mul",
                                            "selector": {
                                                "professionMask": 512,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        },
                                        {
                                            "key": "char_attribute_mul",
                                            "selector": {
                                                "professionMask": 512,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.05
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 30
                            },
                            "char_cost_pioneer_2": {
                                "runeId": "char_cost_pioneer_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_pioneer_2",
                                "runeName": "先锋干员的部署费用符文二星",
                                "score": 20,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_pioneer_2",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "先锋干员的部署 费用提升至<@crisisv2.nag>5</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 512,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 31
                            },
                            "cost_battlebegin_5": {
                                "runeId": "cost_battlebegin_5",
                                "runeGroupId": null,
                                "runeIcon": "g_cost_battlebegin_2",
                                "runeName": "目标：经费充足II",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "cost_battlebegin_5",
                                    "points": 0,
                                    "mutexGroupKey": "cost_battlebegin",
                                    "description": "初始费用增加<@crisisv2.pos>{value}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "global_initial_cost_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 100
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 50
                            },
                            "trap_wpnsts_1": {
                                "runeId": "trap_wpnsts_1",
                                "runeGroupId": null,
                                "runeIcon": "g_trap_wpnsts_1",
                                "runeName": "环境：加量奶油I",
                                "score": 20,
                                "dimension": 5,
                                "packedRune": {
                                    "id": "trap_wpnsts_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "“冰淇淋机”嘲讽等级增加至1级",
                                    "runes": [
                                        {
                                            "key": "char_attribute_add",
                                            "selector": {
                                                "professionMask": 256,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "taunt_level",
                                                    "value": 2
                                                },
                                                {
                                                    "key": "char",
                                                    "valueStr": "trap_057_wpnsts"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 32
                            },
                            "cost_battlebegin_2": {
                                "runeId": "cost_battlebegin_2",
                                "runeGroupId": null,
                                "runeIcon": "g_cost_battlebegin_1",
                                "runeName": "目标：经费充足I",
                                "score": 0,
                                "dimension": -1,
                                "packedRune": {
                                    "id": "cost_battlebegin_2",
                                    "points": 0,
                                    "mutexGroupKey": "cost_battlebegin",
                                    "description": "初始费用增加<@crisisv2.pos>{value}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "global_initial_cost_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 50
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 51
                            },
                            "char_cost_special_1": {
                                "runeId": "char_cost_special_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_special_1",
                                "runeName": "目标：复杂流程I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_special_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "特种干员的部署费用提升至<@crisisv2.nag>2</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 64,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 40
                            },
                            "char_cost_warrior_1": {
                                "runeId": "char_cost_warrior_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_cost_warrior_1",
                                "runeName": "目标：复杂流程I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "char_cost_warrior_1",
                                    "points": 0,
                                    "mutexGroupKey": "char_exclude",
                                    "description": "近卫干员的部署费用提升至<@crisisv2.nag>2</>倍",
                                    "runes": [
                                        {
                                            "key": "char_cost_mul",
                                            "selector": {
                                                "professionMask": 1,
                                                "buildableMask": 3,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "scale",
                                                    "value": 2
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 41
                            },
                            "global_forbidloc_low_1": {
                                "runeId": "global_forbidloc_low_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_forbidloc_low_1",
                                "runeName": "环 境：地表施工",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_forbidloc_low_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场中地面<@crisisv2.nag>6</>个位置无法部署我方单位",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(4,5)|(5,6)|(6,7)|(4,9)|(3,8)|(2,7)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 33
                            },
                            "global_forbidloc_high_1": {
                                "runeId": "global_forbidloc_high_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_forbidloc_high_1",
                                "runeName": "环境：高台施工",
                                "score": 20,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_forbidloc_high_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "战场中高台<@crisisv2.nag>4</>个位置无法部署我方单位",
                                    "runes": [
                                        {
                                            "key": "global_forbid_location",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "location",
                                                    "valueStr": "(3,6)|(4,6)|(4,8)|(5,8)"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 34
                            },
                            "global_lifepoint_1": {
                                "runeId": "global_lifepoint_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_lifepoint_1",
                                "runeName": "目标：最后防线",
                                "score": 10,
                                "dimension": 4,
                                "packedRune": {
                                    "id": "global_lifepoint_1",
                                    "points": 0,
                                    "mutexGroupKey": null,
                                    "description": "我方防御点可承受的敌方数量变为<@crisisv2.nag>1</>",
                                    "runes": [
                                        {
                                            "key": "global_lifepoint",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 48
                            },
                            "global_squadnum_1": {
                                "runeId": "global_squadnum_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_squadnum_1",
                                "runeName": "目标：隐秘行动I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_squadnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "最多可编入<@crisisv2.nag>10</>名干员进入作战",
                                    "runes": [
                                        {
                                            "key": "global_squad_num_limit",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": 10
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 43
                            },
                            "global_pcharnum_1": {
                                "runeId": "global_pcharnum_1",
                                "runeGroupId": null,
                                "runeIcon": "g_global_pcharnum_1",
                                "runeName": "环境：交战区I",
                                "score": 10,
                                "dimension": 2,
                                "packedRune": {
                                    "id": "global_pcharnum_1",
                                    "points": 0,
                                    "mutexGroupKey": "squad_limit",
                                    "description": "可同时部署的单位数量减少至<@crisisv2.nag>7</>",
                                    "runes": [
                                        {
                                            "key": "global_placable_char_num_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "value",
                                                    "value": -1
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 44
                            },
                            "enemy_atk_1": {
                                "runeId": "enemy_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_1",
                                "runeName": "环境：高价值目标I",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.25
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 20
                            },
                            "enemy_atk_2": {
                                "runeId": "enemy_atk_2",
                                "runeGroupId": null,
                                "runeIcon": "g_char_atk_2",
                                "runeName": "环境：高价值目标II",
                                "score": 40,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_atk_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_atk",
                                    "description": "所有敌人的攻击力<@crisisv2.nag>+{atk:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.3
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 11
                            },
                            "enemy_hp_1": {
                                "runeId": "enemy_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_hp_1",
                                "runeName": "源石环境：活性I",
                                "score": 30,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hp",
                                    "description": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.5
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 21
                            },
                            "enemy_hp_2": {
                                "runeId": "enemy_hp_2",
                                "runeGroupId": null,
                                "runeIcon": "g_enemy_hp_2",
                                "runeName": "源石环境：活性II",
                                "score": 40,
                                "dimension": 0,
                                "packedRune": {
                                    "id": "enemy_hp_2",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_hp",
                                    "description": "所有敌人的最大生命值<@crisisv2.nag>+{max_hp:0%}</>(同类效果叠加)",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.7
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 10
                            },
                            "rageth_atkspeed_1": {
                                "runeId": "rageth_atkspeed_1",
                                "runeGroupId": null,
                                "runeIcon": "g_rageth_atkspeed_1",
                                "runeName": "狂暴宿主投掷手：极度疯狂I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "rageth_atkspeed_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_rageth",
                                    "description": "<狂暴宿主投掷手>的攻击速度<@crisisv2.nag>+{attack_speed}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_add",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "attack_speed",
                                                    "value": 200
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1063_rageth"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 45
                            },
                            "rageth_hp_1": {
                                "runeId": "rageth_hp_1",
                                "runeGroupId": null,
                                "runeIcon": "g_rageth_hp_1",
                                "runeName": "狂暴宿主投掷手：异变躯体I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "rageth_hp_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_rageth",
                                    "description": "<狂暴宿主投掷手>的生命值<@crisisv2.nag>+{max_hp:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "max_hp",
                                                    "value": 0.6
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1063_rageth"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "rageth"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 47
                            },
                            "rageth_atk_1": {
                                "runeId": "rageth_atk_1",
                                "runeGroupId": null,
                                "runeIcon": "g_rageth_atk_1",
                                "runeName": "狂暴宿主投掷手：骇人怪力I",
                                "score": 10,
                                "dimension": 1,
                                "packedRune": {
                                    "id": "rageth_atk_1",
                                    "points": 0,
                                    "mutexGroupKey": "enemy_rageth",
                                    "description": "<狂暴宿主投掷手>的攻击力<@crisisv2.nag>+{atk:0%}</>",
                                    "runes": [
                                        {
                                            "key": "enemy_attribute_additive_mul",
                                            "selector": {
                                                "professionMask": 0,
                                                "buildableMask": 0,
                                                "charIdFilter": null,
                                                "enemyIdFilter": null,
                                                "enemyIdExcludeFilter": null,
                                                "skillIdFilter": null,
                                                "tileKeyFilter": null,
                                                "groupTagFilter": null,
                                                "filterTagFilter": null,
                                                "subProfessionExcludeFilter": null
                                            },
                                            "blackboard": [
                                                {
                                                    "key": "atk",
                                                    "value": 0.5
                                                },
                                                {
                                                    "key": "enemy",
                                                    "valueStr": "enemy_1063_rageth"
                                                },
                                                {
                                                    "key": "group",
                                                    "valueStr": "rageth"
                                                }
                                            ]
                                        }
                                    ]
                                },
                                "sortId": 46
                            }
                        }
                    }
                },
                "seasonConst": {
                    "focusRuneSlotId": "node_2",
                    "focusTreasureSlotId": "reward_1",
                    "focusKeyPointSlotId": "keypoint_1",
                    "focusBagId": "pack_1"
                },
                "achievementDataMap": {}
            },
            "ts": 1700000000,
            "playerDataDelta": {
                "modified": {},
                "deleted": {}
            }
        }"#;
        write("./data/crisisv2/cc1.json", contents);
    }
}
