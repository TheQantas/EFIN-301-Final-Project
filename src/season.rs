use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

pub type SeasonId = (String,i16);
pub type SeasonMap = HashMap<SeasonId,Season>;

pub struct Season {
    team: String,
    year: i16,

    touchdown: i16, //scoring
    field_goal: i16, //scoring
    punt: i16, //punt
    downs: i16, //turnover
    safety: i16, //safety
    turnover: i16, //turnover
    end_of_period: i16, //eop
    missed_field_goal: i16, //turnover
    blocked_punt: i16, //turnover
    blocked_field_goal: i16, //turnover

    playoff: i16
}
impl Season {
    pub fn get_team(&self) -> &str {
        return &self.team;
    }
    pub fn get_year(&self) -> i16 {
        return self.year;
    }
    fn get_games_in_season(year: i16,team: &str) -> i8 {
        return if year <= 2020 {
            16
        } else if year == 2022 && (team == "BUF" || team == "CIN") {
            16
        } else {
            17
        }
    }
    pub fn get_game_count(&self) -> i8 {
        return Self::get_games_in_season(self.year,&self.team);
    }

    #[inline]
    pub fn get_touchdown(&self) -> i16 {
        return self.touchdown;
    }
    #[inline]
    pub fn get_field_goal(&self) -> i16 {
        return self.field_goal;
    }
    #[inline]
    pub fn get_safety(&self) -> i16 {
        return self.safety;
    }
    #[inline]
    pub fn get_turnover(&self) -> i16 {
        return self.turnover + self.missed_field_goal + self.blocked_field_goal + self.blocked_punt + self.downs;
    }
    #[inline]
    pub fn get_eop(&self) -> i16 {
        return self.end_of_period;
    }
    #[inline]
    pub fn get_punts(&self) -> i16 {
        return self.punt;
    }
    #[inline]
    pub fn get_total(&self) -> i16 {
        return self.get_touchdown() + self.get_field_goal() + self.get_safety() + self.get_turnover() +
        self.get_eop() + self.get_punts();
    }
    
    pub fn get_zero_count(&self) -> i8 {
        return
            (if self.touchdown==0 { 1 } else { 0 }) +
            (if self.field_goal==0 { 1 } else { 0 }) +
            (if self.punt==0 { 1 } else { 0 }) +
            (if self.downs==0 { 1 } else { 0 }) +
            (if self.safety==0 { 1 } else { 0 }) +
            (if self.turnover==0 { 1 } else { 0 }) +
            (if self.end_of_period==0 { 1 } else { 0 }) +
            (if self.missed_field_goal==0 { 1 } else { 0 }) +
            (if self.blocked_field_goal==0 { 1 } else { 0 }) +
            (if self.blocked_punt==0 { 1 } else { 0 });
    }

