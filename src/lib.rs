// Github search
//

use crate::engines::github::Github;
use crate::engines::Buildrequest;
use crate::types::ProfileData;
mod engines;
mod types;

pub struct ProfileRequest {
    username: String,
    email: String,
}

impl ProfileRequest {
    pub fn new(user_email: String) -> Self {
        //TODO: Email validator
        let username = match user_email.clone().split("@").nth(0) {
            Some(sub_str) => sub_str.to_owned(),
            None => panic!("Failed to parse Email"), //.unwrap().to_owned()
        };
        Self {
            username,
            email: user_email,
        }
    }

    pub async fn build(&self) -> Result<String, anyhow::Error> {
        let mut response = ProfileData::new();

        let github = Github::new(&self.email);
        // build all data and error handle
        self.run_request(&github, &mut response).await.unwrap();

        println!("running github engine rn  {:?}", &github);
        let json_response = serde_json::to_string(&response)?;
        Ok(json_response)
    }

    async fn run_request(
        &self,
        engine: &impl Buildrequest,
        response: &mut ProfileData,
    ) -> Result<(), anyhow::Error> {
        let response = engine.search(response).await?;

        println!("{:?}", response);

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_new_request_works() {
        let user_email = String::from("user@gmail.com");
        let profile = ProfileRequest::new(user_email);
        assert_eq!(profile.username, String::from("user"));
    }
}
