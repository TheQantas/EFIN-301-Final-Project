library(dplyr)

static <- read.csv("data/sim_static.csv")
dynamic <- read.csv("data/sim_dynamic.csv")

odds <- read.csv("data/odds.csv")
odds$total <- 1/odds$away_odds + 1/odds$home_odds

offense <- read.csv("data/drives_offense.csv")
defense <- read.csv("data/drives_defense.csv")

offense$game_count <- ifelse(
  offense$year<=2020,
  16,
  ifelse(offense$year==2022&(offense$team=='BUF'|offense$team=='CIN'),16,17)
)
offense$td_per_game <- offense$touchdown / offense$game_count
offense$fg_per_game <- offense$field_goal / offense$game_count
offense$td_per_drive <- offense$touchdown / offense$total
offense$fg_per_drive <- offense$field_goal / offense$total

defense$game_count <- ifelse(
  defense$year<=2020,
  16,
  ifelse(defense$year==2022&(defense$team=='BUF'|defense$team=='CIN'),16,17)
)
defense$td_per_game <- defense$touchdown / defense$game_count
defense$fg_per_game <- defense$field_goal / defense$game_count
defense$td_per_drive <- defense$touchdown / defense$total
defense$fg_per_drive <- defense$field_goal / defense$total

offense_by_year <- offense %>%
  group_by(year) %>%
  summarise_at(vars(-team), mean, na.rm = TRUE) %>%
  ungroup()
defense_by_year <- defense %>%
  group_by(year) %>%
  summarise_at(vars(-team), mean, na.rm = TRUE) %>%
  ungroup()

offense$lag_td_per_drive <- lag(offense$td_per_drive,32)
offense$lag_fg_per_drive <- lag(offense$fg_per_drive,32)
offense$lag_playoff <- lag(offense$playoff,32)
defense$lag_td_per_drive <- lag(defense$td_per_drive,32)
defense$lag_fg_per_drive <- lag(defense$fg_per_drive,32)
defense$lag_playoff <- lag(defense$playoff,32)

# offense_by_year$game_count <- c(16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,17,17,17)

team_ids <- unique(offense$team)