    #[inline]
    pub fn get_touchdown_perc(&self) -> f64 {
        return (self.get_touchdown() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_field_goal_perc(&self) -> f64 {
        return (self.get_field_goal() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_safety_perc(&self) -> f64 {
        return (self.get_safety() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_turnover_perc(&self) -> f64 {
        return (self.get_turnover() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_eop_perc(&self) -> f64 {
        return (self.get_eop() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_punt_perc(&self) -> f64 {
        return (self.get_punts() as f64) / (self.get_total() as f64);
    }
    #[inline]
    pub fn get_ending_perc(&self) -> f64 {
        return ((self.get_punts() + self.get_turnover()) as f64) / (self.get_total() as f64); 
    }

    pub fn get_distribution_old(&self) -> [f64;5] {
        return [
            self.get_touchdown_perc(),
            self.get_field_goal_perc(),
            self.get_safety_perc(),
            self.get_ending_perc(),
            // self.get_turnover_perc(),
            // self.get_punt_perc(),
            self.get_eop_perc(),
        ];
    }
    pub fn get_distribution(&self) -> [f64;4] {
        return [
            self.get_touchdown_perc(),
            self.get_field_goal_perc(),
            self.get_safety_perc() + self.get_ending_perc(),
            // self.get_ending_perc(),
            // self.get_turnover_perc(),
            // self.get_punt_perc(),
            self.get_eop_perc(),
        ];
    }


    pub fn predict_off_distr(&self) -> [f64;4] {
        let td_perc = (
            -2.7829883
            + 0.0145582 * (self.playoff as f64)
            + 0.3843043 * self.get_touchdown_perc().sqrt()
            + 0.0015158 * (self.year as f64 + 1.0)
        ).powf(2.0);
        let fg_perc = 0.0013130 * (self.year as f64 + 1.0) - 2.5040872;
        // let total = self.get_total() as f64;
        // let games_next_season = Self::get_games_in_season(self.year+1,&self.team) as f64;
        // let td_perc = tds_per_game * games_next_season / total;
        // let fg_perc = fgs_per_game * games_next_season / total;
        let ending_perc = self.get_eop_perc();
        return [
            td_perc,
            fg_perc,
            1.0 - td_perc - fg_perc - ending_perc,
            ending_perc
        ];
    }
    pub fn predict_def_distr(&self) -> [f64;4] {
        let td_perc = (
            -3.1289204
            // + 0.0145582 * (self.playoff as f64)
            + 0.3245221 * self.get_touchdown_perc().sqrt()
            + 0.0017040 * (self.year as f64 + 1.0)
        ).powi(2);
        let fg_perc = (
            -3.1580175
            + -0.0081590 * (self.playoff as f64)
            + 0.0017545 * (self.year as f64 + 1.0)
        ).powi(2);
        // let total = self.get_total() as f64;
        // let games_next_season = Self::get_games_in_season(self.year+1,&self.team) as f64;
        // let td_perc = tds_per_game * games_next_season / total;
        // let fg_perc = fgs_per_game * games_next_season / total;
        let ending_perc = self.get_eop_perc();
        return [
            td_perc,
            fg_perc,
            1.0 - td_perc - fg_perc - ending_perc,
            ending_perc
        ];
    }
    
}

// pub fn read_in_offenses() -> Result<SeasonMap,Box<dyn Error>> {
//     let file = File::open(&"data/drives.csv")?;
//     let mut reader = csv::Reader::from_reader(file);
//     let mut season_map: SeasonMap = HashMap::new();

//     for result in reader.records() {
//         let result = result.unwrap();
//         let team_id = String::from( result.get(0).unwrap() );
//         let year = (result.get(1).unwrap().parse() as Result<i16,_>).unwrap();
//         let touchdown = (result.get(2).unwrap().parse() as Result<i16,_>).unwrap();
//         let field_goal = (result.get(3).unwrap().parse() as Result<i16,_>).unwrap();
//         let punt = (result.get(4).unwrap().parse() as Result<i16,_>).unwrap();
//         let downs = (result.get(5).unwrap().parse() as Result<i16,_>).unwrap();
//         let safety = (result.get(6).unwrap().parse() as Result<i16,_>).unwrap();
//         let interception = (result.get(7).unwrap().parse() as Result<i16,_>).unwrap();
//         let fumble = (result.get(8).unwrap().parse() as Result<i16,_>).unwrap();
//         let end_of_half = (result.get(9).unwrap().parse() as Result<i16,_>).unwrap();
//         let end_of_game = (result.get(10).unwrap().parse() as Result<i16,_>).unwrap();
//         let missed_field_goal = (result.get(11).unwrap().parse() as Result<i16,_>).unwrap();
//         let blocked_punt = (result.get(12).unwrap().parse() as Result<i16,_>).unwrap();
//         let blocked_field_goal = (result.get(13).unwrap().parse() as Result<i16,_>).unwrap();
//         let playoff = (result.get(16).unwrap().parse() as Result<i16,_>).unwrap()==1;

//         season_map.insert((team_id.clone(),year),Season {
//             team: team_id,
//             year,
//             touchdown,
//             field_goal,
//             punt,
//             downs,
//             safety,
//             turnover: interception + fumble,
//             end_of_period: end_of_game + end_of_half,
//             missed_field_goal,
//             blocked_punt,
//             blocked_field_goal,
//             playoff
//         });
//     }

//     return Ok( season_map );
// }

pub fn read_in_offenses() -> Result<SeasonMap,Box<dyn Error>> {
    return read_in_teams(true);
}

pub fn read_in_defenses() -> Result<SeasonMap,Box<dyn Error>> {
    return read_in_teams(false);
}

fn read_in_teams(is_offense: bool) -> Result<SeasonMap,Box<dyn Error>> {
    let file = File::open(if is_offense {
        &"data/drives_offense.csv"
    } else {
        &"data/drives_defense.csv"
    })?;
    let mut reader = csv::Reader::from_reader(file);
    let mut season_map: SeasonMap = HashMap::new();

    //0-team
    //1-year
    //2-touchdown
    //3-fieldgoal
    //4-missedfg
    //5-punt
    //6-turnovers
    //7-safety
    //8-total
    //9-endofperiod
    //10-blockedpunt
    //11-downs
    //12-blockedfg
    //13-playoff

    for result in reader.records() {
        let result = result.unwrap();
        let team_id = String::from( result.get(0).unwrap() );
        let year = (result.get(1).unwrap().parse() as Result<i16,_>).unwrap();
        let touchdown = (result.get(2).unwrap().parse() as Result<i16,_>).unwrap();
        let field_goal = (result.get(3).unwrap().parse() as Result<i16,_>).unwrap();
        let missed_field_goal = (result.get(4).unwrap().parse() as Result<i16,_>).unwrap();
        let punt = (result.get(5).unwrap().parse() as Result<i16,_>).unwrap();
        let turnover = (result.get(6).unwrap().parse() as Result<i16,_>).unwrap();
        let safety = (result.get(7).unwrap().parse() as Result<i16,_>).unwrap();
        //8-total
        let end_of_period = (result.get(9).unwrap().parse() as Result<i16,_>).unwrap();
        let blocked_punt = (result.get(10).unwrap().parse() as Result<i16,_>).unwrap();
        let downs = (result.get(11).unwrap().parse() as Result<i16,_>).unwrap();
        let blocked_field_goal = (result.get(12).unwrap().parse() as Result<i16,_>).unwrap();
        let playoff = (result.get(13).unwrap().parse() as Result<i16,_>).unwrap();

        season_map.insert((team_id.clone(),year),Season {
            team: team_id,
            year,
            touchdown,
            field_goal,
            punt,
            downs,
            safety,
            turnover,
            end_of_period,
            missed_field_goal,
            blocked_punt,
            blocked_field_goal,
            playoff
        });
    }

    return Ok( season_map );
}

