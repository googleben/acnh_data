#![allow(dead_code, non_camel_case_types, non_snake_case)]
use std::collections::HashMap;
use crate::msbt::*;

pub struct MSBTs {
    pub JPja: Language,
    pub KRko: Language,
    pub USes: Language,
    pub EUde: Language,
    pub CNzh: Language,
    pub USfr: Language,
    pub USen: Language,
    pub EUfr: Language,
    pub TWzh: Language,
    pub EUnl: Language,
    pub EUit: Language,
    pub EUen: Language,
    pub EUes: Language,
    pub EUru: Language,
}

impl MSBTs {
    pub fn new() -> std::io::Result<Self> {
        let data = read_msbts()?;
        Ok(MSBTs {
        JPja: Language::new(data.get("JPja").unwrap()),
        KRko: Language::new(data.get("KRko").unwrap()),
        USes: Language::new(data.get("USes").unwrap()),
        EUde: Language::new(data.get("EUde").unwrap()),
        CNzh: Language::new(data.get("CNzh").unwrap()),
        USfr: Language::new(data.get("USfr").unwrap()),
        USen: Language::new(data.get("USen").unwrap()),
        EUfr: Language::new(data.get("EUfr").unwrap()),
        TWzh: Language::new(data.get("TWzh").unwrap()),
        EUnl: Language::new(data.get("EUnl").unwrap()),
        EUit: Language::new(data.get("EUit").unwrap()),
        EUen: Language::new(data.get("EUen").unwrap()),
        EUes: Language::new(data.get("EUes").unwrap()),
        EUru: Language::new(data.get("EUru").unwrap()),
        })
    }
}

pub struct Language {
    pub TalkSys: Box<TalkSys>,
    pub Tutorials: Box<Tutorials>,
    pub TalkSNpc: Box<TalkSNpc>,
    pub Dialog: Box<Dialog>,
    pub Mail: Box<Mail>,
    pub System: Box<System>,
    pub TalkFtr: Box<TalkFtr>,
    pub LayoutMsg: Box<LayoutMsg>,
    pub TalkObj: Box<TalkObj>,
    pub String: Box<String>,
    pub TalkNNpc: Box<TalkNNpc>,
}

impl Language {
    fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Language {
        TalkSys: Box::new(TalkSys::new(msbts)),
        Tutorials: Box::new(Tutorials::new(msbts)),
        TalkSNpc: Box::new(TalkSNpc::new(msbts)),
        Dialog: Box::new(Dialog::new(msbts)),
        Mail: Box::new(Mail::new(msbts)),
        System: Box::new(System::new(msbts)),
        TalkFtr: Box::new(TalkFtr::new(msbts)),
        LayoutMsg: Box::new(LayoutMsg::new(msbts)),
        TalkObj: Box::new(TalkObj::new(msbts)),
        String: Box::new(String::new(msbts)),
        TalkNNpc: Box::new(TalkNNpc::new(msbts)),
        }
    }
}

pub struct TalkSys {
    pub SYS_Save: File,
    pub SYS_PhoneApp: File,
    pub SYS_Guide: File,
    pub SYS_Motto: File,
    pub SYS_Network: File,
    pub SYS_SharePlay: File,
    pub SYS_Get_Insect_Fish_Fes: File,
    pub SYS_CloseMenu: File,
    pub SYS_PhoneApp2: File,
    pub SYS_Get_Fish: File,
    pub SYS_Door: File,
    pub SYS_Telepathy: File,
    pub SYS_Player: File,
    pub SYS_Get_ShellFish: File,
    pub SYS_Museum_Art: File,
    pub SYS_GEvent: File,
    pub SYS_Get_Insect: File,
    pub SYS_Get_Fish_BeyDailyQ: File,
    pub SYS_Get_Emoticons: File,
}

impl TalkSys {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSys {
        SYS_Save: File::new("/TalkSys/SYS_Save.msbt", msbts.get("/TalkSys/SYS_Save.msbt").unwrap()),
        SYS_PhoneApp: File::new("/TalkSys/SYS_PhoneApp.msbt", msbts.get("/TalkSys/SYS_PhoneApp.msbt").unwrap()),
        SYS_Guide: File::new("/TalkSys/SYS_Guide.msbt", msbts.get("/TalkSys/SYS_Guide.msbt").unwrap()),
        SYS_Motto: File::new("/TalkSys/SYS_Motto.msbt", msbts.get("/TalkSys/SYS_Motto.msbt").unwrap()),
        SYS_Network: File::new("/TalkSys/SYS_Network.msbt", msbts.get("/TalkSys/SYS_Network.msbt").unwrap()),
        SYS_SharePlay: File::new("/TalkSys/SYS_SharePlay.msbt", msbts.get("/TalkSys/SYS_SharePlay.msbt").unwrap()),
        SYS_Get_Insect_Fish_Fes: File::new("/TalkSys/SYS_Get_Insect_Fish_Fes.msbt", msbts.get("/TalkSys/SYS_Get_Insect_Fish_Fes.msbt").unwrap()),
        SYS_CloseMenu: File::new("/TalkSys/SYS_CloseMenu.msbt", msbts.get("/TalkSys/SYS_CloseMenu.msbt").unwrap()),
        SYS_PhoneApp2: File::new("/TalkSys/SYS_PhoneApp2.msbt", msbts.get("/TalkSys/SYS_PhoneApp2.msbt").unwrap()),
        SYS_Get_Fish: File::new("/TalkSys/SYS_Get_Fish.msbt", msbts.get("/TalkSys/SYS_Get_Fish.msbt").unwrap()),
        SYS_Door: File::new("/TalkSys/SYS_Door.msbt", msbts.get("/TalkSys/SYS_Door.msbt").unwrap()),
        SYS_Telepathy: File::new("/TalkSys/SYS_Telepathy.msbt", msbts.get("/TalkSys/SYS_Telepathy.msbt").unwrap()),
        SYS_Player: File::new("/TalkSys/SYS_Player.msbt", msbts.get("/TalkSys/SYS_Player.msbt").unwrap()),
        SYS_Get_ShellFish: File::new("/TalkSys/SYS_Get_ShellFish.msbt", msbts.get("/TalkSys/SYS_Get_ShellFish.msbt").unwrap()),
        SYS_Museum_Art: File::new("/TalkSys/SYS_Museum_Art.msbt", msbts.get("/TalkSys/SYS_Museum_Art.msbt").unwrap()),
        SYS_GEvent: File::new("/TalkSys/SYS_GEvent.msbt", msbts.get("/TalkSys/SYS_GEvent.msbt").unwrap()),
        SYS_Get_Insect: File::new("/TalkSys/SYS_Get_Insect.msbt", msbts.get("/TalkSys/SYS_Get_Insect.msbt").unwrap()),
        SYS_Get_Fish_BeyDailyQ: File::new("/TalkSys/SYS_Get_Fish_BeyDailyQ.msbt", msbts.get("/TalkSys/SYS_Get_Fish_BeyDailyQ.msbt").unwrap()),
        SYS_Get_Emoticons: File::new("/TalkSys/SYS_Get_Emoticons.msbt", msbts.get("/TalkSys/SYS_Get_Emoticons.msbt").unwrap()),
        }
    }
}

pub struct Tutorials {
    pub Tutorials_Prologue1_AreaSelect: File,
    pub Tutorials_Prologue4_Orientation3: File,
    pub Tutorials_Prologue_FreeTalk_HA: File,
    pub Tutorials_Prologue1_Airport: File,
    pub Tutorials_Prologue4_Orientation2_Rco: File,
    pub Tutorials_Prologue2_Arrival: File,
    pub Tutorials_Prologue3_Orientation1: File,
    pub Tutorials_Prologue4_Orientation2_AN: File,
    pub Tutorials_Prologue_FreeTalk_AN: File,
    pub Tutorials_Prologue3_Orientation2_Rcm: File,
    pub Tutorials_Prologue4_Orientation2_HA: File,
}

impl Tutorials {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Tutorials {
        Tutorials_Prologue1_AreaSelect: File::new("/Tutorials/Tutorials_Prologue1_AreaSelect.msbt", msbts.get("/Tutorials/Tutorials_Prologue1_AreaSelect.msbt").unwrap()),
        Tutorials_Prologue4_Orientation3: File::new("/Tutorials/Tutorials_Prologue4_Orientation3.msbt", msbts.get("/Tutorials/Tutorials_Prologue4_Orientation3.msbt").unwrap()),
        Tutorials_Prologue_FreeTalk_HA: File::new("/Tutorials/Tutorials_Prologue_FreeTalk_HA.msbt", msbts.get("/Tutorials/Tutorials_Prologue_FreeTalk_HA.msbt").unwrap()),
        Tutorials_Prologue1_Airport: File::new("/Tutorials/Tutorials_Prologue1_Airport.msbt", msbts.get("/Tutorials/Tutorials_Prologue1_Airport.msbt").unwrap()),
        Tutorials_Prologue4_Orientation2_Rco: File::new("/Tutorials/Tutorials_Prologue4_Orientation2_Rco.msbt", msbts.get("/Tutorials/Tutorials_Prologue4_Orientation2_Rco.msbt").unwrap()),
        Tutorials_Prologue2_Arrival: File::new("/Tutorials/Tutorials_Prologue2_Arrival.msbt", msbts.get("/Tutorials/Tutorials_Prologue2_Arrival.msbt").unwrap()),
        Tutorials_Prologue3_Orientation1: File::new("/Tutorials/Tutorials_Prologue3_Orientation1.msbt", msbts.get("/Tutorials/Tutorials_Prologue3_Orientation1.msbt").unwrap()),
        Tutorials_Prologue4_Orientation2_AN: File::new("/Tutorials/Tutorials_Prologue4_Orientation2_AN.msbt", msbts.get("/Tutorials/Tutorials_Prologue4_Orientation2_AN.msbt").unwrap()),
        Tutorials_Prologue_FreeTalk_AN: File::new("/Tutorials/Tutorials_Prologue_FreeTalk_AN.msbt", msbts.get("/Tutorials/Tutorials_Prologue_FreeTalk_AN.msbt").unwrap()),
        Tutorials_Prologue3_Orientation2_Rcm: File::new("/Tutorials/Tutorials_Prologue3_Orientation2_Rcm.msbt", msbts.get("/Tutorials/Tutorials_Prologue3_Orientation2_Rcm.msbt").unwrap()),
        Tutorials_Prologue4_Orientation2_HA: File::new("/Tutorials/Tutorials_Prologue4_Orientation2_HA.msbt", msbts.get("/Tutorials/Tutorials_Prologue4_Orientation2_HA.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_owl {
    pub SP_owl_00_Common: File,
    pub SP_owl_99_Select: File,
    pub SP_owl_Comment_Insect: File,
    pub SP_owl_22_Museum_RenualQuest: File,
    pub SP_owl_Comment_Fish: File,
    pub SP_owl_Amiibo: File,
    pub SP_owl_01_MuseumTent: File,
    pub SP_owl_Comment_Fossil: File,
    pub SP_owl_24_Museum_StampRally: File,
    pub SP_owl_11_Museum1stTalk: File,
}

impl TalkSNpc_owl {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_owl {
        SP_owl_00_Common: File::new("/TalkSNpc/owl/SP_owl_00_Common.msbt", msbts.get("/TalkSNpc/owl/SP_owl_00_Common.msbt").unwrap()),
        SP_owl_99_Select: File::new("/TalkSNpc/owl/SP_owl_99_Select.msbt", msbts.get("/TalkSNpc/owl/SP_owl_99_Select.msbt").unwrap()),
        SP_owl_Comment_Insect: File::new("/TalkSNpc/owl/SP_owl_Comment_Insect.msbt", msbts.get("/TalkSNpc/owl/SP_owl_Comment_Insect.msbt").unwrap()),
        SP_owl_22_Museum_RenualQuest: File::new("/TalkSNpc/owl/SP_owl_22_Museum_RenualQuest.msbt", msbts.get("/TalkSNpc/owl/SP_owl_22_Museum_RenualQuest.msbt").unwrap()),
        SP_owl_Comment_Fish: File::new("/TalkSNpc/owl/SP_owl_Comment_Fish.msbt", msbts.get("/TalkSNpc/owl/SP_owl_Comment_Fish.msbt").unwrap()),
        SP_owl_Amiibo: File::new("/TalkSNpc/owl/SP_owl_Amiibo.msbt", msbts.get("/TalkSNpc/owl/SP_owl_Amiibo.msbt").unwrap()),
        SP_owl_01_MuseumTent: File::new("/TalkSNpc/owl/SP_owl_01_MuseumTent.msbt", msbts.get("/TalkSNpc/owl/SP_owl_01_MuseumTent.msbt").unwrap()),
        SP_owl_Comment_Fossil: File::new("/TalkSNpc/owl/SP_owl_Comment_Fossil.msbt", msbts.get("/TalkSNpc/owl/SP_owl_Comment_Fossil.msbt").unwrap()),
        SP_owl_24_Museum_StampRally: File::new("/TalkSNpc/owl/SP_owl_24_Museum_StampRally.msbt", msbts.get("/TalkSNpc/owl/SP_owl_24_Museum_StampRally.msbt").unwrap()),
        SP_owl_11_Museum1stTalk: File::new("/TalkSNpc/owl/SP_owl_11_Museum1stTalk.msbt", msbts.get("/TalkSNpc/owl/SP_owl_11_Museum1stTalk.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_gul {
    pub SP_gul_Amiibo: File,
    pub SP_gul: File,
}

impl TalkSNpc_gul {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_gul {
        SP_gul_Amiibo: File::new("/TalkSNpc/gul/SP_gul_Amiibo.msbt", msbts.get("/TalkSNpc/gul/SP_gul_Amiibo.msbt").unwrap()),
        SP_gul: File::new("/TalkSNpc/gul/SP_gul.msbt", msbts.get("/TalkSNpc/gul/SP_gul.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_PublicAnnouncement {
    pub SP_PublicAnnouncement_EventStart: File,
    pub SP_PublicAnnouncement_11_Sza_Once: File,
    pub SP_PublicAnnouncement_01_Sza: File,
    pub SP_PublicAnnouncement_00_Rco: File,
    pub SP_PublicAnnouncement_10_Rco_Once: File,
}

impl TalkSNpc_PublicAnnouncement {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_PublicAnnouncement {
        SP_PublicAnnouncement_EventStart: File::new("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_EventStart.msbt", msbts.get("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_EventStart.msbt").unwrap()),
        SP_PublicAnnouncement_11_Sza_Once: File::new("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_11_Sza_Once.msbt", msbts.get("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_11_Sza_Once.msbt").unwrap()),
        SP_PublicAnnouncement_01_Sza: File::new("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_01_Sza.msbt", msbts.get("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_01_Sza.msbt").unwrap()),
        SP_PublicAnnouncement_00_Rco: File::new("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_00_Rco.msbt", msbts.get("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_00_Rco.msbt").unwrap()),
        SP_PublicAnnouncement_10_Rco_Once: File::new("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_10_Rco_Once.msbt", msbts.get("/TalkSNpc/PublicAnnouncement/SP_PublicAnnouncement_10_Rco_Once.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_gst {
    pub SP_gst: File,
}

impl TalkSNpc_gst {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_gst {
        SP_gst: File::new("/TalkSNpc/gst/SP_gst.msbt", msbts.get("/TalkSNpc/gst/SP_gst.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_spn_GEvent {
    pub SP_spn_GEvent_JuneBride: File,
}

impl TalkSNpc_spn_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_spn_GEvent {
        SP_spn_GEvent_JuneBride: File::new("/TalkSNpc/spn/GEvent/SP_spn_GEvent_JuneBride.msbt", msbts.get("/TalkSNpc/spn/GEvent/SP_spn_GEvent_JuneBride.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_spn {
    pub GEvent: TalkSNpc_spn_GEvent,
    pub SP_spn_80_PhoneCall: File,
    pub SP_spn_01_MainField: File,
    pub SP_spn_02_Commune: File,
    pub SP_spn_99_Select: File,
}

impl TalkSNpc_spn {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_spn {
        GEvent: TalkSNpc_spn_GEvent::new(msbts),
        SP_spn_80_PhoneCall: File::new("/TalkSNpc/spn/SP_spn_80_PhoneCall.msbt", msbts.get("/TalkSNpc/spn/SP_spn_80_PhoneCall.msbt").unwrap()),
        SP_spn_01_MainField: File::new("/TalkSNpc/spn/SP_spn_01_MainField.msbt", msbts.get("/TalkSNpc/spn/SP_spn_01_MainField.msbt").unwrap()),
        SP_spn_02_Commune: File::new("/TalkSNpc/spn/SP_spn_02_Commune.msbt", msbts.get("/TalkSNpc/spn/SP_spn_02_Commune.msbt").unwrap()),
        SP_spn_99_Select: File::new("/TalkSNpc/spn/SP_spn_99_Select.msbt", msbts.get("/TalkSNpc/spn/SP_spn_99_Select.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_doc {
    pub SP_doc_30_Flight_Announce: File,
    pub SP_doc_51_MysteryTour: File,
    pub SP_doc_52_CommuneIsland: File,
}

impl TalkSNpc_doc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_doc {
        SP_doc_30_Flight_Announce: File::new("/TalkSNpc/doc/SP_doc_30_Flight_Announce.msbt", msbts.get("/TalkSNpc/doc/SP_doc_30_Flight_Announce.msbt").unwrap()),
        SP_doc_51_MysteryTour: File::new("/TalkSNpc/doc/SP_doc_51_MysteryTour.msbt", msbts.get("/TalkSNpc/doc/SP_doc_51_MysteryTour.msbt").unwrap()),
        SP_doc_52_CommuneIsland: File::new("/TalkSNpc/doc/SP_doc_52_CommuneIsland.msbt", msbts.get("/TalkSNpc/doc/SP_doc_52_CommuneIsland.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_pyn {
    pub SP_pyn_Amiibo: File,
    pub SP_pyn: File,
}

impl TalkSNpc_pyn {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_pyn {
        SP_pyn_Amiibo: File::new("/TalkSNpc/pyn/SP_pyn_Amiibo.msbt", msbts.get("/TalkSNpc/pyn/SP_pyn_Amiibo.msbt").unwrap()),
        SP_pyn: File::new("/TalkSNpc/pyn/SP_pyn.msbt", msbts.get("/TalkSNpc/pyn/SP_pyn.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_chy {
    pub SP_chy_daily: File,
    pub SP_chy_event: File,
}

impl TalkSNpc_chy {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_chy {
        SP_chy_daily: File::new("/TalkSNpc/chy/SP_chy_daily.msbt", msbts.get("/TalkSNpc/chy/SP_chy_daily.msbt").unwrap()),
        SP_chy_event: File::new("/TalkSNpc/chy/SP_chy_event.msbt", msbts.get("/TalkSNpc/chy/SP_chy_event.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_bey {
    pub SP_bey_event: File,
    pub SP_bey_daily: File,
}

impl TalkSNpc_bey {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_bey {
        SP_bey_event: File::new("/TalkSNpc/bey/SP_bey_event.msbt", msbts.get("/TalkSNpc/bey/SP_bey_event.msbt").unwrap()),
        SP_bey_daily: File::new("/TalkSNpc/bey/SP_bey_daily.msbt", msbts.get("/TalkSNpc/bey/SP_bey_daily.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_alw_GEvent {
    pub SP_alw_GEvent_JuneBride: File,
}

impl TalkSNpc_alw_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_alw_GEvent {
        SP_alw_GEvent_JuneBride: File::new("/TalkSNpc/alw/GEvent/SP_alw_GEvent_JuneBride.msbt", msbts.get("/TalkSNpc/alw/GEvent/SP_alw_GEvent_JuneBride.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_alw {
    pub GEvent: TalkSNpc_alw_GEvent,
    pub SP_alw_Amiibo: File,
}

impl TalkSNpc_alw {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_alw {
        GEvent: TalkSNpc_alw_GEvent::new(msbts),
        SP_alw_Amiibo: File::new("/TalkSNpc/alw/SP_alw_Amiibo.msbt", msbts.get("/TalkSNpc/alw/SP_alw_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_hgc {
    pub SP_hgc: File,
    pub SP_hgc_Amiibo: File,
}

impl TalkSNpc_hgc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_hgc {
        SP_hgc: File::new("/TalkSNpc/hgc/SP_hgc.msbt", msbts.get("/TalkSNpc/hgc/SP_hgc.msbt").unwrap()),
        SP_hgc_Amiibo: File::new("/TalkSNpc/hgc/SP_hgc_Amiibo.msbt", msbts.get("/TalkSNpc/hgc/SP_hgc_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_Event {
    pub SP_Ceremony: File,
}

impl TalkSNpc_Event {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_Event {
        SP_Ceremony: File::new("/TalkSNpc/Event/SP_Ceremony.msbt", msbts.get("/TalkSNpc/Event/SP_Ceremony.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_rco_GEvent {
    pub SP_rco_GEvent_Countdown: File,
}

impl TalkSNpc_rco_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_rco_GEvent {
        SP_rco_GEvent_Countdown: File::new("/TalkSNpc/rco/GEvent/SP_rco_GEvent_Countdown.msbt", msbts.get("/TalkSNpc/rco/GEvent/SP_rco_GEvent_Countdown.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_rco {
    pub GEvent: TalkSNpc_rco_GEvent,
    pub SP_rco_05_MigrantsQuest: File,
    pub SP_rco_20_PhoneCall: File,
    pub SP_rco_02_Loan: File,
    pub SP_rco_06_Office: File,
    pub SP_rco_00_Common: File,
    pub SP_rco_Amiibo: File,
    pub SP_rco_01_StartingMaingame: File,
    pub SP_rco_09_RemakeWorkShop: File,
    pub SP_rco_13_Infra_MoveStructure: File,
    pub SP_rco_07_IslandDevelopment: File,
    pub SP_rco_99_Select: File,
    pub SP_rco_93_BlancoSettings: File,
    pub SP_rco_04_DonationReward: File,
    pub SP_rco_91_BreakInSpeak: File,
    pub SP_rco_10_ForVisitor: File,
    pub SP_rco_62_Event_MayDay: File,
    pub SP_rco_00_Common_Choice: File,
    pub SP_rco_11_TkkFirstLive: File,
    pub SP_rco_92_OtherSettings: File,
    pub SP_rco_08_Ceremony: File,
    pub SP_rco_12_Infra_BridgeSlope: File,
    pub SP_rco_61_Event_EarthDay: File,
    pub SP_rco_03_PcHouseTent: File,
}

impl TalkSNpc_rco {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_rco {
        GEvent: TalkSNpc_rco_GEvent::new(msbts),
        SP_rco_05_MigrantsQuest: File::new("/TalkSNpc/rco/SP_rco_05_MigrantsQuest.msbt", msbts.get("/TalkSNpc/rco/SP_rco_05_MigrantsQuest.msbt").unwrap()),
        SP_rco_20_PhoneCall: File::new("/TalkSNpc/rco/SP_rco_20_PhoneCall.msbt", msbts.get("/TalkSNpc/rco/SP_rco_20_PhoneCall.msbt").unwrap()),
        SP_rco_02_Loan: File::new("/TalkSNpc/rco/SP_rco_02_Loan.msbt", msbts.get("/TalkSNpc/rco/SP_rco_02_Loan.msbt").unwrap()),
        SP_rco_06_Office: File::new("/TalkSNpc/rco/SP_rco_06_Office.msbt", msbts.get("/TalkSNpc/rco/SP_rco_06_Office.msbt").unwrap()),
        SP_rco_00_Common: File::new("/TalkSNpc/rco/SP_rco_00_Common.msbt", msbts.get("/TalkSNpc/rco/SP_rco_00_Common.msbt").unwrap()),
        SP_rco_Amiibo: File::new("/TalkSNpc/rco/SP_rco_Amiibo.msbt", msbts.get("/TalkSNpc/rco/SP_rco_Amiibo.msbt").unwrap()),
        SP_rco_01_StartingMaingame: File::new("/TalkSNpc/rco/SP_rco_01_StartingMaingame.msbt", msbts.get("/TalkSNpc/rco/SP_rco_01_StartingMaingame.msbt").unwrap()),
        SP_rco_09_RemakeWorkShop: File::new("/TalkSNpc/rco/SP_rco_09_RemakeWorkShop.msbt", msbts.get("/TalkSNpc/rco/SP_rco_09_RemakeWorkShop.msbt").unwrap()),
        SP_rco_13_Infra_MoveStructure: File::new("/TalkSNpc/rco/SP_rco_13_Infra_MoveStructure.msbt", msbts.get("/TalkSNpc/rco/SP_rco_13_Infra_MoveStructure.msbt").unwrap()),
        SP_rco_07_IslandDevelopment: File::new("/TalkSNpc/rco/SP_rco_07_IslandDevelopment.msbt", msbts.get("/TalkSNpc/rco/SP_rco_07_IslandDevelopment.msbt").unwrap()),
        SP_rco_99_Select: File::new("/TalkSNpc/rco/SP_rco_99_Select.msbt", msbts.get("/TalkSNpc/rco/SP_rco_99_Select.msbt").unwrap()),
        SP_rco_93_BlancoSettings: File::new("/TalkSNpc/rco/SP_rco_93_BlancoSettings.msbt", msbts.get("/TalkSNpc/rco/SP_rco_93_BlancoSettings.msbt").unwrap()),
        SP_rco_04_DonationReward: File::new("/TalkSNpc/rco/SP_rco_04_DonationReward.msbt", msbts.get("/TalkSNpc/rco/SP_rco_04_DonationReward.msbt").unwrap()),
        SP_rco_91_BreakInSpeak: File::new("/TalkSNpc/rco/SP_rco_91_BreakInSpeak.msbt", msbts.get("/TalkSNpc/rco/SP_rco_91_BreakInSpeak.msbt").unwrap()),
        SP_rco_10_ForVisitor: File::new("/TalkSNpc/rco/SP_rco_10_ForVisitor.msbt", msbts.get("/TalkSNpc/rco/SP_rco_10_ForVisitor.msbt").unwrap()),
        SP_rco_62_Event_MayDay: File::new("/TalkSNpc/rco/SP_rco_62_Event_MayDay.msbt", msbts.get("/TalkSNpc/rco/SP_rco_62_Event_MayDay.msbt").unwrap()),
        SP_rco_00_Common_Choice: File::new("/TalkSNpc/rco/SP_rco_00_Common_Choice.msbt", msbts.get("/TalkSNpc/rco/SP_rco_00_Common_Choice.msbt").unwrap()),
        SP_rco_11_TkkFirstLive: File::new("/TalkSNpc/rco/SP_rco_11_TkkFirstLive.msbt", msbts.get("/TalkSNpc/rco/SP_rco_11_TkkFirstLive.msbt").unwrap()),
        SP_rco_92_OtherSettings: File::new("/TalkSNpc/rco/SP_rco_92_OtherSettings.msbt", msbts.get("/TalkSNpc/rco/SP_rco_92_OtherSettings.msbt").unwrap()),
        SP_rco_08_Ceremony: File::new("/TalkSNpc/rco/SP_rco_08_Ceremony.msbt", msbts.get("/TalkSNpc/rco/SP_rco_08_Ceremony.msbt").unwrap()),
        SP_rco_12_Infra_BridgeSlope: File::new("/TalkSNpc/rco/SP_rco_12_Infra_BridgeSlope.msbt", msbts.get("/TalkSNpc/rco/SP_rco_12_Infra_BridgeSlope.msbt").unwrap()),
        SP_rco_61_Event_EarthDay: File::new("/TalkSNpc/rco/SP_rco_61_Event_EarthDay.msbt", msbts.get("/TalkSNpc/rco/SP_rco_61_Event_EarthDay.msbt").unwrap()),
        SP_rco_03_PcHouseTent: File::new("/TalkSNpc/rco/SP_rco_03_PcHouseTent.msbt", msbts.get("/TalkSNpc/rco/SP_rco_03_PcHouseTent.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_alp_GEvent {
    pub SP_alp_GEvent_JuneBride: File,
}

impl TalkSNpc_alp_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_alp_GEvent {
        SP_alp_GEvent_JuneBride: File::new("/TalkSNpc/alp/GEvent/SP_alp_GEvent_JuneBride.msbt", msbts.get("/TalkSNpc/alp/GEvent/SP_alp_GEvent_JuneBride.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_alp {
    pub GEvent: TalkSNpc_alp_GEvent,
    pub SP_alp_Amiibo: File,
}

impl TalkSNpc_alp {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_alp {
        GEvent: TalkSNpc_alp_GEvent::new(msbts),
        SP_alp_Amiibo: File::new("/TalkSNpc/alp/SP_alp_Amiibo.msbt", msbts.get("/TalkSNpc/alp/SP_alp_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_boc {
    pub SP_boc: File,
}

impl TalkSNpc_boc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_boc {
        SP_boc: File::new("/TalkSNpc/boc/SP_boc.msbt", msbts.get("/TalkSNpc/boc/SP_boc.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_rcm {
    pub SP_rcm_99_Select: File,
    pub SP_rcm_Twins: File,
    pub SP_rcm_00_common: File,
    pub SP_rcm_10_Renual: File,
    pub SP_rcm_80_PhoneCall: File,
    pub SP_rcm_02_TentRequest: File,
    pub SP_rcm_01_TentCommon: File,
    pub SP_rcm_Amiibo: File,
    pub SP_rcm_20_Market: File,
}

impl TalkSNpc_rcm {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_rcm {
        SP_rcm_99_Select: File::new("/TalkSNpc/rcm/SP_rcm_99_Select.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_99_Select.msbt").unwrap()),
        SP_rcm_Twins: File::new("/TalkSNpc/rcm/SP_rcm_Twins.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_Twins.msbt").unwrap()),
        SP_rcm_00_common: File::new("/TalkSNpc/rcm/SP_rcm_00_common.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_00_common.msbt").unwrap()),
        SP_rcm_10_Renual: File::new("/TalkSNpc/rcm/SP_rcm_10_Renual.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_10_Renual.msbt").unwrap()),
        SP_rcm_80_PhoneCall: File::new("/TalkSNpc/rcm/SP_rcm_80_PhoneCall.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_80_PhoneCall.msbt").unwrap()),
        SP_rcm_02_TentRequest: File::new("/TalkSNpc/rcm/SP_rcm_02_TentRequest.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_02_TentRequest.msbt").unwrap()),
        SP_rcm_01_TentCommon: File::new("/TalkSNpc/rcm/SP_rcm_01_TentCommon.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_01_TentCommon.msbt").unwrap()),
        SP_rcm_Amiibo: File::new("/TalkSNpc/rcm/SP_rcm_Amiibo.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_Amiibo.msbt").unwrap()),
        SP_rcm_20_Market: File::new("/TalkSNpc/rcm/SP_rcm_20_Market.msbt", msbts.get("/TalkSNpc/rcm/SP_rcm_20_Market.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_hgs {
    pub SP_hgs_Amiibo: File,
    pub SP_hgs: File,
}

impl TalkSNpc_hgs {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_hgs {
        SP_hgs_Amiibo: File::new("/TalkSNpc/hgs/SP_hgs_Amiibo.msbt", msbts.get("/TalkSNpc/hgs/SP_hgs_Amiibo.msbt").unwrap()),
        SP_hgs: File::new("/TalkSNpc/hgs/SP_hgs.msbt", msbts.get("/TalkSNpc/hgs/SP_hgs.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_fox {
    pub SP_fox_Amiibo: File,
    pub SP_fox_01_PreVisit: File,
    pub SP_fox: File,
}

impl TalkSNpc_fox {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_fox {
        SP_fox_Amiibo: File::new("/TalkSNpc/fox/SP_fox_Amiibo.msbt", msbts.get("/TalkSNpc/fox/SP_fox_Amiibo.msbt").unwrap()),
        SP_fox_01_PreVisit: File::new("/TalkSNpc/fox/SP_fox_01_PreVisit.msbt", msbts.get("/TalkSNpc/fox/SP_fox_01_PreVisit.msbt").unwrap()),
        SP_fox: File::new("/TalkSNpc/fox/SP_fox.msbt", msbts.get("/TalkSNpc/fox/SP_fox.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_rct {
    pub SP_rct_Amiibo: File,
    pub SP_rct: File,
}

impl TalkSNpc_rct {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_rct {
        SP_rct_Amiibo: File::new("/TalkSNpc/rct/SP_rct_Amiibo.msbt", msbts.get("/TalkSNpc/rct/SP_rct_Amiibo.msbt").unwrap()),
        SP_rct: File::new("/TalkSNpc/rct/SP_rct.msbt", msbts.get("/TalkSNpc/rct/SP_rct.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_cml {
    pub SP_cml_Amiibo: File,
    pub SP_cml: File,
}

impl TalkSNpc_cml {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_cml {
        SP_cml_Amiibo: File::new("/TalkSNpc/cml/SP_cml_Amiibo.msbt", msbts.get("/TalkSNpc/cml/SP_cml_Amiibo.msbt").unwrap()),
        SP_cml: File::new("/TalkSNpc/cml/SP_cml.msbt", msbts.get("/TalkSNpc/cml/SP_cml.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_ows {
    pub SP_ows: File,
    pub SP_ows_Amiibo: File,
}

impl TalkSNpc_ows {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_ows {
        SP_ows: File::new("/TalkSNpc/ows/SP_ows.msbt", msbts.get("/TalkSNpc/ows/SP_ows.msbt").unwrap()),
        SP_ows_Amiibo: File::new("/TalkSNpc/ows/SP_ows_Amiibo.msbt", msbts.get("/TalkSNpc/ows/SP_ows_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_xct {
    pub SP_xct: File,
    pub SP_xct_Amiibo: File,
}

impl TalkSNpc_xct {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_xct {
        SP_xct: File::new("/TalkSNpc/xct/SP_xct.msbt", msbts.get("/TalkSNpc/xct/SP_xct.msbt").unwrap()),
        SP_xct_Amiibo: File::new("/TalkSNpc/xct/SP_xct_Amiibo.msbt", msbts.get("/TalkSNpc/xct/SP_xct_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_hgh {
    pub SP_hgh_10_Mydesign: File,
    pub SP_hgh_00_Peddler: File,
    pub SP_hgh_99_Select: File,
    pub SP_hgh_30_Mannequin: File,
    pub SP_hgh_Amiibo: File,
    pub SP_hgh: File,
    pub SP_hgh_20_FittingRoom: File,
}

impl TalkSNpc_hgh {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_hgh {
        SP_hgh_10_Mydesign: File::new("/TalkSNpc/hgh/SP_hgh_10_Mydesign.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_10_Mydesign.msbt").unwrap()),
        SP_hgh_00_Peddler: File::new("/TalkSNpc/hgh/SP_hgh_00_Peddler.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_00_Peddler.msbt").unwrap()),
        SP_hgh_99_Select: File::new("/TalkSNpc/hgh/SP_hgh_99_Select.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_99_Select.msbt").unwrap()),
        SP_hgh_30_Mannequin: File::new("/TalkSNpc/hgh/SP_hgh_30_Mannequin.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_30_Mannequin.msbt").unwrap()),
        SP_hgh_Amiibo: File::new("/TalkSNpc/hgh/SP_hgh_Amiibo.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_Amiibo.msbt").unwrap()),
        SP_hgh: File::new("/TalkSNpc/hgh/SP_hgh.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh.msbt").unwrap()),
        SP_hgh_20_FittingRoom: File::new("/TalkSNpc/hgh/SP_hgh_20_FittingRoom.msbt", msbts.get("/TalkSNpc/hgh/SP_hgh_20_FittingRoom.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_slo {
    pub SP_slo: File,
    pub SP_slo_Amiibo: File,
}

impl TalkSNpc_slo {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_slo {
        SP_slo: File::new("/TalkSNpc/slo/SP_slo.msbt", msbts.get("/TalkSNpc/slo/SP_slo.msbt").unwrap()),
        SP_slo_Amiibo: File::new("/TalkSNpc/slo/SP_slo_Amiibo.msbt", msbts.get("/TalkSNpc/slo/SP_slo_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_skk {
    pub SP_skk: File,
    pub SP_skk_Amiibo: File,
}

impl TalkSNpc_skk {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_skk {
        SP_skk: File::new("/TalkSNpc/skk/SP_skk.msbt", msbts.get("/TalkSNpc/skk/SP_skk.msbt").unwrap()),
        SP_skk_Amiibo: File::new("/TalkSNpc/skk/SP_skk_Amiibo.msbt", msbts.get("/TalkSNpc/skk/SP_skk_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_sza_GEvent {
    pub SP_sza_GEvent_Countdown: File,
}

impl TalkSNpc_sza_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_sza_GEvent {
        SP_sza_GEvent_Countdown: File::new("/TalkSNpc/sza/GEvent/SP_sza_GEvent_Countdown.msbt", msbts.get("/TalkSNpc/sza/GEvent/SP_sza_GEvent_Countdown.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_sza {
    pub GEvent: TalkSNpc_sza_GEvent,
    pub SP_sza_99_Select: File,
    pub SP_sza_2_Office: File,
    pub SP_sza_50_IslandEvaluation: File,
    pub SP_sza_0_common: File,
    pub SP_sza_Amiibo: File,
}

impl TalkSNpc_sza {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_sza {
        GEvent: TalkSNpc_sza_GEvent::new(msbts),
        SP_sza_99_Select: File::new("/TalkSNpc/sza/SP_sza_99_Select.msbt", msbts.get("/TalkSNpc/sza/SP_sza_99_Select.msbt").unwrap()),
        SP_sza_2_Office: File::new("/TalkSNpc/sza/SP_sza_2_Office.msbt", msbts.get("/TalkSNpc/sza/SP_sza_2_Office.msbt").unwrap()),
        SP_sza_50_IslandEvaluation: File::new("/TalkSNpc/sza/SP_sza_50_IslandEvaluation.msbt", msbts.get("/TalkSNpc/sza/SP_sza_50_IslandEvaluation.msbt").unwrap()),
        SP_sza_0_common: File::new("/TalkSNpc/sza/SP_sza_0_common.msbt", msbts.get("/TalkSNpc/sza/SP_sza_0_common.msbt").unwrap()),
        SP_sza_Amiibo: File::new("/TalkSNpc/sza/SP_sza_Amiibo.msbt", msbts.get("/TalkSNpc/sza/SP_sza_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_dod {
    pub SP_dod_50_PostOffice: File,
    pub SP_dod_99_Select: File,
    pub SP_dod_AirPort: File,
    pub SP_dod_41_MultiPlayInet: File,
}

impl TalkSNpc_dod {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_dod {
        SP_dod_50_PostOffice: File::new("/TalkSNpc/dod/SP_dod_50_PostOffice.msbt", msbts.get("/TalkSNpc/dod/SP_dod_50_PostOffice.msbt").unwrap()),
        SP_dod_99_Select: File::new("/TalkSNpc/dod/SP_dod_99_Select.msbt", msbts.get("/TalkSNpc/dod/SP_dod_99_Select.msbt").unwrap()),
        SP_dod_AirPort: File::new("/TalkSNpc/dod/SP_dod_AirPort.msbt", msbts.get("/TalkSNpc/dod/SP_dod_AirPort.msbt").unwrap()),
        SP_dod_41_MultiPlayInet: File::new("/TalkSNpc/dod/SP_dod_41_MultiPlayInet.msbt", msbts.get("/TalkSNpc/dod/SP_dod_41_MultiPlayInet.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc_tkk {
    pub SP_tkk_1stLive: File,
    pub SP_tkk_dream: File,
    pub SP_tkk_Live: File,
    pub SP_tkk_Amiibo: File,
    pub SP_tkk: File,
}

impl TalkSNpc_tkk {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc_tkk {
        SP_tkk_1stLive: File::new("/TalkSNpc/tkk/SP_tkk_1stLive.msbt", msbts.get("/TalkSNpc/tkk/SP_tkk_1stLive.msbt").unwrap()),
        SP_tkk_dream: File::new("/TalkSNpc/tkk/SP_tkk_dream.msbt", msbts.get("/TalkSNpc/tkk/SP_tkk_dream.msbt").unwrap()),
        SP_tkk_Live: File::new("/TalkSNpc/tkk/SP_tkk_Live.msbt", msbts.get("/TalkSNpc/tkk/SP_tkk_Live.msbt").unwrap()),
        SP_tkk_Amiibo: File::new("/TalkSNpc/tkk/SP_tkk_Amiibo.msbt", msbts.get("/TalkSNpc/tkk/SP_tkk_Amiibo.msbt").unwrap()),
        SP_tkk: File::new("/TalkSNpc/tkk/SP_tkk.msbt", msbts.get("/TalkSNpc/tkk/SP_tkk.msbt").unwrap()),
        }
    }
}

pub struct TalkSNpc {
    pub owl: TalkSNpc_owl,
    pub gul: TalkSNpc_gul,
    pub PublicAnnouncement: TalkSNpc_PublicAnnouncement,
    pub gst: TalkSNpc_gst,
    pub spn: TalkSNpc_spn,
    pub doc: TalkSNpc_doc,
    pub pyn: TalkSNpc_pyn,
    pub chy: TalkSNpc_chy,
    pub bey: TalkSNpc_bey,
    pub alw: TalkSNpc_alw,
    pub hgc: TalkSNpc_hgc,
    pub Event: TalkSNpc_Event,
    pub rco: TalkSNpc_rco,
    pub alp: TalkSNpc_alp,
    pub boc: TalkSNpc_boc,
    pub rcm: TalkSNpc_rcm,
    pub hgs: TalkSNpc_hgs,
    pub fox: TalkSNpc_fox,
    pub rct: TalkSNpc_rct,
    pub cml: TalkSNpc_cml,
    pub ows: TalkSNpc_ows,
    pub xct: TalkSNpc_xct,
    pub hgh: TalkSNpc_hgh,
    pub slo: TalkSNpc_slo,
    pub skk: TalkSNpc_skk,
    pub sza: TalkSNpc_sza,
    pub dod: TalkSNpc_dod,
    pub tkk: TalkSNpc_tkk,
}

impl TalkSNpc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkSNpc {
        owl: TalkSNpc_owl::new(msbts),
        gul: TalkSNpc_gul::new(msbts),
        PublicAnnouncement: TalkSNpc_PublicAnnouncement::new(msbts),
        gst: TalkSNpc_gst::new(msbts),
        spn: TalkSNpc_spn::new(msbts),
        doc: TalkSNpc_doc::new(msbts),
        pyn: TalkSNpc_pyn::new(msbts),
        chy: TalkSNpc_chy::new(msbts),
        bey: TalkSNpc_bey::new(msbts),
        alw: TalkSNpc_alw::new(msbts),
        hgc: TalkSNpc_hgc::new(msbts),
        Event: TalkSNpc_Event::new(msbts),
        rco: TalkSNpc_rco::new(msbts),
        alp: TalkSNpc_alp::new(msbts),
        boc: TalkSNpc_boc::new(msbts),
        rcm: TalkSNpc_rcm::new(msbts),
        hgs: TalkSNpc_hgs::new(msbts),
        fox: TalkSNpc_fox::new(msbts),
        rct: TalkSNpc_rct::new(msbts),
        cml: TalkSNpc_cml::new(msbts),
        ows: TalkSNpc_ows::new(msbts),
        xct: TalkSNpc_xct::new(msbts),
        hgh: TalkSNpc_hgh::new(msbts),
        slo: TalkSNpc_slo::new(msbts),
        skk: TalkSNpc_skk::new(msbts),
        sza: TalkSNpc_sza::new(msbts),
        dod: TalkSNpc_dod::new(msbts),
        tkk: TalkSNpc_tkk::new(msbts),
        }
    }
}

pub struct Dialog {
    pub DIALOG_Msg: File,
    pub DIALOG_Button: File,
    pub DIALOG_Region: File,
}

impl Dialog {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Dialog {
        DIALOG_Msg: File::new("/Dialog/DIALOG_Msg.msbt", msbts.get("/Dialog/DIALOG_Msg.msbt").unwrap()),
        DIALOG_Button: File::new("/Dialog/DIALOG_Button.msbt", msbts.get("/Dialog/DIALOG_Button.msbt").unwrap()),
        DIALOG_Region: File::new("/Dialog/DIALOG_Region.msbt", msbts.get("/Dialog/DIALOG_Region.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_G4_An {
    pub MAIL_AN_LifeStart: File,
    pub MAIL_AN_ReShort: File,
    pub MAIL_AN_ReLong: File,
    pub MAIL_AN_SendPresent: File,
    pub MAIL_AN_ReBlank: File,
    pub MAIL_AN_ReMax: File,
    pub MAIL_AN_SendNormal: File,
    pub MAIL_AN_HHA_BuildCelebration: File,
    pub MAIL_AN_BirthDayP: File,
    pub MAIL_AN_OverFlow: File,
    pub MAIL_AN_ReWinter: File,
    pub MAIL_AN_ReMoveIn: File,
    pub MAIL_AN_BirthDayN: File,
    pub MAIL_AN_ReSpecial: File,
    pub z_MAIL_AN_HeaderFooter: File,
    pub MAIL_AN_ReBirthDay: File,
}

impl Mail_NNpc_G4_An {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_G4_An {
        MAIL_AN_LifeStart: File::new("/Mail/NNpc/G4_An/MAIL_AN_LifeStart.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_LifeStart.msbt").unwrap()),
        MAIL_AN_ReShort: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReShort.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReShort.msbt").unwrap()),
        MAIL_AN_ReLong: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReLong.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReLong.msbt").unwrap()),
        MAIL_AN_SendPresent: File::new("/Mail/NNpc/G4_An/MAIL_AN_SendPresent.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_SendPresent.msbt").unwrap()),
        MAIL_AN_ReBlank: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReBlank.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReBlank.msbt").unwrap()),
        MAIL_AN_ReMax: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReMax.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReMax.msbt").unwrap()),
        MAIL_AN_SendNormal: File::new("/Mail/NNpc/G4_An/MAIL_AN_SendNormal.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_SendNormal.msbt").unwrap()),
        MAIL_AN_HHA_BuildCelebration: File::new("/Mail/NNpc/G4_An/MAIL_AN_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_AN_BirthDayP: File::new("/Mail/NNpc/G4_An/MAIL_AN_BirthDayP.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_BirthDayP.msbt").unwrap()),
        MAIL_AN_OverFlow: File::new("/Mail/NNpc/G4_An/MAIL_AN_OverFlow.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_OverFlow.msbt").unwrap()),
        MAIL_AN_ReWinter: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReWinter.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReWinter.msbt").unwrap()),
        MAIL_AN_ReMoveIn: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReMoveIn.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReMoveIn.msbt").unwrap()),
        MAIL_AN_BirthDayN: File::new("/Mail/NNpc/G4_An/MAIL_AN_BirthDayN.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_BirthDayN.msbt").unwrap()),
        MAIL_AN_ReSpecial: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReSpecial.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReSpecial.msbt").unwrap()),
        z_MAIL_AN_HeaderFooter: File::new("/Mail/NNpc/G4_An/z_MAIL_AN_HeaderFooter.msbt", msbts.get("/Mail/NNpc/G4_An/z_MAIL_AN_HeaderFooter.msbt").unwrap()),
        MAIL_AN_ReBirthDay: File::new("/Mail/NNpc/G4_An/MAIL_AN_ReBirthDay.msbt", msbts.get("/Mail/NNpc/G4_An/MAIL_AN_ReBirthDay.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_B4_Zk {
    pub MAIL_ZK_OverFlow: File,
    pub MAIL_ZK_ReWinter: File,
    pub MAIL_ZK_ReMoveIn: File,
    pub MAIL_ZK_BirthDayN: File,
    pub MAIL_ZK_ReLong: File,
    pub MAIL_ZK_ReSpecial: File,
    pub MAIL_ZK_SendNormal: File,
    pub MAIL_ZK_SendPresent: File,
    pub MAIL_ZK_ReMax: File,
    pub MAIL_ZK_HHA_BuildCelebration: File,
    pub z_MAIL_ZK_HeaderFooter: File,
    pub MAIL_ZK_ReBirthDay: File,
    pub MAIL_ZK_ReShort: File,
    pub MAIL_ZK_ReBlank: File,
    pub MAIL_ZK_BirthDayP: File,
}

impl Mail_NNpc_B4_Zk {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_B4_Zk {
        MAIL_ZK_OverFlow: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_OverFlow.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_OverFlow.msbt").unwrap()),
        MAIL_ZK_ReWinter: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReWinter.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReWinter.msbt").unwrap()),
        MAIL_ZK_ReMoveIn: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReMoveIn.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReMoveIn.msbt").unwrap()),
        MAIL_ZK_BirthDayN: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_BirthDayN.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_BirthDayN.msbt").unwrap()),
        MAIL_ZK_ReLong: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReLong.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReLong.msbt").unwrap()),
        MAIL_ZK_ReSpecial: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReSpecial.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReSpecial.msbt").unwrap()),
        MAIL_ZK_SendNormal: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_SendNormal.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_SendNormal.msbt").unwrap()),
        MAIL_ZK_SendPresent: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_SendPresent.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_SendPresent.msbt").unwrap()),
        MAIL_ZK_ReMax: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReMax.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReMax.msbt").unwrap()),
        MAIL_ZK_HHA_BuildCelebration: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_HHA_BuildCelebration.msbt").unwrap()),
        z_MAIL_ZK_HeaderFooter: File::new("/Mail/NNpc/B4_Zk/z_MAIL_ZK_HeaderFooter.msbt", msbts.get("/Mail/NNpc/B4_Zk/z_MAIL_ZK_HeaderFooter.msbt").unwrap()),
        MAIL_ZK_ReBirthDay: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReBirthDay.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReBirthDay.msbt").unwrap()),
        MAIL_ZK_ReShort: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReShort.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReShort.msbt").unwrap()),
        MAIL_ZK_ReBlank: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_ReBlank.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_ReBlank.msbt").unwrap()),
        MAIL_ZK_BirthDayP: File::new("/Mail/NNpc/B4_Zk/MAIL_ZK_BirthDayP.msbt", msbts.get("/Mail/NNpc/B4_Zk/MAIL_ZK_BirthDayP.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_B3_Ko {
    pub MAIL_KO_BirthDayN: File,
    pub MAIL_KO_ReWinter: File,
    pub MAIL_KO_ReMoveIn: File,
    pub z_MAIL_KO_HeaderFooter: File,
    pub MAIL_KO_ReBirthDay: File,
    pub MAIL_KO_ReSpecial: File,
    pub MAIL_KO_ReShort: File,
    pub MAIL_KO_ReBlank: File,
    pub MAIL_KO_ReMax: File,
    pub MAIL_KO_HHA_BuildCelebration: File,
    pub MAIL_KO_SendNormal: File,
    pub MAIL_KO_ReLong: File,
    pub MAIL_KO_SendPresent: File,
    pub MAIL_KO_BirthDayP: File,
    pub MAIL_KO_OverFlow: File,
}

impl Mail_NNpc_B3_Ko {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_B3_Ko {
        MAIL_KO_BirthDayN: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_BirthDayN.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_BirthDayN.msbt").unwrap()),
        MAIL_KO_ReWinter: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReWinter.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReWinter.msbt").unwrap()),
        MAIL_KO_ReMoveIn: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReMoveIn.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReMoveIn.msbt").unwrap()),
        z_MAIL_KO_HeaderFooter: File::new("/Mail/NNpc/B3_Ko/z_MAIL_KO_HeaderFooter.msbt", msbts.get("/Mail/NNpc/B3_Ko/z_MAIL_KO_HeaderFooter.msbt").unwrap()),
        MAIL_KO_ReBirthDay: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReBirthDay.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReBirthDay.msbt").unwrap()),
        MAIL_KO_ReSpecial: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReSpecial.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReSpecial.msbt").unwrap()),
        MAIL_KO_ReShort: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReShort.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReShort.msbt").unwrap()),
        MAIL_KO_ReBlank: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReBlank.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReBlank.msbt").unwrap()),
        MAIL_KO_ReMax: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReMax.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReMax.msbt").unwrap()),
        MAIL_KO_HHA_BuildCelebration: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_KO_SendNormal: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_SendNormal.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_SendNormal.msbt").unwrap()),
        MAIL_KO_ReLong: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_ReLong.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_ReLong.msbt").unwrap()),
        MAIL_KO_SendPresent: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_SendPresent.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_SendPresent.msbt").unwrap()),
        MAIL_KO_BirthDayP: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_BirthDayP.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_BirthDayP.msbt").unwrap()),
        MAIL_KO_OverFlow: File::new("/Mail/NNpc/B3_Ko/MAIL_KO_OverFlow.msbt", msbts.get("/Mail/NNpc/B3_Ko/MAIL_KO_OverFlow.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_B1_Bo {
    pub MAIL_BO_OverFlow: File,
    pub MAIL_BO_ReBirthDay: File,
    pub MAIL_BO_ReWinter: File,
    pub MAIL_BO_ReMoveIn: File,
    pub z_MAIL_BO_HeaderFooter: File,
    pub MAIL_BO_BirthDayP: File,
    pub MAIL_BO_HHA_BuildCelebration: File,
    pub MAIL_BO_BirthDayN: File,
    pub MAIL_BO_SendNormal: File,
    pub MAIL_BO_ReSpecial: File,
    pub MAIL_BO_SendPresent: File,
    pub MAIL_BO_ReMax: File,
    pub MAIL_BO_ReLong: File,
    pub MAIL_BO_ReShort: File,
    pub MAIL_BO_ReBlank: File,
}

impl Mail_NNpc_B1_Bo {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_B1_Bo {
        MAIL_BO_OverFlow: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_OverFlow.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_OverFlow.msbt").unwrap()),
        MAIL_BO_ReBirthDay: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReBirthDay.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReBirthDay.msbt").unwrap()),
        MAIL_BO_ReWinter: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReWinter.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReWinter.msbt").unwrap()),
        MAIL_BO_ReMoveIn: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReMoveIn.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReMoveIn.msbt").unwrap()),
        z_MAIL_BO_HeaderFooter: File::new("/Mail/NNpc/B1_Bo/z_MAIL_BO_HeaderFooter.msbt", msbts.get("/Mail/NNpc/B1_Bo/z_MAIL_BO_HeaderFooter.msbt").unwrap()),
        MAIL_BO_BirthDayP: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_BirthDayP.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_BirthDayP.msbt").unwrap()),
        MAIL_BO_HHA_BuildCelebration: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_BO_BirthDayN: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_BirthDayN.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_BirthDayN.msbt").unwrap()),
        MAIL_BO_SendNormal: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_SendNormal.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_SendNormal.msbt").unwrap()),
        MAIL_BO_ReSpecial: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReSpecial.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReSpecial.msbt").unwrap()),
        MAIL_BO_SendPresent: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_SendPresent.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_SendPresent.msbt").unwrap()),
        MAIL_BO_ReMax: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReMax.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReMax.msbt").unwrap()),
        MAIL_BO_ReLong: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReLong.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReLong.msbt").unwrap()),
        MAIL_BO_ReShort: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReShort.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReShort.msbt").unwrap()),
        MAIL_BO_ReBlank: File::new("/Mail/NNpc/B1_Bo/MAIL_BO_ReBlank.msbt", msbts.get("/Mail/NNpc/B1_Bo/MAIL_BO_ReBlank.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_G3_Ot {
    pub MAIL_OT_SendPresent: File,
    pub z_MAIL_OT_HeaderFooter: File,
    pub MAIL_OT_OverFlow: File,
    pub MAIL_OT_ReBirthDay: File,
    pub MAIL_OT_ReMax: File,
    pub MAIL_OT_ReWinter: File,
    pub MAIL_OT_ReMoveIn: File,
    pub MAIL_OT_BirthDayP: File,
    pub MAIL_OT_ReLong: File,
    pub MAIL_OT_ReShort: File,
    pub MAIL_OT_ReBlank: File,
    pub MAIL_OT_HHA_BuildCelebration: File,
    pub MAIL_OT_BirthDayN: File,
    pub MAIL_OT_ReSpecial: File,
    pub MAIL_OT_SendNormal: File,
}

impl Mail_NNpc_G3_Ot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_G3_Ot {
        MAIL_OT_SendPresent: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_SendPresent.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_SendPresent.msbt").unwrap()),
        z_MAIL_OT_HeaderFooter: File::new("/Mail/NNpc/G3_Ot/z_MAIL_OT_HeaderFooter.msbt", msbts.get("/Mail/NNpc/G3_Ot/z_MAIL_OT_HeaderFooter.msbt").unwrap()),
        MAIL_OT_OverFlow: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_OverFlow.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_OverFlow.msbt").unwrap()),
        MAIL_OT_ReBirthDay: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReBirthDay.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReBirthDay.msbt").unwrap()),
        MAIL_OT_ReMax: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReMax.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReMax.msbt").unwrap()),
        MAIL_OT_ReWinter: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReWinter.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReWinter.msbt").unwrap()),
        MAIL_OT_ReMoveIn: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReMoveIn.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReMoveIn.msbt").unwrap()),
        MAIL_OT_BirthDayP: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_BirthDayP.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_BirthDayP.msbt").unwrap()),
        MAIL_OT_ReLong: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReLong.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReLong.msbt").unwrap()),
        MAIL_OT_ReShort: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReShort.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReShort.msbt").unwrap()),
        MAIL_OT_ReBlank: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReBlank.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReBlank.msbt").unwrap()),
        MAIL_OT_HHA_BuildCelebration: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_OT_BirthDayN: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_BirthDayN.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_BirthDayN.msbt").unwrap()),
        MAIL_OT_ReSpecial: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_ReSpecial.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_ReSpecial.msbt").unwrap()),
        MAIL_OT_SendNormal: File::new("/Mail/NNpc/G3_Ot/MAIL_OT_SendNormal.msbt", msbts.get("/Mail/NNpc/G3_Ot/MAIL_OT_SendNormal.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_B2_Ha {
    pub MAIL_HA_ReBirthDay: File,
    pub MAIL_HA_LifeStart: File,
    pub MAIL_HA_SendNormal: File,
    pub MAIL_HA_OverFlow: File,
    pub MAIL_HA_ReShort: File,
    pub MAIL_HA_SendPresent: File,
    pub MAIL_HA_ReWinter: File,
    pub MAIL_HA_ReMoveIn: File,
    pub MAIL_HA_ReBlank: File,
    pub z_MAIL_HA_HeaderFooter: File,
    pub MAIL_HA_HHA_BuildCelebration: File,
    pub MAIL_HA_BirthDayP: File,
    pub MAIL_HA_BirthDayN: File,
    pub MAIL_HA_ReSpecial: File,
    pub MAIL_HA_ReMax: File,
    pub MAIL_HA_ReLong: File,
}

impl Mail_NNpc_B2_Ha {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_B2_Ha {
        MAIL_HA_ReBirthDay: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReBirthDay.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReBirthDay.msbt").unwrap()),
        MAIL_HA_LifeStart: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_LifeStart.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_LifeStart.msbt").unwrap()),
        MAIL_HA_SendNormal: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_SendNormal.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_SendNormal.msbt").unwrap()),
        MAIL_HA_OverFlow: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_OverFlow.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_OverFlow.msbt").unwrap()),
        MAIL_HA_ReShort: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReShort.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReShort.msbt").unwrap()),
        MAIL_HA_SendPresent: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_SendPresent.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_SendPresent.msbt").unwrap()),
        MAIL_HA_ReWinter: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReWinter.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReWinter.msbt").unwrap()),
        MAIL_HA_ReMoveIn: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReMoveIn.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReMoveIn.msbt").unwrap()),
        MAIL_HA_ReBlank: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReBlank.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReBlank.msbt").unwrap()),
        z_MAIL_HA_HeaderFooter: File::new("/Mail/NNpc/B2_Ha/z_MAIL_HA_HeaderFooter.msbt", msbts.get("/Mail/NNpc/B2_Ha/z_MAIL_HA_HeaderFooter.msbt").unwrap()),
        MAIL_HA_HHA_BuildCelebration: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_HA_BirthDayP: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_BirthDayP.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_BirthDayP.msbt").unwrap()),
        MAIL_HA_BirthDayN: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_BirthDayN.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_BirthDayN.msbt").unwrap()),
        MAIL_HA_ReSpecial: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReSpecial.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReSpecial.msbt").unwrap()),
        MAIL_HA_ReMax: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReMax.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReMax.msbt").unwrap()),
        MAIL_HA_ReLong: File::new("/Mail/NNpc/B2_Ha/MAIL_HA_ReLong.msbt", msbts.get("/Mail/NNpc/B2_Ha/MAIL_HA_ReLong.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_G2_Ge {
    pub z_MAIL_GE_HeaderFooter: File,
    pub MAIL_GE_HHA_BuildCelebration: File,
    pub MAIL_GE_OverFlow: File,
    pub MAIL_GE_ReWinter: File,
    pub MAIL_GE_ReMoveIn: File,
    pub MAIL_GE_ReShort: File,
    pub MAIL_GE_ReBlank: File,
    pub MAIL_GE_SendPresent: File,
    pub MAIL_GE_ReMax: File,
    pub MAIL_GE_ReBirthDay: File,
    pub MAIL_GE_BirthDayP: File,
    pub MAIL_GE_BirthDayN: File,
    pub MAIL_GE_ReSpecial: File,
    pub MAIL_GE_SendNormal: File,
    pub MAIL_GE_ReLong: File,
}

impl Mail_NNpc_G2_Ge {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_G2_Ge {
        z_MAIL_GE_HeaderFooter: File::new("/Mail/NNpc/G2_Ge/z_MAIL_GE_HeaderFooter.msbt", msbts.get("/Mail/NNpc/G2_Ge/z_MAIL_GE_HeaderFooter.msbt").unwrap()),
        MAIL_GE_HHA_BuildCelebration: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_GE_OverFlow: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_OverFlow.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_OverFlow.msbt").unwrap()),
        MAIL_GE_ReWinter: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReWinter.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReWinter.msbt").unwrap()),
        MAIL_GE_ReMoveIn: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReMoveIn.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReMoveIn.msbt").unwrap()),
        MAIL_GE_ReShort: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReShort.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReShort.msbt").unwrap()),
        MAIL_GE_ReBlank: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReBlank.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReBlank.msbt").unwrap()),
        MAIL_GE_SendPresent: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_SendPresent.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_SendPresent.msbt").unwrap()),
        MAIL_GE_ReMax: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReMax.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReMax.msbt").unwrap()),
        MAIL_GE_ReBirthDay: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReBirthDay.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReBirthDay.msbt").unwrap()),
        MAIL_GE_BirthDayP: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_BirthDayP.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_BirthDayP.msbt").unwrap()),
        MAIL_GE_BirthDayN: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_BirthDayN.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_BirthDayN.msbt").unwrap()),
        MAIL_GE_ReSpecial: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReSpecial.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReSpecial.msbt").unwrap()),
        MAIL_GE_SendNormal: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_SendNormal.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_SendNormal.msbt").unwrap()),
        MAIL_GE_ReLong: File::new("/Mail/NNpc/G2_Ge/MAIL_GE_ReLong.msbt", msbts.get("/Mail/NNpc/G2_Ge/MAIL_GE_ReLong.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc_G1_Fu {
    pub z_MAIL_FU_HeaderFooter: File,
    pub MAIL_FU_SendPresent: File,
    pub MAIL_FU_ReShort: File,
    pub MAIL_FU_ReBlank: File,
    pub MAIL_FU_ReMax: File,
    pub MAIL_FU_ReBirthDay: File,
    pub MAIL_FU_HHA_BuildCelebration: File,
    pub MAIL_FU_BirthDayP: File,
    pub MAIL_FU_OverFlow: File,
    pub MAIL_FU_ReLong: File,
    pub MAIL_FU_ReWinter: File,
    pub MAIL_FU_ReMoveIn: File,
    pub MAIL_FU_BirthDayN: File,
    pub MAIL_FU_SendNormal: File,
    pub MAIL_FU_ReSpecial: File,
}

impl Mail_NNpc_G1_Fu {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc_G1_Fu {
        z_MAIL_FU_HeaderFooter: File::new("/Mail/NNpc/G1_Fu/z_MAIL_FU_HeaderFooter.msbt", msbts.get("/Mail/NNpc/G1_Fu/z_MAIL_FU_HeaderFooter.msbt").unwrap()),
        MAIL_FU_SendPresent: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_SendPresent.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_SendPresent.msbt").unwrap()),
        MAIL_FU_ReShort: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReShort.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReShort.msbt").unwrap()),
        MAIL_FU_ReBlank: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReBlank.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReBlank.msbt").unwrap()),
        MAIL_FU_ReMax: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReMax.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReMax.msbt").unwrap()),
        MAIL_FU_ReBirthDay: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReBirthDay.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReBirthDay.msbt").unwrap()),
        MAIL_FU_HHA_BuildCelebration: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_HHA_BuildCelebration.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_HHA_BuildCelebration.msbt").unwrap()),
        MAIL_FU_BirthDayP: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_BirthDayP.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_BirthDayP.msbt").unwrap()),
        MAIL_FU_OverFlow: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_OverFlow.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_OverFlow.msbt").unwrap()),
        MAIL_FU_ReLong: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReLong.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReLong.msbt").unwrap()),
        MAIL_FU_ReWinter: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReWinter.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReWinter.msbt").unwrap()),
        MAIL_FU_ReMoveIn: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReMoveIn.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReMoveIn.msbt").unwrap()),
        MAIL_FU_BirthDayN: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_BirthDayN.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_BirthDayN.msbt").unwrap()),
        MAIL_FU_SendNormal: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_SendNormal.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_SendNormal.msbt").unwrap()),
        MAIL_FU_ReSpecial: File::new("/Mail/NNpc/G1_Fu/MAIL_FU_ReSpecial.msbt", msbts.get("/Mail/NNpc/G1_Fu/MAIL_FU_ReSpecial.msbt").unwrap()),
        }
    }
}

pub struct Mail_NNpc {
    pub G4_An: Mail_NNpc_G4_An,
    pub B4_Zk: Mail_NNpc_B4_Zk,
    pub B3_Ko: Mail_NNpc_B3_Ko,
    pub B1_Bo: Mail_NNpc_B1_Bo,
    pub G3_Ot: Mail_NNpc_G3_Ot,
    pub B2_Ha: Mail_NNpc_B2_Ha,
    pub G2_Ge: Mail_NNpc_G2_Ge,
    pub G1_Fu: Mail_NNpc_G1_Fu,
}

impl Mail_NNpc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_NNpc {
        G4_An: Mail_NNpc_G4_An::new(msbts),
        B4_Zk: Mail_NNpc_B4_Zk::new(msbts),
        B3_Ko: Mail_NNpc_B3_Ko::new(msbts),
        B1_Bo: Mail_NNpc_B1_Bo::new(msbts),
        G3_Ot: Mail_NNpc_G3_Ot::new(msbts),
        B2_Ha: Mail_NNpc_B2_Ha::new(msbts),
        G2_Ge: Mail_NNpc_G2_Ge::new(msbts),
        G1_Fu: Mail_NNpc_G1_Fu::new(msbts),
        }
    }
}

pub struct Mail_Sp_HHA {
    pub MAIL_SP_HHA_GradeFixedPoints: File,
    pub MAIL_SP_HHA_Mission: File,
    pub MAIL_SP_HHA_Grade: File,
    pub MAIL_SP_HHA_GradeMission: File,
    pub MAIL_SP_HHA_BuildCelebration: File,
}

impl Mail_Sp_HHA {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Sp_HHA {
        MAIL_SP_HHA_GradeFixedPoints: File::new("/Mail/Sp/HHA/MAIL_SP_HHA_GradeFixedPoints.msbt", msbts.get("/Mail/Sp/HHA/MAIL_SP_HHA_GradeFixedPoints.msbt").unwrap()),
        MAIL_SP_HHA_Mission: File::new("/Mail/Sp/HHA/MAIL_SP_HHA_Mission.msbt", msbts.get("/Mail/Sp/HHA/MAIL_SP_HHA_Mission.msbt").unwrap()),
        MAIL_SP_HHA_Grade: File::new("/Mail/Sp/HHA/MAIL_SP_HHA_Grade.msbt", msbts.get("/Mail/Sp/HHA/MAIL_SP_HHA_Grade.msbt").unwrap()),
        MAIL_SP_HHA_GradeMission: File::new("/Mail/Sp/HHA/MAIL_SP_HHA_GradeMission.msbt", msbts.get("/Mail/Sp/HHA/MAIL_SP_HHA_GradeMission.msbt").unwrap()),
        MAIL_SP_HHA_BuildCelebration: File::new("/Mail/Sp/HHA/MAIL_SP_HHA_BuildCelebration.msbt", msbts.get("/Mail/Sp/HHA/MAIL_SP_HHA_BuildCelebration.msbt").unwrap()),
        }
    }
}

pub struct Mail_Sp_Mother {
    pub MAIL_SP_Mother: File,
}

impl Mail_Sp_Mother {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Sp_Mother {
        MAIL_SP_Mother: File::new("/Mail/Sp/Mother/MAIL_SP_Mother.msbt", msbts.get("/Mail/Sp/Mother/MAIL_SP_Mother.msbt").unwrap()),
        }
    }
}

pub struct Mail_Sp_Nintendo {
    pub MAIL_SP_Nintendo: File,
}

impl Mail_Sp_Nintendo {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Sp_Nintendo {
        MAIL_SP_Nintendo: File::new("/Mail/Sp/Nintendo/MAIL_SP_Nintendo.msbt", msbts.get("/Mail/Sp/Nintendo/MAIL_SP_Nintendo.msbt").unwrap()),
        }
    }
}

pub struct Mail_Sp {
    pub HHA: Mail_Sp_HHA,
    pub Mother: Mail_Sp_Mother,
    pub Nintendo: Mail_Sp_Nintendo,
    pub MAIL_SP_CatalogShop: File,
    pub MAIL_SP_MilageProgram: File,
    pub MAIL_SP_Airline: File,
    pub MAIL_SP_FarawayMuseum: File,
    pub MAIL_SP_Snowman: File,
    pub MAIL_SP_ATM: File,
}

impl Mail_Sp {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Sp {
        HHA: Mail_Sp_HHA::new(msbts),
        Mother: Mail_Sp_Mother::new(msbts),
        Nintendo: Mail_Sp_Nintendo::new(msbts),
        MAIL_SP_CatalogShop: File::new("/Mail/Sp/MAIL_SP_CatalogShop.msbt", msbts.get("/Mail/Sp/MAIL_SP_CatalogShop.msbt").unwrap()),
        MAIL_SP_MilageProgram: File::new("/Mail/Sp/MAIL_SP_MilageProgram.msbt", msbts.get("/Mail/Sp/MAIL_SP_MilageProgram.msbt").unwrap()),
        MAIL_SP_Airline: File::new("/Mail/Sp/MAIL_SP_Airline.msbt", msbts.get("/Mail/Sp/MAIL_SP_Airline.msbt").unwrap()),
        MAIL_SP_FarawayMuseum: File::new("/Mail/Sp/MAIL_SP_FarawayMuseum.msbt", msbts.get("/Mail/Sp/MAIL_SP_FarawayMuseum.msbt").unwrap()),
        MAIL_SP_Snowman: File::new("/Mail/Sp/MAIL_SP_Snowman.msbt", msbts.get("/Mail/Sp/MAIL_SP_Snowman.msbt").unwrap()),
        MAIL_SP_ATM: File::new("/Mail/Sp/MAIL_SP_ATM.msbt", msbts.get("/Mail/Sp/MAIL_SP_ATM.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_hgc {
    pub MAIL_SNpc_hgc: File,
}

impl Mail_SNpc_hgc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_hgc {
        MAIL_SNpc_hgc: File::new("/Mail/SNpc/hgc/MAIL_SNpc_hgc.msbt", msbts.get("/Mail/SNpc/hgc/MAIL_SNpc_hgc.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_bey {
    pub MAIL_SNpc_bey_event: File,
    pub MAIL_SNpc_bey_daily: File,
}

impl Mail_SNpc_bey {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_bey {
        MAIL_SNpc_bey_event: File::new("/Mail/SNpc/bey/MAIL_SNpc_bey_event.msbt", msbts.get("/Mail/SNpc/bey/MAIL_SNpc_bey_event.msbt").unwrap()),
        MAIL_SNpc_bey_daily: File::new("/Mail/SNpc/bey/MAIL_SNpc_bey_daily.msbt", msbts.get("/Mail/SNpc/bey/MAIL_SNpc_bey_daily.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_boc {
    pub MAIL_SNpc_boc: File,
}

impl Mail_SNpc_boc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_boc {
        MAIL_SNpc_boc: File::new("/Mail/SNpc/boc/MAIL_SNpc_boc.msbt", msbts.get("/Mail/SNpc/boc/MAIL_SNpc_boc.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_rco {
    pub MAIL_SNpc_rco: File,
}

impl Mail_SNpc_rco {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_rco {
        MAIL_SNpc_rco: File::new("/Mail/SNpc/rco/MAIL_SNpc_rco.msbt", msbts.get("/Mail/SNpc/rco/MAIL_SNpc_rco.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_fox {
    pub MAIL_SNpc_fox: File,
}

impl Mail_SNpc_fox {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_fox {
        MAIL_SNpc_fox: File::new("/Mail/SNpc/fox/MAIL_SNpc_fox.msbt", msbts.get("/Mail/SNpc/fox/MAIL_SNpc_fox.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_chy {
    pub MAIL_SNpc_chy_daily: File,
    pub MAIL_SNpc_chy_event: File,
}

impl Mail_SNpc_chy {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_chy {
        MAIL_SNpc_chy_daily: File::new("/Mail/SNpc/chy/MAIL_SNpc_chy_daily.msbt", msbts.get("/Mail/SNpc/chy/MAIL_SNpc_chy_daily.msbt").unwrap()),
        MAIL_SNpc_chy_event: File::new("/Mail/SNpc/chy/MAIL_SNpc_chy_event.msbt", msbts.get("/Mail/SNpc/chy/MAIL_SNpc_chy_event.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_tkk {
    pub MAIL_SNpc_tkk: File,
}

impl Mail_SNpc_tkk {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_tkk {
        MAIL_SNpc_tkk: File::new("/Mail/SNpc/tkk/MAIL_SNpc_tkk.msbt", msbts.get("/Mail/SNpc/tkk/MAIL_SNpc_tkk.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_alw {
    pub MAIL_SNpc_alw_JuneBride: File,
}

impl Mail_SNpc_alw {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_alw {
        MAIL_SNpc_alw_JuneBride: File::new("/Mail/SNpc/alw/MAIL_SNpc_alw_JuneBride.msbt", msbts.get("/Mail/SNpc/alw/MAIL_SNpc_alw_JuneBride.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_gul {
    pub MAIL_SNpc_gul: File,
}

impl Mail_SNpc_gul {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_gul {
        MAIL_SNpc_gul: File::new("/Mail/SNpc/gul/MAIL_SNpc_gul.msbt", msbts.get("/Mail/SNpc/gul/MAIL_SNpc_gul.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc_pyn {
    pub MAIL_SNpc_pyn_let: File,
    pub MAIL_SNpc_pyn_mes: File,
}

impl Mail_SNpc_pyn {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc_pyn {
        MAIL_SNpc_pyn_let: File::new("/Mail/SNpc/pyn/MAIL_SNpc_pyn_let.msbt", msbts.get("/Mail/SNpc/pyn/MAIL_SNpc_pyn_let.msbt").unwrap()),
        MAIL_SNpc_pyn_mes: File::new("/Mail/SNpc/pyn/MAIL_SNpc_pyn_mes.msbt", msbts.get("/Mail/SNpc/pyn/MAIL_SNpc_pyn_mes.msbt").unwrap()),
        }
    }
}

pub struct Mail_SNpc {
    pub hgc: Mail_SNpc_hgc,
    pub bey: Mail_SNpc_bey,
    pub boc: Mail_SNpc_boc,
    pub rco: Mail_SNpc_rco,
    pub fox: Mail_SNpc_fox,
    pub chy: Mail_SNpc_chy,
    pub tkk: Mail_SNpc_tkk,
    pub alw: Mail_SNpc_alw,
    pub gul: Mail_SNpc_gul,
    pub pyn: Mail_SNpc_pyn,
}

impl Mail_SNpc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_SNpc {
        hgc: Mail_SNpc_hgc::new(msbts),
        bey: Mail_SNpc_bey::new(msbts),
        boc: Mail_SNpc_boc::new(msbts),
        rco: Mail_SNpc_rco::new(msbts),
        fox: Mail_SNpc_fox::new(msbts),
        chy: Mail_SNpc_chy::new(msbts),
        tkk: Mail_SNpc_tkk::new(msbts),
        alw: Mail_SNpc_alw::new(msbts),
        gul: Mail_SNpc_gul::new(msbts),
        pyn: Mail_SNpc_pyn::new(msbts),
        }
    }
}

pub struct Mail_Rnd_MsgB {
    pub MAIL_RND_MsgB_KO: File,
    pub MAIL_RND_MsgB_AN: File,
    pub MAIL_RND_MsgB_HA: File,
    pub MAIL_RND_MsgB_ZK: File,
    pub MAIL_RND_MsgB_BO: File,
    pub MAIL_RND_MsgB_OT: File,
    pub MAIL_RND_MsgB_FU: File,
    pub MAIL_RND_MsgB_GE: File,
}

impl Mail_Rnd_MsgB {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd_MsgB {
        MAIL_RND_MsgB_KO: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_KO.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_KO.msbt").unwrap()),
        MAIL_RND_MsgB_AN: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_AN.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_AN.msbt").unwrap()),
        MAIL_RND_MsgB_HA: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_HA.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_HA.msbt").unwrap()),
        MAIL_RND_MsgB_ZK: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_ZK.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_ZK.msbt").unwrap()),
        MAIL_RND_MsgB_BO: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_BO.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_BO.msbt").unwrap()),
        MAIL_RND_MsgB_OT: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_OT.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_OT.msbt").unwrap()),
        MAIL_RND_MsgB_FU: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_FU.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_FU.msbt").unwrap()),
        MAIL_RND_MsgB_GE: File::new("/Mail/Rnd/MsgB/MAIL_RND_MsgB_GE.msbt", msbts.get("/Mail/Rnd/MsgB/MAIL_RND_MsgB_GE.msbt").unwrap()),
        }
    }
}

pub struct Mail_Rnd_MsgA {
    pub MAIL_RND_MsgA_FU: File,
    pub MAIL_RND_MsgA_GE: File,
    pub MAIL_RND_MsgA_KO: File,
    pub MAIL_RND_MsgA_AN: File,
    pub MAIL_RND_MsgA_HA: File,
    pub MAIL_RND_MsgA_ZK: File,
    pub MAIL_RND_MsgA_BO: File,
    pub MAIL_RND_MsgA_OT: File,
}

impl Mail_Rnd_MsgA {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd_MsgA {
        MAIL_RND_MsgA_FU: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_FU.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_FU.msbt").unwrap()),
        MAIL_RND_MsgA_GE: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_GE.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_GE.msbt").unwrap()),
        MAIL_RND_MsgA_KO: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_KO.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_KO.msbt").unwrap()),
        MAIL_RND_MsgA_AN: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_AN.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_AN.msbt").unwrap()),
        MAIL_RND_MsgA_HA: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_HA.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_HA.msbt").unwrap()),
        MAIL_RND_MsgA_ZK: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_ZK.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_ZK.msbt").unwrap()),
        MAIL_RND_MsgA_BO: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_BO.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_BO.msbt").unwrap()),
        MAIL_RND_MsgA_OT: File::new("/Mail/Rnd/MsgA/MAIL_RND_MsgA_OT.msbt", msbts.get("/Mail/Rnd/MsgA/MAIL_RND_MsgA_OT.msbt").unwrap()),
        }
    }
}

pub struct Mail_Rnd_MsgD {
    pub MAIL_RND_MsgD_HA: File,
    pub MAIL_RND_MsgD_ZK: File,
    pub MAIL_RND_MsgD_BO: File,
    pub MAIL_RND_MsgD_OT: File,
    pub MAIL_RND_MsgD_FU: File,
    pub MAIL_RND_MsgD_GE: File,
    pub MAIL_RND_MsgD_KO: File,
    pub MAIL_RND_MsgD_AN: File,
}

impl Mail_Rnd_MsgD {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd_MsgD {
        MAIL_RND_MsgD_HA: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_HA.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_HA.msbt").unwrap()),
        MAIL_RND_MsgD_ZK: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_ZK.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_ZK.msbt").unwrap()),
        MAIL_RND_MsgD_BO: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_BO.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_BO.msbt").unwrap()),
        MAIL_RND_MsgD_OT: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_OT.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_OT.msbt").unwrap()),
        MAIL_RND_MsgD_FU: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_FU.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_FU.msbt").unwrap()),
        MAIL_RND_MsgD_GE: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_GE.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_GE.msbt").unwrap()),
        MAIL_RND_MsgD_KO: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_KO.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_KO.msbt").unwrap()),
        MAIL_RND_MsgD_AN: File::new("/Mail/Rnd/MsgD/MAIL_RND_MsgD_AN.msbt", msbts.get("/Mail/Rnd/MsgD/MAIL_RND_MsgD_AN.msbt").unwrap()),
        }
    }
}

pub struct Mail_Rnd_z_HeaderFooter {
    pub z_MAIL_RND_HeaderFooter_HA: File,
    pub z_MAIL_RND_HeaderFooter_ZK: File,
    pub z_MAIL_RND_HeaderFooter_BO: File,
    pub z_MAIL_RND_HeaderFooter_OT: File,
    pub z_MAIL_RND_HeaderFooter_FU: File,
    pub z_MAIL_RND_HeaderFooter_GE: File,
    pub z_MAIL_RND_HeaderFooter_KO: File,
    pub z_MAIL_RND_HeaderFooter_AN: File,
}

impl Mail_Rnd_z_HeaderFooter {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd_z_HeaderFooter {
        z_MAIL_RND_HeaderFooter_HA: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_HA.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_HA.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_ZK: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_ZK.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_ZK.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_BO: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_BO.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_BO.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_OT: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_OT.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_OT.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_FU: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_FU.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_FU.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_GE: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_GE.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_GE.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_KO: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_KO.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_KO.msbt").unwrap()),
        z_MAIL_RND_HeaderFooter_AN: File::new("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_AN.msbt", msbts.get("/Mail/Rnd/z_HeaderFooter/z_MAIL_RND_HeaderFooter_AN.msbt").unwrap()),
        }
    }
}

pub struct Mail_Rnd_MsgC {
    pub MAIL_RND_MsgC_HA: File,
    pub MAIL_RND_MsgC_ZK: File,
    pub MAIL_RND_MsgC_BO: File,
    pub MAIL_RND_MsgC_OT: File,
    pub MAIL_RND_MsgC_FU: File,
    pub MAIL_RND_MsgC_GE: File,
    pub MAIL_RND_MsgC_KO: File,
    pub MAIL_RND_MsgC_AN: File,
}

impl Mail_Rnd_MsgC {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd_MsgC {
        MAIL_RND_MsgC_HA: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_HA.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_HA.msbt").unwrap()),
        MAIL_RND_MsgC_ZK: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_ZK.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_ZK.msbt").unwrap()),
        MAIL_RND_MsgC_BO: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_BO.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_BO.msbt").unwrap()),
        MAIL_RND_MsgC_OT: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_OT.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_OT.msbt").unwrap()),
        MAIL_RND_MsgC_FU: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_FU.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_FU.msbt").unwrap()),
        MAIL_RND_MsgC_GE: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_GE.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_GE.msbt").unwrap()),
        MAIL_RND_MsgC_KO: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_KO.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_KO.msbt").unwrap()),
        MAIL_RND_MsgC_AN: File::new("/Mail/Rnd/MsgC/MAIL_RND_MsgC_AN.msbt", msbts.get("/Mail/Rnd/MsgC/MAIL_RND_MsgC_AN.msbt").unwrap()),
        }
    }
}

pub struct Mail_Rnd {
    pub MsgB: Mail_Rnd_MsgB,
    pub MsgA: Mail_Rnd_MsgA,
    pub MsgD: Mail_Rnd_MsgD,
    pub z_HeaderFooter: Mail_Rnd_z_HeaderFooter,
    pub MsgC: Mail_Rnd_MsgC,
}

impl Mail_Rnd {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail_Rnd {
        MsgB: Mail_Rnd_MsgB::new(msbts),
        MsgA: Mail_Rnd_MsgA::new(msbts),
        MsgD: Mail_Rnd_MsgD::new(msbts),
        z_HeaderFooter: Mail_Rnd_z_HeaderFooter::new(msbts),
        MsgC: Mail_Rnd_MsgC::new(msbts),
        }
    }
}

pub struct Mail {
    pub NNpc: Mail_NNpc,
    pub Sp: Mail_Sp,
    pub SNpc: Mail_SNpc,
    pub Rnd: Mail_Rnd,
    pub MAIL_PlayerDefault: File,
    pub MAIL_Tkk_BirthdayDemo: File,
}

impl Mail {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        Mail {
        NNpc: Mail_NNpc::new(msbts),
        Sp: Mail_Sp::new(msbts),
        SNpc: Mail_SNpc::new(msbts),
        Rnd: Mail_Rnd::new(msbts),
        MAIL_PlayerDefault: File::new("/Mail/MAIL_PlayerDefault.msbt", msbts.get("/Mail/MAIL_PlayerDefault.msbt").unwrap()),
        MAIL_Tkk_BirthdayDemo: File::new("/Mail/MAIL_Tkk_BirthdayDemo.msbt", msbts.get("/Mail/MAIL_Tkk_BirthdayDemo.msbt").unwrap()),
        }
    }
}

pub struct System_Balloon {
    pub BLN_GEvent_FishInsectFes: File,
    pub BLN_Common: File,
    pub BLN_Spot_Tailor: File,
    pub BLN_Spot_Gstore: File,
    pub BLN_Shop: File,
    pub BLN_Spot_Museum: File,
    pub BLN_Approach: File,
    pub BLN_GEvent_BirthdayP: File,
}

impl System_Balloon {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        System_Balloon {
        BLN_GEvent_FishInsectFes: File::new("/System/Balloon/BLN_GEvent_FishInsectFes.msbt", msbts.get("/System/Balloon/BLN_GEvent_FishInsectFes.msbt").unwrap()),
        BLN_Common: File::new("/System/Balloon/BLN_Common.msbt", msbts.get("/System/Balloon/BLN_Common.msbt").unwrap()),
        BLN_Spot_Tailor: File::new("/System/Balloon/BLN_Spot_Tailor.msbt", msbts.get("/System/Balloon/BLN_Spot_Tailor.msbt").unwrap()),
        BLN_Spot_Gstore: File::new("/System/Balloon/BLN_Spot_Gstore.msbt", msbts.get("/System/Balloon/BLN_Spot_Gstore.msbt").unwrap()),
        BLN_Shop: File::new("/System/Balloon/BLN_Shop.msbt", msbts.get("/System/Balloon/BLN_Shop.msbt").unwrap()),
        BLN_Spot_Museum: File::new("/System/Balloon/BLN_Spot_Museum.msbt", msbts.get("/System/Balloon/BLN_Spot_Museum.msbt").unwrap()),
        BLN_Approach: File::new("/System/Balloon/BLN_Approach.msbt", msbts.get("/System/Balloon/BLN_Approach.msbt").unwrap()),
        BLN_GEvent_BirthdayP: File::new("/System/Balloon/BLN_GEvent_BirthdayP.msbt", msbts.get("/System/Balloon/BLN_GEvent_BirthdayP.msbt").unwrap()),
        }
    }
}

pub struct System_Bbs {
    pub Bbs_EventNotice: File,
    pub Bbs_EventNotice_Birthday: File,
    pub Bbs_Information: File,
}

impl System_Bbs {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        System_Bbs {
        Bbs_EventNotice: File::new("/System/Bbs/Bbs_EventNotice.msbt", msbts.get("/System/Bbs/Bbs_EventNotice.msbt").unwrap()),
        Bbs_EventNotice_Birthday: File::new("/System/Bbs/Bbs_EventNotice_Birthday.msbt", msbts.get("/System/Bbs/Bbs_EventNotice_Birthday.msbt").unwrap()),
        Bbs_Information: File::new("/System/Bbs/Bbs_Information.msbt", msbts.get("/System/Bbs/Bbs_Information.msbt").unwrap()),
        }
    }
}

pub struct System_NookMilage {
    pub NookMilage_Keyword_Noun_List: File,
    pub NookMilagePlus_Title: File,
    pub NookMilage_List: File,
    pub NookMilage_Keyword_Modifier_List: File,
    pub NookMilage_Keyword_Modifier: File,
    pub NookMilage_Keyword_Noun: File,
}

impl System_NookMilage {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        System_NookMilage {
        NookMilage_Keyword_Noun_List: File::new("/System/NookMilage/NookMilage_Keyword_Noun_List.msbt", msbts.get("/System/NookMilage/NookMilage_Keyword_Noun_List.msbt").unwrap()),
        NookMilagePlus_Title: File::new("/System/NookMilage/NookMilagePlus_Title.msbt", msbts.get("/System/NookMilage/NookMilagePlus_Title.msbt").unwrap()),
        NookMilage_List: File::new("/System/NookMilage/NookMilage_List.msbt", msbts.get("/System/NookMilage/NookMilage_List.msbt").unwrap()),
        NookMilage_Keyword_Modifier_List: File::new("/System/NookMilage/NookMilage_Keyword_Modifier_List.msbt", msbts.get("/System/NookMilage/NookMilage_Keyword_Modifier_List.msbt").unwrap()),
        NookMilage_Keyword_Modifier: File::new("/System/NookMilage/NookMilage_Keyword_Modifier.msbt", msbts.get("/System/NookMilage/NookMilage_Keyword_Modifier.msbt").unwrap()),
        NookMilage_Keyword_Noun: File::new("/System/NookMilage/NookMilage_Keyword_Noun.msbt", msbts.get("/System/NookMilage/NookMilage_Keyword_Noun.msbt").unwrap()),
        }
    }
}

pub struct System {
    pub Balloon: System_Balloon,
    pub Bbs: System_Bbs,
    pub NookMilage: System_NookMilage,
    pub ETC_Swkbd: File,
    pub ETC_ChatBalloon: File,
}

impl System {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        System {
        Balloon: System_Balloon::new(msbts),
        Bbs: System_Bbs::new(msbts),
        NookMilage: System_NookMilage::new(msbts),
        ETC_Swkbd: File::new("/System/ETC_Swkbd.msbt", msbts.get("/System/ETC_Swkbd.msbt").unwrap()),
        ETC_ChatBalloon: File::new("/System/ETC_ChatBalloon.msbt", msbts.get("/System/ETC_ChatBalloon.msbt").unwrap()),
        }
    }
}

pub struct TalkFtr {
    pub FTR_TrashBox: File,
    pub FTR_Audio: File,
    pub FTR_Radio: File,
    pub FTR_Bromide: File,
    pub FTR_Chest: File,
    pub FTR_CampBed: File,
    pub FTR_StampRally: File,
    pub FTR_Toilet: File,
    pub FTR_WorkingTable: File,
    pub FTR_DressingTable: File,
}

impl TalkFtr {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkFtr {
        FTR_TrashBox: File::new("/TalkFtr/FTR_TrashBox.msbt", msbts.get("/TalkFtr/FTR_TrashBox.msbt").unwrap()),
        FTR_Audio: File::new("/TalkFtr/FTR_Audio.msbt", msbts.get("/TalkFtr/FTR_Audio.msbt").unwrap()),
        FTR_Radio: File::new("/TalkFtr/FTR_Radio.msbt", msbts.get("/TalkFtr/FTR_Radio.msbt").unwrap()),
        FTR_Bromide: File::new("/TalkFtr/FTR_Bromide.msbt", msbts.get("/TalkFtr/FTR_Bromide.msbt").unwrap()),
        FTR_Chest: File::new("/TalkFtr/FTR_Chest.msbt", msbts.get("/TalkFtr/FTR_Chest.msbt").unwrap()),
        FTR_CampBed: File::new("/TalkFtr/FTR_CampBed.msbt", msbts.get("/TalkFtr/FTR_CampBed.msbt").unwrap()),
        FTR_StampRally: File::new("/TalkFtr/FTR_StampRally.msbt", msbts.get("/TalkFtr/FTR_StampRally.msbt").unwrap()),
        FTR_Toilet: File::new("/TalkFtr/FTR_Toilet.msbt", msbts.get("/TalkFtr/FTR_Toilet.msbt").unwrap()),
        FTR_WorkingTable: File::new("/TalkFtr/FTR_WorkingTable.msbt", msbts.get("/TalkFtr/FTR_WorkingTable.msbt").unwrap()),
        FTR_DressingTable: File::new("/TalkFtr/FTR_DressingTable.msbt", msbts.get("/TalkFtr/FTR_DressingTable.msbt").unwrap()),
        }
    }
}

pub struct LayoutMsg_Parts {
    pub PARTS_SortNoticeWindow: File,
    pub PARTS_NameWindow: File,
}

impl LayoutMsg_Parts {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        LayoutMsg_Parts {
        PARTS_SortNoticeWindow: File::new("/LayoutMsg/Parts/PARTS_SortNoticeWindow.msbt", msbts.get("/LayoutMsg/Parts/PARTS_SortNoticeWindow.msbt").unwrap()),
        PARTS_NameWindow: File::new("/LayoutMsg/Parts/PARTS_NameWindow.msbt", msbts.get("/LayoutMsg/Parts/PARTS_NameWindow.msbt").unwrap()),
        }
    }
}

pub struct LayoutMsg_Swkbd_Footer {
    pub Footer: File,
}

impl LayoutMsg_Swkbd_Footer {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        LayoutMsg_Swkbd_Footer {
        Footer: File::new("/LayoutMsg/Swkbd/Footer/Footer.msbt", msbts.get("/LayoutMsg/Swkbd/Footer/Footer.msbt").unwrap()),
        }
    }
}

pub struct LayoutMsg_Swkbd {
    pub Footer: LayoutMsg_Swkbd_Footer,
}

impl LayoutMsg_Swkbd {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        LayoutMsg_Swkbd {
        Footer: LayoutMsg_Swkbd_Footer::new(msbts),
        }
    }
}

pub struct LayoutMsg {
    pub Parts: LayoutMsg_Parts,
    pub Swkbd: LayoutMsg_Swkbd,
    pub CameraGuide: File,
    pub CharaMakeMWSelWin: File,
    pub ProfileWinPhoto: File,
    pub Audio: File,
    pub ShopBuyTailor: File,
    pub SwkbdTextAreaTaste: File,
    pub MenuPointTopQuest: File,
    pub PlayerNameMarker: File,
    pub MainScreen: File,
    pub CordeIconSelecter: File,
    pub SwkbdTextAreaGreeting: File,
    pub MenuPointResultList: File,
    pub ClosetCorde: File,
    pub MyDesignEdit: File,
    pub Interior: File,
    pub MenuPointResultInfo: File,
    pub Sonkatsu: File,
    pub LostList: File,
    pub MenuPointTopQuestEvent: File,
    pub BoardEdit: File,
    pub MenuMobileBank: File,
    pub Melody: File,
    pub SwkbdTextAreaInputBank: File,
    pub MenuPointResultDialog: File,
    pub Addressee: File,
    pub MenuRecipe: File,
    pub MenuMapFull: File,
    pub SwkbdTextAreaKaburiba: File,
    pub MenuChatLog: File,
    pub SwkbdTextAreaMyDesignName: File,
    pub MyDesignColorSelect: File,
    pub Telop: File,
    pub MyDesignExchList: File,
    pub CatalogGift: File,
    pub StorageInterior: File,
    pub Board: File,
    pub MyDesignList: File,
    pub GuideBalloonWeddingParty: File,
    pub MyDesignExchInfo: File,
    pub ShopBridgeSlope: File,
    pub BoardInfo: File,
    pub Closet: File,
    pub PocketMenu: File,
    pub SwkbdTextAreaCoord: File,
    pub AmiiboTouchGuide: File,
    pub MenuDevice: File,
    pub Trip: File,
    pub MsgCardDesignSelect: File,
    pub KeyGuide: File,
    pub AddresseeSelMulti: File,
    pub CharaMakeMap: File,
    pub RingMenuCorde: File,
    pub SwkbdTextAreaPlayerName: File,
    pub StorageInteriorStudio: File,
    pub AddresseeFriend: File,
    pub SwkbdTextAreaPasswordNum: File,
    pub Version: File,
    pub ShopBuy: File,
    pub CharaMake: File,
    pub HighAndLow: File,
    pub CmnSubWin: File,
    pub Road: File,
    pub MenuPointTopBar: File,
    pub SwkbdTextAreaCatalogShop: File,
    pub MsgCardView: File,
    pub ProfileWinPlayerTitle: File,
    pub CustomHome: File,
    pub Title: File,
    pub MenuBookDetail: File,
    pub MyDesignEditCanvasSelWin: File,
    pub WorkBench: File,
    pub SwkbdTextAreaProfile: File,
    pub ErrorWindow: File,
    pub SwkbdTextAreaFavoritePhrase: File,
    pub SwkbdTextAreaHaniwa: File,
    pub MyDesignExchInfoBtn: File,
    pub MsgBottle: File,
    pub StaffCredit: File,
    pub WorkBenchRemakeSelect: File,
    pub WorkBenchInfo: File,
    pub SwkbdTextAreaNickname: File,
    pub FriendList: File,
    pub AmiiboTouchGuideL: File,
    pub MyDesignExchMenu: File,
    pub SwkbdTextAreaBestFriend: File,
    pub MyDesignExchListTop: File,
    pub Catalog: File,
    pub DateInputYear: File,
    pub MyDesignSet: File,
    pub DateInput: File,
    pub SwkbdTextAreaMyDesignSearch: File,
    pub MenuPointQuestList: File,
    pub MenuBook: File,
    pub SwkbdTextAreaVillageName: File,
    pub Report: File,
    pub SwkbdTextAreaLive: File,
    pub MapRemove: File,
    pub BirthdayMessage: File,
    pub KeyGuideSkip: File,
    pub Post: File,
    pub SwkbdTextAreaPassword: File,
    pub MenuProfile: File,
    pub LoadGoOut: File,
    pub ShopSonkatsu: File,
}

impl LayoutMsg {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        LayoutMsg {
        Parts: LayoutMsg_Parts::new(msbts),
        Swkbd: LayoutMsg_Swkbd::new(msbts),
        CameraGuide: File::new("/LayoutMsg/CameraGuide.msbt", msbts.get("/LayoutMsg/CameraGuide.msbt").unwrap()),
        CharaMakeMWSelWin: File::new("/LayoutMsg/CharaMakeMWSelWin.msbt", msbts.get("/LayoutMsg/CharaMakeMWSelWin.msbt").unwrap()),
        ProfileWinPhoto: File::new("/LayoutMsg/ProfileWinPhoto.msbt", msbts.get("/LayoutMsg/ProfileWinPhoto.msbt").unwrap()),
        Audio: File::new("/LayoutMsg/Audio.msbt", msbts.get("/LayoutMsg/Audio.msbt").unwrap()),
        ShopBuyTailor: File::new("/LayoutMsg/ShopBuyTailor.msbt", msbts.get("/LayoutMsg/ShopBuyTailor.msbt").unwrap()),
        SwkbdTextAreaTaste: File::new("/LayoutMsg/SwkbdTextAreaTaste.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaTaste.msbt").unwrap()),
        MenuPointTopQuest: File::new("/LayoutMsg/MenuPointTopQuest.msbt", msbts.get("/LayoutMsg/MenuPointTopQuest.msbt").unwrap()),
        PlayerNameMarker: File::new("/LayoutMsg/PlayerNameMarker.msbt", msbts.get("/LayoutMsg/PlayerNameMarker.msbt").unwrap()),
        MainScreen: File::new("/LayoutMsg/MainScreen.msbt", msbts.get("/LayoutMsg/MainScreen.msbt").unwrap()),
        CordeIconSelecter: File::new("/LayoutMsg/CordeIconSelecter.msbt", msbts.get("/LayoutMsg/CordeIconSelecter.msbt").unwrap()),
        SwkbdTextAreaGreeting: File::new("/LayoutMsg/SwkbdTextAreaGreeting.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaGreeting.msbt").unwrap()),
        MenuPointResultList: File::new("/LayoutMsg/MenuPointResultList.msbt", msbts.get("/LayoutMsg/MenuPointResultList.msbt").unwrap()),
        ClosetCorde: File::new("/LayoutMsg/ClosetCorde.msbt", msbts.get("/LayoutMsg/ClosetCorde.msbt").unwrap()),
        MyDesignEdit: File::new("/LayoutMsg/MyDesignEdit.msbt", msbts.get("/LayoutMsg/MyDesignEdit.msbt").unwrap()),
        Interior: File::new("/LayoutMsg/Interior.msbt", msbts.get("/LayoutMsg/Interior.msbt").unwrap()),
        MenuPointResultInfo: File::new("/LayoutMsg/MenuPointResultInfo.msbt", msbts.get("/LayoutMsg/MenuPointResultInfo.msbt").unwrap()),
        Sonkatsu: File::new("/LayoutMsg/Sonkatsu.msbt", msbts.get("/LayoutMsg/Sonkatsu.msbt").unwrap()),
        LostList: File::new("/LayoutMsg/LostList.msbt", msbts.get("/LayoutMsg/LostList.msbt").unwrap()),
        MenuPointTopQuestEvent: File::new("/LayoutMsg/MenuPointTopQuestEvent.msbt", msbts.get("/LayoutMsg/MenuPointTopQuestEvent.msbt").unwrap()),
        BoardEdit: File::new("/LayoutMsg/BoardEdit.msbt", msbts.get("/LayoutMsg/BoardEdit.msbt").unwrap()),
        MenuMobileBank: File::new("/LayoutMsg/MenuMobileBank.msbt", msbts.get("/LayoutMsg/MenuMobileBank.msbt").unwrap()),
        Melody: File::new("/LayoutMsg/Melody.msbt", msbts.get("/LayoutMsg/Melody.msbt").unwrap()),
        SwkbdTextAreaInputBank: File::new("/LayoutMsg/SwkbdTextAreaInputBank.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaInputBank.msbt").unwrap()),
        MenuPointResultDialog: File::new("/LayoutMsg/MenuPointResultDialog.msbt", msbts.get("/LayoutMsg/MenuPointResultDialog.msbt").unwrap()),
        Addressee: File::new("/LayoutMsg/Addressee.msbt", msbts.get("/LayoutMsg/Addressee.msbt").unwrap()),
        MenuRecipe: File::new("/LayoutMsg/MenuRecipe.msbt", msbts.get("/LayoutMsg/MenuRecipe.msbt").unwrap()),
        MenuMapFull: File::new("/LayoutMsg/MenuMapFull.msbt", msbts.get("/LayoutMsg/MenuMapFull.msbt").unwrap()),
        SwkbdTextAreaKaburiba: File::new("/LayoutMsg/SwkbdTextAreaKaburiba.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaKaburiba.msbt").unwrap()),
        MenuChatLog: File::new("/LayoutMsg/MenuChatLog.msbt", msbts.get("/LayoutMsg/MenuChatLog.msbt").unwrap()),
        SwkbdTextAreaMyDesignName: File::new("/LayoutMsg/SwkbdTextAreaMyDesignName.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaMyDesignName.msbt").unwrap()),
        MyDesignColorSelect: File::new("/LayoutMsg/MyDesignColorSelect.msbt", msbts.get("/LayoutMsg/MyDesignColorSelect.msbt").unwrap()),
        Telop: File::new("/LayoutMsg/Telop.msbt", msbts.get("/LayoutMsg/Telop.msbt").unwrap()),
        MyDesignExchList: File::new("/LayoutMsg/MyDesignExchList.msbt", msbts.get("/LayoutMsg/MyDesignExchList.msbt").unwrap()),
        CatalogGift: File::new("/LayoutMsg/CatalogGift.msbt", msbts.get("/LayoutMsg/CatalogGift.msbt").unwrap()),
        StorageInterior: File::new("/LayoutMsg/StorageInterior.msbt", msbts.get("/LayoutMsg/StorageInterior.msbt").unwrap()),
        Board: File::new("/LayoutMsg/Board.msbt", msbts.get("/LayoutMsg/Board.msbt").unwrap()),
        MyDesignList: File::new("/LayoutMsg/MyDesignList.msbt", msbts.get("/LayoutMsg/MyDesignList.msbt").unwrap()),
        GuideBalloonWeddingParty: File::new("/LayoutMsg/GuideBalloonWeddingParty.msbt", msbts.get("/LayoutMsg/GuideBalloonWeddingParty.msbt").unwrap()),
        MyDesignExchInfo: File::new("/LayoutMsg/MyDesignExchInfo.msbt", msbts.get("/LayoutMsg/MyDesignExchInfo.msbt").unwrap()),
        ShopBridgeSlope: File::new("/LayoutMsg/ShopBridgeSlope.msbt", msbts.get("/LayoutMsg/ShopBridgeSlope.msbt").unwrap()),
        BoardInfo: File::new("/LayoutMsg/BoardInfo.msbt", msbts.get("/LayoutMsg/BoardInfo.msbt").unwrap()),
        Closet: File::new("/LayoutMsg/Closet.msbt", msbts.get("/LayoutMsg/Closet.msbt").unwrap()),
        PocketMenu: File::new("/LayoutMsg/PocketMenu.msbt", msbts.get("/LayoutMsg/PocketMenu.msbt").unwrap()),
        SwkbdTextAreaCoord: File::new("/LayoutMsg/SwkbdTextAreaCoord.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaCoord.msbt").unwrap()),
        AmiiboTouchGuide: File::new("/LayoutMsg/AmiiboTouchGuide.msbt", msbts.get("/LayoutMsg/AmiiboTouchGuide.msbt").unwrap()),
        MenuDevice: File::new("/LayoutMsg/MenuDevice.msbt", msbts.get("/LayoutMsg/MenuDevice.msbt").unwrap()),
        Trip: File::new("/LayoutMsg/Trip.msbt", msbts.get("/LayoutMsg/Trip.msbt").unwrap()),
        MsgCardDesignSelect: File::new("/LayoutMsg/MsgCardDesignSelect.msbt", msbts.get("/LayoutMsg/MsgCardDesignSelect.msbt").unwrap()),
        KeyGuide: File::new("/LayoutMsg/KeyGuide.msbt", msbts.get("/LayoutMsg/KeyGuide.msbt").unwrap()),
        AddresseeSelMulti: File::new("/LayoutMsg/AddresseeSelMulti.msbt", msbts.get("/LayoutMsg/AddresseeSelMulti.msbt").unwrap()),
        CharaMakeMap: File::new("/LayoutMsg/CharaMakeMap.msbt", msbts.get("/LayoutMsg/CharaMakeMap.msbt").unwrap()),
        RingMenuCorde: File::new("/LayoutMsg/RingMenuCorde.msbt", msbts.get("/LayoutMsg/RingMenuCorde.msbt").unwrap()),
        SwkbdTextAreaPlayerName: File::new("/LayoutMsg/SwkbdTextAreaPlayerName.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaPlayerName.msbt").unwrap()),
        StorageInteriorStudio: File::new("/LayoutMsg/StorageInteriorStudio.msbt", msbts.get("/LayoutMsg/StorageInteriorStudio.msbt").unwrap()),
        AddresseeFriend: File::new("/LayoutMsg/AddresseeFriend.msbt", msbts.get("/LayoutMsg/AddresseeFriend.msbt").unwrap()),
        SwkbdTextAreaPasswordNum: File::new("/LayoutMsg/SwkbdTextAreaPasswordNum.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaPasswordNum.msbt").unwrap()),
        Version: File::new("/LayoutMsg/Version.msbt", msbts.get("/LayoutMsg/Version.msbt").unwrap()),
        ShopBuy: File::new("/LayoutMsg/ShopBuy.msbt", msbts.get("/LayoutMsg/ShopBuy.msbt").unwrap()),
        CharaMake: File::new("/LayoutMsg/CharaMake.msbt", msbts.get("/LayoutMsg/CharaMake.msbt").unwrap()),
        HighAndLow: File::new("/LayoutMsg/HighAndLow.msbt", msbts.get("/LayoutMsg/HighAndLow.msbt").unwrap()),
        CmnSubWin: File::new("/LayoutMsg/CmnSubWin.msbt", msbts.get("/LayoutMsg/CmnSubWin.msbt").unwrap()),
        Road: File::new("/LayoutMsg/Road.msbt", msbts.get("/LayoutMsg/Road.msbt").unwrap()),
        MenuPointTopBar: File::new("/LayoutMsg/MenuPointTopBar.msbt", msbts.get("/LayoutMsg/MenuPointTopBar.msbt").unwrap()),
        SwkbdTextAreaCatalogShop: File::new("/LayoutMsg/SwkbdTextAreaCatalogShop.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaCatalogShop.msbt").unwrap()),
        MsgCardView: File::new("/LayoutMsg/MsgCardView.msbt", msbts.get("/LayoutMsg/MsgCardView.msbt").unwrap()),
        ProfileWinPlayerTitle: File::new("/LayoutMsg/ProfileWinPlayerTitle.msbt", msbts.get("/LayoutMsg/ProfileWinPlayerTitle.msbt").unwrap()),
        CustomHome: File::new("/LayoutMsg/CustomHome.msbt", msbts.get("/LayoutMsg/CustomHome.msbt").unwrap()),
        Title: File::new("/LayoutMsg/Title.msbt", msbts.get("/LayoutMsg/Title.msbt").unwrap()),
        MenuBookDetail: File::new("/LayoutMsg/MenuBookDetail.msbt", msbts.get("/LayoutMsg/MenuBookDetail.msbt").unwrap()),
        MyDesignEditCanvasSelWin: File::new("/LayoutMsg/MyDesignEditCanvasSelWin.msbt", msbts.get("/LayoutMsg/MyDesignEditCanvasSelWin.msbt").unwrap()),
        WorkBench: File::new("/LayoutMsg/WorkBench.msbt", msbts.get("/LayoutMsg/WorkBench.msbt").unwrap()),
        SwkbdTextAreaProfile: File::new("/LayoutMsg/SwkbdTextAreaProfile.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaProfile.msbt").unwrap()),
        ErrorWindow: File::new("/LayoutMsg/ErrorWindow.msbt", msbts.get("/LayoutMsg/ErrorWindow.msbt").unwrap()),
        SwkbdTextAreaFavoritePhrase: File::new("/LayoutMsg/SwkbdTextAreaFavoritePhrase.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaFavoritePhrase.msbt").unwrap()),
        SwkbdTextAreaHaniwa: File::new("/LayoutMsg/SwkbdTextAreaHaniwa.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaHaniwa.msbt").unwrap()),
        MyDesignExchInfoBtn: File::new("/LayoutMsg/MyDesignExchInfoBtn.msbt", msbts.get("/LayoutMsg/MyDesignExchInfoBtn.msbt").unwrap()),
        MsgBottle: File::new("/LayoutMsg/MsgBottle.msbt", msbts.get("/LayoutMsg/MsgBottle.msbt").unwrap()),
        StaffCredit: File::new("/LayoutMsg/StaffCredit.msbt", msbts.get("/LayoutMsg/StaffCredit.msbt").unwrap()),
        WorkBenchRemakeSelect: File::new("/LayoutMsg/WorkBenchRemakeSelect.msbt", msbts.get("/LayoutMsg/WorkBenchRemakeSelect.msbt").unwrap()),
        WorkBenchInfo: File::new("/LayoutMsg/WorkBenchInfo.msbt", msbts.get("/LayoutMsg/WorkBenchInfo.msbt").unwrap()),
        SwkbdTextAreaNickname: File::new("/LayoutMsg/SwkbdTextAreaNickname.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaNickname.msbt").unwrap()),
        FriendList: File::new("/LayoutMsg/FriendList.msbt", msbts.get("/LayoutMsg/FriendList.msbt").unwrap()),
        AmiiboTouchGuideL: File::new("/LayoutMsg/AmiiboTouchGuideL.msbt", msbts.get("/LayoutMsg/AmiiboTouchGuideL.msbt").unwrap()),
        MyDesignExchMenu: File::new("/LayoutMsg/MyDesignExchMenu.msbt", msbts.get("/LayoutMsg/MyDesignExchMenu.msbt").unwrap()),
        SwkbdTextAreaBestFriend: File::new("/LayoutMsg/SwkbdTextAreaBestFriend.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaBestFriend.msbt").unwrap()),
        MyDesignExchListTop: File::new("/LayoutMsg/MyDesignExchListTop.msbt", msbts.get("/LayoutMsg/MyDesignExchListTop.msbt").unwrap()),
        Catalog: File::new("/LayoutMsg/Catalog.msbt", msbts.get("/LayoutMsg/Catalog.msbt").unwrap()),
        DateInputYear: File::new("/LayoutMsg/DateInputYear.msbt", msbts.get("/LayoutMsg/DateInputYear.msbt").unwrap()),
        MyDesignSet: File::new("/LayoutMsg/MyDesignSet.msbt", msbts.get("/LayoutMsg/MyDesignSet.msbt").unwrap()),
        DateInput: File::new("/LayoutMsg/DateInput.msbt", msbts.get("/LayoutMsg/DateInput.msbt").unwrap()),
        SwkbdTextAreaMyDesignSearch: File::new("/LayoutMsg/SwkbdTextAreaMyDesignSearch.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaMyDesignSearch.msbt").unwrap()),
        MenuPointQuestList: File::new("/LayoutMsg/MenuPointQuestList.msbt", msbts.get("/LayoutMsg/MenuPointQuestList.msbt").unwrap()),
        MenuBook: File::new("/LayoutMsg/MenuBook.msbt", msbts.get("/LayoutMsg/MenuBook.msbt").unwrap()),
        SwkbdTextAreaVillageName: File::new("/LayoutMsg/SwkbdTextAreaVillageName.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaVillageName.msbt").unwrap()),
        Report: File::new("/LayoutMsg/Report.msbt", msbts.get("/LayoutMsg/Report.msbt").unwrap()),
        SwkbdTextAreaLive: File::new("/LayoutMsg/SwkbdTextAreaLive.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaLive.msbt").unwrap()),
        MapRemove: File::new("/LayoutMsg/MapRemove.msbt", msbts.get("/LayoutMsg/MapRemove.msbt").unwrap()),
        BirthdayMessage: File::new("/LayoutMsg/BirthdayMessage.msbt", msbts.get("/LayoutMsg/BirthdayMessage.msbt").unwrap()),
        KeyGuideSkip: File::new("/LayoutMsg/KeyGuideSkip.msbt", msbts.get("/LayoutMsg/KeyGuideSkip.msbt").unwrap()),
        Post: File::new("/LayoutMsg/Post.msbt", msbts.get("/LayoutMsg/Post.msbt").unwrap()),
        SwkbdTextAreaPassword: File::new("/LayoutMsg/SwkbdTextAreaPassword.msbt", msbts.get("/LayoutMsg/SwkbdTextAreaPassword.msbt").unwrap()),
        MenuProfile: File::new("/LayoutMsg/MenuProfile.msbt", msbts.get("/LayoutMsg/MenuProfile.msbt").unwrap()),
        LoadGoOut: File::new("/LayoutMsg/LoadGoOut.msbt", msbts.get("/LayoutMsg/LoadGoOut.msbt").unwrap()),
        ShopSonkatsu: File::new("/LayoutMsg/ShopSonkatsu.msbt", msbts.get("/LayoutMsg/ShopSonkatsu.msbt").unwrap()),
        }
    }
}

pub struct TalkObj {
    pub OBJ_RcoMachine: File,
    pub OBJ_Sign: File,
    pub OBJ_Nameboard: File,
    pub OBJ_MigrantsQuest: File,
    pub OBJ_MessageCardRack: File,
    pub OBJ_PurchaseBox: File,
    pub OBJ_WorldMyDesignGallery: File,
    pub OBJ_InsectFishFesBOX: File,
    pub OBJ_Gyroid: File,
    pub OBJ_Snowman: File,
}

impl TalkObj {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkObj {
        OBJ_RcoMachine: File::new("/TalkObj/OBJ_RcoMachine.msbt", msbts.get("/TalkObj/OBJ_RcoMachine.msbt").unwrap()),
        OBJ_Sign: File::new("/TalkObj/OBJ_Sign.msbt", msbts.get("/TalkObj/OBJ_Sign.msbt").unwrap()),
        OBJ_Nameboard: File::new("/TalkObj/OBJ_Nameboard.msbt", msbts.get("/TalkObj/OBJ_Nameboard.msbt").unwrap()),
        OBJ_MigrantsQuest: File::new("/TalkObj/OBJ_MigrantsQuest.msbt", msbts.get("/TalkObj/OBJ_MigrantsQuest.msbt").unwrap()),
        OBJ_MessageCardRack: File::new("/TalkObj/OBJ_MessageCardRack.msbt", msbts.get("/TalkObj/OBJ_MessageCardRack.msbt").unwrap()),
        OBJ_PurchaseBox: File::new("/TalkObj/OBJ_PurchaseBox.msbt", msbts.get("/TalkObj/OBJ_PurchaseBox.msbt").unwrap()),
        OBJ_WorldMyDesignGallery: File::new("/TalkObj/OBJ_WorldMyDesignGallery.msbt", msbts.get("/TalkObj/OBJ_WorldMyDesignGallery.msbt").unwrap()),
        OBJ_InsectFishFesBOX: File::new("/TalkObj/OBJ_InsectFishFesBOX.msbt", msbts.get("/TalkObj/OBJ_InsectFishFesBOX.msbt").unwrap()),
        OBJ_Gyroid: File::new("/TalkObj/OBJ_Gyroid.msbt", msbts.get("/TalkObj/OBJ_Gyroid.msbt").unwrap()),
        OBJ_Snowman: File::new("/TalkObj/OBJ_Snowman.msbt", msbts.get("/TalkObj/OBJ_Snowman.msbt").unwrap()),
        }
    }
}

pub struct String_Item {
    pub STR_ItemName_61_HouseDoorDeco: File,
    pub STR_ItemName_31_Fish: File,
    pub STR_ItemName_70_Craft: File,
    pub STR_ItemName_83_Fence: File,
    pub STR_ItemName_40_Plant: File,
    pub STR_ItemName_33_Shell: File,
    pub STR_ItemName_41_Turnip: File,
    pub STR_ItemName_19_Umbrella: File,
    pub STR_ItemName_90_Money: File,
    pub STR_ItemName_50_RoomWall: File,
    pub STR_ItemName_20_Tool: File,
    pub STR_ItemName_85_BridgeSlope: File,
    pub STR_ItemName_01_Art: File,
    pub STR_ItemName_86_Poster: File,
    pub STR_ItemName_52_RoomRug: File,
    pub STR_ItemName_37_FishToy: File,
    pub STR_ItemName_81_Event: File,
    pub STR_ItemName_34_Fossil: File,
    pub STR_ItemName_82_Music: File,
    pub STR_ItemName_15_Cap: File,
    pub STR_ItemName_80_Etc: File,
    pub STR_ItemName_36_InsectToy: File,
    pub STR_ItemName_62_HousePost: File,
    pub STR_ItemName_51_RoomFloor: File,
    pub STR_ItemName_91_PhotoStudioList: File,
    pub STR_ItemName_30_Insect: File,
    pub STR_ItemName_00_Ftr: File,
    pub STR_ItemName_84_Bromide: File,
}

impl String_Item {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Item {
        STR_ItemName_61_HouseDoorDeco: File::new("/String/Item/STR_ItemName_61_HouseDoorDeco.msbt", msbts.get("/String/Item/STR_ItemName_61_HouseDoorDeco.msbt").unwrap()),
        STR_ItemName_31_Fish: File::new("/String/Item/STR_ItemName_31_Fish.msbt", msbts.get("/String/Item/STR_ItemName_31_Fish.msbt").unwrap()),
        STR_ItemName_70_Craft: File::new("/String/Item/STR_ItemName_70_Craft.msbt", msbts.get("/String/Item/STR_ItemName_70_Craft.msbt").unwrap()),
        STR_ItemName_83_Fence: File::new("/String/Item/STR_ItemName_83_Fence.msbt", msbts.get("/String/Item/STR_ItemName_83_Fence.msbt").unwrap()),
        STR_ItemName_40_Plant: File::new("/String/Item/STR_ItemName_40_Plant.msbt", msbts.get("/String/Item/STR_ItemName_40_Plant.msbt").unwrap()),
        STR_ItemName_33_Shell: File::new("/String/Item/STR_ItemName_33_Shell.msbt", msbts.get("/String/Item/STR_ItemName_33_Shell.msbt").unwrap()),
        STR_ItemName_41_Turnip: File::new("/String/Item/STR_ItemName_41_Turnip.msbt", msbts.get("/String/Item/STR_ItemName_41_Turnip.msbt").unwrap()),
        STR_ItemName_19_Umbrella: File::new("/String/Item/STR_ItemName_19_Umbrella.msbt", msbts.get("/String/Item/STR_ItemName_19_Umbrella.msbt").unwrap()),
        STR_ItemName_90_Money: File::new("/String/Item/STR_ItemName_90_Money.msbt", msbts.get("/String/Item/STR_ItemName_90_Money.msbt").unwrap()),
        STR_ItemName_50_RoomWall: File::new("/String/Item/STR_ItemName_50_RoomWall.msbt", msbts.get("/String/Item/STR_ItemName_50_RoomWall.msbt").unwrap()),
        STR_ItemName_20_Tool: File::new("/String/Item/STR_ItemName_20_Tool.msbt", msbts.get("/String/Item/STR_ItemName_20_Tool.msbt").unwrap()),
        STR_ItemName_85_BridgeSlope: File::new("/String/Item/STR_ItemName_85_BridgeSlope.msbt", msbts.get("/String/Item/STR_ItemName_85_BridgeSlope.msbt").unwrap()),
        STR_ItemName_01_Art: File::new("/String/Item/STR_ItemName_01_Art.msbt", msbts.get("/String/Item/STR_ItemName_01_Art.msbt").unwrap()),
        STR_ItemName_86_Poster: File::new("/String/Item/STR_ItemName_86_Poster.msbt", msbts.get("/String/Item/STR_ItemName_86_Poster.msbt").unwrap()),
        STR_ItemName_52_RoomRug: File::new("/String/Item/STR_ItemName_52_RoomRug.msbt", msbts.get("/String/Item/STR_ItemName_52_RoomRug.msbt").unwrap()),
        STR_ItemName_37_FishToy: File::new("/String/Item/STR_ItemName_37_FishToy.msbt", msbts.get("/String/Item/STR_ItemName_37_FishToy.msbt").unwrap()),
        STR_ItemName_81_Event: File::new("/String/Item/STR_ItemName_81_Event.msbt", msbts.get("/String/Item/STR_ItemName_81_Event.msbt").unwrap()),
        STR_ItemName_34_Fossil: File::new("/String/Item/STR_ItemName_34_Fossil.msbt", msbts.get("/String/Item/STR_ItemName_34_Fossil.msbt").unwrap()),
        STR_ItemName_82_Music: File::new("/String/Item/STR_ItemName_82_Music.msbt", msbts.get("/String/Item/STR_ItemName_82_Music.msbt").unwrap()),
        STR_ItemName_15_Cap: File::new("/String/Item/STR_ItemName_15_Cap.msbt", msbts.get("/String/Item/STR_ItemName_15_Cap.msbt").unwrap()),
        STR_ItemName_80_Etc: File::new("/String/Item/STR_ItemName_80_Etc.msbt", msbts.get("/String/Item/STR_ItemName_80_Etc.msbt").unwrap()),
        STR_ItemName_36_InsectToy: File::new("/String/Item/STR_ItemName_36_InsectToy.msbt", msbts.get("/String/Item/STR_ItemName_36_InsectToy.msbt").unwrap()),
        STR_ItemName_62_HousePost: File::new("/String/Item/STR_ItemName_62_HousePost.msbt", msbts.get("/String/Item/STR_ItemName_62_HousePost.msbt").unwrap()),
        STR_ItemName_51_RoomFloor: File::new("/String/Item/STR_ItemName_51_RoomFloor.msbt", msbts.get("/String/Item/STR_ItemName_51_RoomFloor.msbt").unwrap()),
        STR_ItemName_91_PhotoStudioList: File::new("/String/Item/STR_ItemName_91_PhotoStudioList.msbt", msbts.get("/String/Item/STR_ItemName_91_PhotoStudioList.msbt").unwrap()),
        STR_ItemName_30_Insect: File::new("/String/Item/STR_ItemName_30_Insect.msbt", msbts.get("/String/Item/STR_ItemName_30_Insect.msbt").unwrap()),
        STR_ItemName_00_Ftr: File::new("/String/Item/STR_ItemName_00_Ftr.msbt", msbts.get("/String/Item/STR_ItemName_00_Ftr.msbt").unwrap()),
        STR_ItemName_84_Bromide: File::new("/String/Item/STR_ItemName_84_Bromide.msbt", msbts.get("/String/Item/STR_ItemName_84_Bromide.msbt").unwrap()),
        }
    }
}

pub struct String_Sp_HHA {
    pub STR_HHA_Set: File,
    pub STR_HHA_RoomLocation: File,
    pub STR_HHA_Situation: File,
    pub STR_HHA_Rank: File,
    pub STR_HHA_Theme: File,
}

impl String_Sp_HHA {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Sp_HHA {
        STR_HHA_Set: File::new("/String/Sp/HHA/STR_HHA_Set.msbt", msbts.get("/String/Sp/HHA/STR_HHA_Set.msbt").unwrap()),
        STR_HHA_RoomLocation: File::new("/String/Sp/HHA/STR_HHA_RoomLocation.msbt", msbts.get("/String/Sp/HHA/STR_HHA_RoomLocation.msbt").unwrap()),
        STR_HHA_Situation: File::new("/String/Sp/HHA/STR_HHA_Situation.msbt", msbts.get("/String/Sp/HHA/STR_HHA_Situation.msbt").unwrap()),
        STR_HHA_Rank: File::new("/String/Sp/HHA/STR_HHA_Rank.msbt", msbts.get("/String/Sp/HHA/STR_HHA_Rank.msbt").unwrap()),
        STR_HHA_Theme: File::new("/String/Sp/HHA/STR_HHA_Theme.msbt", msbts.get("/String/Sp/HHA/STR_HHA_Theme.msbt").unwrap()),
        }
    }
}

pub struct String_Sp {
    pub HHA: String_Sp_HHA,
    pub STR_BestFriendNotice: File,
    pub STR_Emoticon: File,
    pub STR_FashionTheme: File,
    pub STR_Flower: File,
    pub STR_Favorite: File,
    pub STR_Dinosaur: File,
}

impl String_Sp {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Sp {
        HHA: String_Sp_HHA::new(msbts),
        STR_BestFriendNotice: File::new("/String/Sp/STR_BestFriendNotice.msbt", msbts.get("/String/Sp/STR_BestFriendNotice.msbt").unwrap()),
        STR_Emoticon: File::new("/String/Sp/STR_Emoticon.msbt", msbts.get("/String/Sp/STR_Emoticon.msbt").unwrap()),
        STR_FashionTheme: File::new("/String/Sp/STR_FashionTheme.msbt", msbts.get("/String/Sp/STR_FashionTheme.msbt").unwrap()),
        STR_Flower: File::new("/String/Sp/STR_Flower.msbt", msbts.get("/String/Sp/STR_Flower.msbt").unwrap()),
        STR_Favorite: File::new("/String/Sp/STR_Favorite.msbt", msbts.get("/String/Sp/STR_Favorite.msbt").unwrap()),
        STR_Dinosaur: File::new("/String/Sp/STR_Dinosaur.msbt", msbts.get("/String/Sp/STR_Dinosaur.msbt").unwrap()),
        }
    }
}

pub struct String_Rnd {
    pub STR_Obj: File,
    pub STR_Drink: File,
    pub STR_Music: File,
    pub STR_Food: File,
    pub STR_Sweets: File,
    pub STR_Hobby: File,
    pub STR_Book: File,
    pub STR_Sports: File,
}

impl String_Rnd {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Rnd {
        STR_Obj: File::new("/String/Rnd/STR_Obj.msbt", msbts.get("/String/Rnd/STR_Obj.msbt").unwrap()),
        STR_Drink: File::new("/String/Rnd/STR_Drink.msbt", msbts.get("/String/Rnd/STR_Drink.msbt").unwrap()),
        STR_Music: File::new("/String/Rnd/STR_Music.msbt", msbts.get("/String/Rnd/STR_Music.msbt").unwrap()),
        STR_Food: File::new("/String/Rnd/STR_Food.msbt", msbts.get("/String/Rnd/STR_Food.msbt").unwrap()),
        STR_Sweets: File::new("/String/Rnd/STR_Sweets.msbt", msbts.get("/String/Rnd/STR_Sweets.msbt").unwrap()),
        STR_Hobby: File::new("/String/Rnd/STR_Hobby.msbt", msbts.get("/String/Rnd/STR_Hobby.msbt").unwrap()),
        STR_Book: File::new("/String/Rnd/STR_Book.msbt", msbts.get("/String/Rnd/STR_Book.msbt").unwrap()),
        STR_Sports: File::new("/String/Rnd/STR_Sports.msbt", msbts.get("/String/Rnd/STR_Sports.msbt").unwrap()),
        }
    }
}

pub struct String_Npc {
    pub STR_NNpcName: File,
    pub STR_SNpcOtherName: File,
    pub STR_NNpcPhrase: File,
    pub STR_SNpcName: File,
}

impl String_Npc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Npc {
        STR_NNpcName: File::new("/String/Npc/STR_NNpcName.msbt", msbts.get("/String/Npc/STR_NNpcName.msbt").unwrap()),
        STR_SNpcOtherName: File::new("/String/Npc/STR_SNpcOtherName.msbt", msbts.get("/String/Npc/STR_SNpcOtherName.msbt").unwrap()),
        STR_NNpcPhrase: File::new("/String/Npc/STR_NNpcPhrase.msbt", msbts.get("/String/Npc/STR_NNpcPhrase.msbt").unwrap()),
        STR_SNpcName: File::new("/String/Npc/STR_SNpcName.msbt", msbts.get("/String/Npc/STR_SNpcName.msbt").unwrap()),
        }
    }
}

pub struct String_Remake {
    pub STR_Remake_BodyPartsCommon: File,
    pub STR_Remake_BodyParts: File,
    pub STR_Remake_FabricColor: File,
    pub STR_Remake_BodyColor: File,
    pub STR_Remake_BodyColorCommon: File,
    pub STR_Remake_CmnFabricCategory: File,
    pub STR_Remake_FabricColorCommon: File,
    pub STR_Remake_FabricParts: File,
}

impl String_Remake {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Remake {
        STR_Remake_BodyPartsCommon: File::new("/String/Remake/STR_Remake_BodyPartsCommon.msbt", msbts.get("/String/Remake/STR_Remake_BodyPartsCommon.msbt").unwrap()),
        STR_Remake_BodyParts: File::new("/String/Remake/STR_Remake_BodyParts.msbt", msbts.get("/String/Remake/STR_Remake_BodyParts.msbt").unwrap()),
        STR_Remake_FabricColor: File::new("/String/Remake/STR_Remake_FabricColor.msbt", msbts.get("/String/Remake/STR_Remake_FabricColor.msbt").unwrap()),
        STR_Remake_BodyColor: File::new("/String/Remake/STR_Remake_BodyColor.msbt", msbts.get("/String/Remake/STR_Remake_BodyColor.msbt").unwrap()),
        STR_Remake_BodyColorCommon: File::new("/String/Remake/STR_Remake_BodyColorCommon.msbt", msbts.get("/String/Remake/STR_Remake_BodyColorCommon.msbt").unwrap()),
        STR_Remake_CmnFabricCategory: File::new("/String/Remake/STR_Remake_CmnFabricCategory.msbt", msbts.get("/String/Remake/STR_Remake_CmnFabricCategory.msbt").unwrap()),
        STR_Remake_FabricColorCommon: File::new("/String/Remake/STR_Remake_FabricColorCommon.msbt", msbts.get("/String/Remake/STR_Remake_FabricColorCommon.msbt").unwrap()),
        STR_Remake_FabricParts: File::new("/String/Remake/STR_Remake_FabricParts.msbt", msbts.get("/String/Remake/STR_Remake_FabricParts.msbt").unwrap()),
        }
    }
}

pub struct String_Outfit_GroupName {
    pub STR_OutfitGroupName_Accessory: File,
    pub STR_OutfitGroupName_Tops: File,
    pub STR_OutfitGroupName_OnePiece: File,
    pub STR_OutfitGroupName_Socks: File,
    pub STR_OutfitGroupName_Cap: File,
    pub STR_OutfitGroupName_Bottoms: File,
    pub STR_OutfitGroupName_Shoes: File,
    pub STR_OutfitGroupName_Helmet: File,
    pub STR_OutfitGroupName_Bag: File,
}

impl String_Outfit_GroupName {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Outfit_GroupName {
        STR_OutfitGroupName_Accessory: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Accessory.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Accessory.msbt").unwrap()),
        STR_OutfitGroupName_Tops: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Tops.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Tops.msbt").unwrap()),
        STR_OutfitGroupName_OnePiece: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_OnePiece.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_OnePiece.msbt").unwrap()),
        STR_OutfitGroupName_Socks: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Socks.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Socks.msbt").unwrap()),
        STR_OutfitGroupName_Cap: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Cap.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Cap.msbt").unwrap()),
        STR_OutfitGroupName_Bottoms: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Bottoms.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Bottoms.msbt").unwrap()),
        STR_OutfitGroupName_Shoes: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Shoes.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Shoes.msbt").unwrap()),
        STR_OutfitGroupName_Helmet: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Helmet.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Helmet.msbt").unwrap()),
        STR_OutfitGroupName_Bag: File::new("/String/Outfit/GroupName/STR_OutfitGroupName_Bag.msbt", msbts.get("/String/Outfit/GroupName/STR_OutfitGroupName_Bag.msbt").unwrap()),
        }
    }
}

pub struct String_Outfit_GroupColor {
    pub STR_OutfitGroupColor_Socks: File,
    pub STR_OutfitGroupColor_Cap: File,
    pub STR_OutfitGroupColor_Bottoms: File,
    pub STR_OutfitGroupColor_Tops: File,
    pub STR_OutfitGroupColor_Bag: File,
    pub STR_OutfitGroupColor_Accessory: File,
    pub STR_OutfitGroupColor_Shoes: File,
    pub STR_OutfitGroupColor_Helmet: File,
    pub STR_OutfitGroupColor_OnePiece: File,
}

impl String_Outfit_GroupColor {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Outfit_GroupColor {
        STR_OutfitGroupColor_Socks: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Socks.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Socks.msbt").unwrap()),
        STR_OutfitGroupColor_Cap: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Cap.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Cap.msbt").unwrap()),
        STR_OutfitGroupColor_Bottoms: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Bottoms.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Bottoms.msbt").unwrap()),
        STR_OutfitGroupColor_Tops: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Tops.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Tops.msbt").unwrap()),
        STR_OutfitGroupColor_Bag: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Bag.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Bag.msbt").unwrap()),
        STR_OutfitGroupColor_Accessory: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Accessory.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Accessory.msbt").unwrap()),
        STR_OutfitGroupColor_Shoes: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Shoes.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Shoes.msbt").unwrap()),
        STR_OutfitGroupColor_Helmet: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_Helmet.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_Helmet.msbt").unwrap()),
        STR_OutfitGroupColor_OnePiece: File::new("/String/Outfit/GroupColor/STR_OutfitGroupColor_OnePiece.msbt", msbts.get("/String/Outfit/GroupColor/STR_OutfitGroupColor_OnePiece.msbt").unwrap()),
        }
    }
}

pub struct String_Outfit {
    pub GroupName: String_Outfit_GroupName,
    pub GroupColor: String_Outfit_GroupColor,
}

impl String_Outfit {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_Outfit {
        GroupName: String_Outfit_GroupName::new(msbts),
        GroupColor: String_Outfit_GroupColor::new(msbts),
        }
    }
}

pub struct String_MailCheck {
    pub STR_Mailcheck: File,
    pub STR_Mailcheck_BirthDay: File,
    pub STR_Mailcheck_Winter: File,
    pub STR_Mailcheck_Move: File,
}

impl String_MailCheck {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String_MailCheck {
        STR_Mailcheck: File::new("/String/MailCheck/STR_Mailcheck.msbt", msbts.get("/String/MailCheck/STR_Mailcheck.msbt").unwrap()),
        STR_Mailcheck_BirthDay: File::new("/String/MailCheck/STR_Mailcheck_BirthDay.msbt", msbts.get("/String/MailCheck/STR_Mailcheck_BirthDay.msbt").unwrap()),
        STR_Mailcheck_Winter: File::new("/String/MailCheck/STR_Mailcheck_Winter.msbt", msbts.get("/String/MailCheck/STR_Mailcheck_Winter.msbt").unwrap()),
        STR_Mailcheck_Move: File::new("/String/MailCheck/STR_Mailcheck_Move.msbt", msbts.get("/String/MailCheck/STR_Mailcheck_Move.msbt").unwrap()),
        }
    }
}

pub struct String {
    pub Item: String_Item,
    pub Sp: String_Sp,
    pub Rnd: String_Rnd,
    pub Npc: String_Npc,
    pub Remake: String_Remake,
    pub Outfit: String_Outfit,
    pub MailCheck: String_MailCheck,
    pub STR_MyDesignProType: File,
    pub STR_Select_Talk: File,
    pub STR_Day: File,
    pub STR_Common_Color: File,
    pub STR_CategoryName: File,
    pub STR_EventName: File,
    pub STR_TownName: File,
    pub STR_MailAddress: File,
    pub STR_Common: File,
    pub STR_Constellation: File,
    pub STR_StaffList: File,
    pub STR_SaveSetting: File,
    pub STR_Week: File,
    pub STR_Month: File,
    pub STR_ItemColor: File,
    pub STR_RegionEventArea: File,
    pub STR_StaffList_Name: File,
    pub STR_HouseWallName: File,
    pub STR_Article: File,
    pub STR_FixedForm: File,
    pub STR_GroundMaker: File,
    pub STR_MydesignDefaultName: File,
    pub STR_HouseRoofName: File,
    pub STR_Itchy: File,
    pub STR_Race: File,
    pub STR_Common_Color_SetClothes: File,
    pub STR_MyDesignExchangeUsage: File,
    pub STR_KeyIcon: File,
    pub STR_MailDesign: File,
    pub STR_Nickname: File,
    pub STR_StaffList_Title: File,
    pub STR_StampRally: File,
    pub STR_Common_Select: File,
    pub STR_Common_Select_Event: File,
    pub STR_HouseDoorName: File,
}

impl String {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        String {
        Item: String_Item::new(msbts),
        Sp: String_Sp::new(msbts),
        Rnd: String_Rnd::new(msbts),
        Npc: String_Npc::new(msbts),
        Remake: String_Remake::new(msbts),
        Outfit: String_Outfit::new(msbts),
        MailCheck: String_MailCheck::new(msbts),
        STR_MyDesignProType: File::new("/String/STR_MyDesignProType.msbt", msbts.get("/String/STR_MyDesignProType.msbt").unwrap()),
        STR_Select_Talk: File::new("/String/STR_Select_Talk.msbt", msbts.get("/String/STR_Select_Talk.msbt").unwrap()),
        STR_Day: File::new("/String/STR_Day.msbt", msbts.get("/String/STR_Day.msbt").unwrap()),
        STR_Common_Color: File::new("/String/STR_Common_Color.msbt", msbts.get("/String/STR_Common_Color.msbt").unwrap()),
        STR_CategoryName: File::new("/String/STR_CategoryName.msbt", msbts.get("/String/STR_CategoryName.msbt").unwrap()),
        STR_EventName: File::new("/String/STR_EventName.msbt", msbts.get("/String/STR_EventName.msbt").unwrap()),
        STR_TownName: File::new("/String/STR_TownName.msbt", msbts.get("/String/STR_TownName.msbt").unwrap()),
        STR_MailAddress: File::new("/String/STR_MailAddress.msbt", msbts.get("/String/STR_MailAddress.msbt").unwrap()),
        STR_Common: File::new("/String/STR_Common.msbt", msbts.get("/String/STR_Common.msbt").unwrap()),
        STR_Constellation: File::new("/String/STR_Constellation.msbt", msbts.get("/String/STR_Constellation.msbt").unwrap()),
        STR_StaffList: File::new("/String/STR_StaffList.msbt", msbts.get("/String/STR_StaffList.msbt").unwrap()),
        STR_SaveSetting: File::new("/String/STR_SaveSetting.msbt", msbts.get("/String/STR_SaveSetting.msbt").unwrap()),
        STR_Week: File::new("/String/STR_Week.msbt", msbts.get("/String/STR_Week.msbt").unwrap()),
        STR_Month: File::new("/String/STR_Month.msbt", msbts.get("/String/STR_Month.msbt").unwrap()),
        STR_ItemColor: File::new("/String/STR_ItemColor.msbt", msbts.get("/String/STR_ItemColor.msbt").unwrap()),
        STR_RegionEventArea: File::new("/String/STR_RegionEventArea.msbt", msbts.get("/String/STR_RegionEventArea.msbt").unwrap()),
        STR_StaffList_Name: File::new("/String/STR_StaffList_Name.msbt", msbts.get("/String/STR_StaffList_Name.msbt").unwrap()),
        STR_HouseWallName: File::new("/String/STR_HouseWallName.msbt", msbts.get("/String/STR_HouseWallName.msbt").unwrap()),
        STR_Article: File::new("/String/STR_Article.msbt", msbts.get("/String/STR_Article.msbt").unwrap()),
        STR_FixedForm: File::new("/String/STR_FixedForm.msbt", msbts.get("/String/STR_FixedForm.msbt").unwrap()),
        STR_GroundMaker: File::new("/String/STR_GroundMaker.msbt", msbts.get("/String/STR_GroundMaker.msbt").unwrap()),
        STR_MydesignDefaultName: File::new("/String/STR_MydesignDefaultName.msbt", msbts.get("/String/STR_MydesignDefaultName.msbt").unwrap()),
        STR_HouseRoofName: File::new("/String/STR_HouseRoofName.msbt", msbts.get("/String/STR_HouseRoofName.msbt").unwrap()),
        STR_Itchy: File::new("/String/STR_Itchy.msbt", msbts.get("/String/STR_Itchy.msbt").unwrap()),
        STR_Race: File::new("/String/STR_Race.msbt", msbts.get("/String/STR_Race.msbt").unwrap()),
        STR_Common_Color_SetClothes: File::new("/String/STR_Common_Color_SetClothes.msbt", msbts.get("/String/STR_Common_Color_SetClothes.msbt").unwrap()),
        STR_MyDesignExchangeUsage: File::new("/String/STR_MyDesignExchangeUsage.msbt", msbts.get("/String/STR_MyDesignExchangeUsage.msbt").unwrap()),
        STR_KeyIcon: File::new("/String/STR_KeyIcon.msbt", msbts.get("/String/STR_KeyIcon.msbt").unwrap()),
        STR_MailDesign: File::new("/String/STR_MailDesign.msbt", msbts.get("/String/STR_MailDesign.msbt").unwrap()),
        STR_Nickname: File::new("/String/STR_Nickname.msbt", msbts.get("/String/STR_Nickname.msbt").unwrap()),
        STR_StaffList_Title: File::new("/String/STR_StaffList_Title.msbt", msbts.get("/String/STR_StaffList_Title.msbt").unwrap()),
        STR_StampRally: File::new("/String/STR_StampRally.msbt", msbts.get("/String/STR_StampRally.msbt").unwrap()),
        STR_Common_Select: File::new("/String/STR_Common_Select.msbt", msbts.get("/String/STR_Common_Select.msbt").unwrap()),
        STR_Common_Select_Event: File::new("/String/STR_Common_Select_Event.msbt", msbts.get("/String/STR_Common_Select_Event.msbt").unwrap()),
        STR_HouseDoorName: File::new("/String/STR_HouseDoorName.msbt", msbts.get("/String/STR_HouseDoorName.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Greet {
    pub GE_Greet_House2: File,
    pub GE_Greet_Spot: File,
    pub GE_Greet_House_H: File,
    pub GE_Greet_Again2: File,
    pub GE_Greet_Rain2: File,
    pub GE_Greet_Snow2: File,
    pub GE_Greet_Again_LifeStart: File,
    pub GE_Greet_House3: File,
    pub GE_Greet_House1: File,
    pub GE_Greet_House_G: File,
    pub GE_Greet_Field3: File,
    pub GE_Greet_Again1: File,
    pub GE_Greet_Fine2: File,
    pub GE_Greet_Field1: File,
}

impl TalkNNpc_G2_Ge_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Greet {
        GE_Greet_House2: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_House2.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_House2.msbt").unwrap()),
        GE_Greet_Spot: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Spot.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Spot.msbt").unwrap()),
        GE_Greet_House_H: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_House_H.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_House_H.msbt").unwrap()),
        GE_Greet_Again2: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again2.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again2.msbt").unwrap()),
        GE_Greet_Rain2: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Rain2.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Rain2.msbt").unwrap()),
        GE_Greet_Snow2: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Snow2.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Snow2.msbt").unwrap()),
        GE_Greet_Again_LifeStart: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again_LifeStart.msbt").unwrap()),
        GE_Greet_House3: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_House3.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_House3.msbt").unwrap()),
        GE_Greet_House1: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_House1.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_House1.msbt").unwrap()),
        GE_Greet_House_G: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_House_G.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_House_G.msbt").unwrap()),
        GE_Greet_Field3: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Field3.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Field3.msbt").unwrap()),
        GE_Greet_Again1: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again1.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Again1.msbt").unwrap()),
        GE_Greet_Fine2: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Fine2.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Fine2.msbt").unwrap()),
        GE_Greet_Field1: File::new("/TalkNNpc/G2_Ge/Greet/GE_Greet_Field1.msbt", msbts.get("/TalkNNpc/G2_Ge/Greet/GE_Greet_Field1.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Force {
    pub GE_Force_Hit: File,
    pub GE_Force_Flea: File,
    pub GE_Force_Push: File,
}

impl TalkNNpc_G2_Ge_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Force {
        GE_Force_Hit: File::new("/TalkNNpc/G2_Ge/Force/GE_Force_Hit.msbt", msbts.get("/TalkNNpc/G2_Ge/Force/GE_Force_Hit.msbt").unwrap()),
        GE_Force_Flea: File::new("/TalkNNpc/G2_Ge/Force/GE_Force_Flea.msbt", msbts.get("/TalkNNpc/G2_Ge/Force/GE_Force_Flea.msbt").unwrap()),
        GE_Force_Push: File::new("/TalkNNpc/G2_Ge/Force/GE_Force_Push.msbt", msbts.get("/TalkNNpc/G2_Ge/Force/GE_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Quest {
    pub GE_Quest_Sick_Cured: File,
    pub GE_Quest_Sick_End: File,
    pub GE_Quest_Delivery_End: File,
    pub GE_Quest_CatchFishInsect_End: File,
    pub GE_Quest_CatchFishInsect_Begin: File,
    pub GE_Quest_Delivery_Begin: File,
    pub GE_Quest_Delivery_After: File,
    pub GE_Quest_TreasureHunt_Begin: File,
    pub GE_Quest_Delivery_Cloth: File,
    pub GE_Quest_TreasureHunt_End: File,
    pub GE_Quest_Sick_Begin: File,
    pub GE_Quest_LostProperty_End: File,
    pub GE_Quest_LostProperty_Begin: File,
    pub GE_Quest_Delivery_Give: File,
    pub GE_Quest_TreasureHunt_Talk: File,
}

impl TalkNNpc_G2_Ge_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Quest {
        GE_Quest_Sick_Cured: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_Cured.msbt").unwrap()),
        GE_Quest_Sick_End: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_End.msbt").unwrap()),
        GE_Quest_Delivery_End: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_End.msbt").unwrap()),
        GE_Quest_CatchFishInsect_End: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_CatchFishInsect_End.msbt").unwrap()),
        GE_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        GE_Quest_Delivery_Begin: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Begin.msbt").unwrap()),
        GE_Quest_Delivery_After: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_After.msbt").unwrap()),
        GE_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_Begin.msbt").unwrap()),
        GE_Quest_Delivery_Cloth: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Cloth.msbt").unwrap()),
        GE_Quest_TreasureHunt_End: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_End.msbt").unwrap()),
        GE_Quest_Sick_Begin: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Sick_Begin.msbt").unwrap()),
        GE_Quest_LostProperty_End: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_LostProperty_End.msbt").unwrap()),
        GE_Quest_LostProperty_Begin: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_LostProperty_Begin.msbt").unwrap()),
        GE_Quest_Delivery_Give: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_Delivery_Give.msbt").unwrap()),
        GE_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/G2_Ge/Quest/GE_Quest_TreasureHunt_Talk.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Conv {
    pub GE_Conv_KO: File,
    pub GE_Conv_AN: File,
    pub GE_Conv_HA: File,
    pub GE_Conv_ZK: File,
    pub GE_Conv_BO: File,
    pub GE_Conv_OT: File,
    pub GE_Conv_FU: File,
    pub GE_Conv_GE: File,
}

impl TalkNNpc_G2_Ge_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Conv {
        GE_Conv_KO: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_KO.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_KO.msbt").unwrap()),
        GE_Conv_AN: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_AN.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_AN.msbt").unwrap()),
        GE_Conv_HA: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_HA.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_HA.msbt").unwrap()),
        GE_Conv_ZK: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_ZK.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_ZK.msbt").unwrap()),
        GE_Conv_BO: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_BO.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_BO.msbt").unwrap()),
        GE_Conv_OT: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_OT.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_OT.msbt").unwrap()),
        GE_Conv_FU: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_FU.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_FU.msbt").unwrap()),
        GE_Conv_GE: File::new("/TalkNNpc/G2_Ge/Conv/GE_Conv_GE.msbt", msbts.get("/TalkNNpc/G2_Ge/Conv/GE_Conv_GE.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_GreetV {
    pub GE_GreetV_Again1: File,
    pub GE_GreetV_Field1: File,
    pub GE_GreetV_Fine2: File,
    pub GE_GreetV_Again2: File,
    pub GE_GreetV_Rain2: File,
    pub GE_GreetV_Snow2: File,
}

impl TalkNNpc_G2_Ge_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_GreetV {
        GE_GreetV_Again1: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Again1.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Again1.msbt").unwrap()),
        GE_GreetV_Field1: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Field1.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Field1.msbt").unwrap()),
        GE_GreetV_Fine2: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Fine2.msbt").unwrap()),
        GE_GreetV_Again2: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Again2.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Again2.msbt").unwrap()),
        GE_GreetV_Rain2: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Rain2.msbt").unwrap()),
        GE_GreetV_Snow2: File::new("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/G2_Ge/GreetV/GE_GreetV_Snow2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_React {
    pub GE_React_Itching: File,
    pub GE_React_Quest_Sick: File,
    pub GE_React_Watching_Fossil: File,
    pub GE_React_MoveOut: File,
    pub GE_React_Poison: File,
    pub GE_React_Beeface: File,
    pub GE_React_GEvent_BirthdayP: File,
    pub GE_React_Run: File,
    pub GE_React_Missing30: File,
    pub GE_React_Sad: File,
    pub GE_React_Receive_Easter: File,
    pub GE_React_Fall: File,
    pub GE_React_Worry: File,
    pub GE_React_Napping: File,
    pub GE_React_Talkative: File,
    pub GE_React_Catching: File,
    pub GE_React_LookingUp: File,
    pub GE_React_Watching_Insect: File,
    pub GE_React_Anger: File,
    pub GE_React_Watching_Art: File,
    pub GE_React_GEvent_Quest: File,
    pub GE_React_Shopping_Tailor: File,
    pub GE_React_Shopping_Gstore: File,
    pub GE_React_Watching_Fish: File,
    pub GE_React_Happy: File,
    pub GE_React_Quest_TreasureHunt: File,
    pub GE_React_DIY: File,
    pub GE_React_Fishing: File,
    pub GE_React_MoveIn: File,
    pub GE_React_Missing7: File,
    pub GE_React_NewYear_Chinese: File,
    pub GE_React_First_Acquaintance: File,
    pub GE_React_NewYear: File,
    pub GE_React_FirstV_Stranger: File,
    pub GE_React_FirstV_Acquaintance: File,
    pub GE_React_First_Stranger: File,
    pub GE_React_EarlyLate: File,
    pub GE_React_Sitting: File,
}

impl TalkNNpc_G2_Ge_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_React {
        GE_React_Itching: File::new("/TalkNNpc/G2_Ge/React/GE_React_Itching.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Itching.msbt").unwrap()),
        GE_React_Quest_Sick: File::new("/TalkNNpc/G2_Ge/React/GE_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Quest_Sick.msbt").unwrap()),
        GE_React_Watching_Fossil: File::new("/TalkNNpc/G2_Ge/React/GE_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Watching_Fossil.msbt").unwrap()),
        GE_React_MoveOut: File::new("/TalkNNpc/G2_Ge/React/GE_React_MoveOut.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_MoveOut.msbt").unwrap()),
        GE_React_Poison: File::new("/TalkNNpc/G2_Ge/React/GE_React_Poison.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Poison.msbt").unwrap()),
        GE_React_Beeface: File::new("/TalkNNpc/G2_Ge/React/GE_React_Beeface.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Beeface.msbt").unwrap()),
        GE_React_GEvent_BirthdayP: File::new("/TalkNNpc/G2_Ge/React/GE_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_GEvent_BirthdayP.msbt").unwrap()),
        GE_React_Run: File::new("/TalkNNpc/G2_Ge/React/GE_React_Run.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Run.msbt").unwrap()),
        GE_React_Missing30: File::new("/TalkNNpc/G2_Ge/React/GE_React_Missing30.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Missing30.msbt").unwrap()),
        GE_React_Sad: File::new("/TalkNNpc/G2_Ge/React/GE_React_Sad.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Sad.msbt").unwrap()),
        GE_React_Receive_Easter: File::new("/TalkNNpc/G2_Ge/React/GE_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Receive_Easter.msbt").unwrap()),
        GE_React_Fall: File::new("/TalkNNpc/G2_Ge/React/GE_React_Fall.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Fall.msbt").unwrap()),
        GE_React_Worry: File::new("/TalkNNpc/G2_Ge/React/GE_React_Worry.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Worry.msbt").unwrap()),
        GE_React_Napping: File::new("/TalkNNpc/G2_Ge/React/GE_React_Napping.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Napping.msbt").unwrap()),
        GE_React_Talkative: File::new("/TalkNNpc/G2_Ge/React/GE_React_Talkative.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Talkative.msbt").unwrap()),
        GE_React_Catching: File::new("/TalkNNpc/G2_Ge/React/GE_React_Catching.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Catching.msbt").unwrap()),
        GE_React_LookingUp: File::new("/TalkNNpc/G2_Ge/React/GE_React_LookingUp.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_LookingUp.msbt").unwrap()),
        GE_React_Watching_Insect: File::new("/TalkNNpc/G2_Ge/React/GE_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Watching_Insect.msbt").unwrap()),
        GE_React_Anger: File::new("/TalkNNpc/G2_Ge/React/GE_React_Anger.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Anger.msbt").unwrap()),
        GE_React_Watching_Art: File::new("/TalkNNpc/G2_Ge/React/GE_React_Watching_Art.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Watching_Art.msbt").unwrap()),
        GE_React_GEvent_Quest: File::new("/TalkNNpc/G2_Ge/React/GE_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_GEvent_Quest.msbt").unwrap()),
        GE_React_Shopping_Tailor: File::new("/TalkNNpc/G2_Ge/React/GE_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Shopping_Tailor.msbt").unwrap()),
        GE_React_Shopping_Gstore: File::new("/TalkNNpc/G2_Ge/React/GE_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Shopping_Gstore.msbt").unwrap()),
        GE_React_Watching_Fish: File::new("/TalkNNpc/G2_Ge/React/GE_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Watching_Fish.msbt").unwrap()),
        GE_React_Happy: File::new("/TalkNNpc/G2_Ge/React/GE_React_Happy.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Happy.msbt").unwrap()),
        GE_React_Quest_TreasureHunt: File::new("/TalkNNpc/G2_Ge/React/GE_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Quest_TreasureHunt.msbt").unwrap()),
        GE_React_DIY: File::new("/TalkNNpc/G2_Ge/React/GE_React_DIY.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_DIY.msbt").unwrap()),
        GE_React_Fishing: File::new("/TalkNNpc/G2_Ge/React/GE_React_Fishing.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Fishing.msbt").unwrap()),
        GE_React_MoveIn: File::new("/TalkNNpc/G2_Ge/React/GE_React_MoveIn.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_MoveIn.msbt").unwrap()),
        GE_React_Missing7: File::new("/TalkNNpc/G2_Ge/React/GE_React_Missing7.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Missing7.msbt").unwrap()),
        GE_React_NewYear_Chinese: File::new("/TalkNNpc/G2_Ge/React/GE_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_NewYear_Chinese.msbt").unwrap()),
        GE_React_First_Acquaintance: File::new("/TalkNNpc/G2_Ge/React/GE_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_First_Acquaintance.msbt").unwrap()),
        GE_React_NewYear: File::new("/TalkNNpc/G2_Ge/React/GE_React_NewYear.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_NewYear.msbt").unwrap()),
        GE_React_FirstV_Stranger: File::new("/TalkNNpc/G2_Ge/React/GE_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_FirstV_Stranger.msbt").unwrap()),
        GE_React_FirstV_Acquaintance: File::new("/TalkNNpc/G2_Ge/React/GE_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_FirstV_Acquaintance.msbt").unwrap()),
        GE_React_First_Stranger: File::new("/TalkNNpc/G2_Ge/React/GE_React_First_Stranger.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_First_Stranger.msbt").unwrap()),
        GE_React_EarlyLate: File::new("/TalkNNpc/G2_Ge/React/GE_React_EarlyLate.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_EarlyLate.msbt").unwrap()),
        GE_React_Sitting: File::new("/TalkNNpc/G2_Ge/React/GE_React_Sitting.msbt", msbts.get("/TalkNNpc/G2_Ge/React/GE_React_Sitting.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Approach {
    pub GE_ApproachC_Present: File,
    pub GE_ApproachB_NickName: File,
    pub GE_ApproachD_Stay: File,
    pub GE_ApproachC_Want: File,
    pub GE_ApproachC_Trade: File,
    pub GE_ApproachF_First: File,
    pub GE_ApproachE_Easter: File,
    pub GE_ApproachD_Moving: File,
    pub GE_ApproachC_Sell: File,
    pub GE_ApproachA_Always: File,
    pub GE_ApproachE_MainSeq: File,
    pub GE_ApproachA_Emoticons: File,
    pub GE_ApproachB_Habit: File,
    pub GE_ApproachB_Greeting: File,
}

impl TalkNNpc_G2_Ge_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Approach {
        GE_ApproachC_Present: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Present.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Present.msbt").unwrap()),
        GE_ApproachB_NickName: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_NickName.msbt").unwrap()),
        GE_ApproachD_Stay: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachD_Stay.msbt").unwrap()),
        GE_ApproachC_Want: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Want.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Want.msbt").unwrap()),
        GE_ApproachC_Trade: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Trade.msbt").unwrap()),
        GE_ApproachF_First: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachF_First.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachF_First.msbt").unwrap()),
        GE_ApproachE_Easter: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachE_Easter.msbt").unwrap()),
        GE_ApproachD_Moving: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachD_Moving.msbt").unwrap()),
        GE_ApproachC_Sell: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachC_Sell.msbt").unwrap()),
        GE_ApproachA_Always: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachA_Always.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachA_Always.msbt").unwrap()),
        GE_ApproachE_MainSeq: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachE_MainSeq.msbt").unwrap()),
        GE_ApproachA_Emoticons: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachA_Emoticons.msbt").unwrap()),
        GE_ApproachB_Habit: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_Habit.msbt").unwrap()),
        GE_ApproachB_Greeting: File::new("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/G2_Ge/Approach/GE_ApproachB_Greeting.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Spot {
    pub GE_Spot_Museum_Fossil: File,
    pub GE_Spot_Camp: File,
    pub GE_Spot_Camp_Amiibo: File,
    pub GE_Spot_Museum_Fish: File,
    pub GE_Spot_Camp_Game: File,
    pub GE_Spot_Museum_Art: File,
    pub GE_Spot_MysteryTour: File,
    pub GE_Spot_Museum_Insect: File,
    pub GE_Spot_Camp_Invite: File,
    pub GE_Spot_Office: File,
    pub GE_Spot_Tailor: File,
    pub GE_Spot_Gstore: File,
    pub GE_Spot_Camp_Quest: File,
}

impl TalkNNpc_G2_Ge_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Spot {
        GE_Spot_Museum_Fossil: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Fossil.msbt").unwrap()),
        GE_Spot_Camp: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp.msbt").unwrap()),
        GE_Spot_Camp_Amiibo: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Amiibo.msbt").unwrap()),
        GE_Spot_Museum_Fish: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Fish.msbt").unwrap()),
        GE_Spot_Camp_Game: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Game.msbt").unwrap()),
        GE_Spot_Museum_Art: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Art.msbt").unwrap()),
        GE_Spot_MysteryTour: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_MysteryTour.msbt").unwrap()),
        GE_Spot_Museum_Insect: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Museum_Insect.msbt").unwrap()),
        GE_Spot_Camp_Invite: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Invite.msbt").unwrap()),
        GE_Spot_Office: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Office.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Office.msbt").unwrap()),
        GE_Spot_Tailor: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Tailor.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Tailor.msbt").unwrap()),
        GE_Spot_Gstore: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Gstore.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Gstore.msbt").unwrap()),
        GE_Spot_Camp_Quest: File::new("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/G2_Ge/Spot/GE_Spot_Camp_Quest.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_GEvent {
    pub GE_GEvent_Countdown: File,
    pub GE_GEvent_BirthdayP_G: File,
    pub GE_GEvent_CatchInsectFes: File,
    pub GE_GEvent_BirthdayP: File,
    pub GE_GEvent_CatchFishFes: File,
    pub GE_GEvent_BirthdayN_H: File,
    pub GE_GEvent_Easter: File,
    pub GE_GEvent_BirthdayP_H: File,
    pub GE_GEvent_BirthdayN_G: File,
}

impl TalkNNpc_G2_Ge_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_GEvent {
        GE_GEvent_Countdown: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_Countdown.msbt").unwrap()),
        GE_GEvent_BirthdayP_G: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP_G.msbt").unwrap()),
        GE_GEvent_CatchInsectFes: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_CatchInsectFes.msbt").unwrap()),
        GE_GEvent_BirthdayP: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP.msbt").unwrap()),
        GE_GEvent_CatchFishFes: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_CatchFishFes.msbt").unwrap()),
        GE_GEvent_BirthdayN_H: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayN_H.msbt").unwrap()),
        GE_GEvent_Easter: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_Easter.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_Easter.msbt").unwrap()),
        GE_GEvent_BirthdayP_H: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayP_H.msbt").unwrap()),
        GE_GEvent_BirthdayN_G: File::new("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/G2_Ge/GEvent/GE_GEvent_BirthdayN_G.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Select {
    pub GE_Select_General: File,
    pub GE_Select_Present: File,
}

impl TalkNNpc_G2_Ge_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Select {
        GE_Select_General: File::new("/TalkNNpc/G2_Ge/Select/GE_Select_General.msbt", msbts.get("/TalkNNpc/G2_Ge/Select/GE_Select_General.msbt").unwrap()),
        GE_Select_Present: File::new("/TalkNNpc/G2_Ge/Select/GE_Select_Present.msbt", msbts.get("/TalkNNpc/G2_Ge/Select/GE_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge_Free {
    pub GE_FreeD_RumorOP_Action: File,
    pub GE_FreeA_FirstTent: File,
    pub GE_FreeC_FurnitureN_Genre: File,
    pub GE_FreeA_Always: File,
    pub GE_FreeF_NewYear_Chinese: File,
    pub GE_FreeD_RumorP_Favorite: File,
    pub GE_FreeG_Host: File,
    pub GE_FreeC_FurnitureP_Theme: File,
    pub GE_FreeD_RumorOP_Favorite: File,
    pub GE_FreeF_Easter: File,
    pub GE_FreeD_RumorP_Action: File,
    pub GE_FreeE_Event: File,
    pub GE_FreeA_First01: File,
    pub GE_FreeD_RumorN2: File,
    pub GE_FreeF_NewYear: File,
    pub GE_FreeA_Week: File,
    pub GE_FreeA_AlwaysB: File,
    pub GE_FreeA_Want: File,
    pub GE_FreeE_Snpc: File,
    pub GE_FreeB_Furniture_Theme: File,
    pub GE_FreeB_Weather: File,
    pub GE_FreeB_ItemP: File,
    pub GE_FreeH_Progress: File,
    pub GE_FreeF_EarthDay: File,
    pub GE_FreeC_RoomG: File,
    pub GE_FreeI_Present: File,
    pub GE_FreeF_MayDay: File,
    pub GE_FreeA_ClothesP: File,
    pub GE_FreeB_ItemN: File,
    pub GE_FreeA_ClothesN: File,
    pub GE_FreeF_Countdown: File,
    pub GE_FreeD_Moving: File,
    pub GE_FreeC_FurnitureP_Genre: File,
    pub GE_FreeA_First02: File,
    pub GE_FreeF_CatchFishFes: File,
    pub GE_FreeD_Keyword: File,
    pub GE_FreeC_FurnitureP_Same: File,
    pub GE_FreeF_CatchInsectFes: File,
    pub GE_FreeA_Questions: File,
    pub GE_FreeF_NewYear_Zodiac: File,
    pub GE_FreeD_RumorN1: File,
    pub GE_FreeB_Seasons: File,
    pub GE_FreeA_AlwaysA: File,
    pub GE_FreeB_Spot: File,
    pub GE_FreeC_RoomH: File,
    pub GE_FreeB_Furniture_Genre: File,
    pub GE_FreeG_Visitor: File,
}

impl TalkNNpc_G2_Ge_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge_Free {
        GE_FreeD_RumorOP_Action: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorOP_Action.msbt").unwrap()),
        GE_FreeA_FirstTent: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_FirstTent.msbt").unwrap()),
        GE_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureN_Genre.msbt").unwrap()),
        GE_FreeA_Always: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_Always.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_Always.msbt").unwrap()),
        GE_FreeF_NewYear_Chinese: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear_Chinese.msbt").unwrap()),
        GE_FreeD_RumorP_Favorite: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorP_Favorite.msbt").unwrap()),
        GE_FreeG_Host: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeG_Host.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeG_Host.msbt").unwrap()),
        GE_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Theme.msbt").unwrap()),
        GE_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorOP_Favorite.msbt").unwrap()),
        GE_FreeF_Easter: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_Easter.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_Easter.msbt").unwrap()),
        GE_FreeD_RumorP_Action: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorP_Action.msbt").unwrap()),
        GE_FreeE_Event: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeE_Event.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeE_Event.msbt").unwrap()),
        GE_FreeA_First01: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_First01.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_First01.msbt").unwrap()),
        GE_FreeD_RumorN2: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorN2.msbt").unwrap()),
        GE_FreeF_NewYear: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear.msbt").unwrap()),
        GE_FreeA_Week: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_Week.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_Week.msbt").unwrap()),
        GE_FreeA_AlwaysB: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_AlwaysB.msbt").unwrap()),
        GE_FreeA_Want: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_Want.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_Want.msbt").unwrap()),
        GE_FreeE_Snpc: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeE_Snpc.msbt").unwrap()),
        GE_FreeB_Furniture_Theme: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_Furniture_Theme.msbt").unwrap()),
        GE_FreeB_Weather: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_Weather.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_Weather.msbt").unwrap()),
        GE_FreeB_ItemP: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_ItemP.msbt").unwrap()),
        GE_FreeH_Progress: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeH_Progress.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeH_Progress.msbt").unwrap()),
        GE_FreeF_EarthDay: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_EarthDay.msbt").unwrap()),
        GE_FreeC_RoomG: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_RoomG.msbt").unwrap()),
        GE_FreeI_Present: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeI_Present.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeI_Present.msbt").unwrap()),
        GE_FreeF_MayDay: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_MayDay.msbt").unwrap()),
        GE_FreeA_ClothesP: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_ClothesP.msbt").unwrap()),
        GE_FreeB_ItemN: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_ItemN.msbt").unwrap()),
        GE_FreeA_ClothesN: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_ClothesN.msbt").unwrap()),
        GE_FreeF_Countdown: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_Countdown.msbt").unwrap()),
        GE_FreeD_Moving: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_Moving.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_Moving.msbt").unwrap()),
        GE_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Genre.msbt").unwrap()),
        GE_FreeA_First02: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_First02.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_First02.msbt").unwrap()),
        GE_FreeF_CatchFishFes: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_CatchFishFes.msbt").unwrap()),
        GE_FreeD_Keyword: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_Keyword.msbt").unwrap()),
        GE_FreeC_FurnitureP_Same: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_FurnitureP_Same.msbt").unwrap()),
        GE_FreeF_CatchInsectFes: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_CatchInsectFes.msbt").unwrap()),
        GE_FreeA_Questions: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_Questions.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_Questions.msbt").unwrap()),
        GE_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeF_NewYear_Zodiac.msbt").unwrap()),
        GE_FreeD_RumorN1: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeD_RumorN1.msbt").unwrap()),
        GE_FreeB_Seasons: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_Seasons.msbt").unwrap()),
        GE_FreeA_AlwaysA: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeA_AlwaysA.msbt").unwrap()),
        GE_FreeB_Spot: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_Spot.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_Spot.msbt").unwrap()),
        GE_FreeC_RoomH: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeC_RoomH.msbt").unwrap()),
        GE_FreeB_Furniture_Genre: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeB_Furniture_Genre.msbt").unwrap()),
        GE_FreeG_Visitor: File::new("/TalkNNpc/G2_Ge/Free/GE_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/G2_Ge/Free/GE_FreeG_Visitor.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G2_Ge {
    pub Greet: TalkNNpc_G2_Ge_Greet,
    pub Force: TalkNNpc_G2_Ge_Force,
    pub Quest: TalkNNpc_G2_Ge_Quest,
    pub Conv: TalkNNpc_G2_Ge_Conv,
    pub GreetV: TalkNNpc_G2_Ge_GreetV,
    pub React: TalkNNpc_G2_Ge_React,
    pub Approach: TalkNNpc_G2_Ge_Approach,
    pub Spot: TalkNNpc_G2_Ge_Spot,
    pub GEvent: TalkNNpc_G2_Ge_GEvent,
    pub Select: TalkNNpc_G2_Ge_Select,
    pub Free: TalkNNpc_G2_Ge_Free,
    pub GE_End: File,
    pub GE_Connect_StandingUp: File,
}

impl TalkNNpc_G2_Ge {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G2_Ge {
        Greet: TalkNNpc_G2_Ge_Greet::new(msbts),
        Force: TalkNNpc_G2_Ge_Force::new(msbts),
        Quest: TalkNNpc_G2_Ge_Quest::new(msbts),
        Conv: TalkNNpc_G2_Ge_Conv::new(msbts),
        GreetV: TalkNNpc_G2_Ge_GreetV::new(msbts),
        React: TalkNNpc_G2_Ge_React::new(msbts),
        Approach: TalkNNpc_G2_Ge_Approach::new(msbts),
        Spot: TalkNNpc_G2_Ge_Spot::new(msbts),
        GEvent: TalkNNpc_G2_Ge_GEvent::new(msbts),
        Select: TalkNNpc_G2_Ge_Select::new(msbts),
        Free: TalkNNpc_G2_Ge_Free::new(msbts),
        GE_End: File::new("/TalkNNpc/G2_Ge/GE_End.msbt", msbts.get("/TalkNNpc/G2_Ge/GE_End.msbt").unwrap()),
        GE_Connect_StandingUp: File::new("/TalkNNpc/G2_Ge/GE_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/G2_Ge/GE_Connect_StandingUp.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_React {
    pub OT_React_Anger: File,
    pub OT_React_Missing30: File,
    pub OT_React_Sitting: File,
    pub OT_React_Itching: File,
    pub OT_React_MoveOut: File,
    pub OT_React_First_Acquaintance: File,
    pub OT_React_Happy: File,
    pub OT_React_Beeface: File,
    pub OT_React_Quest_Sick: File,
    pub OT_React_Talkative: File,
    pub OT_React_Poison: File,
    pub OT_React_Missing7: File,
    pub OT_React_LookingUp: File,
    pub OT_React_Run: File,
    pub OT_React_Watching_Insect: File,
    pub OT_React_Napping: File,
    pub OT_React_Sad: File,
    pub OT_React_Shopping_Tailor: File,
    pub OT_React_Shopping_Gstore: File,
    pub OT_React_Fall: File,
    pub OT_React_First_Stranger: File,
    pub OT_React_FirstV_Acquaintance: File,
    pub OT_React_NewYear_Chinese: File,
    pub OT_React_Watching_Art: File,
    pub OT_React_Receive_Easter: File,
    pub OT_React_GEvent_Quest: File,
    pub OT_React_Fishing: File,
    pub OT_React_FirstV_Stranger: File,
    pub OT_React_GEvent_BirthdayP: File,
    pub OT_React_Worry: File,
    pub OT_React_DIY: File,
    pub OT_React_EarlyLate: File,
    pub OT_React_Catching: File,
    pub OT_React_Quest_TreasureHunt: File,
    pub OT_React_Watching_Fish: File,
    pub OT_React_NewYear: File,
    pub OT_React_Watching_Fossil: File,
    pub OT_React_MoveIn: File,
}

impl TalkNNpc_G3_Ot_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_React {
        OT_React_Anger: File::new("/TalkNNpc/G3_Ot/React/OT_React_Anger.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Anger.msbt").unwrap()),
        OT_React_Missing30: File::new("/TalkNNpc/G3_Ot/React/OT_React_Missing30.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Missing30.msbt").unwrap()),
        OT_React_Sitting: File::new("/TalkNNpc/G3_Ot/React/OT_React_Sitting.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Sitting.msbt").unwrap()),
        OT_React_Itching: File::new("/TalkNNpc/G3_Ot/React/OT_React_Itching.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Itching.msbt").unwrap()),
        OT_React_MoveOut: File::new("/TalkNNpc/G3_Ot/React/OT_React_MoveOut.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_MoveOut.msbt").unwrap()),
        OT_React_First_Acquaintance: File::new("/TalkNNpc/G3_Ot/React/OT_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_First_Acquaintance.msbt").unwrap()),
        OT_React_Happy: File::new("/TalkNNpc/G3_Ot/React/OT_React_Happy.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Happy.msbt").unwrap()),
        OT_React_Beeface: File::new("/TalkNNpc/G3_Ot/React/OT_React_Beeface.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Beeface.msbt").unwrap()),
        OT_React_Quest_Sick: File::new("/TalkNNpc/G3_Ot/React/OT_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Quest_Sick.msbt").unwrap()),
        OT_React_Talkative: File::new("/TalkNNpc/G3_Ot/React/OT_React_Talkative.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Talkative.msbt").unwrap()),
        OT_React_Poison: File::new("/TalkNNpc/G3_Ot/React/OT_React_Poison.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Poison.msbt").unwrap()),
        OT_React_Missing7: File::new("/TalkNNpc/G3_Ot/React/OT_React_Missing7.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Missing7.msbt").unwrap()),
        OT_React_LookingUp: File::new("/TalkNNpc/G3_Ot/React/OT_React_LookingUp.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_LookingUp.msbt").unwrap()),
        OT_React_Run: File::new("/TalkNNpc/G3_Ot/React/OT_React_Run.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Run.msbt").unwrap()),
        OT_React_Watching_Insect: File::new("/TalkNNpc/G3_Ot/React/OT_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Watching_Insect.msbt").unwrap()),
        OT_React_Napping: File::new("/TalkNNpc/G3_Ot/React/OT_React_Napping.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Napping.msbt").unwrap()),
        OT_React_Sad: File::new("/TalkNNpc/G3_Ot/React/OT_React_Sad.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Sad.msbt").unwrap()),
        OT_React_Shopping_Tailor: File::new("/TalkNNpc/G3_Ot/React/OT_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Shopping_Tailor.msbt").unwrap()),
        OT_React_Shopping_Gstore: File::new("/TalkNNpc/G3_Ot/React/OT_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Shopping_Gstore.msbt").unwrap()),
        OT_React_Fall: File::new("/TalkNNpc/G3_Ot/React/OT_React_Fall.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Fall.msbt").unwrap()),
        OT_React_First_Stranger: File::new("/TalkNNpc/G3_Ot/React/OT_React_First_Stranger.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_First_Stranger.msbt").unwrap()),
        OT_React_FirstV_Acquaintance: File::new("/TalkNNpc/G3_Ot/React/OT_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_FirstV_Acquaintance.msbt").unwrap()),
        OT_React_NewYear_Chinese: File::new("/TalkNNpc/G3_Ot/React/OT_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_NewYear_Chinese.msbt").unwrap()),
        OT_React_Watching_Art: File::new("/TalkNNpc/G3_Ot/React/OT_React_Watching_Art.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Watching_Art.msbt").unwrap()),
        OT_React_Receive_Easter: File::new("/TalkNNpc/G3_Ot/React/OT_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Receive_Easter.msbt").unwrap()),
        OT_React_GEvent_Quest: File::new("/TalkNNpc/G3_Ot/React/OT_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_GEvent_Quest.msbt").unwrap()),
        OT_React_Fishing: File::new("/TalkNNpc/G3_Ot/React/OT_React_Fishing.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Fishing.msbt").unwrap()),
        OT_React_FirstV_Stranger: File::new("/TalkNNpc/G3_Ot/React/OT_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_FirstV_Stranger.msbt").unwrap()),
        OT_React_GEvent_BirthdayP: File::new("/TalkNNpc/G3_Ot/React/OT_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_GEvent_BirthdayP.msbt").unwrap()),
        OT_React_Worry: File::new("/TalkNNpc/G3_Ot/React/OT_React_Worry.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Worry.msbt").unwrap()),
        OT_React_DIY: File::new("/TalkNNpc/G3_Ot/React/OT_React_DIY.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_DIY.msbt").unwrap()),
        OT_React_EarlyLate: File::new("/TalkNNpc/G3_Ot/React/OT_React_EarlyLate.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_EarlyLate.msbt").unwrap()),
        OT_React_Catching: File::new("/TalkNNpc/G3_Ot/React/OT_React_Catching.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Catching.msbt").unwrap()),
        OT_React_Quest_TreasureHunt: File::new("/TalkNNpc/G3_Ot/React/OT_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Quest_TreasureHunt.msbt").unwrap()),
        OT_React_Watching_Fish: File::new("/TalkNNpc/G3_Ot/React/OT_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Watching_Fish.msbt").unwrap()),
        OT_React_NewYear: File::new("/TalkNNpc/G3_Ot/React/OT_React_NewYear.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_NewYear.msbt").unwrap()),
        OT_React_Watching_Fossil: File::new("/TalkNNpc/G3_Ot/React/OT_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_Watching_Fossil.msbt").unwrap()),
        OT_React_MoveIn: File::new("/TalkNNpc/G3_Ot/React/OT_React_MoveIn.msbt", msbts.get("/TalkNNpc/G3_Ot/React/OT_React_MoveIn.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Approach {
    pub OT_ApproachB_Habit: File,
    pub OT_ApproachC_Present: File,
    pub OT_ApproachE_Easter: File,
    pub OT_ApproachB_Greeting: File,
    pub OT_ApproachD_Moving: File,
    pub OT_ApproachC_Trade: File,
    pub OT_ApproachB_NickName: File,
    pub OT_ApproachA_Emoticons: File,
    pub OT_ApproachC_Sell: File,
    pub OT_ApproachF_First: File,
    pub OT_ApproachA_Always: File,
    pub OT_ApproachD_Stay: File,
    pub OT_ApproachC_Want: File,
    pub OT_ApproachE_MainSeq: File,
}

impl TalkNNpc_G3_Ot_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Approach {
        OT_ApproachB_Habit: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_Habit.msbt").unwrap()),
        OT_ApproachC_Present: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Present.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Present.msbt").unwrap()),
        OT_ApproachE_Easter: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachE_Easter.msbt").unwrap()),
        OT_ApproachB_Greeting: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_Greeting.msbt").unwrap()),
        OT_ApproachD_Moving: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachD_Moving.msbt").unwrap()),
        OT_ApproachC_Trade: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Trade.msbt").unwrap()),
        OT_ApproachB_NickName: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachB_NickName.msbt").unwrap()),
        OT_ApproachA_Emoticons: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachA_Emoticons.msbt").unwrap()),
        OT_ApproachC_Sell: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Sell.msbt").unwrap()),
        OT_ApproachF_First: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachF_First.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachF_First.msbt").unwrap()),
        OT_ApproachA_Always: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachA_Always.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachA_Always.msbt").unwrap()),
        OT_ApproachD_Stay: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachD_Stay.msbt").unwrap()),
        OT_ApproachC_Want: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Want.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachC_Want.msbt").unwrap()),
        OT_ApproachE_MainSeq: File::new("/TalkNNpc/G3_Ot/Approach/OT_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/G3_Ot/Approach/OT_ApproachE_MainSeq.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Select {
    pub OT_Select_Present: File,
    pub OT_Select_General: File,
}

impl TalkNNpc_G3_Ot_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Select {
        OT_Select_Present: File::new("/TalkNNpc/G3_Ot/Select/OT_Select_Present.msbt", msbts.get("/TalkNNpc/G3_Ot/Select/OT_Select_Present.msbt").unwrap()),
        OT_Select_General: File::new("/TalkNNpc/G3_Ot/Select/OT_Select_General.msbt", msbts.get("/TalkNNpc/G3_Ot/Select/OT_Select_General.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Force {
    pub OT_Force_Hit: File,
    pub OT_Force_Flea: File,
    pub OT_Force_Push: File,
}

impl TalkNNpc_G3_Ot_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Force {
        OT_Force_Hit: File::new("/TalkNNpc/G3_Ot/Force/OT_Force_Hit.msbt", msbts.get("/TalkNNpc/G3_Ot/Force/OT_Force_Hit.msbt").unwrap()),
        OT_Force_Flea: File::new("/TalkNNpc/G3_Ot/Force/OT_Force_Flea.msbt", msbts.get("/TalkNNpc/G3_Ot/Force/OT_Force_Flea.msbt").unwrap()),
        OT_Force_Push: File::new("/TalkNNpc/G3_Ot/Force/OT_Force_Push.msbt", msbts.get("/TalkNNpc/G3_Ot/Force/OT_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_GreetV {
    pub OT_GreetV_Again1: File,
    pub OT_GreetV_Rain2: File,
    pub OT_GreetV_Snow2: File,
    pub OT_GreetV_Field1: File,
    pub OT_GreetV_Again2: File,
    pub OT_GreetV_Fine2: File,
}

impl TalkNNpc_G3_Ot_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_GreetV {
        OT_GreetV_Again1: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Again1.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Again1.msbt").unwrap()),
        OT_GreetV_Rain2: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Rain2.msbt").unwrap()),
        OT_GreetV_Snow2: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Snow2.msbt").unwrap()),
        OT_GreetV_Field1: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Field1.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Field1.msbt").unwrap()),
        OT_GreetV_Again2: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Again2.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Again2.msbt").unwrap()),
        OT_GreetV_Fine2: File::new("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/G3_Ot/GreetV/OT_GreetV_Fine2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_GEvent {
    pub OT_GEvent_BirthdayP_G: File,
    pub OT_GEvent_BirthdayP: File,
    pub OT_GEvent_BirthdayN_H: File,
    pub OT_GEvent_Easter: File,
    pub OT_GEvent_BirthdayP_H: File,
    pub OT_GEvent_CatchFishFes: File,
    pub OT_GEvent_BirthdayN_G: File,
    pub OT_GEvent_CatchInsectFes: File,
    pub OT_GEvent_Countdown: File,
}

impl TalkNNpc_G3_Ot_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_GEvent {
        OT_GEvent_BirthdayP_G: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP_G.msbt").unwrap()),
        OT_GEvent_BirthdayP: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP.msbt").unwrap()),
        OT_GEvent_BirthdayN_H: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayN_H.msbt").unwrap()),
        OT_GEvent_Easter: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_Easter.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_Easter.msbt").unwrap()),
        OT_GEvent_BirthdayP_H: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayP_H.msbt").unwrap()),
        OT_GEvent_CatchFishFes: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_CatchFishFes.msbt").unwrap()),
        OT_GEvent_BirthdayN_G: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_BirthdayN_G.msbt").unwrap()),
        OT_GEvent_CatchInsectFes: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_CatchInsectFes.msbt").unwrap()),
        OT_GEvent_Countdown: File::new("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/G3_Ot/GEvent/OT_GEvent_Countdown.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Conv {
    pub OT_Conv_BO: File,
    pub OT_Conv_OT: File,
    pub OT_Conv_FU: File,
    pub OT_Conv_GE: File,
    pub OT_Conv_KO: File,
    pub OT_Conv_AN: File,
    pub OT_Conv_HA: File,
    pub OT_Conv_ZK: File,
}

impl TalkNNpc_G3_Ot_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Conv {
        OT_Conv_BO: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_BO.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_BO.msbt").unwrap()),
        OT_Conv_OT: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_OT.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_OT.msbt").unwrap()),
        OT_Conv_FU: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_FU.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_FU.msbt").unwrap()),
        OT_Conv_GE: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_GE.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_GE.msbt").unwrap()),
        OT_Conv_KO: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_KO.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_KO.msbt").unwrap()),
        OT_Conv_AN: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_AN.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_AN.msbt").unwrap()),
        OT_Conv_HA: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_HA.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_HA.msbt").unwrap()),
        OT_Conv_ZK: File::new("/TalkNNpc/G3_Ot/Conv/OT_Conv_ZK.msbt", msbts.get("/TalkNNpc/G3_Ot/Conv/OT_Conv_ZK.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Quest {
    pub OT_Quest_LostProperty_Begin: File,
    pub OT_Quest_Sick_Cured: File,
    pub OT_Quest_Delivery_Cloth: File,
    pub OT_Quest_CatchFishInsect_Begin: File,
    pub OT_Quest_Delivery_End: File,
    pub OT_Quest_Delivery_Give: File,
    pub OT_Quest_TreasureHunt_End: File,
    pub OT_Quest_LostProperty_End: File,
    pub OT_Quest_Sick_End: File,
    pub OT_Quest_TreasureHunt_Begin: File,
    pub OT_Quest_Delivery_Begin: File,
    pub OT_Quest_Sick_Begin: File,
    pub OT_Quest_CatchFishInsect_End: File,
    pub OT_Quest_Delivery_After: File,
    pub OT_Quest_TreasureHunt_Talk: File,
}

impl TalkNNpc_G3_Ot_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Quest {
        OT_Quest_LostProperty_Begin: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_LostProperty_Begin.msbt").unwrap()),
        OT_Quest_Sick_Cured: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_Cured.msbt").unwrap()),
        OT_Quest_Delivery_Cloth: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Cloth.msbt").unwrap()),
        OT_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        OT_Quest_Delivery_End: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_End.msbt").unwrap()),
        OT_Quest_Delivery_Give: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Give.msbt").unwrap()),
        OT_Quest_TreasureHunt_End: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_End.msbt").unwrap()),
        OT_Quest_LostProperty_End: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_LostProperty_End.msbt").unwrap()),
        OT_Quest_Sick_End: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_End.msbt").unwrap()),
        OT_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_Begin.msbt").unwrap()),
        OT_Quest_Delivery_Begin: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_Begin.msbt").unwrap()),
        OT_Quest_Sick_Begin: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Sick_Begin.msbt").unwrap()),
        OT_Quest_CatchFishInsect_End: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_CatchFishInsect_End.msbt").unwrap()),
        OT_Quest_Delivery_After: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_Delivery_After.msbt").unwrap()),
        OT_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/G3_Ot/Quest/OT_Quest_TreasureHunt_Talk.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Spot {
    pub OT_Spot_Camp_Amiibo: File,
    pub OT_Spot_Museum_Fish: File,
    pub OT_Spot_Camp: File,
    pub OT_Spot_Camp_Quest: File,
    pub OT_Spot_MysteryTour: File,
    pub OT_Spot_Museum_Insect: File,
    pub OT_Spot_Camp_Invite: File,
    pub OT_Spot_Office: File,
    pub OT_Spot_Tailor: File,
    pub OT_Spot_Gstore: File,
    pub OT_Spot_Museum_Art: File,
    pub OT_Spot_Camp_Game: File,
    pub OT_Spot_Museum_Fossil: File,
}

impl TalkNNpc_G3_Ot_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Spot {
        OT_Spot_Camp_Amiibo: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Amiibo.msbt").unwrap()),
        OT_Spot_Museum_Fish: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Fish.msbt").unwrap()),
        OT_Spot_Camp: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp.msbt").unwrap()),
        OT_Spot_Camp_Quest: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Quest.msbt").unwrap()),
        OT_Spot_MysteryTour: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_MysteryTour.msbt").unwrap()),
        OT_Spot_Museum_Insect: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Insect.msbt").unwrap()),
        OT_Spot_Camp_Invite: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Invite.msbt").unwrap()),
        OT_Spot_Office: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Office.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Office.msbt").unwrap()),
        OT_Spot_Tailor: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Tailor.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Tailor.msbt").unwrap()),
        OT_Spot_Gstore: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Gstore.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Gstore.msbt").unwrap()),
        OT_Spot_Museum_Art: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Art.msbt").unwrap()),
        OT_Spot_Camp_Game: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Camp_Game.msbt").unwrap()),
        OT_Spot_Museum_Fossil: File::new("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/G3_Ot/Spot/OT_Spot_Museum_Fossil.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Greet {
    pub OT_Greet_Field3: File,
    pub OT_Greet_Again1: File,
    pub OT_Greet_Field1: File,
    pub OT_Greet_House_H: File,
    pub OT_Greet_Fine2: File,
    pub OT_Greet_Spot: File,
    pub OT_Greet_House2: File,
    pub OT_Greet_Again_LifeStart: File,
    pub OT_Greet_Again2: File,
    pub OT_Greet_Rain2: File,
    pub OT_Greet_Snow2: File,
    pub OT_Greet_House_G: File,
    pub OT_Greet_House3: File,
    pub OT_Greet_House1: File,
}

impl TalkNNpc_G3_Ot_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Greet {
        OT_Greet_Field3: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Field3.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Field3.msbt").unwrap()),
        OT_Greet_Again1: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again1.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again1.msbt").unwrap()),
        OT_Greet_Field1: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Field1.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Field1.msbt").unwrap()),
        OT_Greet_House_H: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_House_H.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_House_H.msbt").unwrap()),
        OT_Greet_Fine2: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Fine2.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Fine2.msbt").unwrap()),
        OT_Greet_Spot: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Spot.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Spot.msbt").unwrap()),
        OT_Greet_House2: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_House2.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_House2.msbt").unwrap()),
        OT_Greet_Again_LifeStart: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again_LifeStart.msbt").unwrap()),
        OT_Greet_Again2: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again2.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Again2.msbt").unwrap()),
        OT_Greet_Rain2: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Rain2.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Rain2.msbt").unwrap()),
        OT_Greet_Snow2: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_Snow2.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_Snow2.msbt").unwrap()),
        OT_Greet_House_G: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_House_G.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_House_G.msbt").unwrap()),
        OT_Greet_House3: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_House3.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_House3.msbt").unwrap()),
        OT_Greet_House1: File::new("/TalkNNpc/G3_Ot/Greet/OT_Greet_House1.msbt", msbts.get("/TalkNNpc/G3_Ot/Greet/OT_Greet_House1.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot_Free {
    pub OT_FreeC_FurnitureP_Theme: File,
    pub OT_FreeH_Progress: File,
    pub OT_FreeD_RumorOP_Favorite: File,
    pub OT_FreeF_EarthDay: File,
    pub OT_FreeA_ClothesP: File,
    pub OT_FreeA_First01: File,
    pub OT_FreeG_Host: File,
    pub OT_FreeE_Event: File,
    pub OT_FreeD_RumorN2: File,
    pub OT_FreeF_NewYear: File,
    pub OT_FreeC_FurnitureP_Same: File,
    pub OT_FreeA_AlwaysB: File,
    pub OT_FreeF_MayDay: File,
    pub OT_FreeF_Countdown: File,
    pub OT_FreeA_ClothesN: File,
    pub OT_FreeA_Week: File,
    pub OT_FreeA_Want: File,
    pub OT_FreeB_Weather: File,
    pub OT_FreeD_Moving: File,
    pub OT_FreeE_Snpc: File,
    pub OT_FreeB_Furniture_Genre: File,
    pub OT_FreeA_Questions: File,
    pub OT_FreeB_ItemP: File,
    pub OT_FreeI_Present: File,
    pub OT_FreeC_RoomG: File,
    pub OT_FreeF_CatchInsectFes: File,
    pub OT_FreeF_NewYear_Zodiac: File,
    pub OT_FreeF_NewYear_Chinese: File,
    pub OT_FreeB_ItemN: File,
    pub OT_FreeA_FirstTent: File,
    pub OT_FreeD_RumorP_Favorite: File,
    pub OT_FreeC_FurnitureP_Genre: File,
    pub OT_FreeA_First02: File,
    pub OT_FreeD_RumorOP_Action: File,
    pub OT_FreeF_CatchFishFes: File,
    pub OT_FreeD_Keyword: File,
    pub OT_FreeA_Always: File,
    pub OT_FreeD_RumorN1: File,
    pub OT_FreeB_Seasons: File,
    pub OT_FreeA_AlwaysA: File,
    pub OT_FreeG_Visitor: File,
    pub OT_FreeB_Spot: File,
    pub OT_FreeB_Furniture_Theme: File,
    pub OT_FreeC_RoomH: File,
    pub OT_FreeF_Easter: File,
    pub OT_FreeC_FurnitureN_Genre: File,
    pub OT_FreeD_RumorP_Action: File,
}

impl TalkNNpc_G3_Ot_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot_Free {
        OT_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Theme.msbt").unwrap()),
        OT_FreeH_Progress: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeH_Progress.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeH_Progress.msbt").unwrap()),
        OT_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorOP_Favorite.msbt").unwrap()),
        OT_FreeF_EarthDay: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_EarthDay.msbt").unwrap()),
        OT_FreeA_ClothesP: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_ClothesP.msbt").unwrap()),
        OT_FreeA_First01: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_First01.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_First01.msbt").unwrap()),
        OT_FreeG_Host: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeG_Host.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeG_Host.msbt").unwrap()),
        OT_FreeE_Event: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeE_Event.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeE_Event.msbt").unwrap()),
        OT_FreeD_RumorN2: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorN2.msbt").unwrap()),
        OT_FreeF_NewYear: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear.msbt").unwrap()),
        OT_FreeC_FurnitureP_Same: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Same.msbt").unwrap()),
        OT_FreeA_AlwaysB: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_AlwaysB.msbt").unwrap()),
        OT_FreeF_MayDay: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_MayDay.msbt").unwrap()),
        OT_FreeF_Countdown: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_Countdown.msbt").unwrap()),
        OT_FreeA_ClothesN: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_ClothesN.msbt").unwrap()),
        OT_FreeA_Week: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_Week.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_Week.msbt").unwrap()),
        OT_FreeA_Want: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_Want.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_Want.msbt").unwrap()),
        OT_FreeB_Weather: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_Weather.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_Weather.msbt").unwrap()),
        OT_FreeD_Moving: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_Moving.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_Moving.msbt").unwrap()),
        OT_FreeE_Snpc: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeE_Snpc.msbt").unwrap()),
        OT_FreeB_Furniture_Genre: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_Furniture_Genre.msbt").unwrap()),
        OT_FreeA_Questions: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_Questions.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_Questions.msbt").unwrap()),
        OT_FreeB_ItemP: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_ItemP.msbt").unwrap()),
        OT_FreeI_Present: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeI_Present.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeI_Present.msbt").unwrap()),
        OT_FreeC_RoomG: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_RoomG.msbt").unwrap()),
        OT_FreeF_CatchInsectFes: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_CatchInsectFes.msbt").unwrap()),
        OT_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear_Zodiac.msbt").unwrap()),
        OT_FreeF_NewYear_Chinese: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_NewYear_Chinese.msbt").unwrap()),
        OT_FreeB_ItemN: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_ItemN.msbt").unwrap()),
        OT_FreeA_FirstTent: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_FirstTent.msbt").unwrap()),
        OT_FreeD_RumorP_Favorite: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorP_Favorite.msbt").unwrap()),
        OT_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureP_Genre.msbt").unwrap()),
        OT_FreeA_First02: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_First02.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_First02.msbt").unwrap()),
        OT_FreeD_RumorOP_Action: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorOP_Action.msbt").unwrap()),
        OT_FreeF_CatchFishFes: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_CatchFishFes.msbt").unwrap()),
        OT_FreeD_Keyword: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_Keyword.msbt").unwrap()),
        OT_FreeA_Always: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_Always.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_Always.msbt").unwrap()),
        OT_FreeD_RumorN1: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorN1.msbt").unwrap()),
        OT_FreeB_Seasons: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_Seasons.msbt").unwrap()),
        OT_FreeA_AlwaysA: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeA_AlwaysA.msbt").unwrap()),
        OT_FreeG_Visitor: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeG_Visitor.msbt").unwrap()),
        OT_FreeB_Spot: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_Spot.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_Spot.msbt").unwrap()),
        OT_FreeB_Furniture_Theme: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeB_Furniture_Theme.msbt").unwrap()),
        OT_FreeC_RoomH: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_RoomH.msbt").unwrap()),
        OT_FreeF_Easter: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeF_Easter.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeF_Easter.msbt").unwrap()),
        OT_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeC_FurnitureN_Genre.msbt").unwrap()),
        OT_FreeD_RumorP_Action: File::new("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/G3_Ot/Free/OT_FreeD_RumorP_Action.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G3_Ot {
    pub React: TalkNNpc_G3_Ot_React,
    pub Approach: TalkNNpc_G3_Ot_Approach,
    pub Select: TalkNNpc_G3_Ot_Select,
    pub Force: TalkNNpc_G3_Ot_Force,
    pub GreetV: TalkNNpc_G3_Ot_GreetV,
    pub GEvent: TalkNNpc_G3_Ot_GEvent,
    pub Conv: TalkNNpc_G3_Ot_Conv,
    pub Quest: TalkNNpc_G3_Ot_Quest,
    pub Spot: TalkNNpc_G3_Ot_Spot,
    pub Greet: TalkNNpc_G3_Ot_Greet,
    pub Free: TalkNNpc_G3_Ot_Free,
    pub OT_Connect_StandingUp: File,
    pub OT_End: File,
}

impl TalkNNpc_G3_Ot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G3_Ot {
        React: TalkNNpc_G3_Ot_React::new(msbts),
        Approach: TalkNNpc_G3_Ot_Approach::new(msbts),
        Select: TalkNNpc_G3_Ot_Select::new(msbts),
        Force: TalkNNpc_G3_Ot_Force::new(msbts),
        GreetV: TalkNNpc_G3_Ot_GreetV::new(msbts),
        GEvent: TalkNNpc_G3_Ot_GEvent::new(msbts),
        Conv: TalkNNpc_G3_Ot_Conv::new(msbts),
        Quest: TalkNNpc_G3_Ot_Quest::new(msbts),
        Spot: TalkNNpc_G3_Ot_Spot::new(msbts),
        Greet: TalkNNpc_G3_Ot_Greet::new(msbts),
        Free: TalkNNpc_G3_Ot_Free::new(msbts),
        OT_Connect_StandingUp: File::new("/TalkNNpc/G3_Ot/OT_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/G3_Ot/OT_Connect_StandingUp.msbt").unwrap()),
        OT_End: File::new("/TalkNNpc/G3_Ot/OT_End.msbt", msbts.get("/TalkNNpc/G3_Ot/OT_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_React {
    pub FU_React_Watching_Insect: File,
    pub FU_React_Anger: File,
    pub FU_React_Missing7: File,
    pub FU_React_Shopping_Tailor: File,
    pub FU_React_Shopping_Gstore: File,
    pub FU_React_Happy: File,
    pub FU_React_Watching_Art: File,
    pub FU_React_Talkative: File,
    pub FU_React_GEvent_Quest: File,
    pub FU_React_Napping: File,
    pub FU_React_LookingUp: File,
    pub FU_React_NewYear_Chinese: File,
    pub FU_React_First_Stranger: File,
    pub FU_React_FirstV_Stranger: File,
    pub FU_React_Run: File,
    pub FU_React_Sad: File,
    pub FU_React_Fishing: File,
    pub FU_React_MoveIn: File,
    pub FU_React_Watching_Fossil: File,
    pub FU_React_Receive_Easter: File,
    pub FU_React_Catching: File,
    pub FU_React_NewYear: File,
    pub FU_React_Quest_TreasureHunt: File,
    pub FU_React_FirstV_Acquaintance: File,
    pub FU_React_Quest_Sick: File,
    pub FU_React_GEvent_BirthdayP: File,
    pub FU_React_EarlyLate: File,
    pub FU_React_Watching_Fish: File,
    pub FU_React_Poison: File,
    pub FU_React_Sitting: File,
    pub FU_React_Worry: File,
    pub FU_React_Itching: File,
    pub FU_React_MoveOut: File,
    pub FU_React_DIY: File,
    pub FU_React_Fall: File,
    pub FU_React_Beeface: File,
    pub FU_React_First_Acquaintance: File,
    pub FU_React_Missing30: File,
}

impl TalkNNpc_G1_Fu_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_React {
        FU_React_Watching_Insect: File::new("/TalkNNpc/G1_Fu/React/FU_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Watching_Insect.msbt").unwrap()),
        FU_React_Anger: File::new("/TalkNNpc/G1_Fu/React/FU_React_Anger.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Anger.msbt").unwrap()),
        FU_React_Missing7: File::new("/TalkNNpc/G1_Fu/React/FU_React_Missing7.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Missing7.msbt").unwrap()),
        FU_React_Shopping_Tailor: File::new("/TalkNNpc/G1_Fu/React/FU_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Shopping_Tailor.msbt").unwrap()),
        FU_React_Shopping_Gstore: File::new("/TalkNNpc/G1_Fu/React/FU_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Shopping_Gstore.msbt").unwrap()),
        FU_React_Happy: File::new("/TalkNNpc/G1_Fu/React/FU_React_Happy.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Happy.msbt").unwrap()),
        FU_React_Watching_Art: File::new("/TalkNNpc/G1_Fu/React/FU_React_Watching_Art.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Watching_Art.msbt").unwrap()),
        FU_React_Talkative: File::new("/TalkNNpc/G1_Fu/React/FU_React_Talkative.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Talkative.msbt").unwrap()),
        FU_React_GEvent_Quest: File::new("/TalkNNpc/G1_Fu/React/FU_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_GEvent_Quest.msbt").unwrap()),
        FU_React_Napping: File::new("/TalkNNpc/G1_Fu/React/FU_React_Napping.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Napping.msbt").unwrap()),
        FU_React_LookingUp: File::new("/TalkNNpc/G1_Fu/React/FU_React_LookingUp.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_LookingUp.msbt").unwrap()),
        FU_React_NewYear_Chinese: File::new("/TalkNNpc/G1_Fu/React/FU_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_NewYear_Chinese.msbt").unwrap()),
        FU_React_First_Stranger: File::new("/TalkNNpc/G1_Fu/React/FU_React_First_Stranger.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_First_Stranger.msbt").unwrap()),
        FU_React_FirstV_Stranger: File::new("/TalkNNpc/G1_Fu/React/FU_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_FirstV_Stranger.msbt").unwrap()),
        FU_React_Run: File::new("/TalkNNpc/G1_Fu/React/FU_React_Run.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Run.msbt").unwrap()),
        FU_React_Sad: File::new("/TalkNNpc/G1_Fu/React/FU_React_Sad.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Sad.msbt").unwrap()),
        FU_React_Fishing: File::new("/TalkNNpc/G1_Fu/React/FU_React_Fishing.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Fishing.msbt").unwrap()),
        FU_React_MoveIn: File::new("/TalkNNpc/G1_Fu/React/FU_React_MoveIn.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_MoveIn.msbt").unwrap()),
        FU_React_Watching_Fossil: File::new("/TalkNNpc/G1_Fu/React/FU_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Watching_Fossil.msbt").unwrap()),
        FU_React_Receive_Easter: File::new("/TalkNNpc/G1_Fu/React/FU_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Receive_Easter.msbt").unwrap()),
        FU_React_Catching: File::new("/TalkNNpc/G1_Fu/React/FU_React_Catching.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Catching.msbt").unwrap()),
        FU_React_NewYear: File::new("/TalkNNpc/G1_Fu/React/FU_React_NewYear.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_NewYear.msbt").unwrap()),
        FU_React_Quest_TreasureHunt: File::new("/TalkNNpc/G1_Fu/React/FU_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Quest_TreasureHunt.msbt").unwrap()),
        FU_React_FirstV_Acquaintance: File::new("/TalkNNpc/G1_Fu/React/FU_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_FirstV_Acquaintance.msbt").unwrap()),
        FU_React_Quest_Sick: File::new("/TalkNNpc/G1_Fu/React/FU_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Quest_Sick.msbt").unwrap()),
        FU_React_GEvent_BirthdayP: File::new("/TalkNNpc/G1_Fu/React/FU_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_GEvent_BirthdayP.msbt").unwrap()),
        FU_React_EarlyLate: File::new("/TalkNNpc/G1_Fu/React/FU_React_EarlyLate.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_EarlyLate.msbt").unwrap()),
        FU_React_Watching_Fish: File::new("/TalkNNpc/G1_Fu/React/FU_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Watching_Fish.msbt").unwrap()),
        FU_React_Poison: File::new("/TalkNNpc/G1_Fu/React/FU_React_Poison.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Poison.msbt").unwrap()),
        FU_React_Sitting: File::new("/TalkNNpc/G1_Fu/React/FU_React_Sitting.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Sitting.msbt").unwrap()),
        FU_React_Worry: File::new("/TalkNNpc/G1_Fu/React/FU_React_Worry.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Worry.msbt").unwrap()),
        FU_React_Itching: File::new("/TalkNNpc/G1_Fu/React/FU_React_Itching.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Itching.msbt").unwrap()),
        FU_React_MoveOut: File::new("/TalkNNpc/G1_Fu/React/FU_React_MoveOut.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_MoveOut.msbt").unwrap()),
        FU_React_DIY: File::new("/TalkNNpc/G1_Fu/React/FU_React_DIY.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_DIY.msbt").unwrap()),
        FU_React_Fall: File::new("/TalkNNpc/G1_Fu/React/FU_React_Fall.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Fall.msbt").unwrap()),
        FU_React_Beeface: File::new("/TalkNNpc/G1_Fu/React/FU_React_Beeface.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Beeface.msbt").unwrap()),
        FU_React_First_Acquaintance: File::new("/TalkNNpc/G1_Fu/React/FU_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_First_Acquaintance.msbt").unwrap()),
        FU_React_Missing30: File::new("/TalkNNpc/G1_Fu/React/FU_React_Missing30.msbt", msbts.get("/TalkNNpc/G1_Fu/React/FU_React_Missing30.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Approach {
    pub FU_ApproachC_Sell: File,
    pub FU_ApproachF_First: File,
    pub FU_ApproachE_MainSeq: File,
    pub FU_ApproachA_Emoticons: File,
    pub FU_ApproachB_Greeting: File,
    pub FU_ApproachA_Always: File,
    pub FU_ApproachC_Present: File,
    pub FU_ApproachB_NickName: File,
    pub FU_ApproachD_Stay: File,
    pub FU_ApproachC_Want: File,
    pub FU_ApproachB_Habit: File,
    pub FU_ApproachE_Easter: File,
    pub FU_ApproachD_Moving: File,
    pub FU_ApproachC_Trade: File,
}

impl TalkNNpc_G1_Fu_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Approach {
        FU_ApproachC_Sell: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Sell.msbt").unwrap()),
        FU_ApproachF_First: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachF_First.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachF_First.msbt").unwrap()),
        FU_ApproachE_MainSeq: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachE_MainSeq.msbt").unwrap()),
        FU_ApproachA_Emoticons: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachA_Emoticons.msbt").unwrap()),
        FU_ApproachB_Greeting: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_Greeting.msbt").unwrap()),
        FU_ApproachA_Always: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachA_Always.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachA_Always.msbt").unwrap()),
        FU_ApproachC_Present: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Present.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Present.msbt").unwrap()),
        FU_ApproachB_NickName: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_NickName.msbt").unwrap()),
        FU_ApproachD_Stay: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachD_Stay.msbt").unwrap()),
        FU_ApproachC_Want: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Want.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Want.msbt").unwrap()),
        FU_ApproachB_Habit: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachB_Habit.msbt").unwrap()),
        FU_ApproachE_Easter: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachE_Easter.msbt").unwrap()),
        FU_ApproachD_Moving: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachD_Moving.msbt").unwrap()),
        FU_ApproachC_Trade: File::new("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/G1_Fu/Approach/FU_ApproachC_Trade.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_GreetV {
    pub FU_GreetV_Fine2: File,
    pub FU_GreetV_Again1: File,
    pub FU_GreetV_Field1: File,
    pub FU_GreetV_Rain2: File,
    pub FU_GreetV_Again2: File,
    pub FU_GreetV_Snow2: File,
}

impl TalkNNpc_G1_Fu_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_GreetV {
        FU_GreetV_Fine2: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Fine2.msbt").unwrap()),
        FU_GreetV_Again1: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Again1.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Again1.msbt").unwrap()),
        FU_GreetV_Field1: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Field1.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Field1.msbt").unwrap()),
        FU_GreetV_Rain2: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Rain2.msbt").unwrap()),
        FU_GreetV_Again2: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Again2.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Again2.msbt").unwrap()),
        FU_GreetV_Snow2: File::new("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/G1_Fu/GreetV/FU_GreetV_Snow2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Spot {
    pub FU_Spot_Camp_Quest: File,
    pub FU_Spot_Camp_Game: File,
    pub FU_Spot_Camp: File,
    pub FU_Spot_Office: File,
    pub FU_Spot_Museum_Fossil: File,
    pub FU_Spot_Camp_Amiibo: File,
    pub FU_Spot_Tailor: File,
    pub FU_Spot_Gstore: File,
    pub FU_Spot_Museum_Fish: File,
    pub FU_Spot_Museum_Art: File,
    pub FU_Spot_MysteryTour: File,
    pub FU_Spot_Museum_Insect: File,
    pub FU_Spot_Camp_Invite: File,
}

impl TalkNNpc_G1_Fu_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Spot {
        FU_Spot_Camp_Quest: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Quest.msbt").unwrap()),
        FU_Spot_Camp_Game: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Game.msbt").unwrap()),
        FU_Spot_Camp: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp.msbt").unwrap()),
        FU_Spot_Office: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Office.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Office.msbt").unwrap()),
        FU_Spot_Museum_Fossil: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Fossil.msbt").unwrap()),
        FU_Spot_Camp_Amiibo: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Amiibo.msbt").unwrap()),
        FU_Spot_Tailor: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Tailor.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Tailor.msbt").unwrap()),
        FU_Spot_Gstore: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Gstore.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Gstore.msbt").unwrap()),
        FU_Spot_Museum_Fish: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Fish.msbt").unwrap()),
        FU_Spot_Museum_Art: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Art.msbt").unwrap()),
        FU_Spot_MysteryTour: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_MysteryTour.msbt").unwrap()),
        FU_Spot_Museum_Insect: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Museum_Insect.msbt").unwrap()),
        FU_Spot_Camp_Invite: File::new("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/G1_Fu/Spot/FU_Spot_Camp_Invite.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Free {
    pub FU_FreeF_Countdown: File,
    pub FU_FreeF_Easter: File,
    pub FU_FreeF_CatchFishFes: File,
    pub FU_FreeB_Furniture_Genre: File,
    pub FU_FreeB_Weather: File,
    pub FU_FreeC_FurnitureP_Theme: File,
    pub FU_FreeA_Questions: File,
    pub FU_FreeG_Host: File,
    pub FU_FreeD_RumorOP_Favorite: File,
    pub FU_FreeB_ItemP: File,
    pub FU_FreeF_NewYear_Chinese: File,
    pub FU_FreeI_Present: File,
    pub FU_FreeD_RumorP_Action: File,
    pub FU_FreeC_RoomG: File,
    pub FU_FreeD_RumorP_Favorite: File,
    pub FU_FreeB_ItemN: File,
    pub FU_FreeF_CatchInsectFes: File,
    pub FU_FreeA_Week: File,
    pub FU_FreeF_NewYear_Zodiac: File,
    pub FU_FreeA_Want: File,
    pub FU_FreeA_FirstTent: File,
    pub FU_FreeE_Snpc: File,
    pub FU_FreeA_First02: File,
    pub FU_FreeF_MayDay: File,
    pub FU_FreeD_Keyword: File,
    pub FU_FreeH_Progress: File,
    pub FU_FreeF_EarthDay: File,
    pub FU_FreeD_RumorOP_Action: File,
    pub FU_FreeD_Moving: File,
    pub FU_FreeD_RumorN1: File,
    pub FU_FreeA_ClothesP: File,
    pub FU_FreeB_Seasons: File,
    pub FU_FreeA_AlwaysA: File,
    pub FU_FreeB_Furniture_Theme: File,
    pub FU_FreeG_Visitor: File,
    pub FU_FreeA_ClothesN: File,
    pub FU_FreeC_RoomH: File,
    pub FU_FreeC_FurnitureP_Genre: File,
    pub FU_FreeB_Spot: File,
    pub FU_FreeA_Always: File,
    pub FU_FreeC_FurnitureP_Same: File,
    pub FU_FreeA_First01: File,
    pub FU_FreeE_Event: File,
    pub FU_FreeD_RumorN2: File,
    pub FU_FreeF_NewYear: File,
    pub FU_FreeA_AlwaysB: File,
    pub FU_FreeC_FurnitureN_Genre: File,
}

impl TalkNNpc_G1_Fu_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Free {
        FU_FreeF_Countdown: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_Countdown.msbt").unwrap()),
        FU_FreeF_Easter: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_Easter.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_Easter.msbt").unwrap()),
        FU_FreeF_CatchFishFes: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_CatchFishFes.msbt").unwrap()),
        FU_FreeB_Furniture_Genre: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_Furniture_Genre.msbt").unwrap()),
        FU_FreeB_Weather: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_Weather.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_Weather.msbt").unwrap()),
        FU_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Theme.msbt").unwrap()),
        FU_FreeA_Questions: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_Questions.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_Questions.msbt").unwrap()),
        FU_FreeG_Host: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeG_Host.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeG_Host.msbt").unwrap()),
        FU_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorOP_Favorite.msbt").unwrap()),
        FU_FreeB_ItemP: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_ItemP.msbt").unwrap()),
        FU_FreeF_NewYear_Chinese: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear_Chinese.msbt").unwrap()),
        FU_FreeI_Present: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeI_Present.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeI_Present.msbt").unwrap()),
        FU_FreeD_RumorP_Action: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorP_Action.msbt").unwrap()),
        FU_FreeC_RoomG: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_RoomG.msbt").unwrap()),
        FU_FreeD_RumorP_Favorite: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorP_Favorite.msbt").unwrap()),
        FU_FreeB_ItemN: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_ItemN.msbt").unwrap()),
        FU_FreeF_CatchInsectFes: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_CatchInsectFes.msbt").unwrap()),
        FU_FreeA_Week: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_Week.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_Week.msbt").unwrap()),
        FU_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear_Zodiac.msbt").unwrap()),
        FU_FreeA_Want: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_Want.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_Want.msbt").unwrap()),
        FU_FreeA_FirstTent: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_FirstTent.msbt").unwrap()),
        FU_FreeE_Snpc: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeE_Snpc.msbt").unwrap()),
        FU_FreeA_First02: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_First02.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_First02.msbt").unwrap()),
        FU_FreeF_MayDay: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_MayDay.msbt").unwrap()),
        FU_FreeD_Keyword: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_Keyword.msbt").unwrap()),
        FU_FreeH_Progress: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeH_Progress.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeH_Progress.msbt").unwrap()),
        FU_FreeF_EarthDay: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_EarthDay.msbt").unwrap()),
        FU_FreeD_RumorOP_Action: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorOP_Action.msbt").unwrap()),
        FU_FreeD_Moving: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_Moving.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_Moving.msbt").unwrap()),
        FU_FreeD_RumorN1: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorN1.msbt").unwrap()),
        FU_FreeA_ClothesP: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_ClothesP.msbt").unwrap()),
        FU_FreeB_Seasons: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_Seasons.msbt").unwrap()),
        FU_FreeA_AlwaysA: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_AlwaysA.msbt").unwrap()),
        FU_FreeB_Furniture_Theme: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_Furniture_Theme.msbt").unwrap()),
        FU_FreeG_Visitor: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeG_Visitor.msbt").unwrap()),
        FU_FreeA_ClothesN: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_ClothesN.msbt").unwrap()),
        FU_FreeC_RoomH: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_RoomH.msbt").unwrap()),
        FU_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Genre.msbt").unwrap()),
        FU_FreeB_Spot: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeB_Spot.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeB_Spot.msbt").unwrap()),
        FU_FreeA_Always: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_Always.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_Always.msbt").unwrap()),
        FU_FreeC_FurnitureP_Same: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureP_Same.msbt").unwrap()),
        FU_FreeA_First01: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_First01.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_First01.msbt").unwrap()),
        FU_FreeE_Event: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeE_Event.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeE_Event.msbt").unwrap()),
        FU_FreeD_RumorN2: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeD_RumorN2.msbt").unwrap()),
        FU_FreeF_NewYear: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeF_NewYear.msbt").unwrap()),
        FU_FreeA_AlwaysB: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeA_AlwaysB.msbt").unwrap()),
        FU_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/G1_Fu/Free/FU_FreeC_FurnitureN_Genre.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Select {
    pub FU_Select_Present: File,
    pub FU_Select_General: File,
}

impl TalkNNpc_G1_Fu_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Select {
        FU_Select_Present: File::new("/TalkNNpc/G1_Fu/Select/FU_Select_Present.msbt", msbts.get("/TalkNNpc/G1_Fu/Select/FU_Select_Present.msbt").unwrap()),
        FU_Select_General: File::new("/TalkNNpc/G1_Fu/Select/FU_Select_General.msbt", msbts.get("/TalkNNpc/G1_Fu/Select/FU_Select_General.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Greet {
    pub FU_Greet_House2: File,
    pub FU_Greet_Again2: File,
    pub FU_Greet_Again_LifeStart: File,
    pub FU_Greet_House_H: File,
    pub FU_Greet_Fine2: File,
    pub FU_Greet_House3: File,
    pub FU_Greet_House1: File,
    pub FU_Greet_Field3: File,
    pub FU_Greet_Again1: File,
    pub FU_Greet_House_G: File,
    pub FU_Greet_Field1: File,
    pub FU_Greet_Rain2: File,
    pub FU_Greet_Spot: File,
    pub FU_Greet_Snow2: File,
}

impl TalkNNpc_G1_Fu_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Greet {
        FU_Greet_House2: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_House2.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_House2.msbt").unwrap()),
        FU_Greet_Again2: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again2.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again2.msbt").unwrap()),
        FU_Greet_Again_LifeStart: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again_LifeStart.msbt").unwrap()),
        FU_Greet_House_H: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_House_H.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_House_H.msbt").unwrap()),
        FU_Greet_Fine2: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Fine2.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Fine2.msbt").unwrap()),
        FU_Greet_House3: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_House3.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_House3.msbt").unwrap()),
        FU_Greet_House1: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_House1.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_House1.msbt").unwrap()),
        FU_Greet_Field3: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Field3.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Field3.msbt").unwrap()),
        FU_Greet_Again1: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again1.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Again1.msbt").unwrap()),
        FU_Greet_House_G: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_House_G.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_House_G.msbt").unwrap()),
        FU_Greet_Field1: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Field1.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Field1.msbt").unwrap()),
        FU_Greet_Rain2: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Rain2.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Rain2.msbt").unwrap()),
        FU_Greet_Spot: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Spot.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Spot.msbt").unwrap()),
        FU_Greet_Snow2: File::new("/TalkNNpc/G1_Fu/Greet/FU_Greet_Snow2.msbt", msbts.get("/TalkNNpc/G1_Fu/Greet/FU_Greet_Snow2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_GEvent {
    pub FU_GEvent_BirthdayP_G: File,
    pub FU_GEvent_Countdown: File,
    pub FU_GEvent_BirthdayP: File,
    pub FU_GEvent_BirthdayN_H: File,
    pub FU_GEvent_CatchInsectFes: File,
    pub FU_GEvent_CatchFishFes: File,
    pub FU_GEvent_BirthdayP_H: File,
    pub FU_GEvent_Easter: File,
    pub FU_GEvent_BirthdayN_G: File,
}

impl TalkNNpc_G1_Fu_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_GEvent {
        FU_GEvent_BirthdayP_G: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP_G.msbt").unwrap()),
        FU_GEvent_Countdown: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_Countdown.msbt").unwrap()),
        FU_GEvent_BirthdayP: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP.msbt").unwrap()),
        FU_GEvent_BirthdayN_H: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayN_H.msbt").unwrap()),
        FU_GEvent_CatchInsectFes: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_CatchInsectFes.msbt").unwrap()),
        FU_GEvent_CatchFishFes: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_CatchFishFes.msbt").unwrap()),
        FU_GEvent_BirthdayP_H: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayP_H.msbt").unwrap()),
        FU_GEvent_Easter: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_Easter.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_Easter.msbt").unwrap()),
        FU_GEvent_BirthdayN_G: File::new("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/G1_Fu/GEvent/FU_GEvent_BirthdayN_G.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Conv {
    pub FU_Conv_OT: File,
    pub FU_Conv_FU: File,
    pub FU_Conv_GE: File,
    pub FU_Conv_KO: File,
    pub FU_Conv_AN: File,
    pub FU_Conv_HA: File,
    pub FU_Conv_ZK: File,
    pub FU_Conv_BO: File,
}

impl TalkNNpc_G1_Fu_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Conv {
        FU_Conv_OT: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_OT.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_OT.msbt").unwrap()),
        FU_Conv_FU: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_FU.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_FU.msbt").unwrap()),
        FU_Conv_GE: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_GE.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_GE.msbt").unwrap()),
        FU_Conv_KO: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_KO.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_KO.msbt").unwrap()),
        FU_Conv_AN: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_AN.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_AN.msbt").unwrap()),
        FU_Conv_HA: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_HA.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_HA.msbt").unwrap()),
        FU_Conv_ZK: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_ZK.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_ZK.msbt").unwrap()),
        FU_Conv_BO: File::new("/TalkNNpc/G1_Fu/Conv/FU_Conv_BO.msbt", msbts.get("/TalkNNpc/G1_Fu/Conv/FU_Conv_BO.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Quest {
    pub FU_Quest_TreasureHunt_Talk: File,
    pub FU_Quest_Delivery_Cloth: File,
    pub FU_Quest_CatchFishInsect_Begin: File,
    pub FU_Quest_Delivery_Give: File,
    pub FU_Quest_Sick_End: File,
    pub FU_Quest_TreasureHunt_End: File,
    pub FU_Quest_LostProperty_End: File,
    pub FU_Quest_Sick_Begin: File,
    pub FU_Quest_TreasureHunt_Begin: File,
    pub FU_Quest_Delivery_End: File,
    pub FU_Quest_Delivery_Begin: File,
    pub FU_Quest_Sick_Cured: File,
    pub FU_Quest_Delivery_After: File,
    pub FU_Quest_LostProperty_Begin: File,
    pub FU_Quest_CatchFishInsect_End: File,
}

impl TalkNNpc_G1_Fu_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Quest {
        FU_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_Talk.msbt").unwrap()),
        FU_Quest_Delivery_Cloth: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Cloth.msbt").unwrap()),
        FU_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        FU_Quest_Delivery_Give: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Give.msbt").unwrap()),
        FU_Quest_Sick_End: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_End.msbt").unwrap()),
        FU_Quest_TreasureHunt_End: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_End.msbt").unwrap()),
        FU_Quest_LostProperty_End: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_LostProperty_End.msbt").unwrap()),
        FU_Quest_Sick_Begin: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_Begin.msbt").unwrap()),
        FU_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_TreasureHunt_Begin.msbt").unwrap()),
        FU_Quest_Delivery_End: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_End.msbt").unwrap()),
        FU_Quest_Delivery_Begin: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_Begin.msbt").unwrap()),
        FU_Quest_Sick_Cured: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Sick_Cured.msbt").unwrap()),
        FU_Quest_Delivery_After: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_Delivery_After.msbt").unwrap()),
        FU_Quest_LostProperty_Begin: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_LostProperty_Begin.msbt").unwrap()),
        FU_Quest_CatchFishInsect_End: File::new("/TalkNNpc/G1_Fu/Quest/FU_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/G1_Fu/Quest/FU_Quest_CatchFishInsect_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu_Force {
    pub FU_Force_Hit: File,
    pub FU_Force_Flea: File,
    pub FU_Force_Push: File,
}

impl TalkNNpc_G1_Fu_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu_Force {
        FU_Force_Hit: File::new("/TalkNNpc/G1_Fu/Force/FU_Force_Hit.msbt", msbts.get("/TalkNNpc/G1_Fu/Force/FU_Force_Hit.msbt").unwrap()),
        FU_Force_Flea: File::new("/TalkNNpc/G1_Fu/Force/FU_Force_Flea.msbt", msbts.get("/TalkNNpc/G1_Fu/Force/FU_Force_Flea.msbt").unwrap()),
        FU_Force_Push: File::new("/TalkNNpc/G1_Fu/Force/FU_Force_Push.msbt", msbts.get("/TalkNNpc/G1_Fu/Force/FU_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G1_Fu {
    pub React: TalkNNpc_G1_Fu_React,
    pub Approach: TalkNNpc_G1_Fu_Approach,
    pub GreetV: TalkNNpc_G1_Fu_GreetV,
    pub Spot: TalkNNpc_G1_Fu_Spot,
    pub Free: TalkNNpc_G1_Fu_Free,
    pub Select: TalkNNpc_G1_Fu_Select,
    pub Greet: TalkNNpc_G1_Fu_Greet,
    pub GEvent: TalkNNpc_G1_Fu_GEvent,
    pub Conv: TalkNNpc_G1_Fu_Conv,
    pub Quest: TalkNNpc_G1_Fu_Quest,
    pub Force: TalkNNpc_G1_Fu_Force,
    pub FU_Connect_StandingUp: File,
    pub FU_End: File,
}

impl TalkNNpc_G1_Fu {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G1_Fu {
        React: TalkNNpc_G1_Fu_React::new(msbts),
        Approach: TalkNNpc_G1_Fu_Approach::new(msbts),
        GreetV: TalkNNpc_G1_Fu_GreetV::new(msbts),
        Spot: TalkNNpc_G1_Fu_Spot::new(msbts),
        Free: TalkNNpc_G1_Fu_Free::new(msbts),
        Select: TalkNNpc_G1_Fu_Select::new(msbts),
        Greet: TalkNNpc_G1_Fu_Greet::new(msbts),
        GEvent: TalkNNpc_G1_Fu_GEvent::new(msbts),
        Conv: TalkNNpc_G1_Fu_Conv::new(msbts),
        Quest: TalkNNpc_G1_Fu_Quest::new(msbts),
        Force: TalkNNpc_G1_Fu_Force::new(msbts),
        FU_Connect_StandingUp: File::new("/TalkNNpc/G1_Fu/FU_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/G1_Fu/FU_Connect_StandingUp.msbt").unwrap()),
        FU_End: File::new("/TalkNNpc/G1_Fu/FU_End.msbt", msbts.get("/TalkNNpc/G1_Fu/FU_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_GEvent {
    pub KO_GEvent_BirthdayN_H: File,
    pub KO_GEvent_CatchInsectFes: File,
    pub KO_GEvent_BirthdayP_H: File,
    pub KO_GEvent_Easter: File,
    pub KO_GEvent_BirthdayN_G: File,
    pub KO_GEvent_CatchFishFes: File,
    pub KO_GEvent_Countdown: File,
    pub KO_GEvent_BirthdayP: File,
    pub KO_GEvent_BirthdayP_G: File,
}

impl TalkNNpc_B3_Ko_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_GEvent {
        KO_GEvent_BirthdayN_H: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayN_H.msbt").unwrap()),
        KO_GEvent_CatchInsectFes: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_CatchInsectFes.msbt").unwrap()),
        KO_GEvent_BirthdayP_H: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP_H.msbt").unwrap()),
        KO_GEvent_Easter: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_Easter.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_Easter.msbt").unwrap()),
        KO_GEvent_BirthdayN_G: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayN_G.msbt").unwrap()),
        KO_GEvent_CatchFishFes: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_CatchFishFes.msbt").unwrap()),
        KO_GEvent_Countdown: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_Countdown.msbt").unwrap()),
        KO_GEvent_BirthdayP: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP.msbt").unwrap()),
        KO_GEvent_BirthdayP_G: File::new("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/B3_Ko/GEvent/KO_GEvent_BirthdayP_G.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Spot {
    pub KO_Spot_Office: File,
    pub KO_Spot_Tailor: File,
    pub KO_Spot_Gstore: File,
    pub KO_Spot_Camp_Quest: File,
    pub KO_Spot_Camp_Amiibo: File,
    pub KO_Spot_Museum_Insect: File,
    pub KO_Spot_Museum_Fish: File,
    pub KO_Spot_Camp: File,
    pub KO_Spot_MysteryTour: File,
    pub KO_Spot_Camp_Game: File,
    pub KO_Spot_Camp_Invite: File,
    pub KO_Spot_Museum_Art: File,
    pub KO_Spot_Museum_Fossil: File,
}

impl TalkNNpc_B3_Ko_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Spot {
        KO_Spot_Office: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Office.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Office.msbt").unwrap()),
        KO_Spot_Tailor: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Tailor.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Tailor.msbt").unwrap()),
        KO_Spot_Gstore: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Gstore.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Gstore.msbt").unwrap()),
        KO_Spot_Camp_Quest: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Quest.msbt").unwrap()),
        KO_Spot_Camp_Amiibo: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Amiibo.msbt").unwrap()),
        KO_Spot_Museum_Insect: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Insect.msbt").unwrap()),
        KO_Spot_Museum_Fish: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Fish.msbt").unwrap()),
        KO_Spot_Camp: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp.msbt").unwrap()),
        KO_Spot_MysteryTour: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_MysteryTour.msbt").unwrap()),
        KO_Spot_Camp_Game: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Game.msbt").unwrap()),
        KO_Spot_Camp_Invite: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Camp_Invite.msbt").unwrap()),
        KO_Spot_Museum_Art: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Art.msbt").unwrap()),
        KO_Spot_Museum_Fossil: File::new("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/B3_Ko/Spot/KO_Spot_Museum_Fossil.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Free {
    pub KO_FreeA_ClothesP: File,
    pub KO_FreeD_RumorP_Favorite: File,
    pub KO_FreeB_ItemN: File,
    pub KO_FreeD_Keyword: File,
    pub KO_FreeA_ClothesN: File,
    pub KO_FreeD_RumorN1: File,
    pub KO_FreeB_Seasons: File,
    pub KO_FreeA_AlwaysA: File,
    pub KO_FreeF_Countdown: File,
    pub KO_FreeF_CatchInsectFes: File,
    pub KO_FreeF_NewYear_Zodiac: File,
    pub KO_FreeG_Visitor: File,
    pub KO_FreeF_MayDay: File,
    pub KO_FreeA_Questions: File,
    pub KO_FreeC_FurnitureP_Genre: File,
    pub KO_FreeB_Furniture_Theme: File,
    pub KO_FreeD_RumorOP_Action: File,
    pub KO_FreeD_Moving: File,
    pub KO_FreeB_Spot: File,
    pub KO_FreeC_RoomH: File,
    pub KO_FreeD_RumorP_Action: File,
    pub KO_FreeA_FirstTent: File,
    pub KO_FreeA_First01: File,
    pub KO_FreeF_CatchFishFes: File,
    pub KO_FreeD_RumorN2: File,
    pub KO_FreeF_NewYear: File,
    pub KO_FreeA_AlwaysB: File,
    pub KO_FreeC_FurnitureN_Genre: File,
    pub KO_FreeC_FurnitureP_Same: File,
    pub KO_FreeE_Event: File,
    pub KO_FreeB_Weather: File,
    pub KO_FreeA_Always: File,
    pub KO_FreeG_Host: File,
    pub KO_FreeC_FurnitureP_Theme: File,
    pub KO_FreeD_RumorOP_Favorite: File,
    pub KO_FreeI_Present: File,
    pub KO_FreeB_Furniture_Genre: File,
    pub KO_FreeA_Week: File,
    pub KO_FreeA_Want: File,
    pub KO_FreeF_Easter: File,
    pub KO_FreeE_Snpc: File,
    pub KO_FreeB_ItemP: File,
    pub KO_FreeF_NewYear_Chinese: File,
    pub KO_FreeH_Progress: File,
    pub KO_FreeF_EarthDay: File,
    pub KO_FreeA_First02: File,
    pub KO_FreeC_RoomG: File,
}

impl TalkNNpc_B3_Ko_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Free {
        KO_FreeA_ClothesP: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_ClothesP.msbt").unwrap()),
        KO_FreeD_RumorP_Favorite: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorP_Favorite.msbt").unwrap()),
        KO_FreeB_ItemN: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_ItemN.msbt").unwrap()),
        KO_FreeD_Keyword: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_Keyword.msbt").unwrap()),
        KO_FreeA_ClothesN: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_ClothesN.msbt").unwrap()),
        KO_FreeD_RumorN1: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorN1.msbt").unwrap()),
        KO_FreeB_Seasons: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_Seasons.msbt").unwrap()),
        KO_FreeA_AlwaysA: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_AlwaysA.msbt").unwrap()),
        KO_FreeF_Countdown: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_Countdown.msbt").unwrap()),
        KO_FreeF_CatchInsectFes: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_CatchInsectFes.msbt").unwrap()),
        KO_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear_Zodiac.msbt").unwrap()),
        KO_FreeG_Visitor: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeG_Visitor.msbt").unwrap()),
        KO_FreeF_MayDay: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_MayDay.msbt").unwrap()),
        KO_FreeA_Questions: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_Questions.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_Questions.msbt").unwrap()),
        KO_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Genre.msbt").unwrap()),
        KO_FreeB_Furniture_Theme: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_Furniture_Theme.msbt").unwrap()),
        KO_FreeD_RumorOP_Action: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorOP_Action.msbt").unwrap()),
        KO_FreeD_Moving: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_Moving.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_Moving.msbt").unwrap()),
        KO_FreeB_Spot: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_Spot.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_Spot.msbt").unwrap()),
        KO_FreeC_RoomH: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_RoomH.msbt").unwrap()),
        KO_FreeD_RumorP_Action: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorP_Action.msbt").unwrap()),
        KO_FreeA_FirstTent: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_FirstTent.msbt").unwrap()),
        KO_FreeA_First01: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_First01.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_First01.msbt").unwrap()),
        KO_FreeF_CatchFishFes: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_CatchFishFes.msbt").unwrap()),
        KO_FreeD_RumorN2: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorN2.msbt").unwrap()),
        KO_FreeF_NewYear: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear.msbt").unwrap()),
        KO_FreeA_AlwaysB: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_AlwaysB.msbt").unwrap()),
        KO_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureN_Genre.msbt").unwrap()),
        KO_FreeC_FurnitureP_Same: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Same.msbt").unwrap()),
        KO_FreeE_Event: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeE_Event.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeE_Event.msbt").unwrap()),
        KO_FreeB_Weather: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_Weather.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_Weather.msbt").unwrap()),
        KO_FreeA_Always: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_Always.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_Always.msbt").unwrap()),
        KO_FreeG_Host: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeG_Host.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeG_Host.msbt").unwrap()),
        KO_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_FurnitureP_Theme.msbt").unwrap()),
        KO_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeD_RumorOP_Favorite.msbt").unwrap()),
        KO_FreeI_Present: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeI_Present.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeI_Present.msbt").unwrap()),
        KO_FreeB_Furniture_Genre: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_Furniture_Genre.msbt").unwrap()),
        KO_FreeA_Week: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_Week.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_Week.msbt").unwrap()),
        KO_FreeA_Want: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_Want.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_Want.msbt").unwrap()),
        KO_FreeF_Easter: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_Easter.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_Easter.msbt").unwrap()),
        KO_FreeE_Snpc: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeE_Snpc.msbt").unwrap()),
        KO_FreeB_ItemP: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeB_ItemP.msbt").unwrap()),
        KO_FreeF_NewYear_Chinese: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_NewYear_Chinese.msbt").unwrap()),
        KO_FreeH_Progress: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeH_Progress.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeH_Progress.msbt").unwrap()),
        KO_FreeF_EarthDay: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeF_EarthDay.msbt").unwrap()),
        KO_FreeA_First02: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeA_First02.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeA_First02.msbt").unwrap()),
        KO_FreeC_RoomG: File::new("/TalkNNpc/B3_Ko/Free/KO_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/B3_Ko/Free/KO_FreeC_RoomG.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Greet {
    pub KO_Greet_Fine2: File,
    pub KO_Greet_House2: File,
    pub KO_Greet_Again2: File,
    pub KO_Greet_House_G: File,
    pub KO_Greet_Again_LifeStart: File,
    pub KO_Greet_Rain2: File,
    pub KO_Greet_House3: File,
    pub KO_Greet_Snow2: File,
    pub KO_Greet_Spot: File,
    pub KO_Greet_House1: File,
    pub KO_Greet_Field3: File,
    pub KO_Greet_Again1: File,
    pub KO_Greet_House_H: File,
    pub KO_Greet_Field1: File,
}

impl TalkNNpc_B3_Ko_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Greet {
        KO_Greet_Fine2: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Fine2.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Fine2.msbt").unwrap()),
        KO_Greet_House2: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_House2.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_House2.msbt").unwrap()),
        KO_Greet_Again2: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again2.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again2.msbt").unwrap()),
        KO_Greet_House_G: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_House_G.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_House_G.msbt").unwrap()),
        KO_Greet_Again_LifeStart: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again_LifeStart.msbt").unwrap()),
        KO_Greet_Rain2: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Rain2.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Rain2.msbt").unwrap()),
        KO_Greet_House3: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_House3.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_House3.msbt").unwrap()),
        KO_Greet_Snow2: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Snow2.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Snow2.msbt").unwrap()),
        KO_Greet_Spot: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Spot.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Spot.msbt").unwrap()),
        KO_Greet_House1: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_House1.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_House1.msbt").unwrap()),
        KO_Greet_Field3: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Field3.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Field3.msbt").unwrap()),
        KO_Greet_Again1: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again1.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Again1.msbt").unwrap()),
        KO_Greet_House_H: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_House_H.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_House_H.msbt").unwrap()),
        KO_Greet_Field1: File::new("/TalkNNpc/B3_Ko/Greet/KO_Greet_Field1.msbt", msbts.get("/TalkNNpc/B3_Ko/Greet/KO_Greet_Field1.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Force {
    pub KO_Force_Flea: File,
    pub KO_Force_Push: File,
    pub KO_Force_Hit: File,
}

impl TalkNNpc_B3_Ko_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Force {
        KO_Force_Flea: File::new("/TalkNNpc/B3_Ko/Force/KO_Force_Flea.msbt", msbts.get("/TalkNNpc/B3_Ko/Force/KO_Force_Flea.msbt").unwrap()),
        KO_Force_Push: File::new("/TalkNNpc/B3_Ko/Force/KO_Force_Push.msbt", msbts.get("/TalkNNpc/B3_Ko/Force/KO_Force_Push.msbt").unwrap()),
        KO_Force_Hit: File::new("/TalkNNpc/B3_Ko/Force/KO_Force_Hit.msbt", msbts.get("/TalkNNpc/B3_Ko/Force/KO_Force_Hit.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Approach {
    pub KO_ApproachA_Always: File,
    pub KO_ApproachC_Sell: File,
    pub KO_ApproachA_Emoticons: File,
    pub KO_ApproachE_MainSeq: File,
    pub KO_ApproachB_Greeting: File,
    pub KO_ApproachB_NickName: File,
    pub KO_ApproachB_Habit: File,
    pub KO_ApproachC_Present: File,
    pub KO_ApproachD_Stay: File,
    pub KO_ApproachC_Want: File,
    pub KO_ApproachE_Easter: File,
    pub KO_ApproachD_Moving: File,
    pub KO_ApproachC_Trade: File,
    pub KO_ApproachF_First: File,
}

impl TalkNNpc_B3_Ko_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Approach {
        KO_ApproachA_Always: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachA_Always.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachA_Always.msbt").unwrap()),
        KO_ApproachC_Sell: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Sell.msbt").unwrap()),
        KO_ApproachA_Emoticons: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachA_Emoticons.msbt").unwrap()),
        KO_ApproachE_MainSeq: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachE_MainSeq.msbt").unwrap()),
        KO_ApproachB_Greeting: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_Greeting.msbt").unwrap()),
        KO_ApproachB_NickName: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_NickName.msbt").unwrap()),
        KO_ApproachB_Habit: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachB_Habit.msbt").unwrap()),
        KO_ApproachC_Present: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Present.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Present.msbt").unwrap()),
        KO_ApproachD_Stay: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachD_Stay.msbt").unwrap()),
        KO_ApproachC_Want: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Want.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Want.msbt").unwrap()),
        KO_ApproachE_Easter: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachE_Easter.msbt").unwrap()),
        KO_ApproachD_Moving: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachD_Moving.msbt").unwrap()),
        KO_ApproachC_Trade: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachC_Trade.msbt").unwrap()),
        KO_ApproachF_First: File::new("/TalkNNpc/B3_Ko/Approach/KO_ApproachF_First.msbt", msbts.get("/TalkNNpc/B3_Ko/Approach/KO_ApproachF_First.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Conv {
    pub KO_Conv_HA: File,
    pub KO_Conv_ZK: File,
    pub KO_Conv_BO: File,
    pub KO_Conv_OT: File,
    pub KO_Conv_FU: File,
    pub KO_Conv_GE: File,
    pub KO_Conv_KO: File,
    pub KO_Conv_AN: File,
}

impl TalkNNpc_B3_Ko_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Conv {
        KO_Conv_HA: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_HA.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_HA.msbt").unwrap()),
        KO_Conv_ZK: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_ZK.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_ZK.msbt").unwrap()),
        KO_Conv_BO: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_BO.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_BO.msbt").unwrap()),
        KO_Conv_OT: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_OT.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_OT.msbt").unwrap()),
        KO_Conv_FU: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_FU.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_FU.msbt").unwrap()),
        KO_Conv_GE: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_GE.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_GE.msbt").unwrap()),
        KO_Conv_KO: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_KO.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_KO.msbt").unwrap()),
        KO_Conv_AN: File::new("/TalkNNpc/B3_Ko/Conv/KO_Conv_AN.msbt", msbts.get("/TalkNNpc/B3_Ko/Conv/KO_Conv_AN.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_GreetV {
    pub KO_GreetV_Again1: File,
    pub KO_GreetV_Field1: File,
    pub KO_GreetV_Fine2: File,
    pub KO_GreetV_Again2: File,
    pub KO_GreetV_Rain2: File,
    pub KO_GreetV_Snow2: File,
}

impl TalkNNpc_B3_Ko_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_GreetV {
        KO_GreetV_Again1: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Again1.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Again1.msbt").unwrap()),
        KO_GreetV_Field1: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Field1.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Field1.msbt").unwrap()),
        KO_GreetV_Fine2: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Fine2.msbt").unwrap()),
        KO_GreetV_Again2: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Again2.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Again2.msbt").unwrap()),
        KO_GreetV_Rain2: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Rain2.msbt").unwrap()),
        KO_GreetV_Snow2: File::new("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/B3_Ko/GreetV/KO_GreetV_Snow2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_React {
    pub KO_React_Watching_Fish: File,
    pub KO_React_LookingUp: File,
    pub KO_React_Fishing: File,
    pub KO_React_Quest_TreasureHunt: File,
    pub KO_React_First_Stranger: File,
    pub KO_React_Watching_Insect: File,
    pub KO_React_Run: File,
    pub KO_React_Shopping_Tailor: File,
    pub KO_React_Shopping_Gstore: File,
    pub KO_React_NewYear: File,
    pub KO_React_Sad: File,
    pub KO_React_Catching: File,
    pub KO_React_Quest_Follow: File,
    pub KO_React_Quest_Sick: File,
    pub KO_React_Worry: File,
    pub KO_React_First_Acquaintance: File,
    pub KO_React_Sitting: File,
    pub KO_React_Itching: File,
    pub KO_React_Receive_Easter: File,
    pub KO_React_MoveOut: File,
    pub KO_React_FirstV_Acquaintance: File,
    pub KO_React_Beeface: File,
    pub KO_React_EarlyLate: File,
    pub KO_React_NewYear_Chinese: File,
    pub KO_React_Fall: File,
    pub KO_React_MoveIn: File,
    pub KO_React_FirstV_Stranger: File,
    pub KO_React_Anger: File,
    pub KO_React_Happy: File,
    pub KO_React_Missing30: File,
    pub KO_React_DIY: File,
    pub KO_React_Napping: File,
    pub KO_React_Watching_Fossil: File,
    pub KO_React_Missing7: File,
    pub KO_React_GEvent_BirthdayP: File,
    pub KO_React_Poison: File,
    pub KO_React_Watching_Art: File,
    pub KO_React_GEvent_Quest: File,
    pub KO_React_Talkative: File,
}

impl TalkNNpc_B3_Ko_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_React {
        KO_React_Watching_Fish: File::new("/TalkNNpc/B3_Ko/React/KO_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Watching_Fish.msbt").unwrap()),
        KO_React_LookingUp: File::new("/TalkNNpc/B3_Ko/React/KO_React_LookingUp.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_LookingUp.msbt").unwrap()),
        KO_React_Fishing: File::new("/TalkNNpc/B3_Ko/React/KO_React_Fishing.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Fishing.msbt").unwrap()),
        KO_React_Quest_TreasureHunt: File::new("/TalkNNpc/B3_Ko/React/KO_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Quest_TreasureHunt.msbt").unwrap()),
        KO_React_First_Stranger: File::new("/TalkNNpc/B3_Ko/React/KO_React_First_Stranger.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_First_Stranger.msbt").unwrap()),
        KO_React_Watching_Insect: File::new("/TalkNNpc/B3_Ko/React/KO_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Watching_Insect.msbt").unwrap()),
        KO_React_Run: File::new("/TalkNNpc/B3_Ko/React/KO_React_Run.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Run.msbt").unwrap()),
        KO_React_Shopping_Tailor: File::new("/TalkNNpc/B3_Ko/React/KO_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Shopping_Tailor.msbt").unwrap()),
        KO_React_Shopping_Gstore: File::new("/TalkNNpc/B3_Ko/React/KO_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Shopping_Gstore.msbt").unwrap()),
        KO_React_NewYear: File::new("/TalkNNpc/B3_Ko/React/KO_React_NewYear.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_NewYear.msbt").unwrap()),
        KO_React_Sad: File::new("/TalkNNpc/B3_Ko/React/KO_React_Sad.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Sad.msbt").unwrap()),
        KO_React_Catching: File::new("/TalkNNpc/B3_Ko/React/KO_React_Catching.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Catching.msbt").unwrap()),
        KO_React_Quest_Follow: File::new("/TalkNNpc/B3_Ko/React/KO_React_Quest_Follow.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Quest_Follow.msbt").unwrap()),
        KO_React_Quest_Sick: File::new("/TalkNNpc/B3_Ko/React/KO_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Quest_Sick.msbt").unwrap()),
        KO_React_Worry: File::new("/TalkNNpc/B3_Ko/React/KO_React_Worry.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Worry.msbt").unwrap()),
        KO_React_First_Acquaintance: File::new("/TalkNNpc/B3_Ko/React/KO_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_First_Acquaintance.msbt").unwrap()),
        KO_React_Sitting: File::new("/TalkNNpc/B3_Ko/React/KO_React_Sitting.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Sitting.msbt").unwrap()),
        KO_React_Itching: File::new("/TalkNNpc/B3_Ko/React/KO_React_Itching.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Itching.msbt").unwrap()),
        KO_React_Receive_Easter: File::new("/TalkNNpc/B3_Ko/React/KO_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Receive_Easter.msbt").unwrap()),
        KO_React_MoveOut: File::new("/TalkNNpc/B3_Ko/React/KO_React_MoveOut.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_MoveOut.msbt").unwrap()),
        KO_React_FirstV_Acquaintance: File::new("/TalkNNpc/B3_Ko/React/KO_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_FirstV_Acquaintance.msbt").unwrap()),
        KO_React_Beeface: File::new("/TalkNNpc/B3_Ko/React/KO_React_Beeface.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Beeface.msbt").unwrap()),
        KO_React_EarlyLate: File::new("/TalkNNpc/B3_Ko/React/KO_React_EarlyLate.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_EarlyLate.msbt").unwrap()),
        KO_React_NewYear_Chinese: File::new("/TalkNNpc/B3_Ko/React/KO_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_NewYear_Chinese.msbt").unwrap()),
        KO_React_Fall: File::new("/TalkNNpc/B3_Ko/React/KO_React_Fall.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Fall.msbt").unwrap()),
        KO_React_MoveIn: File::new("/TalkNNpc/B3_Ko/React/KO_React_MoveIn.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_MoveIn.msbt").unwrap()),
        KO_React_FirstV_Stranger: File::new("/TalkNNpc/B3_Ko/React/KO_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_FirstV_Stranger.msbt").unwrap()),
        KO_React_Anger: File::new("/TalkNNpc/B3_Ko/React/KO_React_Anger.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Anger.msbt").unwrap()),
        KO_React_Happy: File::new("/TalkNNpc/B3_Ko/React/KO_React_Happy.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Happy.msbt").unwrap()),
        KO_React_Missing30: File::new("/TalkNNpc/B3_Ko/React/KO_React_Missing30.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Missing30.msbt").unwrap()),
        KO_React_DIY: File::new("/TalkNNpc/B3_Ko/React/KO_React_DIY.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_DIY.msbt").unwrap()),
        KO_React_Napping: File::new("/TalkNNpc/B3_Ko/React/KO_React_Napping.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Napping.msbt").unwrap()),
        KO_React_Watching_Fossil: File::new("/TalkNNpc/B3_Ko/React/KO_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Watching_Fossil.msbt").unwrap()),
        KO_React_Missing7: File::new("/TalkNNpc/B3_Ko/React/KO_React_Missing7.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Missing7.msbt").unwrap()),
        KO_React_GEvent_BirthdayP: File::new("/TalkNNpc/B3_Ko/React/KO_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_GEvent_BirthdayP.msbt").unwrap()),
        KO_React_Poison: File::new("/TalkNNpc/B3_Ko/React/KO_React_Poison.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Poison.msbt").unwrap()),
        KO_React_Watching_Art: File::new("/TalkNNpc/B3_Ko/React/KO_React_Watching_Art.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Watching_Art.msbt").unwrap()),
        KO_React_GEvent_Quest: File::new("/TalkNNpc/B3_Ko/React/KO_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_GEvent_Quest.msbt").unwrap()),
        KO_React_Talkative: File::new("/TalkNNpc/B3_Ko/React/KO_React_Talkative.msbt", msbts.get("/TalkNNpc/B3_Ko/React/KO_React_Talkative.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Quest {
    pub KO_Quest_Sick_End: File,
    pub KO_Quest_TreasureHunt_Begin: File,
    pub KO_Quest_Sick_Begin: File,
    pub KO_Quest_Delivery_Give: File,
    pub KO_Quest_TreasureHunt_Talk: File,
    pub KO_Quest_Delivery_End: File,
    pub KO_Quest_Sick_Cured: File,
    pub KO_Quest_TreasureHunt_End: File,
    pub KO_Quest_LostProperty_End: File,
    pub KO_Quest_LostProperty_Begin: File,
    pub KO_Quest_Delivery_Begin: File,
    pub KO_Quest_Delivery_After: File,
    pub KO_Quest_CatchFishInsect_Begin: File,
    pub KO_Quest_CatchFishInsect_End: File,
    pub KO_Quest_Delivery_Cloth: File,
}

impl TalkNNpc_B3_Ko_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Quest {
        KO_Quest_Sick_End: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_End.msbt").unwrap()),
        KO_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_Begin.msbt").unwrap()),
        KO_Quest_Sick_Begin: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_Begin.msbt").unwrap()),
        KO_Quest_Delivery_Give: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Give.msbt").unwrap()),
        KO_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_Talk.msbt").unwrap()),
        KO_Quest_Delivery_End: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_End.msbt").unwrap()),
        KO_Quest_Sick_Cured: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Sick_Cured.msbt").unwrap()),
        KO_Quest_TreasureHunt_End: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_TreasureHunt_End.msbt").unwrap()),
        KO_Quest_LostProperty_End: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_LostProperty_End.msbt").unwrap()),
        KO_Quest_LostProperty_Begin: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_LostProperty_Begin.msbt").unwrap()),
        KO_Quest_Delivery_Begin: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Begin.msbt").unwrap()),
        KO_Quest_Delivery_After: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_After.msbt").unwrap()),
        KO_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        KO_Quest_CatchFishInsect_End: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_CatchFishInsect_End.msbt").unwrap()),
        KO_Quest_Delivery_Cloth: File::new("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/B3_Ko/Quest/KO_Quest_Delivery_Cloth.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko_Select {
    pub KO_Select_General: File,
    pub KO_Select_Present: File,
}

impl TalkNNpc_B3_Ko_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko_Select {
        KO_Select_General: File::new("/TalkNNpc/B3_Ko/Select/KO_Select_General.msbt", msbts.get("/TalkNNpc/B3_Ko/Select/KO_Select_General.msbt").unwrap()),
        KO_Select_Present: File::new("/TalkNNpc/B3_Ko/Select/KO_Select_Present.msbt", msbts.get("/TalkNNpc/B3_Ko/Select/KO_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B3_Ko {
    pub GEvent: TalkNNpc_B3_Ko_GEvent,
    pub Spot: TalkNNpc_B3_Ko_Spot,
    pub Free: TalkNNpc_B3_Ko_Free,
    pub Greet: TalkNNpc_B3_Ko_Greet,
    pub Force: TalkNNpc_B3_Ko_Force,
    pub Approach: TalkNNpc_B3_Ko_Approach,
    pub Conv: TalkNNpc_B3_Ko_Conv,
    pub GreetV: TalkNNpc_B3_Ko_GreetV,
    pub React: TalkNNpc_B3_Ko_React,
    pub Quest: TalkNNpc_B3_Ko_Quest,
    pub Select: TalkNNpc_B3_Ko_Select,
    pub KO_Connect_StandingUp: File,
    pub KO_End: File,
}

impl TalkNNpc_B3_Ko {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B3_Ko {
        GEvent: TalkNNpc_B3_Ko_GEvent::new(msbts),
        Spot: TalkNNpc_B3_Ko_Spot::new(msbts),
        Free: TalkNNpc_B3_Ko_Free::new(msbts),
        Greet: TalkNNpc_B3_Ko_Greet::new(msbts),
        Force: TalkNNpc_B3_Ko_Force::new(msbts),
        Approach: TalkNNpc_B3_Ko_Approach::new(msbts),
        Conv: TalkNNpc_B3_Ko_Conv::new(msbts),
        GreetV: TalkNNpc_B3_Ko_GreetV::new(msbts),
        React: TalkNNpc_B3_Ko_React::new(msbts),
        Quest: TalkNNpc_B3_Ko_Quest::new(msbts),
        Select: TalkNNpc_B3_Ko_Select::new(msbts),
        KO_Connect_StandingUp: File::new("/TalkNNpc/B3_Ko/KO_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/B3_Ko/KO_Connect_StandingUp.msbt").unwrap()),
        KO_End: File::new("/TalkNNpc/B3_Ko/KO_End.msbt", msbts.get("/TalkNNpc/B3_Ko/KO_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Approach {
    pub AN_ApproachB_NickName: File,
    pub AN_ApproachB_Habit: File,
    pub AN_ApproachD_Stay: File,
    pub AN_ApproachE_WelcomeMigrants: File,
    pub AN_ApproachC_Want: File,
    pub AN_ApproachA_Emoticons: File,
    pub AN_ApproachE_Easter: File,
    pub AN_ApproachD_Moving: File,
    pub AN_ApproachC_Trade: File,
    pub AN_ApproachE_MainSeq: File,
    pub AN_ApproachF_First: File,
    pub AN_ApproachC_Present: File,
    pub AN_ApproachB_Greeting: File,
    pub AN_ApproachA_Always: File,
    pub AN_ApproachC_Sell: File,
}

impl TalkNNpc_G4_An_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Approach {
        AN_ApproachB_NickName: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachB_NickName.msbt").unwrap()),
        AN_ApproachB_Habit: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachB_Habit.msbt").unwrap()),
        AN_ApproachD_Stay: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachD_Stay.msbt").unwrap()),
        AN_ApproachE_WelcomeMigrants: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachE_WelcomeMigrants.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachE_WelcomeMigrants.msbt").unwrap()),
        AN_ApproachC_Want: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachC_Want.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachC_Want.msbt").unwrap()),
        AN_ApproachA_Emoticons: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachA_Emoticons.msbt").unwrap()),
        AN_ApproachE_Easter: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachE_Easter.msbt").unwrap()),
        AN_ApproachD_Moving: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachD_Moving.msbt").unwrap()),
        AN_ApproachC_Trade: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachC_Trade.msbt").unwrap()),
        AN_ApproachE_MainSeq: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachE_MainSeq.msbt").unwrap()),
        AN_ApproachF_First: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachF_First.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachF_First.msbt").unwrap()),
        AN_ApproachC_Present: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachC_Present.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachC_Present.msbt").unwrap()),
        AN_ApproachB_Greeting: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachB_Greeting.msbt").unwrap()),
        AN_ApproachA_Always: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachA_Always.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachA_Always.msbt").unwrap()),
        AN_ApproachC_Sell: File::new("/TalkNNpc/G4_An/Approach/AN_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/G4_An/Approach/AN_ApproachC_Sell.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Conv {
    pub AN_Conv_HA: File,
    pub AN_Conv_ZK: File,
    pub AN_Conv_BO: File,
    pub AN_Conv_OT: File,
    pub AN_Conv_FU: File,
    pub AN_Conv_GE: File,
    pub AN_Conv_KO: File,
    pub AN_Conv_AN: File,
}

impl TalkNNpc_G4_An_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Conv {
        AN_Conv_HA: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_HA.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_HA.msbt").unwrap()),
        AN_Conv_ZK: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_ZK.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_ZK.msbt").unwrap()),
        AN_Conv_BO: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_BO.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_BO.msbt").unwrap()),
        AN_Conv_OT: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_OT.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_OT.msbt").unwrap()),
        AN_Conv_FU: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_FU.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_FU.msbt").unwrap()),
        AN_Conv_GE: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_GE.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_GE.msbt").unwrap()),
        AN_Conv_KO: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_KO.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_KO.msbt").unwrap()),
        AN_Conv_AN: File::new("/TalkNNpc/G4_An/Conv/AN_Conv_AN.msbt", msbts.get("/TalkNNpc/G4_An/Conv/AN_Conv_AN.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_GEvent {
    pub AN_GEvent_Countdown: File,
    pub AN_GEvent_CatchFishFes: File,
    pub AN_GEvent_BirthdayP: File,
    pub AN_GEvent_BirthdayN_G: File,
    pub AN_GEvent_BirthdayP_G: File,
    pub AN_GEvent_BirthdayN_H: File,
    pub AN_GEvent_CatchInsectFes: File,
    pub AN_GEvent_Easter: File,
    pub AN_GEvent_BirthdayP_H: File,
}

impl TalkNNpc_G4_An_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_GEvent {
        AN_GEvent_Countdown: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_Countdown.msbt").unwrap()),
        AN_GEvent_CatchFishFes: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_CatchFishFes.msbt").unwrap()),
        AN_GEvent_BirthdayP: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP.msbt").unwrap()),
        AN_GEvent_BirthdayN_G: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayN_G.msbt").unwrap()),
        AN_GEvent_BirthdayP_G: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP_G.msbt").unwrap()),
        AN_GEvent_BirthdayN_H: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayN_H.msbt").unwrap()),
        AN_GEvent_CatchInsectFes: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_CatchInsectFes.msbt").unwrap()),
        AN_GEvent_Easter: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_Easter.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_Easter.msbt").unwrap()),
        AN_GEvent_BirthdayP_H: File::new("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/G4_An/GEvent/AN_GEvent_BirthdayP_H.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Select {
    pub AN_Select_General: File,
    pub AN_Select_Present: File,
}

impl TalkNNpc_G4_An_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Select {
        AN_Select_General: File::new("/TalkNNpc/G4_An/Select/AN_Select_General.msbt", msbts.get("/TalkNNpc/G4_An/Select/AN_Select_General.msbt").unwrap()),
        AN_Select_Present: File::new("/TalkNNpc/G4_An/Select/AN_Select_Present.msbt", msbts.get("/TalkNNpc/G4_An/Select/AN_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_React {
    pub AN_React_GEvent_BirthdayP: File,
    pub AN_React_Napping: File,
    pub AN_React_Quest_TreasureHunt: File,
    pub AN_React_Anger: File,
    pub AN_React_Fall: File,
    pub AN_React_Run: File,
    pub AN_React_Sad: File,
    pub AN_React_Happy: File,
    pub AN_React_Receive_Easter: File,
    pub AN_React_EarlyLate: File,
    pub AN_React_Watching_Insect: File,
    pub AN_React_First_Acquaintance: File,
    pub AN_React_Shopping_Tailor: File,
    pub AN_React_Shopping_Gstore: File,
    pub AN_React_1P_LifeStart: File,
    pub AN_React_Fishing: File,
    pub AN_React_Catching: File,
    pub AN_React_Missing30: File,
    pub AN_React_NewYear: File,
    pub AN_React_NewYear_Chinese: File,
    pub AN_React_Talkative: File,
    pub AN_React_DIY: File,
    pub AN_React_FirstV_Stranger: File,
    pub AN_React_MoveIn: File,
    pub AN_React_Sitting: File,
    pub AN_React_Itching: File,
    pub AN_React_Watching_Fish: File,
    pub AN_React_LookingUp: File,
    pub AN_React_MoveOut: File,
    pub AN_React_Beeface: File,
    pub AN_React_Quest_Sick: File,
    pub AN_React_Watching_Fossil: File,
    pub AN_React_Missing7: File,
    pub AN_React_FirstV_Acquaintance: File,
    pub AN_React_Watching_Art: File,
    pub AN_React_GEvent_Quest: File,
    pub AN_React_Poison: File,
    pub AN_React_Worry: File,
    pub AN_React_First_Stranger: File,
}

impl TalkNNpc_G4_An_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_React {
        AN_React_GEvent_BirthdayP: File::new("/TalkNNpc/G4_An/React/AN_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_GEvent_BirthdayP.msbt").unwrap()),
        AN_React_Napping: File::new("/TalkNNpc/G4_An/React/AN_React_Napping.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Napping.msbt").unwrap()),
        AN_React_Quest_TreasureHunt: File::new("/TalkNNpc/G4_An/React/AN_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Quest_TreasureHunt.msbt").unwrap()),
        AN_React_Anger: File::new("/TalkNNpc/G4_An/React/AN_React_Anger.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Anger.msbt").unwrap()),
        AN_React_Fall: File::new("/TalkNNpc/G4_An/React/AN_React_Fall.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Fall.msbt").unwrap()),
        AN_React_Run: File::new("/TalkNNpc/G4_An/React/AN_React_Run.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Run.msbt").unwrap()),
        AN_React_Sad: File::new("/TalkNNpc/G4_An/React/AN_React_Sad.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Sad.msbt").unwrap()),
        AN_React_Happy: File::new("/TalkNNpc/G4_An/React/AN_React_Happy.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Happy.msbt").unwrap()),
        AN_React_Receive_Easter: File::new("/TalkNNpc/G4_An/React/AN_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Receive_Easter.msbt").unwrap()),
        AN_React_EarlyLate: File::new("/TalkNNpc/G4_An/React/AN_React_EarlyLate.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_EarlyLate.msbt").unwrap()),
        AN_React_Watching_Insect: File::new("/TalkNNpc/G4_An/React/AN_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Watching_Insect.msbt").unwrap()),
        AN_React_First_Acquaintance: File::new("/TalkNNpc/G4_An/React/AN_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_First_Acquaintance.msbt").unwrap()),
        AN_React_Shopping_Tailor: File::new("/TalkNNpc/G4_An/React/AN_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Shopping_Tailor.msbt").unwrap()),
        AN_React_Shopping_Gstore: File::new("/TalkNNpc/G4_An/React/AN_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Shopping_Gstore.msbt").unwrap()),
        AN_React_1P_LifeStart: File::new("/TalkNNpc/G4_An/React/AN_React_1P_LifeStart.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_1P_LifeStart.msbt").unwrap()),
        AN_React_Fishing: File::new("/TalkNNpc/G4_An/React/AN_React_Fishing.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Fishing.msbt").unwrap()),
        AN_React_Catching: File::new("/TalkNNpc/G4_An/React/AN_React_Catching.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Catching.msbt").unwrap()),
        AN_React_Missing30: File::new("/TalkNNpc/G4_An/React/AN_React_Missing30.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Missing30.msbt").unwrap()),
        AN_React_NewYear: File::new("/TalkNNpc/G4_An/React/AN_React_NewYear.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_NewYear.msbt").unwrap()),
        AN_React_NewYear_Chinese: File::new("/TalkNNpc/G4_An/React/AN_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_NewYear_Chinese.msbt").unwrap()),
        AN_React_Talkative: File::new("/TalkNNpc/G4_An/React/AN_React_Talkative.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Talkative.msbt").unwrap()),
        AN_React_DIY: File::new("/TalkNNpc/G4_An/React/AN_React_DIY.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_DIY.msbt").unwrap()),
        AN_React_FirstV_Stranger: File::new("/TalkNNpc/G4_An/React/AN_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_FirstV_Stranger.msbt").unwrap()),
        AN_React_MoveIn: File::new("/TalkNNpc/G4_An/React/AN_React_MoveIn.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_MoveIn.msbt").unwrap()),
        AN_React_Sitting: File::new("/TalkNNpc/G4_An/React/AN_React_Sitting.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Sitting.msbt").unwrap()),
        AN_React_Itching: File::new("/TalkNNpc/G4_An/React/AN_React_Itching.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Itching.msbt").unwrap()),
        AN_React_Watching_Fish: File::new("/TalkNNpc/G4_An/React/AN_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Watching_Fish.msbt").unwrap()),
        AN_React_LookingUp: File::new("/TalkNNpc/G4_An/React/AN_React_LookingUp.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_LookingUp.msbt").unwrap()),
        AN_React_MoveOut: File::new("/TalkNNpc/G4_An/React/AN_React_MoveOut.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_MoveOut.msbt").unwrap()),
        AN_React_Beeface: File::new("/TalkNNpc/G4_An/React/AN_React_Beeface.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Beeface.msbt").unwrap()),
        AN_React_Quest_Sick: File::new("/TalkNNpc/G4_An/React/AN_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Quest_Sick.msbt").unwrap()),
        AN_React_Watching_Fossil: File::new("/TalkNNpc/G4_An/React/AN_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Watching_Fossil.msbt").unwrap()),
        AN_React_Missing7: File::new("/TalkNNpc/G4_An/React/AN_React_Missing7.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Missing7.msbt").unwrap()),
        AN_React_FirstV_Acquaintance: File::new("/TalkNNpc/G4_An/React/AN_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_FirstV_Acquaintance.msbt").unwrap()),
        AN_React_Watching_Art: File::new("/TalkNNpc/G4_An/React/AN_React_Watching_Art.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Watching_Art.msbt").unwrap()),
        AN_React_GEvent_Quest: File::new("/TalkNNpc/G4_An/React/AN_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_GEvent_Quest.msbt").unwrap()),
        AN_React_Poison: File::new("/TalkNNpc/G4_An/React/AN_React_Poison.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Poison.msbt").unwrap()),
        AN_React_Worry: File::new("/TalkNNpc/G4_An/React/AN_React_Worry.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_Worry.msbt").unwrap()),
        AN_React_First_Stranger: File::new("/TalkNNpc/G4_An/React/AN_React_First_Stranger.msbt", msbts.get("/TalkNNpc/G4_An/React/AN_React_First_Stranger.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Spot {
    pub AN_Spot_Museum_Fossil: File,
    pub AN_Spot_Camp: File,
    pub AN_Spot_Camp_Amiibo: File,
    pub AN_Spot_Museum_Fish: File,
    pub AN_Spot_Office: File,
    pub AN_Spot_Museum_Insect: File,
    pub AN_Spot_Tailor: File,
    pub AN_Spot_Gstore: File,
    pub AN_Spot_Camp_Quest: File,
    pub AN_Spot_MysteryTour: File,
    pub AN_Spot_Camp_Game: File,
    pub AN_Spot_Camp_Invite: File,
    pub AN_Spot_Museum_Art: File,
}

impl TalkNNpc_G4_An_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Spot {
        AN_Spot_Museum_Fossil: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Fossil.msbt").unwrap()),
        AN_Spot_Camp: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Camp.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Camp.msbt").unwrap()),
        AN_Spot_Camp_Amiibo: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Amiibo.msbt").unwrap()),
        AN_Spot_Museum_Fish: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Fish.msbt").unwrap()),
        AN_Spot_Office: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Office.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Office.msbt").unwrap()),
        AN_Spot_Museum_Insect: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Insect.msbt").unwrap()),
        AN_Spot_Tailor: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Tailor.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Tailor.msbt").unwrap()),
        AN_Spot_Gstore: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Gstore.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Gstore.msbt").unwrap()),
        AN_Spot_Camp_Quest: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Quest.msbt").unwrap()),
        AN_Spot_MysteryTour: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_MysteryTour.msbt").unwrap()),
        AN_Spot_Camp_Game: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Game.msbt").unwrap()),
        AN_Spot_Camp_Invite: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Camp_Invite.msbt").unwrap()),
        AN_Spot_Museum_Art: File::new("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/G4_An/Spot/AN_Spot_Museum_Art.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Quest {
    pub AN_Quest_TreasureHunt_Talk: File,
    pub AN_Quest_Sick_End: File,
    pub AN_Quest_Delivery_Give: File,
    pub AN_Quest_CatchFishInsect_End: File,
    pub AN_Quest_Delivery_Begin: File,
    pub AN_Quest_CatchFishInsect_Begin: File,
    pub AN_Quest_LostProperty_Begin: File,
    pub AN_Quest_Delivery_End: File,
    pub AN_Quest_Delivery_After: File,
    pub AN_Quest_Sick_Begin: File,
    pub AN_Quest_TreasureHunt_End: File,
    pub AN_Quest_LostProperty_End: File,
    pub AN_Quest_Delivery_Cloth: File,
    pub AN_Quest_Sick_Cured: File,
    pub AN_Quest_TreasureHunt_Begin: File,
}

impl TalkNNpc_G4_An_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Quest {
        AN_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_Talk.msbt").unwrap()),
        AN_Quest_Sick_End: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_End.msbt").unwrap()),
        AN_Quest_Delivery_Give: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Give.msbt").unwrap()),
        AN_Quest_CatchFishInsect_End: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_CatchFishInsect_End.msbt").unwrap()),
        AN_Quest_Delivery_Begin: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Begin.msbt").unwrap()),
        AN_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        AN_Quest_LostProperty_Begin: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_LostProperty_Begin.msbt").unwrap()),
        AN_Quest_Delivery_End: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_End.msbt").unwrap()),
        AN_Quest_Delivery_After: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_After.msbt").unwrap()),
        AN_Quest_Sick_Begin: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_Begin.msbt").unwrap()),
        AN_Quest_TreasureHunt_End: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_End.msbt").unwrap()),
        AN_Quest_LostProperty_End: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_LostProperty_End.msbt").unwrap()),
        AN_Quest_Delivery_Cloth: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Delivery_Cloth.msbt").unwrap()),
        AN_Quest_Sick_Cured: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_Sick_Cured.msbt").unwrap()),
        AN_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/G4_An/Quest/AN_Quest_TreasureHunt_Begin.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Free {
    pub AN_FreeH_ForMarketBuilding: File,
    pub AN_FreeE_Event: File,
    pub AN_FreeH_Progress: File,
    pub AN_FreeA_First01: File,
    pub AN_FreeF_EarthDay: File,
    pub AN_FreeD_RumorN2: File,
    pub AN_FreeF_NewYear: File,
    pub AN_FreeA_AlwaysB: File,
    pub AN_FreeA_ClothesP: File,
    pub AN_FreeF_CatchInsectFes: File,
    pub AN_FreeF_NewYear_Zodiac: File,
    pub AN_FreeA_ClothesN: File,
    pub AN_FreeB_Furniture_Theme: File,
    pub AN_FreeB_ItemP: File,
    pub AN_FreeB_Weather: File,
    pub AN_FreeC_FurnitureN_Genre: File,
    pub AN_FreeC_RoomG: File,
    pub AN_FreeF_MayDay: File,
    pub AN_FreeI_Present: File,
    pub AN_FreeD_RumorOP_Action: File,
    pub AN_FreeB_Spot: File,
    pub AN_FreeB_ItemN: File,
    pub AN_FreeH_LifeStart_Hint: File,
    pub AN_FreeD_Moving: File,
    pub AN_FreeC_FurnitureP_Theme: File,
    pub AN_FreeD_RumorOP_Favorite: File,
    pub AN_FreeF_Countdown: File,
    pub AN_FreeA_First02: File,
    pub AN_FreeD_RumorP_Action: File,
    pub AN_FreeC_FurnitureP_Same: File,
    pub AN_FreeD_Keyword: File,
    pub AN_FreeA_Questions: File,
    pub AN_FreeH_Transition: File,
    pub AN_FreeG_Host: File,
    pub AN_FreeD_RumorN1: File,
    pub AN_FreeB_Seasons: File,
    pub AN_FreeA_AlwaysA: File,
    pub AN_FreeC_RoomH: File,
    pub AN_FreeB_Furniture_Genre: File,
    pub AN_FreeG_Visitor: File,
    pub AN_FreeF_CatchFishFes: File,
    pub AN_FreeA_Always: File,
    pub AN_FreeA_Week: File,
    pub AN_FreeA_FirstTent: File,
    pub AN_FreeA_Want: File,
    pub AN_FreeE_Snpc: File,
    pub AN_FreeF_NewYear_Chinese: File,
    pub AN_FreeD_RumorP_Favorite: File,
    pub AN_FreeF_Easter: File,
    pub AN_FreeC_FurnitureP_Genre: File,
}

impl TalkNNpc_G4_An_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Free {
        AN_FreeH_ForMarketBuilding: File::new("/TalkNNpc/G4_An/Free/AN_FreeH_ForMarketBuilding.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeH_ForMarketBuilding.msbt").unwrap()),
        AN_FreeE_Event: File::new("/TalkNNpc/G4_An/Free/AN_FreeE_Event.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeE_Event.msbt").unwrap()),
        AN_FreeH_Progress: File::new("/TalkNNpc/G4_An/Free/AN_FreeH_Progress.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeH_Progress.msbt").unwrap()),
        AN_FreeA_First01: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_First01.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_First01.msbt").unwrap()),
        AN_FreeF_EarthDay: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_EarthDay.msbt").unwrap()),
        AN_FreeD_RumorN2: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorN2.msbt").unwrap()),
        AN_FreeF_NewYear: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear.msbt").unwrap()),
        AN_FreeA_AlwaysB: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_AlwaysB.msbt").unwrap()),
        AN_FreeA_ClothesP: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_ClothesP.msbt").unwrap()),
        AN_FreeF_CatchInsectFes: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_CatchInsectFes.msbt").unwrap()),
        AN_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear_Zodiac.msbt").unwrap()),
        AN_FreeA_ClothesN: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_ClothesN.msbt").unwrap()),
        AN_FreeB_Furniture_Theme: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_Furniture_Theme.msbt").unwrap()),
        AN_FreeB_ItemP: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_ItemP.msbt").unwrap()),
        AN_FreeB_Weather: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_Weather.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_Weather.msbt").unwrap()),
        AN_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureN_Genre.msbt").unwrap()),
        AN_FreeC_RoomG: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_RoomG.msbt").unwrap()),
        AN_FreeF_MayDay: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_MayDay.msbt").unwrap()),
        AN_FreeI_Present: File::new("/TalkNNpc/G4_An/Free/AN_FreeI_Present.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeI_Present.msbt").unwrap()),
        AN_FreeD_RumorOP_Action: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorOP_Action.msbt").unwrap()),
        AN_FreeB_Spot: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_Spot.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_Spot.msbt").unwrap()),
        AN_FreeB_ItemN: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_ItemN.msbt").unwrap()),
        AN_FreeH_LifeStart_Hint: File::new("/TalkNNpc/G4_An/Free/AN_FreeH_LifeStart_Hint.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeH_LifeStart_Hint.msbt").unwrap()),
        AN_FreeD_Moving: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_Moving.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_Moving.msbt").unwrap()),
        AN_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Theme.msbt").unwrap()),
        AN_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorOP_Favorite.msbt").unwrap()),
        AN_FreeF_Countdown: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_Countdown.msbt").unwrap()),
        AN_FreeA_First02: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_First02.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_First02.msbt").unwrap()),
        AN_FreeD_RumorP_Action: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorP_Action.msbt").unwrap()),
        AN_FreeC_FurnitureP_Same: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Same.msbt").unwrap()),
        AN_FreeD_Keyword: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_Keyword.msbt").unwrap()),
        AN_FreeA_Questions: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_Questions.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_Questions.msbt").unwrap()),
        AN_FreeH_Transition: File::new("/TalkNNpc/G4_An/Free/AN_FreeH_Transition.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeH_Transition.msbt").unwrap()),
        AN_FreeG_Host: File::new("/TalkNNpc/G4_An/Free/AN_FreeG_Host.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeG_Host.msbt").unwrap()),
        AN_FreeD_RumorN1: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorN1.msbt").unwrap()),
        AN_FreeB_Seasons: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_Seasons.msbt").unwrap()),
        AN_FreeA_AlwaysA: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_AlwaysA.msbt").unwrap()),
        AN_FreeC_RoomH: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_RoomH.msbt").unwrap()),
        AN_FreeB_Furniture_Genre: File::new("/TalkNNpc/G4_An/Free/AN_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeB_Furniture_Genre.msbt").unwrap()),
        AN_FreeG_Visitor: File::new("/TalkNNpc/G4_An/Free/AN_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeG_Visitor.msbt").unwrap()),
        AN_FreeF_CatchFishFes: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_CatchFishFes.msbt").unwrap()),
        AN_FreeA_Always: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_Always.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_Always.msbt").unwrap()),
        AN_FreeA_Week: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_Week.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_Week.msbt").unwrap()),
        AN_FreeA_FirstTent: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_FirstTent.msbt").unwrap()),
        AN_FreeA_Want: File::new("/TalkNNpc/G4_An/Free/AN_FreeA_Want.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeA_Want.msbt").unwrap()),
        AN_FreeE_Snpc: File::new("/TalkNNpc/G4_An/Free/AN_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeE_Snpc.msbt").unwrap()),
        AN_FreeF_NewYear_Chinese: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_NewYear_Chinese.msbt").unwrap()),
        AN_FreeD_RumorP_Favorite: File::new("/TalkNNpc/G4_An/Free/AN_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeD_RumorP_Favorite.msbt").unwrap()),
        AN_FreeF_Easter: File::new("/TalkNNpc/G4_An/Free/AN_FreeF_Easter.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeF_Easter.msbt").unwrap()),
        AN_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/G4_An/Free/AN_FreeC_FurnitureP_Genre.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_GreetV {
    pub AN_GreetV_Fine2: File,
    pub AN_GreetV_Again1: File,
    pub AN_GreetV_Field1: File,
    pub AN_GreetV_Rain2: File,
    pub AN_GreetV_Snow2: File,
    pub AN_GreetV_Again2: File,
}

impl TalkNNpc_G4_An_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_GreetV {
        AN_GreetV_Fine2: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Fine2.msbt").unwrap()),
        AN_GreetV_Again1: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Again1.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Again1.msbt").unwrap()),
        AN_GreetV_Field1: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Field1.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Field1.msbt").unwrap()),
        AN_GreetV_Rain2: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Rain2.msbt").unwrap()),
        AN_GreetV_Snow2: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Snow2.msbt").unwrap()),
        AN_GreetV_Again2: File::new("/TalkNNpc/G4_An/GreetV/AN_GreetV_Again2.msbt", msbts.get("/TalkNNpc/G4_An/GreetV/AN_GreetV_Again2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Greet {
    pub AN_Greet_Snow2: File,
    pub AN_Greet_Spot: File,
    pub AN_Greet_House2: File,
    pub AN_Greet_Again2: File,
    pub AN_Greet_Again_LifeStart: File,
    pub AN_Greet_Fine2: File,
    pub AN_Greet_House_G: File,
    pub AN_Greet_House3: File,
    pub AN_Greet_House1: File,
    pub AN_Greet_Field3: File,
    pub AN_Greet_Again1: File,
    pub AN_Greet_Field1: File,
    pub AN_Greet_Rain2: File,
    pub AN_Greet_House_H: File,
}

impl TalkNNpc_G4_An_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Greet {
        AN_Greet_Snow2: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Snow2.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Snow2.msbt").unwrap()),
        AN_Greet_Spot: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Spot.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Spot.msbt").unwrap()),
        AN_Greet_House2: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_House2.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_House2.msbt").unwrap()),
        AN_Greet_Again2: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Again2.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Again2.msbt").unwrap()),
        AN_Greet_Again_LifeStart: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Again_LifeStart.msbt").unwrap()),
        AN_Greet_Fine2: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Fine2.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Fine2.msbt").unwrap()),
        AN_Greet_House_G: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_House_G.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_House_G.msbt").unwrap()),
        AN_Greet_House3: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_House3.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_House3.msbt").unwrap()),
        AN_Greet_House1: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_House1.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_House1.msbt").unwrap()),
        AN_Greet_Field3: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Field3.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Field3.msbt").unwrap()),
        AN_Greet_Again1: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Again1.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Again1.msbt").unwrap()),
        AN_Greet_Field1: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Field1.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Field1.msbt").unwrap()),
        AN_Greet_Rain2: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_Rain2.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_Rain2.msbt").unwrap()),
        AN_Greet_House_H: File::new("/TalkNNpc/G4_An/Greet/AN_Greet_House_H.msbt", msbts.get("/TalkNNpc/G4_An/Greet/AN_Greet_House_H.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An_Force {
    pub AN_Force_Hit: File,
    pub AN_Force_Flea: File,
    pub AN_Force_Push: File,
}

impl TalkNNpc_G4_An_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An_Force {
        AN_Force_Hit: File::new("/TalkNNpc/G4_An/Force/AN_Force_Hit.msbt", msbts.get("/TalkNNpc/G4_An/Force/AN_Force_Hit.msbt").unwrap()),
        AN_Force_Flea: File::new("/TalkNNpc/G4_An/Force/AN_Force_Flea.msbt", msbts.get("/TalkNNpc/G4_An/Force/AN_Force_Flea.msbt").unwrap()),
        AN_Force_Push: File::new("/TalkNNpc/G4_An/Force/AN_Force_Push.msbt", msbts.get("/TalkNNpc/G4_An/Force/AN_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_G4_An {
    pub Approach: TalkNNpc_G4_An_Approach,
    pub Conv: TalkNNpc_G4_An_Conv,
    pub GEvent: TalkNNpc_G4_An_GEvent,
    pub Select: TalkNNpc_G4_An_Select,
    pub React: TalkNNpc_G4_An_React,
    pub Spot: TalkNNpc_G4_An_Spot,
    pub Quest: TalkNNpc_G4_An_Quest,
    pub Free: TalkNNpc_G4_An_Free,
    pub GreetV: TalkNNpc_G4_An_GreetV,
    pub Greet: TalkNNpc_G4_An_Greet,
    pub Force: TalkNNpc_G4_An_Force,
    pub AN_Connect_StandingUp: File,
    pub AN_End: File,
}

impl TalkNNpc_G4_An {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_G4_An {
        Approach: TalkNNpc_G4_An_Approach::new(msbts),
        Conv: TalkNNpc_G4_An_Conv::new(msbts),
        GEvent: TalkNNpc_G4_An_GEvent::new(msbts),
        Select: TalkNNpc_G4_An_Select::new(msbts),
        React: TalkNNpc_G4_An_React::new(msbts),
        Spot: TalkNNpc_G4_An_Spot::new(msbts),
        Quest: TalkNNpc_G4_An_Quest::new(msbts),
        Free: TalkNNpc_G4_An_Free::new(msbts),
        GreetV: TalkNNpc_G4_An_GreetV::new(msbts),
        Greet: TalkNNpc_G4_An_Greet::new(msbts),
        Force: TalkNNpc_G4_An_Force::new(msbts),
        AN_Connect_StandingUp: File::new("/TalkNNpc/G4_An/AN_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/G4_An/AN_Connect_StandingUp.msbt").unwrap()),
        AN_End: File::new("/TalkNNpc/G4_An/AN_End.msbt", msbts.get("/TalkNNpc/G4_An/AN_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Spot {
    pub ZK_Spot_Camp_First: File,
    pub ZK_Spot_Camp_Quest: File,
    pub ZK_Spot_Camp_Amiibo: File,
    pub ZK_Spot_Museum_Fish: File,
    pub ZK_Spot_Office: File,
    pub ZK_Spot_Museum_Art: File,
    pub ZK_Spot_Camp: File,
    pub ZK_Spot_Museum_Fossil: File,
    pub ZK_Spot_Tailor: File,
    pub ZK_Spot_Gstore: File,
    pub ZK_Spot_MysteryTour: File,
    pub ZK_Spot_Camp_Game: File,
    pub ZK_Spot_Camp_Invite: File,
    pub ZK_Spot_Museum_Insect: File,
}

impl TalkNNpc_B4_Zk_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Spot {
        ZK_Spot_Camp_First: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_First.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_First.msbt").unwrap()),
        ZK_Spot_Camp_Quest: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Quest.msbt").unwrap()),
        ZK_Spot_Camp_Amiibo: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Amiibo.msbt").unwrap()),
        ZK_Spot_Museum_Fish: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Fish.msbt").unwrap()),
        ZK_Spot_Office: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Office.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Office.msbt").unwrap()),
        ZK_Spot_Museum_Art: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Art.msbt").unwrap()),
        ZK_Spot_Camp: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp.msbt").unwrap()),
        ZK_Spot_Museum_Fossil: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Fossil.msbt").unwrap()),
        ZK_Spot_Tailor: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Tailor.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Tailor.msbt").unwrap()),
        ZK_Spot_Gstore: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Gstore.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Gstore.msbt").unwrap()),
        ZK_Spot_MysteryTour: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_MysteryTour.msbt").unwrap()),
        ZK_Spot_Camp_Game: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Game.msbt").unwrap()),
        ZK_Spot_Camp_Invite: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Camp_Invite.msbt").unwrap()),
        ZK_Spot_Museum_Insect: File::new("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/B4_Zk/Spot/ZK_Spot_Museum_Insect.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_GEvent {
    pub ZK_GEvent_Easter: File,
    pub ZK_GEvent_BirthdayN_G: File,
    pub ZK_GEvent_Countdown: File,
    pub ZK_GEvent_CatchInsectFes: File,
    pub ZK_GEvent_BirthdayP: File,
    pub ZK_GEvent_BirthdayP_G: File,
    pub ZK_GEvent_BirthdayN_H: File,
    pub ZK_GEvent_BirthdayP_H: File,
    pub ZK_GEvent_CatchFishFes: File,
}

impl TalkNNpc_B4_Zk_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_GEvent {
        ZK_GEvent_Easter: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_Easter.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_Easter.msbt").unwrap()),
        ZK_GEvent_BirthdayN_G: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayN_G.msbt").unwrap()),
        ZK_GEvent_Countdown: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_Countdown.msbt").unwrap()),
        ZK_GEvent_CatchInsectFes: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_CatchInsectFes.msbt").unwrap()),
        ZK_GEvent_BirthdayP: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP.msbt").unwrap()),
        ZK_GEvent_BirthdayP_G: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP_G.msbt").unwrap()),
        ZK_GEvent_BirthdayN_H: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayN_H.msbt").unwrap()),
        ZK_GEvent_BirthdayP_H: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_BirthdayP_H.msbt").unwrap()),
        ZK_GEvent_CatchFishFes: File::new("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/B4_Zk/GEvent/ZK_GEvent_CatchFishFes.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Approach {
    pub ZK_ApproachE_Easter: File,
    pub ZK_ApproachD_Moving: File,
    pub ZK_ApproachC_Sell: File,
    pub ZK_ApproachB_Habit: File,
    pub ZK_ApproachB_Greeting: File,
    pub ZK_ApproachA_Always: File,
    pub ZK_ApproachB_NickName: File,
    pub ZK_ApproachD_Stay: File,
    pub ZK_ApproachC_Want: File,
    pub ZK_ApproachC_Trade: File,
    pub ZK_ApproachE_MainSeq: File,
    pub ZK_ApproachA_Emoticons: File,
    pub ZK_ApproachF_First: File,
    pub ZK_ApproachC_Present: File,
}

impl TalkNNpc_B4_Zk_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Approach {
        ZK_ApproachE_Easter: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachE_Easter.msbt").unwrap()),
        ZK_ApproachD_Moving: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachD_Moving.msbt").unwrap()),
        ZK_ApproachC_Sell: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Sell.msbt").unwrap()),
        ZK_ApproachB_Habit: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_Habit.msbt").unwrap()),
        ZK_ApproachB_Greeting: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_Greeting.msbt").unwrap()),
        ZK_ApproachA_Always: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachA_Always.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachA_Always.msbt").unwrap()),
        ZK_ApproachB_NickName: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachB_NickName.msbt").unwrap()),
        ZK_ApproachD_Stay: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachD_Stay.msbt").unwrap()),
        ZK_ApproachC_Want: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Want.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Want.msbt").unwrap()),
        ZK_ApproachC_Trade: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Trade.msbt").unwrap()),
        ZK_ApproachE_MainSeq: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachE_MainSeq.msbt").unwrap()),
        ZK_ApproachA_Emoticons: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachA_Emoticons.msbt").unwrap()),
        ZK_ApproachF_First: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachF_First.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachF_First.msbt").unwrap()),
        ZK_ApproachC_Present: File::new("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Present.msbt", msbts.get("/TalkNNpc/B4_Zk/Approach/ZK_ApproachC_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_GreetV {
    pub ZK_GreetV_Fine2: File,
    pub ZK_GreetV_Again2: File,
    pub ZK_GreetV_Rain2: File,
    pub ZK_GreetV_Snow2: File,
    pub ZK_GreetV_Again1: File,
    pub ZK_GreetV_Field1: File,
}

impl TalkNNpc_B4_Zk_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_GreetV {
        ZK_GreetV_Fine2: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Fine2.msbt").unwrap()),
        ZK_GreetV_Again2: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Again2.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Again2.msbt").unwrap()),
        ZK_GreetV_Rain2: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Rain2.msbt").unwrap()),
        ZK_GreetV_Snow2: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Snow2.msbt").unwrap()),
        ZK_GreetV_Again1: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Again1.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Again1.msbt").unwrap()),
        ZK_GreetV_Field1: File::new("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Field1.msbt", msbts.get("/TalkNNpc/B4_Zk/GreetV/ZK_GreetV_Field1.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_React {
    pub ZK_React_Happy: File,
    pub ZK_React_Beeface: File,
    pub ZK_React_First_Stranger: File,
    pub ZK_React_Missing30: File,
    pub ZK_React_DIY: File,
    pub ZK_React_Watching_Insect: File,
    pub ZK_React_Talkative: File,
    pub ZK_React_Shopping_Tailor: File,
    pub ZK_React_Shopping_Gstore: File,
    pub ZK_React_Napping: File,
    pub ZK_React_Receive_Easter: File,
    pub ZK_React_Quest_TreasureHunt: File,
    pub ZK_React_FirstV_Acquaintance: File,
    pub ZK_React_Fall: File,
    pub ZK_React_LookingUp: File,
    pub ZK_React_Catching: File,
    pub ZK_React_GEvent_BirthdayP: File,
    pub ZK_React_NewYear_Chinese: File,
    pub ZK_React_First_Acquaintance: File,
    pub ZK_React_MoveIn: File,
    pub ZK_React_FirstV_Stranger: File,
    pub ZK_React_Run: File,
    pub ZK_React_Sad: File,
    pub ZK_React_Fishing: File,
    pub ZK_React_Quest_Sick: File,
    pub ZK_React_Watching_Art: File,
    pub ZK_React_Watching_Fish: File,
    pub ZK_React_GEvent_Quest: File,
    pub ZK_React_Worry: File,
    pub ZK_React_Watching_Fossil: File,
    pub ZK_React_Poison: File,
    pub ZK_React_NewYear: File,
    pub ZK_React_Missing7: File,
    pub ZK_React_EarlyLate: File,
    pub ZK_React_Anger: File,
    pub ZK_React_Sitting: File,
    pub ZK_React_Itching: File,
    pub ZK_React_MoveOut: File,
}

impl TalkNNpc_B4_Zk_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_React {
        ZK_React_Happy: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Happy.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Happy.msbt").unwrap()),
        ZK_React_Beeface: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Beeface.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Beeface.msbt").unwrap()),
        ZK_React_First_Stranger: File::new("/TalkNNpc/B4_Zk/React/ZK_React_First_Stranger.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_First_Stranger.msbt").unwrap()),
        ZK_React_Missing30: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Missing30.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Missing30.msbt").unwrap()),
        ZK_React_DIY: File::new("/TalkNNpc/B4_Zk/React/ZK_React_DIY.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_DIY.msbt").unwrap()),
        ZK_React_Watching_Insect: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Insect.msbt").unwrap()),
        ZK_React_Talkative: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Talkative.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Talkative.msbt").unwrap()),
        ZK_React_Shopping_Tailor: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Shopping_Tailor.msbt").unwrap()),
        ZK_React_Shopping_Gstore: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Shopping_Gstore.msbt").unwrap()),
        ZK_React_Napping: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Napping.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Napping.msbt").unwrap()),
        ZK_React_Receive_Easter: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Receive_Easter.msbt").unwrap()),
        ZK_React_Quest_TreasureHunt: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Quest_TreasureHunt.msbt").unwrap()),
        ZK_React_FirstV_Acquaintance: File::new("/TalkNNpc/B4_Zk/React/ZK_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_FirstV_Acquaintance.msbt").unwrap()),
        ZK_React_Fall: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Fall.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Fall.msbt").unwrap()),
        ZK_React_LookingUp: File::new("/TalkNNpc/B4_Zk/React/ZK_React_LookingUp.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_LookingUp.msbt").unwrap()),
        ZK_React_Catching: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Catching.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Catching.msbt").unwrap()),
        ZK_React_GEvent_BirthdayP: File::new("/TalkNNpc/B4_Zk/React/ZK_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_GEvent_BirthdayP.msbt").unwrap()),
        ZK_React_NewYear_Chinese: File::new("/TalkNNpc/B4_Zk/React/ZK_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_NewYear_Chinese.msbt").unwrap()),
        ZK_React_First_Acquaintance: File::new("/TalkNNpc/B4_Zk/React/ZK_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_First_Acquaintance.msbt").unwrap()),
        ZK_React_MoveIn: File::new("/TalkNNpc/B4_Zk/React/ZK_React_MoveIn.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_MoveIn.msbt").unwrap()),
        ZK_React_FirstV_Stranger: File::new("/TalkNNpc/B4_Zk/React/ZK_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_FirstV_Stranger.msbt").unwrap()),
        ZK_React_Run: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Run.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Run.msbt").unwrap()),
        ZK_React_Sad: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Sad.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Sad.msbt").unwrap()),
        ZK_React_Fishing: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Fishing.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Fishing.msbt").unwrap()),
        ZK_React_Quest_Sick: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Quest_Sick.msbt").unwrap()),
        ZK_React_Watching_Art: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Art.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Art.msbt").unwrap()),
        ZK_React_Watching_Fish: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Fish.msbt").unwrap()),
        ZK_React_GEvent_Quest: File::new("/TalkNNpc/B4_Zk/React/ZK_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_GEvent_Quest.msbt").unwrap()),
        ZK_React_Worry: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Worry.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Worry.msbt").unwrap()),
        ZK_React_Watching_Fossil: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Watching_Fossil.msbt").unwrap()),
        ZK_React_Poison: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Poison.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Poison.msbt").unwrap()),
        ZK_React_NewYear: File::new("/TalkNNpc/B4_Zk/React/ZK_React_NewYear.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_NewYear.msbt").unwrap()),
        ZK_React_Missing7: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Missing7.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Missing7.msbt").unwrap()),
        ZK_React_EarlyLate: File::new("/TalkNNpc/B4_Zk/React/ZK_React_EarlyLate.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_EarlyLate.msbt").unwrap()),
        ZK_React_Anger: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Anger.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Anger.msbt").unwrap()),
        ZK_React_Sitting: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Sitting.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Sitting.msbt").unwrap()),
        ZK_React_Itching: File::new("/TalkNNpc/B4_Zk/React/ZK_React_Itching.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_Itching.msbt").unwrap()),
        ZK_React_MoveOut: File::new("/TalkNNpc/B4_Zk/React/ZK_React_MoveOut.msbt", msbts.get("/TalkNNpc/B4_Zk/React/ZK_React_MoveOut.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Force {
    pub ZK_Force_Hit: File,
    pub ZK_Force_Flea: File,
    pub ZK_Force_Push: File,
}

impl TalkNNpc_B4_Zk_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Force {
        ZK_Force_Hit: File::new("/TalkNNpc/B4_Zk/Force/ZK_Force_Hit.msbt", msbts.get("/TalkNNpc/B4_Zk/Force/ZK_Force_Hit.msbt").unwrap()),
        ZK_Force_Flea: File::new("/TalkNNpc/B4_Zk/Force/ZK_Force_Flea.msbt", msbts.get("/TalkNNpc/B4_Zk/Force/ZK_Force_Flea.msbt").unwrap()),
        ZK_Force_Push: File::new("/TalkNNpc/B4_Zk/Force/ZK_Force_Push.msbt", msbts.get("/TalkNNpc/B4_Zk/Force/ZK_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Select {
    pub ZK_Select_General: File,
    pub ZK_Select_Present: File,
}

impl TalkNNpc_B4_Zk_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Select {
        ZK_Select_General: File::new("/TalkNNpc/B4_Zk/Select/ZK_Select_General.msbt", msbts.get("/TalkNNpc/B4_Zk/Select/ZK_Select_General.msbt").unwrap()),
        ZK_Select_Present: File::new("/TalkNNpc/B4_Zk/Select/ZK_Select_Present.msbt", msbts.get("/TalkNNpc/B4_Zk/Select/ZK_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Conv {
    pub ZK_Conv_ZK: File,
    pub ZK_Conv_BO: File,
    pub ZK_Conv_OT: File,
    pub ZK_Conv_FU: File,
    pub ZK_Conv_GE: File,
    pub ZK_Conv_KO: File,
    pub ZK_Conv_AN: File,
    pub ZK_Conv_HA: File,
}

impl TalkNNpc_B4_Zk_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Conv {
        ZK_Conv_ZK: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_ZK.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_ZK.msbt").unwrap()),
        ZK_Conv_BO: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_BO.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_BO.msbt").unwrap()),
        ZK_Conv_OT: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_OT.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_OT.msbt").unwrap()),
        ZK_Conv_FU: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_FU.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_FU.msbt").unwrap()),
        ZK_Conv_GE: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_GE.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_GE.msbt").unwrap()),
        ZK_Conv_KO: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_KO.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_KO.msbt").unwrap()),
        ZK_Conv_AN: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_AN.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_AN.msbt").unwrap()),
        ZK_Conv_HA: File::new("/TalkNNpc/B4_Zk/Conv/ZK_Conv_HA.msbt", msbts.get("/TalkNNpc/B4_Zk/Conv/ZK_Conv_HA.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Greet {
    pub ZK_Greet_House2: File,
    pub ZK_Greet_Again2: File,
    pub ZK_Greet_House_H: File,
    pub ZK_Greet_Again_LifeStart: File,
    pub ZK_Greet_Spot: File,
    pub ZK_Greet_Fine2: File,
    pub ZK_Greet_House3: File,
    pub ZK_Greet_House1: File,
    pub ZK_Greet_Field3: File,
    pub ZK_Greet_Again1: File,
    pub ZK_Greet_Field1: File,
    pub ZK_Greet_Rain2: File,
    pub ZK_Greet_House_G: File,
    pub ZK_Greet_Snow2: File,
}

impl TalkNNpc_B4_Zk_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Greet {
        ZK_Greet_House2: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House2.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House2.msbt").unwrap()),
        ZK_Greet_Again2: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again2.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again2.msbt").unwrap()),
        ZK_Greet_House_H: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House_H.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House_H.msbt").unwrap()),
        ZK_Greet_Again_LifeStart: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again_LifeStart.msbt").unwrap()),
        ZK_Greet_Spot: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Spot.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Spot.msbt").unwrap()),
        ZK_Greet_Fine2: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Fine2.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Fine2.msbt").unwrap()),
        ZK_Greet_House3: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House3.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House3.msbt").unwrap()),
        ZK_Greet_House1: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House1.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House1.msbt").unwrap()),
        ZK_Greet_Field3: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Field3.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Field3.msbt").unwrap()),
        ZK_Greet_Again1: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again1.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Again1.msbt").unwrap()),
        ZK_Greet_Field1: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Field1.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Field1.msbt").unwrap()),
        ZK_Greet_Rain2: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Rain2.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Rain2.msbt").unwrap()),
        ZK_Greet_House_G: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House_G.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_House_G.msbt").unwrap()),
        ZK_Greet_Snow2: File::new("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Snow2.msbt", msbts.get("/TalkNNpc/B4_Zk/Greet/ZK_Greet_Snow2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Quest {
    pub ZK_Quest_TreasureHunt_End: File,
    pub ZK_Quest_Sick_End: File,
    pub ZK_Quest_LostProperty_End: File,
    pub ZK_Quest_Delivery_Give: File,
    pub ZK_Quest_TreasureHunt_Begin: File,
    pub ZK_Quest_CatchFishInsect_Begin: File,
    pub ZK_Quest_Delivery_End: File,
    pub ZK_Quest_Delivery_Begin: File,
    pub ZK_Quest_Sick_Begin: File,
    pub ZK_Quest_Delivery_After: File,
    pub ZK_Quest_LostProperty_Begin: File,
    pub ZK_Quest_CatchFishInsect_End: File,
    pub ZK_Quest_Sick_Cured: File,
    pub ZK_Quest_TreasureHunt_Talk: File,
    pub ZK_Quest_Delivery_Cloth: File,
}

impl TalkNNpc_B4_Zk_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Quest {
        ZK_Quest_TreasureHunt_End: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_End.msbt").unwrap()),
        ZK_Quest_Sick_End: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_End.msbt").unwrap()),
        ZK_Quest_LostProperty_End: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_LostProperty_End.msbt").unwrap()),
        ZK_Quest_Delivery_Give: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Give.msbt").unwrap()),
        ZK_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_Begin.msbt").unwrap()),
        ZK_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        ZK_Quest_Delivery_End: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_End.msbt").unwrap()),
        ZK_Quest_Delivery_Begin: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Begin.msbt").unwrap()),
        ZK_Quest_Sick_Begin: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_Begin.msbt").unwrap()),
        ZK_Quest_Delivery_After: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_After.msbt").unwrap()),
        ZK_Quest_LostProperty_Begin: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_LostProperty_Begin.msbt").unwrap()),
        ZK_Quest_CatchFishInsect_End: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_CatchFishInsect_End.msbt").unwrap()),
        ZK_Quest_Sick_Cured: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Sick_Cured.msbt").unwrap()),
        ZK_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_TreasureHunt_Talk.msbt").unwrap()),
        ZK_Quest_Delivery_Cloth: File::new("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/B4_Zk/Quest/ZK_Quest_Delivery_Cloth.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk_Free {
    pub ZK_FreeD_RumorP_Favorite: File,
    pub ZK_FreeF_Countdown: File,
    pub ZK_FreeC_FurnitureN_Genre: File,
    pub ZK_FreeA_ClothesN: File,
    pub ZK_FreeE_Event: File,
    pub ZK_FreeA_Questions: File,
    pub ZK_FreeB_Spot: File,
    pub ZK_FreeC_FurnitureP_Theme: File,
    pub ZK_FreeD_RumorOP_Favorite: File,
    pub ZK_FreeA_First01: File,
    pub ZK_FreeF_CatchFishFes: File,
    pub ZK_FreeF_MayDay: File,
    pub ZK_FreeB_Furniture_Theme: File,
    pub ZK_FreeD_RumorN2: File,
    pub ZK_FreeF_NewYear: File,
    pub ZK_FreeA_AlwaysB: File,
    pub ZK_FreeA_FirstTent: File,
    pub ZK_FreeB_ItemP: File,
    pub ZK_FreeC_RoomG: File,
    pub ZK_FreeF_CatchInsectFes: File,
    pub ZK_FreeD_Moving: File,
    pub ZK_FreeF_NewYear_Zodiac: File,
    pub ZK_FreeD_RumorP_Action: File,
    pub ZK_FreeB_ItemN: File,
    pub ZK_FreeB_Weather: File,
    pub ZK_FreeG_Host: File,
    pub ZK_FreeI_Present: File,
    pub ZK_FreeD_RumorOP_Action: File,
    pub ZK_FreeC_FurnitureP_Same: File,
    pub ZK_FreeA_Week: File,
    pub ZK_FreeA_Want: File,
    pub ZK_FreeA_First02: File,
    pub ZK_FreeE_Snpc: File,
    pub ZK_FreeC_FurnitureP_Genre: File,
    pub ZK_FreeA_Always: File,
    pub ZK_FreeD_Keyword: File,
    pub ZK_FreeB_Furniture_Genre: File,
    pub ZK_FreeC_RoomH: File,
    pub ZK_FreeD_RumorN1: File,
    pub ZK_FreeB_Seasons: File,
    pub ZK_FreeA_AlwaysA: File,
    pub ZK_FreeH_Progress: File,
    pub ZK_FreeF_NewYear_Chinese: File,
    pub ZK_FreeF_EarthDay: File,
    pub ZK_FreeF_Easter: File,
    pub ZK_FreeG_Visitor: File,
    pub ZK_FreeA_ClothesP: File,
}

impl TalkNNpc_B4_Zk_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk_Free {
        ZK_FreeD_RumorP_Favorite: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorP_Favorite.msbt").unwrap()),
        ZK_FreeF_Countdown: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_Countdown.msbt").unwrap()),
        ZK_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureN_Genre.msbt").unwrap()),
        ZK_FreeA_ClothesN: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_ClothesN.msbt").unwrap()),
        ZK_FreeE_Event: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeE_Event.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeE_Event.msbt").unwrap()),
        ZK_FreeA_Questions: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Questions.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Questions.msbt").unwrap()),
        ZK_FreeB_Spot: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Spot.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Spot.msbt").unwrap()),
        ZK_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Theme.msbt").unwrap()),
        ZK_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorOP_Favorite.msbt").unwrap()),
        ZK_FreeA_First01: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_First01.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_First01.msbt").unwrap()),
        ZK_FreeF_CatchFishFes: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_CatchFishFes.msbt").unwrap()),
        ZK_FreeF_MayDay: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_MayDay.msbt").unwrap()),
        ZK_FreeB_Furniture_Theme: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Furniture_Theme.msbt").unwrap()),
        ZK_FreeD_RumorN2: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorN2.msbt").unwrap()),
        ZK_FreeF_NewYear: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear.msbt").unwrap()),
        ZK_FreeA_AlwaysB: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_AlwaysB.msbt").unwrap()),
        ZK_FreeA_FirstTent: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_FirstTent.msbt").unwrap()),
        ZK_FreeB_ItemP: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_ItemP.msbt").unwrap()),
        ZK_FreeC_RoomG: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_RoomG.msbt").unwrap()),
        ZK_FreeF_CatchInsectFes: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_CatchInsectFes.msbt").unwrap()),
        ZK_FreeD_Moving: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_Moving.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_Moving.msbt").unwrap()),
        ZK_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear_Zodiac.msbt").unwrap()),
        ZK_FreeD_RumorP_Action: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorP_Action.msbt").unwrap()),
        ZK_FreeB_ItemN: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_ItemN.msbt").unwrap()),
        ZK_FreeB_Weather: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Weather.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Weather.msbt").unwrap()),
        ZK_FreeG_Host: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeG_Host.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeG_Host.msbt").unwrap()),
        ZK_FreeI_Present: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeI_Present.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeI_Present.msbt").unwrap()),
        ZK_FreeD_RumorOP_Action: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorOP_Action.msbt").unwrap()),
        ZK_FreeC_FurnitureP_Same: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Same.msbt").unwrap()),
        ZK_FreeA_Week: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Week.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Week.msbt").unwrap()),
        ZK_FreeA_Want: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Want.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Want.msbt").unwrap()),
        ZK_FreeA_First02: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_First02.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_First02.msbt").unwrap()),
        ZK_FreeE_Snpc: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeE_Snpc.msbt").unwrap()),
        ZK_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_FurnitureP_Genre.msbt").unwrap()),
        ZK_FreeA_Always: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Always.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_Always.msbt").unwrap()),
        ZK_FreeD_Keyword: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_Keyword.msbt").unwrap()),
        ZK_FreeB_Furniture_Genre: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Furniture_Genre.msbt").unwrap()),
        ZK_FreeC_RoomH: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeC_RoomH.msbt").unwrap()),
        ZK_FreeD_RumorN1: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeD_RumorN1.msbt").unwrap()),
        ZK_FreeB_Seasons: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeB_Seasons.msbt").unwrap()),
        ZK_FreeA_AlwaysA: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_AlwaysA.msbt").unwrap()),
        ZK_FreeH_Progress: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeH_Progress.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeH_Progress.msbt").unwrap()),
        ZK_FreeF_NewYear_Chinese: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_NewYear_Chinese.msbt").unwrap()),
        ZK_FreeF_EarthDay: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_EarthDay.msbt").unwrap()),
        ZK_FreeF_Easter: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeF_Easter.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeF_Easter.msbt").unwrap()),
        ZK_FreeG_Visitor: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeG_Visitor.msbt").unwrap()),
        ZK_FreeA_ClothesP: File::new("/TalkNNpc/B4_Zk/Free/ZK_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/B4_Zk/Free/ZK_FreeA_ClothesP.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B4_Zk {
    pub Spot: TalkNNpc_B4_Zk_Spot,
    pub GEvent: TalkNNpc_B4_Zk_GEvent,
    pub Approach: TalkNNpc_B4_Zk_Approach,
    pub GreetV: TalkNNpc_B4_Zk_GreetV,
    pub React: TalkNNpc_B4_Zk_React,
    pub Force: TalkNNpc_B4_Zk_Force,
    pub Select: TalkNNpc_B4_Zk_Select,
    pub Conv: TalkNNpc_B4_Zk_Conv,
    pub Greet: TalkNNpc_B4_Zk_Greet,
    pub Quest: TalkNNpc_B4_Zk_Quest,
    pub Free: TalkNNpc_B4_Zk_Free,
    pub ZK_Connect_StandingUp: File,
    pub ZK_End: File,
}

impl TalkNNpc_B4_Zk {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B4_Zk {
        Spot: TalkNNpc_B4_Zk_Spot::new(msbts),
        GEvent: TalkNNpc_B4_Zk_GEvent::new(msbts),
        Approach: TalkNNpc_B4_Zk_Approach::new(msbts),
        GreetV: TalkNNpc_B4_Zk_GreetV::new(msbts),
        React: TalkNNpc_B4_Zk_React::new(msbts),
        Force: TalkNNpc_B4_Zk_Force::new(msbts),
        Select: TalkNNpc_B4_Zk_Select::new(msbts),
        Conv: TalkNNpc_B4_Zk_Conv::new(msbts),
        Greet: TalkNNpc_B4_Zk_Greet::new(msbts),
        Quest: TalkNNpc_B4_Zk_Quest::new(msbts),
        Free: TalkNNpc_B4_Zk_Free::new(msbts),
        ZK_Connect_StandingUp: File::new("/TalkNNpc/B4_Zk/ZK_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/B4_Zk/ZK_Connect_StandingUp.msbt").unwrap()),
        ZK_End: File::new("/TalkNNpc/B4_Zk/ZK_End.msbt", msbts.get("/TalkNNpc/B4_Zk/ZK_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Greet {
    pub BO_Greet_Spot: File,
    pub BO_Greet_Rain2: File,
    pub BO_Greet_Again_LifeStart: File,
    pub BO_Greet_Snow2: File,
    pub BO_Greet_House3: File,
    pub BO_Greet_House_H: File,
    pub BO_Greet_House1: File,
    pub BO_Greet_Field3: File,
    pub BO_Greet_Again1: File,
    pub BO_Greet_Field1: File,
    pub BO_Greet_Fine2: File,
    pub BO_Greet_House2: File,
    pub BO_Greet_Again2: File,
    pub BO_Greet_House_G: File,
}

impl TalkNNpc_B1_Bo_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Greet {
        BO_Greet_Spot: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Spot.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Spot.msbt").unwrap()),
        BO_Greet_Rain2: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Rain2.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Rain2.msbt").unwrap()),
        BO_Greet_Again_LifeStart: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again_LifeStart.msbt").unwrap()),
        BO_Greet_Snow2: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Snow2.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Snow2.msbt").unwrap()),
        BO_Greet_House3: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_House3.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_House3.msbt").unwrap()),
        BO_Greet_House_H: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_House_H.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_House_H.msbt").unwrap()),
        BO_Greet_House1: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_House1.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_House1.msbt").unwrap()),
        BO_Greet_Field3: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Field3.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Field3.msbt").unwrap()),
        BO_Greet_Again1: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again1.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again1.msbt").unwrap()),
        BO_Greet_Field1: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Field1.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Field1.msbt").unwrap()),
        BO_Greet_Fine2: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Fine2.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Fine2.msbt").unwrap()),
        BO_Greet_House2: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_House2.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_House2.msbt").unwrap()),
        BO_Greet_Again2: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again2.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_Again2.msbt").unwrap()),
        BO_Greet_House_G: File::new("/TalkNNpc/B1_Bo/Greet/BO_Greet_House_G.msbt", msbts.get("/TalkNNpc/B1_Bo/Greet/BO_Greet_House_G.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Free {
    pub BO_FreeF_NewYear: File,
    pub BO_FreeA_AlwaysB: File,
    pub BO_FreeF_CatchInsectFes: File,
    pub BO_FreeC_RoomH: File,
    pub BO_FreeF_NewYear_Zodiac: File,
    pub BO_FreeB_Weather: File,
    pub BO_FreeF_MayDay: File,
    pub BO_FreeF_Countdown: File,
    pub BO_FreeI_Present: File,
    pub BO_FreeF_CatchFishFes: File,
    pub BO_FreeD_RumorOP_Action: File,
    pub BO_FreeC_FurnitureN_Genre: File,
    pub BO_FreeH_Transition: File,
    pub BO_FreeH_LifeStart_Hint: File,
    pub BO_FreeB_Furniture_Theme: File,
    pub BO_FreeD_Moving: File,
    pub BO_FreeE_Event: File,
    pub BO_FreeA_Questions: File,
    pub BO_FreeA_First02: File,
    pub BO_FreeD_RumorP_Action: File,
    pub BO_FreeC_FurnitureP_Theme: File,
    pub BO_FreeB_Spot: File,
    pub BO_FreeD_RumorOP_Favorite: File,
    pub BO_FreeD_Keyword: File,
    pub BO_FreeA_FirstTent: File,
    pub BO_FreeD_RumorN1: File,
    pub BO_FreeB_Seasons: File,
    pub BO_FreeA_AlwaysA: File,
    pub BO_FreeB_ItemP: File,
    pub BO_FreeC_FurnitureP_Same: File,
    pub BO_FreeC_RoomG: File,
    pub BO_FreeA_Always: File,
    pub BO_FreeG_Visitor: File,
    pub BO_FreeB_ItemN: File,
    pub BO_FreeG_Host: File,
    pub BO_FreeH_Progress: File,
    pub BO_FreeF_EarthDay: File,
    pub BO_FreeB_Furniture_Genre: File,
    pub BO_FreeA_ClothesP: File,
    pub BO_FreeF_Easter: File,
    pub BO_FreeA_Week: File,
    pub BO_FreeF_NewYear_Chinese: File,
    pub BO_FreeA_Want: File,
    pub BO_FreeA_ClothesN: File,
    pub BO_FreeE_Snpc: File,
    pub BO_FreeC_FurnitureP_Genre: File,
    pub BO_FreeA_First01: File,
    pub BO_FreeD_RumorP_Favorite: File,
    pub BO_FreeD_RumorN2: File,
}

impl TalkNNpc_B1_Bo_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Free {
        BO_FreeF_NewYear: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear.msbt").unwrap()),
        BO_FreeA_AlwaysB: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_AlwaysB.msbt").unwrap()),
        BO_FreeF_CatchInsectFes: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_CatchInsectFes.msbt").unwrap()),
        BO_FreeC_RoomH: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_RoomH.msbt").unwrap()),
        BO_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear_Zodiac.msbt").unwrap()),
        BO_FreeB_Weather: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_Weather.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_Weather.msbt").unwrap()),
        BO_FreeF_MayDay: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_MayDay.msbt").unwrap()),
        BO_FreeF_Countdown: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_Countdown.msbt").unwrap()),
        BO_FreeI_Present: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeI_Present.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeI_Present.msbt").unwrap()),
        BO_FreeF_CatchFishFes: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_CatchFishFes.msbt").unwrap()),
        BO_FreeD_RumorOP_Action: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorOP_Action.msbt").unwrap()),
        BO_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureN_Genre.msbt").unwrap()),
        BO_FreeH_Transition: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeH_Transition.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeH_Transition.msbt").unwrap()),
        BO_FreeH_LifeStart_Hint: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeH_LifeStart_Hint.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeH_LifeStart_Hint.msbt").unwrap()),
        BO_FreeB_Furniture_Theme: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_Furniture_Theme.msbt").unwrap()),
        BO_FreeD_Moving: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_Moving.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_Moving.msbt").unwrap()),
        BO_FreeE_Event: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeE_Event.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeE_Event.msbt").unwrap()),
        BO_FreeA_Questions: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_Questions.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_Questions.msbt").unwrap()),
        BO_FreeA_First02: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_First02.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_First02.msbt").unwrap()),
        BO_FreeD_RumorP_Action: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorP_Action.msbt").unwrap()),
        BO_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Theme.msbt").unwrap()),
        BO_FreeB_Spot: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_Spot.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_Spot.msbt").unwrap()),
        BO_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorOP_Favorite.msbt").unwrap()),
        BO_FreeD_Keyword: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_Keyword.msbt").unwrap()),
        BO_FreeA_FirstTent: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_FirstTent.msbt").unwrap()),
        BO_FreeD_RumorN1: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorN1.msbt").unwrap()),
        BO_FreeB_Seasons: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_Seasons.msbt").unwrap()),
        BO_FreeA_AlwaysA: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_AlwaysA.msbt").unwrap()),
        BO_FreeB_ItemP: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_ItemP.msbt").unwrap()),
        BO_FreeC_FurnitureP_Same: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Same.msbt").unwrap()),
        BO_FreeC_RoomG: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_RoomG.msbt").unwrap()),
        BO_FreeA_Always: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_Always.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_Always.msbt").unwrap()),
        BO_FreeG_Visitor: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeG_Visitor.msbt").unwrap()),
        BO_FreeB_ItemN: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_ItemN.msbt").unwrap()),
        BO_FreeG_Host: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeG_Host.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeG_Host.msbt").unwrap()),
        BO_FreeH_Progress: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeH_Progress.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeH_Progress.msbt").unwrap()),
        BO_FreeF_EarthDay: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_EarthDay.msbt").unwrap()),
        BO_FreeB_Furniture_Genre: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeB_Furniture_Genre.msbt").unwrap()),
        BO_FreeA_ClothesP: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_ClothesP.msbt").unwrap()),
        BO_FreeF_Easter: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_Easter.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_Easter.msbt").unwrap()),
        BO_FreeA_Week: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_Week.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_Week.msbt").unwrap()),
        BO_FreeF_NewYear_Chinese: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeF_NewYear_Chinese.msbt").unwrap()),
        BO_FreeA_Want: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_Want.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_Want.msbt").unwrap()),
        BO_FreeA_ClothesN: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_ClothesN.msbt").unwrap()),
        BO_FreeE_Snpc: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeE_Snpc.msbt").unwrap()),
        BO_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeC_FurnitureP_Genre.msbt").unwrap()),
        BO_FreeA_First01: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeA_First01.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeA_First01.msbt").unwrap()),
        BO_FreeD_RumorP_Favorite: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorP_Favorite.msbt").unwrap()),
        BO_FreeD_RumorN2: File::new("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/B1_Bo/Free/BO_FreeD_RumorN2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_React {
    pub BO_React_NewYear: File,
    pub BO_React_EarlyLate: File,
    pub BO_React_Watching_Fish: File,
    pub BO_React_Receive_Easter: File,
    pub BO_React_Sitting: File,
    pub BO_React_Itching: File,
    pub BO_React_Run: File,
    pub BO_React_Quest_TreasureHunt: File,
    pub BO_React_Watching_Insect: File,
    pub BO_React_Quest_Sick: File,
    pub BO_React_MoveOut: File,
    pub BO_React_Sad: File,
    pub BO_React_Worry: File,
    pub BO_React_Fall: File,
    pub BO_React_Beeface: File,
    pub BO_React_Catching: File,
    pub BO_React_Missing30: File,
    pub BO_React_Shopping_Tailor: File,
    pub BO_React_Shopping_Gstore: File,
    pub BO_React_First_Acquaintance: File,
    pub BO_React_GEvent_BirthdayP: File,
    pub BO_React_Anger: File,
    pub BO_React_Talkative: File,
    pub BO_React_MoveIn: File,
    pub BO_React_Happy: File,
    pub BO_React_Napping: File,
    pub BO_React_NewYear_Chinese: File,
    pub BO_React_LookingUp: File,
    pub BO_React_FirstV_Stranger: File,
    pub BO_React_DIY: File,
    pub BO_React_FirstV_Acquaintance: File,
    pub BO_React_Missing7: File,
    pub BO_React_Poison: File,
    pub BO_React_Watching_Fossil: File,
    pub BO_React_Watching_Art: File,
    pub BO_React_GEvent_Quest: File,
    pub BO_React_First_Stranger: File,
    pub BO_React_Fishing: File,
}

impl TalkNNpc_B1_Bo_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_React {
        BO_React_NewYear: File::new("/TalkNNpc/B1_Bo/React/BO_React_NewYear.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_NewYear.msbt").unwrap()),
        BO_React_EarlyLate: File::new("/TalkNNpc/B1_Bo/React/BO_React_EarlyLate.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_EarlyLate.msbt").unwrap()),
        BO_React_Watching_Fish: File::new("/TalkNNpc/B1_Bo/React/BO_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Watching_Fish.msbt").unwrap()),
        BO_React_Receive_Easter: File::new("/TalkNNpc/B1_Bo/React/BO_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Receive_Easter.msbt").unwrap()),
        BO_React_Sitting: File::new("/TalkNNpc/B1_Bo/React/BO_React_Sitting.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Sitting.msbt").unwrap()),
        BO_React_Itching: File::new("/TalkNNpc/B1_Bo/React/BO_React_Itching.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Itching.msbt").unwrap()),
        BO_React_Run: File::new("/TalkNNpc/B1_Bo/React/BO_React_Run.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Run.msbt").unwrap()),
        BO_React_Quest_TreasureHunt: File::new("/TalkNNpc/B1_Bo/React/BO_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Quest_TreasureHunt.msbt").unwrap()),
        BO_React_Watching_Insect: File::new("/TalkNNpc/B1_Bo/React/BO_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Watching_Insect.msbt").unwrap()),
        BO_React_Quest_Sick: File::new("/TalkNNpc/B1_Bo/React/BO_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Quest_Sick.msbt").unwrap()),
        BO_React_MoveOut: File::new("/TalkNNpc/B1_Bo/React/BO_React_MoveOut.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_MoveOut.msbt").unwrap()),
        BO_React_Sad: File::new("/TalkNNpc/B1_Bo/React/BO_React_Sad.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Sad.msbt").unwrap()),
        BO_React_Worry: File::new("/TalkNNpc/B1_Bo/React/BO_React_Worry.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Worry.msbt").unwrap()),
        BO_React_Fall: File::new("/TalkNNpc/B1_Bo/React/BO_React_Fall.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Fall.msbt").unwrap()),
        BO_React_Beeface: File::new("/TalkNNpc/B1_Bo/React/BO_React_Beeface.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Beeface.msbt").unwrap()),
        BO_React_Catching: File::new("/TalkNNpc/B1_Bo/React/BO_React_Catching.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Catching.msbt").unwrap()),
        BO_React_Missing30: File::new("/TalkNNpc/B1_Bo/React/BO_React_Missing30.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Missing30.msbt").unwrap()),
        BO_React_Shopping_Tailor: File::new("/TalkNNpc/B1_Bo/React/BO_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Shopping_Tailor.msbt").unwrap()),
        BO_React_Shopping_Gstore: File::new("/TalkNNpc/B1_Bo/React/BO_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Shopping_Gstore.msbt").unwrap()),
        BO_React_First_Acquaintance: File::new("/TalkNNpc/B1_Bo/React/BO_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_First_Acquaintance.msbt").unwrap()),
        BO_React_GEvent_BirthdayP: File::new("/TalkNNpc/B1_Bo/React/BO_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_GEvent_BirthdayP.msbt").unwrap()),
        BO_React_Anger: File::new("/TalkNNpc/B1_Bo/React/BO_React_Anger.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Anger.msbt").unwrap()),
        BO_React_Talkative: File::new("/TalkNNpc/B1_Bo/React/BO_React_Talkative.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Talkative.msbt").unwrap()),
        BO_React_MoveIn: File::new("/TalkNNpc/B1_Bo/React/BO_React_MoveIn.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_MoveIn.msbt").unwrap()),
        BO_React_Happy: File::new("/TalkNNpc/B1_Bo/React/BO_React_Happy.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Happy.msbt").unwrap()),
        BO_React_Napping: File::new("/TalkNNpc/B1_Bo/React/BO_React_Napping.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Napping.msbt").unwrap()),
        BO_React_NewYear_Chinese: File::new("/TalkNNpc/B1_Bo/React/BO_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_NewYear_Chinese.msbt").unwrap()),
        BO_React_LookingUp: File::new("/TalkNNpc/B1_Bo/React/BO_React_LookingUp.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_LookingUp.msbt").unwrap()),
        BO_React_FirstV_Stranger: File::new("/TalkNNpc/B1_Bo/React/BO_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_FirstV_Stranger.msbt").unwrap()),
        BO_React_DIY: File::new("/TalkNNpc/B1_Bo/React/BO_React_DIY.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_DIY.msbt").unwrap()),
        BO_React_FirstV_Acquaintance: File::new("/TalkNNpc/B1_Bo/React/BO_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_FirstV_Acquaintance.msbt").unwrap()),
        BO_React_Missing7: File::new("/TalkNNpc/B1_Bo/React/BO_React_Missing7.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Missing7.msbt").unwrap()),
        BO_React_Poison: File::new("/TalkNNpc/B1_Bo/React/BO_React_Poison.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Poison.msbt").unwrap()),
        BO_React_Watching_Fossil: File::new("/TalkNNpc/B1_Bo/React/BO_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Watching_Fossil.msbt").unwrap()),
        BO_React_Watching_Art: File::new("/TalkNNpc/B1_Bo/React/BO_React_Watching_Art.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Watching_Art.msbt").unwrap()),
        BO_React_GEvent_Quest: File::new("/TalkNNpc/B1_Bo/React/BO_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_GEvent_Quest.msbt").unwrap()),
        BO_React_First_Stranger: File::new("/TalkNNpc/B1_Bo/React/BO_React_First_Stranger.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_First_Stranger.msbt").unwrap()),
        BO_React_Fishing: File::new("/TalkNNpc/B1_Bo/React/BO_React_Fishing.msbt", msbts.get("/TalkNNpc/B1_Bo/React/BO_React_Fishing.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Conv {
    pub BO_Conv_ZK: File,
    pub BO_Conv_BO: File,
    pub BO_Conv_OT: File,
    pub BO_Conv_FU: File,
    pub BO_Conv_GE: File,
    pub BO_Conv_KO: File,
    pub BO_Conv_AN: File,
    pub BO_Conv_HA: File,
}

impl TalkNNpc_B1_Bo_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Conv {
        BO_Conv_ZK: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_ZK.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_ZK.msbt").unwrap()),
        BO_Conv_BO: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_BO.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_BO.msbt").unwrap()),
        BO_Conv_OT: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_OT.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_OT.msbt").unwrap()),
        BO_Conv_FU: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_FU.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_FU.msbt").unwrap()),
        BO_Conv_GE: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_GE.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_GE.msbt").unwrap()),
        BO_Conv_KO: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_KO.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_KO.msbt").unwrap()),
        BO_Conv_AN: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_AN.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_AN.msbt").unwrap()),
        BO_Conv_HA: File::new("/TalkNNpc/B1_Bo/Conv/BO_Conv_HA.msbt", msbts.get("/TalkNNpc/B1_Bo/Conv/BO_Conv_HA.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_GEvent {
    pub BO_GEvent_BirthdayN_H: File,
    pub BO_GEvent_Countdown: File,
    pub BO_GEvent_Easter: File,
    pub BO_GEvent_BirthdayP: File,
    pub BO_GEvent_BirthdayP_H: File,
    pub BO_GEvent_CatchFishFes: File,
    pub BO_GEvent_BirthdayN_G: File,
    pub BO_GEvent_BirthdayP_G: File,
    pub BO_GEvent_CatchInsectFes: File,
}

impl TalkNNpc_B1_Bo_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_GEvent {
        BO_GEvent_BirthdayN_H: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayN_H.msbt").unwrap()),
        BO_GEvent_Countdown: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_Countdown.msbt").unwrap()),
        BO_GEvent_Easter: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_Easter.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_Easter.msbt").unwrap()),
        BO_GEvent_BirthdayP: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP.msbt").unwrap()),
        BO_GEvent_BirthdayP_H: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP_H.msbt").unwrap()),
        BO_GEvent_CatchFishFes: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_CatchFishFes.msbt").unwrap()),
        BO_GEvent_BirthdayN_G: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayN_G.msbt").unwrap()),
        BO_GEvent_BirthdayP_G: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_BirthdayP_G.msbt").unwrap()),
        BO_GEvent_CatchInsectFes: File::new("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B1_Bo/GEvent/BO_GEvent_CatchInsectFes.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_GreetV {
    pub BO_GreetV_Again1: File,
    pub BO_GreetV_Rain2: File,
    pub BO_GreetV_Snow2: File,
    pub BO_GreetV_Field1: File,
    pub BO_GreetV_Again2: File,
    pub BO_GreetV_Fine2: File,
}

impl TalkNNpc_B1_Bo_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_GreetV {
        BO_GreetV_Again1: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Again1.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Again1.msbt").unwrap()),
        BO_GreetV_Rain2: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Rain2.msbt").unwrap()),
        BO_GreetV_Snow2: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Snow2.msbt").unwrap()),
        BO_GreetV_Field1: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Field1.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Field1.msbt").unwrap()),
        BO_GreetV_Again2: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Again2.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Again2.msbt").unwrap()),
        BO_GreetV_Fine2: File::new("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/B1_Bo/GreetV/BO_GreetV_Fine2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Select {
    pub BO_Select_General: File,
    pub BO_Select_Present: File,
}

impl TalkNNpc_B1_Bo_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Select {
        BO_Select_General: File::new("/TalkNNpc/B1_Bo/Select/BO_Select_General.msbt", msbts.get("/TalkNNpc/B1_Bo/Select/BO_Select_General.msbt").unwrap()),
        BO_Select_Present: File::new("/TalkNNpc/B1_Bo/Select/BO_Select_Present.msbt", msbts.get("/TalkNNpc/B1_Bo/Select/BO_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Approach {
    pub BO_ApproachB_Greeting: File,
    pub BO_ApproachB_NickName: File,
    pub BO_ApproachC_Sell: File,
    pub BO_ApproachC_Trade: File,
    pub BO_ApproachF_First: File,
    pub BO_ApproachE_Easter: File,
    pub BO_ApproachD_Moving: File,
    pub BO_ApproachD_Stay: File,
    pub BO_ApproachC_Want: File,
    pub BO_ApproachE_MainSeq: File,
    pub BO_ApproachA_Always: File,
    pub BO_ApproachC_Present: File,
    pub BO_ApproachA_Emoticons: File,
    pub BO_ApproachB_Habit: File,
}

impl TalkNNpc_B1_Bo_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Approach {
        BO_ApproachB_Greeting: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_Greeting.msbt").unwrap()),
        BO_ApproachB_NickName: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_NickName.msbt").unwrap()),
        BO_ApproachC_Sell: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Sell.msbt").unwrap()),
        BO_ApproachC_Trade: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Trade.msbt").unwrap()),
        BO_ApproachF_First: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachF_First.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachF_First.msbt").unwrap()),
        BO_ApproachE_Easter: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachE_Easter.msbt").unwrap()),
        BO_ApproachD_Moving: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachD_Moving.msbt").unwrap()),
        BO_ApproachD_Stay: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachD_Stay.msbt").unwrap()),
        BO_ApproachC_Want: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Want.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Want.msbt").unwrap()),
        BO_ApproachE_MainSeq: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachE_MainSeq.msbt").unwrap()),
        BO_ApproachA_Always: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachA_Always.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachA_Always.msbt").unwrap()),
        BO_ApproachC_Present: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Present.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachC_Present.msbt").unwrap()),
        BO_ApproachA_Emoticons: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachA_Emoticons.msbt").unwrap()),
        BO_ApproachB_Habit: File::new("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/B1_Bo/Approach/BO_ApproachB_Habit.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Force {
    pub BO_Force_Hit: File,
    pub BO_Force_Flea: File,
    pub BO_Force_Push: File,
}

impl TalkNNpc_B1_Bo_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Force {
        BO_Force_Hit: File::new("/TalkNNpc/B1_Bo/Force/BO_Force_Hit.msbt", msbts.get("/TalkNNpc/B1_Bo/Force/BO_Force_Hit.msbt").unwrap()),
        BO_Force_Flea: File::new("/TalkNNpc/B1_Bo/Force/BO_Force_Flea.msbt", msbts.get("/TalkNNpc/B1_Bo/Force/BO_Force_Flea.msbt").unwrap()),
        BO_Force_Push: File::new("/TalkNNpc/B1_Bo/Force/BO_Force_Push.msbt", msbts.get("/TalkNNpc/B1_Bo/Force/BO_Force_Push.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Quest {
    pub BO_Quest_TreasureHunt_Begin: File,
    pub BO_Quest_LostProperty_End: File,
    pub BO_Quest_CatchFishInsect_Begin: File,
    pub BO_Quest_Delivery_Begin: File,
    pub BO_Quest_Sick_Cured: File,
    pub BO_Quest_Delivery_After: File,
    pub BO_Quest_Delivery_End: File,
    pub BO_Quest_TreasureHunt_Talk: File,
    pub BO_Quest_LostProperty_Begin: File,
    pub BO_Quest_Delivery_Give: File,
    pub BO_Quest_Delivery_Cloth: File,
    pub BO_Quest_Sick_End: File,
    pub BO_Quest_CatchFishInsect_End: File,
    pub BO_Quest_Sick_Begin: File,
    pub BO_Quest_TreasureHunt_End: File,
}

impl TalkNNpc_B1_Bo_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Quest {
        BO_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_Begin.msbt").unwrap()),
        BO_Quest_LostProperty_End: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_LostProperty_End.msbt").unwrap()),
        BO_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        BO_Quest_Delivery_Begin: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Begin.msbt").unwrap()),
        BO_Quest_Sick_Cured: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_Cured.msbt").unwrap()),
        BO_Quest_Delivery_After: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_After.msbt").unwrap()),
        BO_Quest_Delivery_End: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_End.msbt").unwrap()),
        BO_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_Talk.msbt").unwrap()),
        BO_Quest_LostProperty_Begin: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_LostProperty_Begin.msbt").unwrap()),
        BO_Quest_Delivery_Give: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Give.msbt").unwrap()),
        BO_Quest_Delivery_Cloth: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Delivery_Cloth.msbt").unwrap()),
        BO_Quest_Sick_End: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_End.msbt").unwrap()),
        BO_Quest_CatchFishInsect_End: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_CatchFishInsect_End.msbt").unwrap()),
        BO_Quest_Sick_Begin: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_Sick_Begin.msbt").unwrap()),
        BO_Quest_TreasureHunt_End: File::new("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/B1_Bo/Quest/BO_Quest_TreasureHunt_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo_Spot {
    pub BO_Spot_Museum_Fish: File,
    pub BO_Spot_MysteryTour: File,
    pub BO_Spot_Camp_Quest: File,
    pub BO_Spot_Camp_Invite: File,
    pub BO_Spot_Camp_Game: File,
    pub BO_Spot_Museum_Fossil: File,
    pub BO_Spot_Office: File,
    pub BO_Spot_Museum_Art: File,
    pub BO_Spot_Tailor: File,
    pub BO_Spot_Gstore: File,
    pub BO_Spot_Camp: File,
    pub BO_Spot_Museum_Insect: File,
    pub BO_Spot_Camp_Amiibo: File,
}

impl TalkNNpc_B1_Bo_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo_Spot {
        BO_Spot_Museum_Fish: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Fish.msbt").unwrap()),
        BO_Spot_MysteryTour: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_MysteryTour.msbt").unwrap()),
        BO_Spot_Camp_Quest: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Quest.msbt").unwrap()),
        BO_Spot_Camp_Invite: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Invite.msbt").unwrap()),
        BO_Spot_Camp_Game: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Game.msbt").unwrap()),
        BO_Spot_Museum_Fossil: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Fossil.msbt").unwrap()),
        BO_Spot_Office: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Office.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Office.msbt").unwrap()),
        BO_Spot_Museum_Art: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Art.msbt").unwrap()),
        BO_Spot_Tailor: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Tailor.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Tailor.msbt").unwrap()),
        BO_Spot_Gstore: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Gstore.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Gstore.msbt").unwrap()),
        BO_Spot_Camp: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp.msbt").unwrap()),
        BO_Spot_Museum_Insect: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Museum_Insect.msbt").unwrap()),
        BO_Spot_Camp_Amiibo: File::new("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/B1_Bo/Spot/BO_Spot_Camp_Amiibo.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B1_Bo {
    pub Greet: TalkNNpc_B1_Bo_Greet,
    pub Free: TalkNNpc_B1_Bo_Free,
    pub React: TalkNNpc_B1_Bo_React,
    pub Conv: TalkNNpc_B1_Bo_Conv,
    pub GEvent: TalkNNpc_B1_Bo_GEvent,
    pub GreetV: TalkNNpc_B1_Bo_GreetV,
    pub Select: TalkNNpc_B1_Bo_Select,
    pub Approach: TalkNNpc_B1_Bo_Approach,
    pub Force: TalkNNpc_B1_Bo_Force,
    pub Quest: TalkNNpc_B1_Bo_Quest,
    pub Spot: TalkNNpc_B1_Bo_Spot,
    pub BO_Connect_StandingUp: File,
    pub BO_End: File,
}

impl TalkNNpc_B1_Bo {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B1_Bo {
        Greet: TalkNNpc_B1_Bo_Greet::new(msbts),
        Free: TalkNNpc_B1_Bo_Free::new(msbts),
        React: TalkNNpc_B1_Bo_React::new(msbts),
        Conv: TalkNNpc_B1_Bo_Conv::new(msbts),
        GEvent: TalkNNpc_B1_Bo_GEvent::new(msbts),
        GreetV: TalkNNpc_B1_Bo_GreetV::new(msbts),
        Select: TalkNNpc_B1_Bo_Select::new(msbts),
        Approach: TalkNNpc_B1_Bo_Approach::new(msbts),
        Force: TalkNNpc_B1_Bo_Force::new(msbts),
        Quest: TalkNNpc_B1_Bo_Quest::new(msbts),
        Spot: TalkNNpc_B1_Bo_Spot::new(msbts),
        BO_Connect_StandingUp: File::new("/TalkNNpc/B1_Bo/BO_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/B1_Bo/BO_Connect_StandingUp.msbt").unwrap()),
        BO_End: File::new("/TalkNNpc/B1_Bo/BO_End.msbt", msbts.get("/TalkNNpc/B1_Bo/BO_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Spot {
    pub HA_Spot_Camp_Invite: File,
    pub HA_Spot_Museum_Art: File,
    pub HA_Spot_Camp: File,
    pub HA_Spot_Museum_Fossil: File,
    pub HA_Spot_Camp_Game: File,
    pub HA_Spot_Camp_Amiibo: File,
    pub HA_Spot_Museum_Fish: File,
    pub HA_Spot_Camp_Quest: File,
    pub HA_Spot_Office: File,
    pub HA_Spot_Tailor: File,
    pub HA_Spot_Gstore: File,
    pub HA_Spot_Museum_Insect: File,
    pub HA_Spot_MysteryTour: File,
}

impl TalkNNpc_B2_Ha_Spot {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Spot {
        HA_Spot_Camp_Invite: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Invite.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Invite.msbt").unwrap()),
        HA_Spot_Museum_Art: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Art.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Art.msbt").unwrap()),
        HA_Spot_Camp: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp.msbt").unwrap()),
        HA_Spot_Museum_Fossil: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Fossil.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Fossil.msbt").unwrap()),
        HA_Spot_Camp_Game: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Game.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Game.msbt").unwrap()),
        HA_Spot_Camp_Amiibo: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Amiibo.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Amiibo.msbt").unwrap()),
        HA_Spot_Museum_Fish: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Fish.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Fish.msbt").unwrap()),
        HA_Spot_Camp_Quest: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Quest.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Camp_Quest.msbt").unwrap()),
        HA_Spot_Office: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Office.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Office.msbt").unwrap()),
        HA_Spot_Tailor: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Tailor.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Tailor.msbt").unwrap()),
        HA_Spot_Gstore: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Gstore.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Gstore.msbt").unwrap()),
        HA_Spot_Museum_Insect: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Insect.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_Museum_Insect.msbt").unwrap()),
        HA_Spot_MysteryTour: File::new("/TalkNNpc/B2_Ha/Spot/HA_Spot_MysteryTour.msbt", msbts.get("/TalkNNpc/B2_Ha/Spot/HA_Spot_MysteryTour.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Greet {
    pub HA_Greet_Snow2: File,
    pub HA_Greet_House2: File,
    pub HA_Greet_House_G: File,
    pub HA_Greet_Again2: File,
    pub HA_Greet_Again_LifeStart: File,
    pub HA_Greet_Fine2: File,
    pub HA_Greet_Spot: File,
    pub HA_Greet_House3: File,
    pub HA_Greet_House_H: File,
    pub HA_Greet_House1: File,
    pub HA_Greet_Field3: File,
    pub HA_Greet_Again1: File,
    pub HA_Greet_Rain2: File,
    pub HA_Greet_Field1: File,
}

impl TalkNNpc_B2_Ha_Greet {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Greet {
        HA_Greet_Snow2: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Snow2.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Snow2.msbt").unwrap()),
        HA_Greet_House2: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_House2.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_House2.msbt").unwrap()),
        HA_Greet_House_G: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_House_G.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_House_G.msbt").unwrap()),
        HA_Greet_Again2: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again2.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again2.msbt").unwrap()),
        HA_Greet_Again_LifeStart: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again_LifeStart.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again_LifeStart.msbt").unwrap()),
        HA_Greet_Fine2: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Fine2.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Fine2.msbt").unwrap()),
        HA_Greet_Spot: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Spot.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Spot.msbt").unwrap()),
        HA_Greet_House3: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_House3.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_House3.msbt").unwrap()),
        HA_Greet_House_H: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_House_H.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_House_H.msbt").unwrap()),
        HA_Greet_House1: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_House1.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_House1.msbt").unwrap()),
        HA_Greet_Field3: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Field3.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Field3.msbt").unwrap()),
        HA_Greet_Again1: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again1.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Again1.msbt").unwrap()),
        HA_Greet_Rain2: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Rain2.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Rain2.msbt").unwrap()),
        HA_Greet_Field1: File::new("/TalkNNpc/B2_Ha/Greet/HA_Greet_Field1.msbt", msbts.get("/TalkNNpc/B2_Ha/Greet/HA_Greet_Field1.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_GreetV {
    pub HA_GreetV_Fine2: File,
    pub HA_GreetV_Again1: File,
    pub HA_GreetV_Field1: File,
    pub HA_GreetV_Rain2: File,
    pub HA_GreetV_Snow2: File,
    pub HA_GreetV_Again2: File,
}

impl TalkNNpc_B2_Ha_GreetV {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_GreetV {
        HA_GreetV_Fine2: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Fine2.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Fine2.msbt").unwrap()),
        HA_GreetV_Again1: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Again1.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Again1.msbt").unwrap()),
        HA_GreetV_Field1: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Field1.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Field1.msbt").unwrap()),
        HA_GreetV_Rain2: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Rain2.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Rain2.msbt").unwrap()),
        HA_GreetV_Snow2: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Snow2.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Snow2.msbt").unwrap()),
        HA_GreetV_Again2: File::new("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Again2.msbt", msbts.get("/TalkNNpc/B2_Ha/GreetV/HA_GreetV_Again2.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_React {
    pub HA_React_Watching_Fossil: File,
    pub HA_React_Receive_Easter: File,
    pub HA_React_Talkative: File,
    pub HA_React_Poison: File,
    pub HA_React_DIY: File,
    pub HA_React_LookingUp: File,
    pub HA_React_Watching_Fish: File,
    pub HA_React_1P_LifeStart: File,
    pub HA_React_Anger: File,
    pub HA_React_Fishing: File,
    pub HA_React_Catching: File,
    pub HA_React_Happy: File,
    pub HA_React_GEvent_BirthdayP: File,
    pub HA_React_NewYear: File,
    pub HA_React_Watching_Insect: File,
    pub HA_React_Sitting: File,
    pub HA_React_Itching: File,
    pub HA_React_Shopping_Tailor: File,
    pub HA_React_MoveOut: File,
    pub HA_React_Shopping_Gstore: File,
    pub HA_React_Beeface: File,
    pub HA_React_Quest_TreasureHunt: File,
    pub HA_React_Run: File,
    pub HA_React_Watching_Art: File,
    pub HA_React_Missing7: File,
    pub HA_React_GEvent_Quest: File,
    pub HA_React_EarlyLate: File,
    pub HA_React_Sad: File,
    pub HA_React_Fall: File,
    pub HA_React_Quest_Sick: File,
    pub HA_React_First_Stranger: File,
    pub HA_React_NewYear_Chinese: File,
    pub HA_React_First_Acquaintance: File,
    pub HA_React_FirstV_Acquaintance: File,
    pub HA_React_MoveIn: File,
    pub HA_React_Napping: File,
    pub HA_React_Missing30: File,
    pub HA_React_FirstV_Stranger: File,
    pub HA_React_Worry: File,
}

impl TalkNNpc_B2_Ha_React {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_React {
        HA_React_Watching_Fossil: File::new("/TalkNNpc/B2_Ha/React/HA_React_Watching_Fossil.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Watching_Fossil.msbt").unwrap()),
        HA_React_Receive_Easter: File::new("/TalkNNpc/B2_Ha/React/HA_React_Receive_Easter.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Receive_Easter.msbt").unwrap()),
        HA_React_Talkative: File::new("/TalkNNpc/B2_Ha/React/HA_React_Talkative.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Talkative.msbt").unwrap()),
        HA_React_Poison: File::new("/TalkNNpc/B2_Ha/React/HA_React_Poison.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Poison.msbt").unwrap()),
        HA_React_DIY: File::new("/TalkNNpc/B2_Ha/React/HA_React_DIY.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_DIY.msbt").unwrap()),
        HA_React_LookingUp: File::new("/TalkNNpc/B2_Ha/React/HA_React_LookingUp.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_LookingUp.msbt").unwrap()),
        HA_React_Watching_Fish: File::new("/TalkNNpc/B2_Ha/React/HA_React_Watching_Fish.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Watching_Fish.msbt").unwrap()),
        HA_React_1P_LifeStart: File::new("/TalkNNpc/B2_Ha/React/HA_React_1P_LifeStart.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_1P_LifeStart.msbt").unwrap()),
        HA_React_Anger: File::new("/TalkNNpc/B2_Ha/React/HA_React_Anger.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Anger.msbt").unwrap()),
        HA_React_Fishing: File::new("/TalkNNpc/B2_Ha/React/HA_React_Fishing.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Fishing.msbt").unwrap()),
        HA_React_Catching: File::new("/TalkNNpc/B2_Ha/React/HA_React_Catching.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Catching.msbt").unwrap()),
        HA_React_Happy: File::new("/TalkNNpc/B2_Ha/React/HA_React_Happy.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Happy.msbt").unwrap()),
        HA_React_GEvent_BirthdayP: File::new("/TalkNNpc/B2_Ha/React/HA_React_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_GEvent_BirthdayP.msbt").unwrap()),
        HA_React_NewYear: File::new("/TalkNNpc/B2_Ha/React/HA_React_NewYear.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_NewYear.msbt").unwrap()),
        HA_React_Watching_Insect: File::new("/TalkNNpc/B2_Ha/React/HA_React_Watching_Insect.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Watching_Insect.msbt").unwrap()),
        HA_React_Sitting: File::new("/TalkNNpc/B2_Ha/React/HA_React_Sitting.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Sitting.msbt").unwrap()),
        HA_React_Itching: File::new("/TalkNNpc/B2_Ha/React/HA_React_Itching.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Itching.msbt").unwrap()),
        HA_React_Shopping_Tailor: File::new("/TalkNNpc/B2_Ha/React/HA_React_Shopping_Tailor.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Shopping_Tailor.msbt").unwrap()),
        HA_React_MoveOut: File::new("/TalkNNpc/B2_Ha/React/HA_React_MoveOut.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_MoveOut.msbt").unwrap()),
        HA_React_Shopping_Gstore: File::new("/TalkNNpc/B2_Ha/React/HA_React_Shopping_Gstore.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Shopping_Gstore.msbt").unwrap()),
        HA_React_Beeface: File::new("/TalkNNpc/B2_Ha/React/HA_React_Beeface.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Beeface.msbt").unwrap()),
        HA_React_Quest_TreasureHunt: File::new("/TalkNNpc/B2_Ha/React/HA_React_Quest_TreasureHunt.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Quest_TreasureHunt.msbt").unwrap()),
        HA_React_Run: File::new("/TalkNNpc/B2_Ha/React/HA_React_Run.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Run.msbt").unwrap()),
        HA_React_Watching_Art: File::new("/TalkNNpc/B2_Ha/React/HA_React_Watching_Art.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Watching_Art.msbt").unwrap()),
        HA_React_Missing7: File::new("/TalkNNpc/B2_Ha/React/HA_React_Missing7.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Missing7.msbt").unwrap()),
        HA_React_GEvent_Quest: File::new("/TalkNNpc/B2_Ha/React/HA_React_GEvent_Quest.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_GEvent_Quest.msbt").unwrap()),
        HA_React_EarlyLate: File::new("/TalkNNpc/B2_Ha/React/HA_React_EarlyLate.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_EarlyLate.msbt").unwrap()),
        HA_React_Sad: File::new("/TalkNNpc/B2_Ha/React/HA_React_Sad.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Sad.msbt").unwrap()),
        HA_React_Fall: File::new("/TalkNNpc/B2_Ha/React/HA_React_Fall.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Fall.msbt").unwrap()),
        HA_React_Quest_Sick: File::new("/TalkNNpc/B2_Ha/React/HA_React_Quest_Sick.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Quest_Sick.msbt").unwrap()),
        HA_React_First_Stranger: File::new("/TalkNNpc/B2_Ha/React/HA_React_First_Stranger.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_First_Stranger.msbt").unwrap()),
        HA_React_NewYear_Chinese: File::new("/TalkNNpc/B2_Ha/React/HA_React_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_NewYear_Chinese.msbt").unwrap()),
        HA_React_First_Acquaintance: File::new("/TalkNNpc/B2_Ha/React/HA_React_First_Acquaintance.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_First_Acquaintance.msbt").unwrap()),
        HA_React_FirstV_Acquaintance: File::new("/TalkNNpc/B2_Ha/React/HA_React_FirstV_Acquaintance.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_FirstV_Acquaintance.msbt").unwrap()),
        HA_React_MoveIn: File::new("/TalkNNpc/B2_Ha/React/HA_React_MoveIn.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_MoveIn.msbt").unwrap()),
        HA_React_Napping: File::new("/TalkNNpc/B2_Ha/React/HA_React_Napping.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Napping.msbt").unwrap()),
        HA_React_Missing30: File::new("/TalkNNpc/B2_Ha/React/HA_React_Missing30.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Missing30.msbt").unwrap()),
        HA_React_FirstV_Stranger: File::new("/TalkNNpc/B2_Ha/React/HA_React_FirstV_Stranger.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_FirstV_Stranger.msbt").unwrap()),
        HA_React_Worry: File::new("/TalkNNpc/B2_Ha/React/HA_React_Worry.msbt", msbts.get("/TalkNNpc/B2_Ha/React/HA_React_Worry.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Quest {
    pub HA_Quest_Delivery_Begin: File,
    pub HA_Quest_CatchFishInsect_End: File,
    pub HA_Quest_TreasureHunt_Talk: File,
    pub HA_Quest_Delivery_End: File,
    pub HA_Quest_Delivery_After: File,
    pub HA_Quest_TreasureHunt_Begin: File,
    pub HA_Quest_Sick_Begin: File,
    pub HA_Quest_Delivery_Cloth: File,
    pub HA_Quest_Delivery_Give: File,
    pub HA_Quest_Sick_Cured: File,
    pub HA_Quest_LostProperty_Begin: File,
    pub HA_Quest_TreasureHunt_End: File,
    pub HA_Quest_Sick_End: File,
    pub HA_Quest_LostProperty_End: File,
    pub HA_Quest_CatchFishInsect_Begin: File,
}

impl TalkNNpc_B2_Ha_Quest {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Quest {
        HA_Quest_Delivery_Begin: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Begin.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Begin.msbt").unwrap()),
        HA_Quest_CatchFishInsect_End: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_CatchFishInsect_End.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_CatchFishInsect_End.msbt").unwrap()),
        HA_Quest_TreasureHunt_Talk: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_Talk.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_Talk.msbt").unwrap()),
        HA_Quest_Delivery_End: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_End.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_End.msbt").unwrap()),
        HA_Quest_Delivery_After: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_After.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_After.msbt").unwrap()),
        HA_Quest_TreasureHunt_Begin: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_Begin.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_Begin.msbt").unwrap()),
        HA_Quest_Sick_Begin: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_Begin.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_Begin.msbt").unwrap()),
        HA_Quest_Delivery_Cloth: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Cloth.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Cloth.msbt").unwrap()),
        HA_Quest_Delivery_Give: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Give.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Delivery_Give.msbt").unwrap()),
        HA_Quest_Sick_Cured: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_Cured.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_Cured.msbt").unwrap()),
        HA_Quest_LostProperty_Begin: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_LostProperty_Begin.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_LostProperty_Begin.msbt").unwrap()),
        HA_Quest_TreasureHunt_End: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_End.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_TreasureHunt_End.msbt").unwrap()),
        HA_Quest_Sick_End: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_End.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_Sick_End.msbt").unwrap()),
        HA_Quest_LostProperty_End: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_LostProperty_End.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_LostProperty_End.msbt").unwrap()),
        HA_Quest_CatchFishInsect_Begin: File::new("/TalkNNpc/B2_Ha/Quest/HA_Quest_CatchFishInsect_Begin.msbt", msbts.get("/TalkNNpc/B2_Ha/Quest/HA_Quest_CatchFishInsect_Begin.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Force {
    pub HA_Force_Flea: File,
    pub HA_Force_Push: File,
    pub HA_Force_Hit: File,
}

impl TalkNNpc_B2_Ha_Force {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Force {
        HA_Force_Flea: File::new("/TalkNNpc/B2_Ha/Force/HA_Force_Flea.msbt", msbts.get("/TalkNNpc/B2_Ha/Force/HA_Force_Flea.msbt").unwrap()),
        HA_Force_Push: File::new("/TalkNNpc/B2_Ha/Force/HA_Force_Push.msbt", msbts.get("/TalkNNpc/B2_Ha/Force/HA_Force_Push.msbt").unwrap()),
        HA_Force_Hit: File::new("/TalkNNpc/B2_Ha/Force/HA_Force_Hit.msbt", msbts.get("/TalkNNpc/B2_Ha/Force/HA_Force_Hit.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Conv {
    pub HA_Conv_BO: File,
    pub HA_Conv_OT: File,
    pub HA_Conv_FU: File,
    pub HA_Conv_GE: File,
    pub HA_Conv_KO: File,
    pub HA_Conv_AN: File,
    pub HA_Conv_HA: File,
    pub HA_Conv_ZK: File,
}

impl TalkNNpc_B2_Ha_Conv {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Conv {
        HA_Conv_BO: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_BO.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_BO.msbt").unwrap()),
        HA_Conv_OT: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_OT.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_OT.msbt").unwrap()),
        HA_Conv_FU: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_FU.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_FU.msbt").unwrap()),
        HA_Conv_GE: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_GE.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_GE.msbt").unwrap()),
        HA_Conv_KO: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_KO.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_KO.msbt").unwrap()),
        HA_Conv_AN: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_AN.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_AN.msbt").unwrap()),
        HA_Conv_HA: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_HA.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_HA.msbt").unwrap()),
        HA_Conv_ZK: File::new("/TalkNNpc/B2_Ha/Conv/HA_Conv_ZK.msbt", msbts.get("/TalkNNpc/B2_Ha/Conv/HA_Conv_ZK.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_GEvent {
    pub HA_GEvent_BirthdayN_H: File,
    pub HA_GEvent_BirthdayP_H: File,
    pub HA_GEvent_Easter: File,
    pub HA_GEvent_Countdown: File,
    pub HA_GEvent_BirthdayP: File,
    pub HA_GEvent_BirthdayN_G: File,
    pub HA_GEvent_CatchInsectFes: File,
    pub HA_GEvent_CatchFishFes: File,
    pub HA_GEvent_BirthdayP_G: File,
}

impl TalkNNpc_B2_Ha_GEvent {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_GEvent {
        HA_GEvent_BirthdayN_H: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayN_H.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayN_H.msbt").unwrap()),
        HA_GEvent_BirthdayP_H: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP_H.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP_H.msbt").unwrap()),
        HA_GEvent_Easter: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_Easter.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_Easter.msbt").unwrap()),
        HA_GEvent_Countdown: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_Countdown.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_Countdown.msbt").unwrap()),
        HA_GEvent_BirthdayP: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP.msbt").unwrap()),
        HA_GEvent_BirthdayN_G: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayN_G.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayN_G.msbt").unwrap()),
        HA_GEvent_CatchInsectFes: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_CatchInsectFes.msbt").unwrap()),
        HA_GEvent_CatchFishFes: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_CatchFishFes.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_CatchFishFes.msbt").unwrap()),
        HA_GEvent_BirthdayP_G: File::new("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP_G.msbt", msbts.get("/TalkNNpc/B2_Ha/GEvent/HA_GEvent_BirthdayP_G.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Approach {
    pub HA_ApproachC_Present: File,
    pub HA_ApproachC_Sell: File,
    pub HA_ApproachB_Greeting: File,
    pub HA_ApproachB_Habit: File,
    pub HA_ApproachE_Easter: File,
    pub HA_ApproachD_Moving: File,
    pub HA_ApproachB_NickName: File,
    pub HA_ApproachC_Trade: File,
    pub HA_ApproachE_WelcomeMigrants: File,
    pub HA_ApproachD_Stay: File,
    pub HA_ApproachC_Want: File,
    pub HA_ApproachA_Emoticons: File,
    pub HA_ApproachF_First: File,
    pub HA_ApproachA_Always: File,
    pub HA_ApproachE_MainSeq: File,
}

impl TalkNNpc_B2_Ha_Approach {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Approach {
        HA_ApproachC_Present: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Present.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Present.msbt").unwrap()),
        HA_ApproachC_Sell: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Sell.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Sell.msbt").unwrap()),
        HA_ApproachB_Greeting: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_Greeting.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_Greeting.msbt").unwrap()),
        HA_ApproachB_Habit: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_Habit.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_Habit.msbt").unwrap()),
        HA_ApproachE_Easter: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_Easter.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_Easter.msbt").unwrap()),
        HA_ApproachD_Moving: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachD_Moving.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachD_Moving.msbt").unwrap()),
        HA_ApproachB_NickName: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_NickName.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachB_NickName.msbt").unwrap()),
        HA_ApproachC_Trade: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Trade.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Trade.msbt").unwrap()),
        HA_ApproachE_WelcomeMigrants: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_WelcomeMigrants.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_WelcomeMigrants.msbt").unwrap()),
        HA_ApproachD_Stay: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachD_Stay.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachD_Stay.msbt").unwrap()),
        HA_ApproachC_Want: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Want.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachC_Want.msbt").unwrap()),
        HA_ApproachA_Emoticons: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachA_Emoticons.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachA_Emoticons.msbt").unwrap()),
        HA_ApproachF_First: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachF_First.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachF_First.msbt").unwrap()),
        HA_ApproachA_Always: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachA_Always.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachA_Always.msbt").unwrap()),
        HA_ApproachE_MainSeq: File::new("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_MainSeq.msbt", msbts.get("/TalkNNpc/B2_Ha/Approach/HA_ApproachE_MainSeq.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Select {
    pub HA_Select_General: File,
    pub HA_Select_Present: File,
}

impl TalkNNpc_B2_Ha_Select {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Select {
        HA_Select_General: File::new("/TalkNNpc/B2_Ha/Select/HA_Select_General.msbt", msbts.get("/TalkNNpc/B2_Ha/Select/HA_Select_General.msbt").unwrap()),
        HA_Select_Present: File::new("/TalkNNpc/B2_Ha/Select/HA_Select_Present.msbt", msbts.get("/TalkNNpc/B2_Ha/Select/HA_Select_Present.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha_Free {
    pub HA_FreeA_FirstTent: File,
    pub HA_FreeD_RumorP_Action: File,
    pub HA_FreeC_FurnitureP_Genre: File,
    pub HA_FreeC_RoomH: File,
    pub HA_FreeB_Furniture_Theme: File,
    pub HA_FreeA_First01: File,
    pub HA_FreeF_CatchFishFes: File,
    pub HA_FreeD_RumorN2: File,
    pub HA_FreeF_NewYear: File,
    pub HA_FreeA_AlwaysB: File,
    pub HA_FreeA_Always: File,
    pub HA_FreeB_Weather: File,
    pub HA_FreeB_Spot: File,
    pub HA_FreeC_FurnitureN_Genre: File,
    pub HA_FreeE_Event: File,
    pub HA_FreeF_CatchInsectFes: File,
    pub HA_FreeI_Present: File,
    pub HA_FreeF_NewYear_Zodiac: File,
    pub HA_FreeC_FurnitureP_Same: File,
    pub HA_FreeF_Easter: File,
    pub HA_FreeH_ForMarketBuilding: File,
    pub HA_FreeC_FurnitureP_Theme: File,
    pub HA_FreeD_RumorOP_Favorite: File,
    pub HA_FreeD_RumorOP_Action: File,
    pub HA_FreeA_First02: File,
    pub HA_FreeB_ItemP: File,
    pub HA_FreeB_Furniture_Genre: File,
    pub HA_FreeH_LifeStart_Hint: File,
    pub HA_FreeF_Countdown: File,
    pub HA_FreeC_RoomG: File,
    pub HA_FreeD_Keyword: File,
    pub HA_FreeG_Host: File,
    pub HA_FreeB_ItemN: File,
    pub HA_FreeF_NewYear_Chinese: File,
    pub HA_FreeH_Progress: File,
    pub HA_FreeD_RumorN1: File,
    pub HA_FreeF_EarthDay: File,
    pub HA_FreeB_Seasons: File,
    pub HA_FreeA_AlwaysA: File,
    pub HA_FreeA_Questions: File,
    pub HA_FreeA_ClothesP: File,
    pub HA_FreeA_Week: File,
    pub HA_FreeD_RumorP_Favorite: File,
    pub HA_FreeF_MayDay: File,
    pub HA_FreeA_Want: File,
    pub HA_FreeG_Visitor: File,
    pub HA_FreeE_Snpc: File,
    pub HA_FreeA_ClothesN: File,
    pub HA_FreeH_Transition: File,
    pub HA_FreeD_Moving: File,
}

impl TalkNNpc_B2_Ha_Free {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha_Free {
        HA_FreeA_FirstTent: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_FirstTent.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_FirstTent.msbt").unwrap()),
        HA_FreeD_RumorP_Action: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorP_Action.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorP_Action.msbt").unwrap()),
        HA_FreeC_FurnitureP_Genre: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Genre.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Genre.msbt").unwrap()),
        HA_FreeC_RoomH: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_RoomH.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_RoomH.msbt").unwrap()),
        HA_FreeB_Furniture_Theme: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_Furniture_Theme.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_Furniture_Theme.msbt").unwrap()),
        HA_FreeA_First01: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_First01.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_First01.msbt").unwrap()),
        HA_FreeF_CatchFishFes: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_CatchFishFes.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_CatchFishFes.msbt").unwrap()),
        HA_FreeD_RumorN2: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorN2.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorN2.msbt").unwrap()),
        HA_FreeF_NewYear: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear.msbt").unwrap()),
        HA_FreeA_AlwaysB: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_AlwaysB.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_AlwaysB.msbt").unwrap()),
        HA_FreeA_Always: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_Always.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_Always.msbt").unwrap()),
        HA_FreeB_Weather: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_Weather.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_Weather.msbt").unwrap()),
        HA_FreeB_Spot: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_Spot.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_Spot.msbt").unwrap()),
        HA_FreeC_FurnitureN_Genre: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureN_Genre.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureN_Genre.msbt").unwrap()),
        HA_FreeE_Event: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeE_Event.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeE_Event.msbt").unwrap()),
        HA_FreeF_CatchInsectFes: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_CatchInsectFes.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_CatchInsectFes.msbt").unwrap()),
        HA_FreeI_Present: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeI_Present.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeI_Present.msbt").unwrap()),
        HA_FreeF_NewYear_Zodiac: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear_Zodiac.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear_Zodiac.msbt").unwrap()),
        HA_FreeC_FurnitureP_Same: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Same.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Same.msbt").unwrap()),
        HA_FreeF_Easter: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_Easter.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_Easter.msbt").unwrap()),
        HA_FreeH_ForMarketBuilding: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeH_ForMarketBuilding.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeH_ForMarketBuilding.msbt").unwrap()),
        HA_FreeC_FurnitureP_Theme: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Theme.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_FurnitureP_Theme.msbt").unwrap()),
        HA_FreeD_RumorOP_Favorite: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorOP_Favorite.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorOP_Favorite.msbt").unwrap()),
        HA_FreeD_RumorOP_Action: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorOP_Action.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorOP_Action.msbt").unwrap()),
        HA_FreeA_First02: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_First02.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_First02.msbt").unwrap()),
        HA_FreeB_ItemP: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_ItemP.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_ItemP.msbt").unwrap()),
        HA_FreeB_Furniture_Genre: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_Furniture_Genre.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_Furniture_Genre.msbt").unwrap()),
        HA_FreeH_LifeStart_Hint: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeH_LifeStart_Hint.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeH_LifeStart_Hint.msbt").unwrap()),
        HA_FreeF_Countdown: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_Countdown.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_Countdown.msbt").unwrap()),
        HA_FreeC_RoomG: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeC_RoomG.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeC_RoomG.msbt").unwrap()),
        HA_FreeD_Keyword: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_Keyword.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_Keyword.msbt").unwrap()),
        HA_FreeG_Host: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeG_Host.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeG_Host.msbt").unwrap()),
        HA_FreeB_ItemN: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_ItemN.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_ItemN.msbt").unwrap()),
        HA_FreeF_NewYear_Chinese: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear_Chinese.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_NewYear_Chinese.msbt").unwrap()),
        HA_FreeH_Progress: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeH_Progress.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeH_Progress.msbt").unwrap()),
        HA_FreeD_RumorN1: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorN1.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorN1.msbt").unwrap()),
        HA_FreeF_EarthDay: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_EarthDay.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_EarthDay.msbt").unwrap()),
        HA_FreeB_Seasons: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeB_Seasons.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeB_Seasons.msbt").unwrap()),
        HA_FreeA_AlwaysA: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_AlwaysA.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_AlwaysA.msbt").unwrap()),
        HA_FreeA_Questions: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_Questions.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_Questions.msbt").unwrap()),
        HA_FreeA_ClothesP: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_ClothesP.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_ClothesP.msbt").unwrap()),
        HA_FreeA_Week: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_Week.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_Week.msbt").unwrap()),
        HA_FreeD_RumorP_Favorite: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorP_Favorite.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_RumorP_Favorite.msbt").unwrap()),
        HA_FreeF_MayDay: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeF_MayDay.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeF_MayDay.msbt").unwrap()),
        HA_FreeA_Want: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_Want.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_Want.msbt").unwrap()),
        HA_FreeG_Visitor: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeG_Visitor.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeG_Visitor.msbt").unwrap()),
        HA_FreeE_Snpc: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeE_Snpc.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeE_Snpc.msbt").unwrap()),
        HA_FreeA_ClothesN: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeA_ClothesN.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeA_ClothesN.msbt").unwrap()),
        HA_FreeH_Transition: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeH_Transition.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeH_Transition.msbt").unwrap()),
        HA_FreeD_Moving: File::new("/TalkNNpc/B2_Ha/Free/HA_FreeD_Moving.msbt", msbts.get("/TalkNNpc/B2_Ha/Free/HA_FreeD_Moving.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc_B2_Ha {
    pub Spot: TalkNNpc_B2_Ha_Spot,
    pub Greet: TalkNNpc_B2_Ha_Greet,
    pub GreetV: TalkNNpc_B2_Ha_GreetV,
    pub React: TalkNNpc_B2_Ha_React,
    pub Quest: TalkNNpc_B2_Ha_Quest,
    pub Force: TalkNNpc_B2_Ha_Force,
    pub Conv: TalkNNpc_B2_Ha_Conv,
    pub GEvent: TalkNNpc_B2_Ha_GEvent,
    pub Approach: TalkNNpc_B2_Ha_Approach,
    pub Select: TalkNNpc_B2_Ha_Select,
    pub Free: TalkNNpc_B2_Ha_Free,
    pub HA_Connect_StandingUp: File,
    pub HA_End: File,
}

impl TalkNNpc_B2_Ha {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc_B2_Ha {
        Spot: TalkNNpc_B2_Ha_Spot::new(msbts),
        Greet: TalkNNpc_B2_Ha_Greet::new(msbts),
        GreetV: TalkNNpc_B2_Ha_GreetV::new(msbts),
        React: TalkNNpc_B2_Ha_React::new(msbts),
        Quest: TalkNNpc_B2_Ha_Quest::new(msbts),
        Force: TalkNNpc_B2_Ha_Force::new(msbts),
        Conv: TalkNNpc_B2_Ha_Conv::new(msbts),
        GEvent: TalkNNpc_B2_Ha_GEvent::new(msbts),
        Approach: TalkNNpc_B2_Ha_Approach::new(msbts),
        Select: TalkNNpc_B2_Ha_Select::new(msbts),
        Free: TalkNNpc_B2_Ha_Free::new(msbts),
        HA_Connect_StandingUp: File::new("/TalkNNpc/B2_Ha/HA_Connect_StandingUp.msbt", msbts.get("/TalkNNpc/B2_Ha/HA_Connect_StandingUp.msbt").unwrap()),
        HA_End: File::new("/TalkNNpc/B2_Ha/HA_End.msbt", msbts.get("/TalkNNpc/B2_Ha/HA_End.msbt").unwrap()),
        }
    }
}

pub struct TalkNNpc {
    pub G2_Ge: TalkNNpc_G2_Ge,
    pub G3_Ot: TalkNNpc_G3_Ot,
    pub G1_Fu: TalkNNpc_G1_Fu,
    pub B3_Ko: TalkNNpc_B3_Ko,
    pub G4_An: TalkNNpc_G4_An,
    pub B4_Zk: TalkNNpc_B4_Zk,
    pub B1_Bo: TalkNNpc_B1_Bo,
    pub B2_Ha: TalkNNpc_B2_Ha,
}

impl TalkNNpc {
    pub fn new(msbts: &HashMap<std::string::String, crate::binary::msbt::MSBT>) -> Self {
        TalkNNpc {
        G2_Ge: TalkNNpc_G2_Ge::new(msbts),
        G3_Ot: TalkNNpc_G3_Ot::new(msbts),
        G1_Fu: TalkNNpc_G1_Fu::new(msbts),
        B3_Ko: TalkNNpc_B3_Ko::new(msbts),
        G4_An: TalkNNpc_G4_An::new(msbts),
        B4_Zk: TalkNNpc_B4_Zk::new(msbts),
        B1_Bo: TalkNNpc_B1_Bo::new(msbts),
        B2_Ha: TalkNNpc_B2_Ha::new(msbts),
        }
    }
}