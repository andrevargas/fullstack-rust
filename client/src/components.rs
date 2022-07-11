use reqwasm::http::Request;
use shared::models::Tweet;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties};

#[function_component(App)]
pub fn app() -> Html {
    let tweets = use_state(|| vec![]);

    {
        let tweets = tweets.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_tweets: Vec<Tweet> = Request::get("/api/tweets")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    tweets.set(fetched_tweets);
                });
                || ()
            },
            (),
        );
    }

    html! {
      <>
        <h1>{ "Tweets" }</h1>
        <TweetsList tweets={(*tweets).clone()} />
      </>
    }
}

#[derive(Properties, PartialEq)]
struct TweetsListProps {
    tweets: Vec<Tweet>,
}

#[function_component(TweetsList)]
fn tweets_list(TweetsListProps { tweets }: &TweetsListProps) -> Html {
    tweets
        .iter()
        .map(|tweet| {
            html! {
              <p>{tweet.content.to_string()}</p>
            }
        })
        .collect()
}
