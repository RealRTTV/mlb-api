# mlb-api

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/mlb-api">
    <img src="https://img.shields.io/crates/v/mlb-api.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/mlb-api">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/mlb-api">
    <img src="https://img.shields.io/crates/d/mlb-api.svg?style=flat-square"
      alt="Crates.io downloads" />
  </a>
</div>

## The Rust MLB API Wrapper

This project and its author are not affiliated with MLB or any MLB team. This crate wraps the existing MLB Stats API, which is subject to the notice posted at <http://gdx.mlb.com/components/copyright.txt>.

### Usage
Endpoints are most commonly used using their module's builder functions.
```rust
use mlb_api::sport::SportId;
use mlb_api::request::RequestUrlBuilderExt;
use mlb_api::schedule::{self, ScheduleResponse, ScheduleDate};

let response: ScheduleResponse = schedule::request()
    .sport_id(SportId::MLB)
    .build_and_get()
    .await?;

let [date]: [ScheduleDate; 1] = response.dates.try_into()?;
```

Play Streams are the recommended way to process live games
```rust
use mlb_api::game::PlayStream;

let game: ScheduleGame = ...;

PlayStream::new(game.game_id).run(|event, _meta, _data| { ... }).await?;
```

Use [`single_stat!`](crate::single_stat) for simple stats requests and make your own hydrations for more complicated requests
```rust
use mlb_api::single_stat;
use mlb_api::person::{self, PeopleResonse};

let season_hitting = single_stat!( Season + Hitting for 660_271 ).await?;
let sabermetrics_pitching = single_stat!( Sabermetrics + Pitching for 660_271; with |builder| builder.season(2024) ).await?;

person_hydrations! {
    struct PersonDisplayHydrations {
        nicknames,
        stats: { [Season, Sabermetrics] = [Hitting, Pitching] },
    }
}

let response: PeopleResponse = person::request_with_hydrations::<PersonDisplayHydrations>(660_271).await?;
```

### Endpoints
This API contains wrappers / bindings for all known public MLB API endpoints (unless incomplete), the table of which can be seen below.
Additional information can be found at <https://github.com/toddrob99/MLB-StatsAPI/wiki/Endpoints> (thanks Todd Roberts)

Stars hightlight the most popular and used endpoints

| Endpoint                   | Description                                      |
|----------------------------|--------------------------------------------------|
| [`attendance`]             | Team attendance data by season                   |
| [`awards`]                 | List of all awards, Cy Young, MVP, etc.          |
| [`conference`]             | Conferences, like divisions but not              |
| [`division`]               | Names, has a wildcard or not, playoff teams      |
| [`draft`]                  | Draft rounds, players, etc.                      |
| [`live_feed`] ⭐           | [`boxscore`], [`linescore`], [`plays`], and misc |
| [`diff_patch`]             | JSON diff patch for live feed                    |
| [`timestamps`]             | List of timestamps for game plays & play events  |
| [`changes`]                | Games that underwent scheduling changes (?)      |
| [`context_metrics`]        | Various metrics for game plays & play events     |
| [`win_probability`]        | Win Probability calculations for games           |
| [`boxscore`]               | Boxscore summary for game, includes stats        |
| [`content`]                | Editorial content regarding games                |
| [`linescore`]              | Linescore for games                              |
| [`plays`]                  | Play by Play Data on a game                      |
| [`uniforms`]               | Game Uniforms                                    |
| [`pace`]                   | Average game durations and league stat totals    |
| [`home_run_derby`]         | Home Run Derby stats                             |
| [`league`]                 | League data, AL, NL, divisions contained, etc.   |
| [`all_star`]               | ASG data                                         |
| [`person`] ⭐              | A person, lots of data here                      |
| [`free_agents`]            | Free agents in any given year                    |
| [`person_stats`]           | Player stats for a single game                   |
| [`jobs`]                   | List of all people with a job, ex: scorer, ump   |
| [`jobs::umpire`]           | List of all umpires                              |
| [`jobs::datacasters`]      | List of all datacasters                          |
| [`jobs::official_scorers`] | List of all official scorers                     |
| [`schedule`] ⭐            | All games played within a certain date range     |
| [`schedule::tied`]         | All games that ended tied (?)                    |
| [`schedule::postseason`]   | Postseason schedule                              |
| [`schedule::postseason::series`] | Postseason series schedule                 |
| [`season`]                 | Date ranges for season states: reg, post, spring |
| [`sport`]                  | List of sports, MLB, AAA, AA, A+, etc.           |
| [`players`] ⭐             | List of all players in a sport and season        |
| [`standings`]              | Standings information for teams                  |
| [`stats`]                  | Stats data                                       |
| [`stats::leaders`]         | League leaders in specific stats                 |
| [`team`] ⭐                | Team data                                        |
| [`team::history`]          | History of a team, such as Brooklyn & LA Dodgers |
| [`team::stats`]            | Stats for a team                                 |
| [`team::affiliates`]       | Minor league affiliate teams                     |
| [`team::alumni`]           | Alumni for a team                                |
| [`team::coaches`]          | List of coaches on a team                        |
| [`team::personnel`]        | Personnel on a team                              |
| [`team::leaders`]          | Stat leaders per team                            |
| [`team::roster`]           | Roster players on a team                         |
| [`team::uniforms`]         | Uniforms a team wears                            |
| [`transactions`]           | Trades, IL moves, etc.                           |
| [`venue`]                  | Yankee Stadium, Dodger Stadium, Fenway Park, etc.|
| [`meta`]                   | Metadata endpoints, typically cached or enums    |

