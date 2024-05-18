err <- NULL

for (i in 32:nrow(offense)) {
  prev_year <- offense[i-32,]
  curr_year <- offense[i,]
  
  exp_td <- beta_0 + beta_playoff*prev_year$playoff + beta_td*prev_year$touchdown + beta_year*curr_year$year
  err <- append(exp_td-curr_year$touchdown,err)
}

hist(err)
print(mean(err))
print(sd(err))
