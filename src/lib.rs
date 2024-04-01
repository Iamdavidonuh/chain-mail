// Github search
//

use crate::engines::github::Github;
use crate::engines::Buildrequest;
mod engines;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ProfileData;

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

    pub async fn build(&self) {
        let github = Github::new(&self.email);
        // build all data and error handle
        let mut response = self.run_request(&github).await.unwrap();

        println!("running github engine rn  {:?}", &github);
    }

    async fn run_request<T: Buildrequest>(&self, engine: &T) -> Result<(), anyhow::Error>
    where
        <T as Buildrequest>::Item: std::fmt::Debug,
    {
        let response = engine.search().await?;

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