### Usage & Appendix
1. This API defaults to using `reqwest` and `tokio` for non-blocking IO, there is a `ureq` feature to switch to `ureq` and `parking_lot` for blocking IO.
3. Use [`PlayStream`](crate::game::PlayStream) for obtaining live updates on games.
4. Use [`single_stat!`](crate::single_stat) for simple stat requests rather than making [`person_hydrations!`] and [`PersonRequest`](crate::person::PersonRequest) yourself.
5. Use [`as_complete_or_request`](crate::cache::RequestableEntrypoint::as_complete_or_request) and the numerous `crate::*_hydrations!` items to obtain additional information in requests, try to minimize request quantity.
6. The [`precache`](crate::cache::precache) function allows caching commonly requested values before usage to make [`as_complete_or_request`](crate::cache::RequestableEntrypoint::as_complete_or_request) faster to use.
7. Supply [`SeasonId`](crate::season::SeasonId)s to requests to not have them break when the year changes.

[`attendance`]: crate::requests::attendance
[`awards`]: crate::requests::awards
[`conference`]: crate::requests::conference
[`division`]: crate::requests::division
[`draft`]: crate::requests::draft
[`live_feed`]: crate::requests::game::live_feed
[`diff_patch`]: crate::requests::game::diff
[`timestamps`]: crate::requests::game::timestamps
[`changes`]: crate::requests::game::changes
[`context_metrics`]: crate::requests::game::context_metrics
[`win_probability`]: crate::requests::game::win_probability
[`boxscore`]: crate::requests::game::boxscore
[`content`]: crate::requests::game::content
[`linescore`]: crate::requests::game::linescore
[`plays`]: crate::requests::game::plays
[`uniforms`]: crate::requests::game::uniforms
[`pace`]: crate::requests::game::pace
[`home_run_derby`]: crate::requests::home_run_derby
[`league`]: crate::requests::league
[`all_star`]: crate::requests::all_star
[`person`]: crate::requests::person
[`free_agents`]: crate::requests::person::free_agents
[`person_stats`]: crate::requests::person::stats
[`jobs`]: crate::requests::jobs
[`jobs::umpire`]: crate::requests::jobs::umpire
[`jobs::datacasters`]: crate::requests::jobs::datacasters
[`jobs::official_scorers`]: crate::requests::jobs::official_scorers
[`schedule`]: crate::requests::schedule
[`schedule::tied`]: crate::requests::schedule::tied
[`schedule::postseason`]: crate::requests::schedule::postseason
[`schedule::postseason::series`]: crate::requests::schedule::postseason::series
[`season`]: crate::requests::season
[`sport`]: crate::requests::sport
[`players`]: crate::requests::person::players
[`standings`]: crate::requests::standings
[`stats`]: crate::requests::stats
[`stats::leaders`]: crate::requests::stats::leaders
[`team`]: crate::requests::team
[`team::history`]: crate::requests::team::history
[`team::stats`]: crate::requests::team::stats
[`team::affiliates`]: crate::requests::team::affiliates
[`team::alumni`]: crate::requests::team::alumni
[`team::coaches`]: crate::requests::team::coaches
[`team::personnel`]: crate::requests::team::personnel
[`team::leaders`]: crate::requests::team::leaders
[`team::roster`]: crate::requests::team::roster
[`team::uniforms`]: crate::requests::team::uniforms
[`transactions`]: crate::requests::transactions
[`venue`]: crate::requests::venue
[`meta`]: crate::requests::meta

License: MIT
