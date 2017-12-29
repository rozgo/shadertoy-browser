
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    #[serde(rename = "Shader")]
    pub shader: Shader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shader {
    pub ver: String,
    pub info: ShaderInfo,
    pub renderpass: Vec<RenderPass>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShaderInfo {
    pub id: String,
    pub date: String,
    pub viewed: u64,
    pub name: String,
    pub username: String,
    pub description: String,
    pub likes: u64,
    pub published: u64,
    pub flags: u64,
    pub tags: Vec<String>,
    pub hasliked: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderPass {
    pub inputs: Vec<RenderPassInput>,
    pub outputs: Vec<RenderPassOutput>,
    pub code: String,
    pub name: String,
    pub description: String,

    #[serde(rename = "type")]
    pub pass_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderPassInput {
    pub id: u64,
    pub src: String,
    pub ctype: String,
    pub channel: u64,
    pub sampler: Sampler,
    pub published: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderPassOutput {
    pub id: u64,
    pub channel: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sampler {
    pub filter: String,
    pub wrap: String,
    pub vflip: String,
    pub srgb: String,
    //pub vflip: bool,
    //pub srgb: bool,
    pub internal: String,
}