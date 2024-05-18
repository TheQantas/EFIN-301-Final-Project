use std::error::Error;
use std::fs::File;

use crate::season::SeasonId;

#[derive(Debug,Clone)]
pub struct Matchup {
    year: i16,
    month: i16,
    date: i16,

    away: String,
    home: String,
    away_score: i16,
    home_score: i16,

    away_odds: f64,
    home_odds: f64,
    home_line: f64,
    over_under: f64
}
impl Matchup {
    pub fn get_season(&self) -> i16 {
        return if self.month < 4 {
            self.year - 1
        } else {
            self.year
        }
    }

    pub fn get_away_id(&self) -> SeasonId {
        return (self.away.clone(),self.get_season());
    }
    pub fn get_home_id(&self) -> SeasonId {
        return (self.home.clone(),self.get_season());
    }
    pub fn get_away_id_prev(&self) -> SeasonId {
        return (self.away.clone(),self.get_season()-1);
    }
    pub fn get_home_id_prev(&self) -> SeasonId {
        return (self.home.clone(),self.get_season()-1);
    }

    pub fn get_away_score(&self) -> i16 {
        return self.away_score;
    }
    pub fn get_home_score(&self) -> i16 {
        return self.home_score;
    }
}

pub fn get_matchups() -> Result<Vec<Matchup>,Box<dyn Error>> {
    let file = File::open(&"data/odds.csv")?;
    let mut reader = csv::Reader::from_reader(file);

    let mut matchups: Vec<Matchup> = vec![];
    for result in reader.records() {
        let result = result.unwrap();

        let mut date = result.get(0).unwrap().split("-");
        let year = ( date.next().unwrap().parse() as Result<i16,_> ).unwrap();
        let month = ( date.next().unwrap().parse() as Result<i16,_> ).unwrap();
        let date = ( date.next().unwrap().parse() as Result<i16,_> ).unwrap();

        let home = String::from( result.get(1).unwrap() );
        let away = String::from( result.get(2).unwrap() );
        let home_score = ( result.get(3).unwrap().parse() as Result<i16,_> ).unwrap();
        let away_score = ( result.get(4).unwrap().parse() as Result<i16,_> ).unwrap();
        //overtime - 5
        //playoff - 6
        //netural - 7
        let home_odds = ( result.get(8).unwrap().parse() as Result<f64,_> ).unwrap();
        let away_odds = ( result.get(9).unwrap().parse() as Result<f64,_> ).unwrap();
        let home_line = ( result.get(10).unwrap().parse() as Result<f64,_> ).unwrap();
        let over_under = ( result.get(11).unwrap().parse() as Result<f64,_> ).unwrap();

        matchups.push(Matchup {
            year,
            month,
            date,
            away,
            home,
            away_score,
            home_score,
            home_odds,
            away_odds,
            home_line,
            over_under
        });
    }

    return Ok( matchups );
}