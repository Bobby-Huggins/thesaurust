use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonyms {
    pub meta: Meta,
    hwi: Hwi,
    pub fl: String,
    def: Vec<Def>,
    pub shortdef: Vec<String>,
    sls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Def {
    sseq: Vec<Vec<Vec<SseqElement>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SseqClass {
    sn: Option<String>,
    dt: Vec<Vec<DtUnion>>,
    syn_list: Option<Vec<Vec<SimListElement>>>,
    rel_list: Option<Vec<Vec<AntListElement>>>,
    near_list: Option<Vec<Vec<AntListElement>>>,
    ant_list: Option<Vec<Vec<AntListElement>>>,
    phrase_list: Option<Vec<Vec<PhraseList>>>,
    sim_list: Option<Vec<Vec<SimListElement>>>,
    opp_list: Option<Vec<Vec<AntListElement>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AntListElement {
    wd: String,
    wvrs: Option<Vec<Wvr>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wvr {
    wvl: Wvl,
    wva: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtClass {
    t: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhraseList {
    wd: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimListElement {
    wd: String,
    wvrs: Option<Vec<Wvr>>,
    wsls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hwi {
    hw: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub id: String,
    uuid: String,
    src: String,
    section: String,
    target: Option<Target>,
    stems: Vec<String>,
    pub syns: Vec<Vec<String>>,
    pub ants: Vec<Vec<String>>,
    pub offensive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    tuuid: String,
    tsrc: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SseqElement {
    Enum(SseqEnum),
    SseqClass(SseqClass),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DtUnion {
    DtClassArray(Vec<DtClass>),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Wvl {
    #[serde(rename = "also")]
    Also,
    #[serde(rename = "or")]
    Or,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SseqEnum {
    #[serde(rename = "sense")]
    Sense,
}

