errs <- NULL
errf <- NULL

for (i in 32:nrow(offense)) {
  prev_year <- offense[i-32,]
  curr_year <- offense[i,]
  
  exp_td <- ( betas2_0 + betas2_play*prev_year$playoff + betas2_td*sqrt(prev_year$td_per_drive) + betas2_year*curr_year$year ) ** 2
  errs <- append(exp_td-curr_year$td_per_game,errs)
  
  exp_fg <- ( betasf2_0 + betasf2_play*prev_year$playoff + betasf2_fg*sqrt(prev_year$fg_per_drive) + betasf2_year*curr_year$year ) ** 2
  errf <- append(exp_fg-curr_year$fg_per_game,errf)
}

hist(errs)
print(mean(errs))
print(sd(errs))

hist(errf)
print(mean(errf))
print(sd(errf))

hist(errs)
hist(errf)
