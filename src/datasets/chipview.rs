use crate::{components::channel, util::tokens_util::channel_names};

pub fn channel_name_from_number(soundchip: &str, n: usize, ignore_dpcm:bool) -> Result<String, String> {

    match soundchip {
        "VRC6" | "2A03" => {
            let mut channels = channel_names(); 
            let dpcm = &"DPCM".to_string();
            if ignore_dpcm && channels.contains(dpcm){
                channels.retain(|x| x!=dpcm);
            }
            if let Some(channel_name) = channels.get(n){
                Ok(channel_name.clone())
            }else{
                Err(format!("Index out of range: {} channels count: {}", n,channels.len()))
            }
        }
        _ => Err(format!("The soundchip {} is not implemented", soundchip)),
    }
}

pub trait TagOps{
    fn into_tag(&self, start_token:bool)->String;
}

impl TagOps for String{
    fn into_tag(&self, start_token:bool)->String{
        let mut sb = String::new();
        sb.push_str("<");
        sb.push_str(self.as_str());
        if start_token{
            sb.push_str("Start");
        }else{
            sb.push_str("End");
        }
        sb.push_str(">");
        sb
    }
}
