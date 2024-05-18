library(dplyr)
library(sandwich)
library(lmtest)

signif_z <- NULL
signif_a <- NULL
signif_b <- NULL
signif_c <- NULL
signif_r <- NULL

for (team_id in team_ids) {
  team_offense <- offense %>% filter(team==team_id)
  
  reg_td_model <- lm(touchdown~lag(touchdown)+year+lag(playoff),data=team_offense) %>% summary()
  
  p_value_int <- reg_td_model$coefficients[1,'Estimate']
  # p_value_lag <- reg_td_model$coefficients[2,'Pr(>|t|)']
  p_value_lag <- reg_td_model$coefficients[2,'Estimate']
  # p_value_time <- reg_td_model$coefficients[3,'Pr(>|t|)']
  p_value_time <- reg_td_model$coefficients[3,'Estimate']
  # p_value_lag2 <- reg_td_model$coefficients[4,'Pr(>|t|)']
  p_value_lag2 <- reg_td_model$coefficients[4,'Estimate']
  r_squared <- reg_td_model$r.squared
  
  signif_z <- append(p_value_int,signif_z)
  signif_a <- append(p_value_lag,signif_a)
  signif_b <- append(p_value_time,signif_b)
  signif_c <- append(p_value_lag2,signif_c)
  signif_r <- append(r_squared,signif_r)
}

beta_0 <- mean(signif_z)
beta_td <- mean(signif_a)
beta_year <- mean(signif_b)
beta_playoff <- mean(signif_c)
# print( mean(signif_r) )


