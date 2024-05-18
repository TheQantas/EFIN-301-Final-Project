library(dplyr)

int_vals <- NULL
td_vals <- NULL
play_vals <- NULL
year_vals <- NULL

int_valsf <- NULL
td_valsf <- NULL
play_valsf <- NULL
year_valsf <- NULL

r_vals <- NULL
ar_vals <- NULL

for (team_id in team_ids) {
  team_offense <- offense %>% filter(team==team_id)
  
  auto_model <- lm(
    sqrt(td_per_drive)~lag(sqrt(td_per_drive))+lag(playoff)+year,
    data=team_offense
  ) %>% summary()
  
  auto_modelf <- lm(
    sqrt(fg_per_drive)~lag(sqrt(fg_per_drive))+lag(playoff)+year,
    data=team_offense
  ) %>% summary()
  
  print(mean(auto_model$residuals))
  print(mean(auto_modelf$residuals))
  
  sel = 'Estimate'
  # sel = 'Pr(>|t|)'
  
  int_vals <- append(auto_model$coefficients[1,sel],int_vals)
  td_vals <- append(auto_model$coefficients[2,sel],td_vals)
  play_vals <- append(auto_model$coefficients[3,sel],play_vals)
  year_vals <- append(auto_model$coefficients[4,sel],year_vals)
  
  int_valsf <- append(auto_modelf$coefficients[1,sel],int_valsf)
  td_valsf <- append(auto_modelf$coefficients[2,sel],td_valsf)
  play_valsf <- append(auto_modelf$coefficients[3,sel],play_valsf)
  year_valsf <- append(auto_modelf$coefficients[4,sel],year_valsf)
  
  r_vals <- append(auto_model$r.squared,r_vals)
  ar_vals <- append(auto_model$adj.r.squared,ar_vals)
}

betas2_0 <- mean(int_vals)
betas2_td <-  mean(td_vals)
betas2_play <- mean(play_vals)
betas2_year <- mean(year_vals)

betasf2_0 <- mean(int_valsf)
betasf2_fg <-  mean(td_valsf)
betasf2_play <- mean(play_valsf)
betasf2_year <- mean(year_valsf)

print( mean(r_vals) )
print( mean(ar_vals) )